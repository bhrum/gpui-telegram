use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

/// Multi-account / multi-member routing strategy modeled after Magpie.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutingStrategy {
    #[default]
    Smart,
    Order,
    Rotate,
    Usage,
    Pace,
    Cheapest,
    Drain,
}

impl RoutingStrategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Smart => "smart",
            Self::Order => "order",
            Self::Rotate => "rotate",
            Self::Usage => "usage",
            Self::Pace => "pace",
            Self::Cheapest => "cheapest",
            Self::Drain => "drain",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().replace('-', "_").as_str() {
            "smart" => Some(Self::Smart),
            "order" | "ordered" => Some(Self::Order),
            "rotate" | "round_robin" => Some(Self::Rotate),
            "usage" | "least_used" => Some(Self::Usage),
            "pace" => Some(Self::Pace),
            "cheapest" | "cheap" => Some(Self::Cheapest),
            "drain" => Some(Self::Drain),
            _ => None,
        }
    }

    pub fn all() -> &'static [RoutingStrategy] {
        &[
            Self::Smart,
            Self::Order,
            Self::Rotate,
            Self::Usage,
            Self::Pace,
            Self::Cheapest,
            Self::Drain,
        ]
    }
}

fn default_weight() -> u32 {
    1
}

fn default_true() -> bool {
    true
}

fn default_sticky_ttl_ms() -> i64 {
    300_000
}

/// One candidate target inside a `RoutingGroup`.
/// `target` may be `"<provider_id>/<model>"`, `"<provider_id>"`, or `"group/<nested_id>"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutingGroupMember {
    pub target: String,
    #[serde(default = "default_weight")]
    pub weight: u32,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_cost_per_1m: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_cost_per_1m: Option<f64>,
    #[serde(default)]
    pub free: bool,
}

impl RoutingGroupMember {
    pub fn new(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            weight: 1,
            enabled: true,
            input_cost_per_1m: None,
            output_cost_per_1m: None,
            free: false,
        }
    }
}

/// Virtual model group addressable via `group/<id>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutingGroup {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub strategy: RoutingStrategy,
    #[serde(default)]
    pub members: Vec<RoutingGroupMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent_fast_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent_deep_model: Option<String>,
    #[serde(default = "default_sticky_ttl_ms")]
    pub sticky_ttl_ms: i64,
}

impl RoutingGroup {
    pub fn new(id: impl Into<String>, name: impl Into<String>, strategy: RoutingStrategy) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            strategy,
            members: Vec::new(),
            intent_fast_model: None,
            intent_deep_model: None,
            sticky_ttl_ms: default_sticky_ttl_ms(),
        }
    }

    pub fn canonical_id(&self) -> String {
        self.id.trim().strip_prefix("group/").unwrap_or(self.id.trim()).to_string()
    }

    pub fn qualified_model_name(&self) -> String {
        format!("group/{}", self.canonical_id())
    }
}

/// Runtime seat accounting with 3600s half-life exponential usage decay and cooldown tracking.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RouteSeatMetrics {
    pub weighted_tokens: f64,
    pub last_decay_ms: i64,
    pub total_requests: u64,
    pub cooldown_until_ms: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_status: Option<u16>,
    pub consecutive_failures: u32,
    pub quota_used_pct: f64,
    pub reset_at_ms: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_latency_ms: Option<u64>,
}

impl RouteSeatMetrics {
    /// Decayed token usage with a 3600-second half-life.
    pub fn decayed_usage(&self, now_ms: i64) -> f64 {
        if self.weighted_tokens <= 0.0 {
            return 0.0;
        }
        let elapsed_secs = ((now_ms - self.last_decay_ms).max(0) as f64) / 1000.0;
        self.weighted_tokens * 0.5_f64.powf(elapsed_secs / 3600.0)
    }

    pub fn is_in_cooldown(&self, now_ms: i64) -> bool {
        now_ms < self.cooldown_until_ms
    }

    pub fn record_usage(&mut self, tokens: u64, latency_ms: Option<u64>, now_ms: i64) {
        let decayed = if self.last_decay_ms > 0 {
            self.decayed_usage(now_ms)
        } else {
            self.weighted_tokens
        };
        self.weighted_tokens = decayed + (tokens as f64);
        self.last_decay_ms = now_ms;
        self.total_requests = self.total_requests.saturating_add(1);
        self.consecutive_failures = 0;
        self.cooldown_until_ms = 0;
        self.last_status = Some(200);
        if let Some(ms) = latency_ms {
            self.last_latency_ms = Some(ms);
        }
    }

    pub fn record_failure(
        &mut self,
        status: Option<u16>,
        retry_after_secs: Option<u64>,
        now_ms: i64,
    ) {
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        self.last_status = status;

        let cooldown_ms = if let Some(secs) = retry_after_secs {
            (secs as i64).saturating_mul(1_000).max(1_000)
        } else {
            match status {
                Some(402) => 300_000,
                Some(429) => {
                    let shift = self.consecutive_failures.saturating_sub(1).min(7);
                    (5_000i64.saturating_mul(1i64 << shift)).min(900_000)
                }
                Some(code) if code >= 500 => {
                    let shift = self.consecutive_failures.saturating_sub(1).min(6);
                    (3_000i64.saturating_mul(1i64 << shift)).min(300_000)
                }
                _ => 0,
            }
        };
        if cooldown_ms > 0 {
            self.cooldown_until_ms = now_ms.saturating_add(cooldown_ms);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionAffinityEntry {
    pub target: String,
    pub expires_at_ms: i64,
}

/// Trace record explaining why the router chose a particular target.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteDecisionTrace {
    pub timestamp_ms: i64,
    pub requested_model: String,
    pub resolved_provider_id: String,
    pub resolved_model: String,
    pub strategy: RoutingStrategy,
    pub reason: String,
    pub candidates_considered: usize,
    #[serde(default)]
    pub masking_hits: usize,
}

/// Mutable routing state holding seat decay counters, rotation cursors, affinity, and trace ring buffer.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RouteRuntimeMetrics {
    #[serde(default)]
    pub seats: HashMap<String, RouteSeatMetrics>,
    #[serde(default)]
    pub rotate_cursor: HashMap<String, usize>,
    #[serde(default)]
    pub drain_pinned: HashMap<String, String>,
    #[serde(default)]
    pub session_affinity: HashMap<String, SessionAffinityEntry>,
    #[serde(default)]
    pub traces: VecDeque<RouteDecisionTrace>,
}

const MAX_TRACE_RING: usize = 64;

impl RouteRuntimeMetrics {
    pub fn record_usage(&mut self, seat_key: &str, tokens: u64, latency_ms: Option<u64>, now_ms: i64) {
        self.seats
            .entry(seat_key.to_string())
            .or_default()
            .record_usage(tokens, latency_ms, now_ms);
    }

    pub fn record_failure(
        &mut self,
        seat_key: &str,
        status: Option<u16>,
        retry_after_secs: Option<u64>,
        now_ms: i64,
    ) {
        self.seats
            .entry(seat_key.to_string())
            .or_default()
            .record_failure(status, retry_after_secs, now_ms);
    }

    pub fn push_trace(&mut self, trace: RouteDecisionTrace) {
        if self.traces.len() >= MAX_TRACE_RING {
            self.traces.pop_front();
        }
        self.traces.push_back(trace);
    }
}

/// Classified intent tier used for optional fast/deep split routing inside a `RoutingGroup`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentTier {
    Fast,
    Standard,
    Deep,
}

/// Classify a turn into `Fast`, `Standard`, or `Deep` without an external classifier model.
pub fn classify_intent(prompt_preview: &str, messages_count: usize, tools_count: usize) -> IntentTier {
    let trimmed = prompt_preview.trim();
    let lower = trimmed.to_ascii_lowercase();
    let chars = trimmed.chars().count();

    let deep_keywords = [
        "architecture",
        "refactor",
        "debug",
        "analyze",
        "proof",
        "theorem",
        "implement",
        "design",
        "security",
        "架构",
        "重构",
        "深入",
        "推理",
        "证明",
        "调试",
        "设计",
        "分析",
    ];
    if chars > 600
        || (tools_count >= 5 && chars > 120)
        || deep_keywords
            .iter()
            .any(|kw| trimmed.contains(kw) || lower.contains(kw))
    {
        return IntentTier::Deep;
    }

    let fast_keywords = [
        "translate",
        "summarize",
        "hello",
        "hi",
        "ping",
        "翻译",
        "总结",
        "你好",
        "简述",
    ];
    if chars <= 160
        && tools_count == 0
        && messages_count <= 4
        && (chars <= 48
            || fast_keywords
                .iter()
                .any(|kw| trimmed.contains(kw) || lower.contains(kw)))
    {
        return IntentTier::Fast;
    }

    IntentTier::Standard
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RoutingError {
    #[error("Routing group '{0}' not found")]
    GroupNotFound(String),
    #[error("Routing group cycle detected involving '{0}'")]
    CycleDetected(String),
    #[error("Routing group nesting exceeded max depth {0}")]
    MaxDepthExceeded(usize),
    #[error("Routing group '{0}' has no enabled members")]
    NoEnabledMembers(String),
}

/// Concrete resolved `(provider_id, model)` candidate produced by resolving a group or target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRouteTarget {
    pub provider_id: String,
    pub model: String,
    pub strategy: RoutingStrategy,
    pub reason: String,
}

pub fn split_target_spec(target: &str, default_provider: &str) -> (String, String) {
    let trimmed = target.trim();
    if let Some((prov, model)) = trimmed.split_once('/') {
        let p = prov.trim();
        let m = model.trim();
        if !p.is_empty() && !m.is_empty() {
            return (p.to_string(), m.to_string());
        }
    }
    (default_provider.to_string(), trimmed.to_string())
}

const MAX_GROUP_DEPTH: usize = 4;

fn flatten_group_members(
    group_id: &str,
    groups: &HashMap<String, RoutingGroup>,
    depth: usize,
    visiting: &mut HashSet<String>,
    out: &mut Vec<RoutingGroupMember>,
) -> Result<(), RoutingError> {
    if depth >= MAX_GROUP_DEPTH {
        return Err(RoutingError::MaxDepthExceeded(MAX_GROUP_DEPTH));
    }
    let clean_id = group_id.trim().strip_prefix("group/").unwrap_or(group_id.trim());
    if !visiting.insert(clean_id.to_string()) {
        return Err(RoutingError::CycleDetected(clean_id.to_string()));
    }
    let group = groups
        .get(clean_id)
        .ok_or_else(|| RoutingError::GroupNotFound(clean_id.to_string()))?;

    for member in &group.members {
        if !member.enabled || member.target.trim().is_empty() {
            continue;
        }
        let target = member.target.trim();
        if let Some(nested) = target.strip_prefix("group/") {
            flatten_group_members(nested, groups, depth + 1, visiting, out)?;
        } else {
            out.push(member.clone());
        }
    }

    visiting.remove(clean_id);
    Ok(())
}

/// Resolve an ordered list of candidate `(provider_id, model)` targets for a `RoutingGroup`.
#[allow(clippy::too_many_arguments)]
pub fn resolve_group_targets(
    group_id: &str,
    groups: &HashMap<String, RoutingGroup>,
    metrics: &mut RouteRuntimeMetrics,
    default_provider_id: &str,
    session_key: Option<&str>,
    prompt_preview: &str,
    messages_count: usize,
    tools_count: usize,
    now_ms: i64,
) -> Result<Vec<ResolvedRouteTarget>, RoutingError> {
    let clean_id = group_id.trim().strip_prefix("group/").unwrap_or(group_id.trim());
    let group = groups
        .get(clean_id)
        .ok_or_else(|| RoutingError::GroupNotFound(clean_id.to_string()))?;

    let mut visited = HashSet::new();
    let mut flat = Vec::new();
    flatten_group_members(clean_id, groups, 0, &mut visited, &mut flat)?;
    if flat.is_empty() {
        return Err(RoutingError::NoEnabledMembers(clean_id.to_string()));
    }

    let pool_key = format!("group/{clean_id}");
    let intent = classify_intent(prompt_preview, messages_count, tools_count);
    let intent_target = match intent {
        IntentTier::Fast => group
            .intent_fast_model
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|m| (m, "intent_fast")),
        IntentTier::Deep => group
            .intent_deep_model
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|m| (m, "intent_deep")),
        IntentTier::Standard => None,
    };

    // Check session affinity first (only if target still exists and is not cooled down)
    let mut pinned_first: Option<(String, String)> = None;
    if intent_target.is_none() && group.sticky_ttl_ms > 0 {
        if let Some(sess) = session_key.map(str::trim).filter(|s| !s.is_empty()) {
            let aff_key = format!("{sess}::{pool_key}");
            if let Some(entry) = metrics.session_affinity.get(&aff_key) {
                if now_ms < entry.expires_at_ms {
                    let exists = flat.iter().any(|m| m.target == entry.target);
                    let in_cooldown = metrics
                        .seats
                        .get(&entry.target)
                        .is_some_and(|s| s.is_in_cooldown(now_ms));
                    if exists && !in_cooldown {
                        pinned_first = Some((entry.target.clone(), "session_affinity".to_string()));
                    }
                }
            }
        }
    }

    let mut ordered_indices: Vec<usize> = (0..flat.len()).collect();
    match group.strategy {
        RoutingStrategy::Order => {}
        RoutingStrategy::Rotate => {
            let cursor = metrics.rotate_cursor.get(&pool_key).copied().unwrap_or(0) % flat.len();
            ordered_indices.rotate_left(cursor);
            if intent_target.is_none() && pinned_first.is_none() {
                metrics
                    .rotate_cursor
                    .insert(pool_key.clone(), (cursor + 1) % flat.len());
            }
        }
        RoutingStrategy::Usage => {
            ordered_indices.sort_by(|&a, &b| {
                let sa = metrics.seats.get(&flat[a].target);
                let sb = metrics.seats.get(&flat[b].target);
                let ca = sa.is_some_and(|s| s.is_in_cooldown(now_ms));
                let cb = sb.is_some_and(|s| s.is_in_cooldown(now_ms));
                let ua = sa.map(|s| s.decayed_usage(now_ms)).unwrap_or(0.0);
                let ub = sb.map(|s| s.decayed_usage(now_ms)).unwrap_or(0.0);
                ca.cmp(&cb)
                    .then_with(|| ua.total_cmp(&ub))
                    .then_with(|| a.cmp(&b))
            });
        }
        RoutingStrategy::Pace => {
            ordered_indices.sort_by(|&a, &b| {
                let sa = metrics.seats.get(&flat[a].target);
                let sb = metrics.seats.get(&flat[b].target);
                let ca = sa.is_some_and(|s| s.is_in_cooldown(now_ms));
                let cb = sb.is_some_and(|s| s.is_in_cooldown(now_ms));
                let score = |m: Option<&RouteSeatMetrics>| -> f64 {
                    let Some(seat) = m else { return 0.0 };
                    let urgency = if seat.reset_at_ms > now_ms {
                        let rem = ((seat.reset_at_ms - now_ms) as f64) / 3_600_000.0;
                        seat.quota_used_pct + rem.min(24.0) * 0.5
                    } else {
                        seat.quota_used_pct
                    };
                    urgency + seat.decayed_usage(now_ms) * 0.001
                };
                ca.cmp(&cb)
                    .then_with(|| score(sa).total_cmp(&score(sb)))
                    .then_with(|| a.cmp(&b))
            });
        }
        RoutingStrategy::Cheapest => {
            ordered_indices.sort_by(|&a, &b| {
                let ma = &flat[a];
                let mb = &flat[b];
                let ca = metrics
                    .seats
                    .get(&ma.target)
                    .is_some_and(|s| s.is_in_cooldown(now_ms));
                let cb = metrics
                    .seats
                    .get(&mb.target)
                    .is_some_and(|s| s.is_in_cooldown(now_ms));
                let price = |m: &RoutingGroupMember| -> f64 {
                    if m.free {
                        0.0
                    } else {
                        m.input_cost_per_1m.unwrap_or(5.0) + m.output_cost_per_1m.unwrap_or(15.0)
                    }
                };
                ca.cmp(&cb)
                    .then_with(|| price(ma).total_cmp(&price(mb)))
                    .then_with(|| a.cmp(&b))
            });
        }
        RoutingStrategy::Drain => {
            let pinned_target = metrics.drain_pinned.get(&pool_key).cloned();
            let valid_pinned = pinned_target.and_then(|pt| {
                let pos = flat.iter().position(|m| m.target == pt)?;
                let cooled = metrics
                    .seats
                    .get(&pt)
                    .is_some_and(|s| s.is_in_cooldown(now_ms) || s.quota_used_pct >= 99.5);
                (!cooled).then_some(pos)
            });
            let chosen_pos = valid_pinned.unwrap_or_else(|| {
                flat.iter()
                    .position(|m| {
                        !metrics
                            .seats
                            .get(&m.target)
                            .is_some_and(|s| s.is_in_cooldown(now_ms) || s.quota_used_pct >= 99.5)
                    })
                    .unwrap_or(0)
            });
            metrics
                .drain_pinned
                .insert(pool_key.clone(), flat[chosen_pos].target.clone());
            ordered_indices.sort_by_key(|&idx| usize::from(idx != chosen_pos));
        }
        RoutingStrategy::Smart => {
            ordered_indices.sort_by(|&a, &b| {
                let ma = &flat[a];
                let mb = &flat[b];
                let sa = metrics.seats.get(&ma.target);
                let sb = metrics.seats.get(&mb.target);
                let smart_cost = |m: &RoutingGroupMember, seat: Option<&RouteSeatMetrics>| -> (u8, f64) {
                    let cooled = seat.is_some_and(|s| s.is_in_cooldown(now_ms));
                    let tier = if cooled { 2 } else if seat.is_some_and(|s| s.quota_used_pct >= 90.0) { 1 } else { 0 };
                    let weight = m.weight.max(1) as f64;
                    let load = seat.map(|s| s.decayed_usage(now_ms)).unwrap_or(0.0) / weight;
                    let free_bonus = if m.free { -1000.0 } else { 0.0 };
                    (tier, load + free_bonus)
                };
                let (ta, la) = smart_cost(ma, sa);
                let (tb, lb) = smart_cost(mb, sb);
                ta.cmp(&tb).then_with(|| la.total_cmp(&lb)).then_with(|| a.cmp(&b))
            });
        }
    }

    let strategy_reason = match group.strategy {
        RoutingStrategy::Smart => "smart_score",
        RoutingStrategy::Order => "order",
        RoutingStrategy::Rotate => "rotate",
        RoutingStrategy::Usage => "least_usage",
        RoutingStrategy::Pace => "pace",
        RoutingStrategy::Cheapest => "cheapest",
        RoutingStrategy::Drain => "drain",
    };

    let mut out = Vec::new();
    let mut push_unique = |target_str: &str, reason: &str| {
        let (provider_id, model) = split_target_spec(target_str, default_provider_id);
        if !model.is_empty()
            && !out.iter().any(|r: &ResolvedRouteTarget| {
                r.provider_id == provider_id && r.model == model
            })
        {
            out.push(ResolvedRouteTarget {
                provider_id,
                model,
                strategy: group.strategy,
                reason: reason.to_string(),
            });
        }
    };

    // Intent fast/deep override has highest priority if configured on the group
    if let Some((override_m, override_reason)) = intent_target {
        push_unique(override_m, override_reason);
    }

    if let Some((ref affinity_target, ref aff_reason)) = pinned_first {
        push_unique(affinity_target, aff_reason);
    }

    for idx in ordered_indices {
        push_unique(&flat[idx].target, strategy_reason);
    }

    if let (Some(sess), Some(primary)) = (
        session_key.map(str::trim).filter(|s| !s.is_empty()),
        out.first(),
    ) {
        if group.sticky_ttl_ms > 0 {
            let aff_key = format!("{sess}::{pool_key}");
            metrics.session_affinity.insert(
                aff_key,
                SessionAffinityEntry {
                    target: format!("{}/{}", primary.provider_id, primary.model),
                    expires_at_ms: now_ms.saturating_add(group.sticky_ttl_ms),
                },
            );
        }
    }

    Ok(out)
}
