//! Pure-Rust Magpie gateway core: provider presets, read-only local subscription discovery,
//! smart routing groups with exponential decay & session affinity, privacy masking, Rosetta Gemini
//! wire translation, and deep-link / `.env` importers.

pub mod import;
pub mod masking;
pub mod presets;
pub mod protocol;
pub mod routing;
pub mod subscriptions;

pub use import::{parse_dotenv_providers, parse_magpie_import_link, ImportError, ImportedProviderSpec};
pub use masking::{
    mask_text, unmask_text, MaskResult, MaskingAction, MaskingConfig, MaskingEngine, MaskingHit,
    StreamingUnmasker,
};
pub use presets::{
    all_presets, canonical_preset_alias, find_preset, resolve_preset_endpoints, MagpiePreset,
    PresetKind, PresetRegion, ProtocolKind, ResolvedPresetEndpoint,
};
pub use protocol::{
    build_gemini_generate_content_payload, inbound_anthropic_to_unified,
    inbound_chat_completions_to_unified, inbound_responses_to_unified, parse_gemini_response_json,
    parse_gemini_sse_data, resolve_anthropic_messages_url, resolve_gemini_endpoint_url,
    unified_to_anthropic_json, unified_to_chat_completions_json, unified_to_responses_json,
    GeminiSseDelta, ProtocolError, UnifiedMessage, UnifiedRequest, UnifiedResponse, UnifiedRole,
    UnifiedToolCall, UnifiedToolSpec, UnifiedUsage,
};
pub use routing::{
    classify_intent, resolve_group_targets, split_target_spec, IntentTier, ResolvedRouteTarget,
    RouteDecisionTrace, RouteRuntimeMetrics, RouteSeatMetrics, RoutingError, RoutingGroup,
    RoutingGroupMember, RoutingStrategy, SessionAffinityEntry,
};
pub use subscriptions::{
    discover_subscriptions_in_home, SubscriptionAccount, SubscriptionKind, SubscriptionRegistry,
};

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine as _;
    use serde_json::json;
    use std::collections::HashMap;
    use std::path::PathBuf;

    struct TempDirGuard {
        path: PathBuf,
    }

    impl TempDirGuard {
        fn new(prefix: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "{prefix}_{}_{}",
                std::process::id(),
                chrono::Utc::now().timestamp_nanos_opt().unwrap_or(1)
            ));
            std::fs::create_dir_all(&path).expect("create temp dir");
            Self { path }
        }
    }

    impl Drop for TempDirGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn presets_cover_vendors_relays_local_remote_custom_and_regions() {
        let presets = all_presets();
        assert!(presets.len() >= 40, "expected >=40 presets, got {}", presets.len());

        for required in [
            "deepseek",
            "moonshot",
            "moonshot-cn",
            "kimi-code",
            "kimi-code-cn",
            "zhipu",
            "zai",
            "minimax",
            "minimax-cn",
            "stepfun",
            "stepfun-cn",
            "xiaomi",
            "qwen",
            "qwen-cn",
            "baidu-qianfan",
            "tencent-cloud",
            "volcengine",
            "longcat",
            "anthropic",
            "openai",
            "google",
            "xai",
            "mistral",
            "groq",
            "nous",
            "openrouter",
            "siliconflow",
            "vercel",
            "together",
            "fireworks",
            "fireworks-nexus",
            "nvidia",
            "modelscope",
            "opencode-zen",
            "kilo",
            "aihubmix",
            "302ai",
            "ollama",
            "lmstudio",
            "omlx",
            "mlx-serve",
            "gpustack",
            "remote-magpie",
            "custom-openai",
            "custom-anthropic",
            "custom-gemini",
            "custom-responses",
        ] {
            assert!(find_preset(required).is_some(), "missing preset {required}");
        }

        let zhipu_coding = resolve_preset_endpoints("zhipu", Some("coding"), None).unwrap();
        assert_eq!(zhipu_coding.base_url, "https://open.bigmodel.cn/api/coding/paas/v4");
        assert_eq!(zhipu_coding.region_id.as_deref(), Some("coding"));

        let zai_coding = resolve_preset_endpoints("zai", Some("coding"), None).unwrap();
        assert_eq!(zai_coding.base_url, "https://api.z.ai/api/coding/paas/v4");

        let ollama = find_preset("ollama").unwrap();
        assert!(ollama.no_key);
        assert_eq!(ollama.chat_url.as_deref(), Some("http://127.0.0.1:11434/v1"));

        let sf_intl = resolve_preset_endpoints("siliconflow", Some("intl"), None).unwrap();
        assert_eq!(sf_intl.base_url, "https://api.siliconflow.com/v1");
    }

    #[test]
    fn subscriptions_discovery_parses_local_credentials_readonly_without_secrets_in_json() {
        let dir = TempDirGuard::new("magpie_subs");
        let home = &dir.path;

        // 1. Codex auth + config
        let codex_dir = home.join(".codex");
        std::fs::create_dir_all(&codex_dir).unwrap();
        let jwt_hdr = URL_SAFE_NO_PAD.encode(br#"{"alg":"none"}"#);
        let jwt_claims = URL_SAFE_NO_PAD.encode(
            br#"{"email":"zen@example.com","https://api.openai.com/auth":{"chatgpt_account_id":"acct_42","chatgpt_plan_type":"pro"}}"#,
        );
        let jwt = format!("{jwt_hdr}.{jwt_claims}.sig");
        std::fs::write(
            codex_dir.join("auth.json"),
            json!({
                "tokens": {
                    "access_token": jwt,
                    "refresh_token": "rt_secret_should_not_serialize",
                    "account_id": "acct_42"
                }
            })
            .to_string(),
        )
        .unwrap();
        std::fs::write(
            codex_dir.join("config.toml"),
            "model = \"gpt-5-codex\"\nopenai_base_url = \"http://127.0.0.1:3425/backend-api/codex\"\n",
        )
        .unwrap();

        // 2. Claude Code credentials
        let claude_dir = home.join(".claude");
        std::fs::create_dir_all(&claude_dir).unwrap();
        std::fs::write(
            claude_dir.join(".credentials.json"),
            json!({
                "claudeAiOauth": {
                    "accessToken": "sk-ant-oat01-secret-token",
                    "refreshToken": "rt-claude",
                    "expiresAt": chrono::Utc::now().timestamp_millis() + 3_600_000,
                    "subscriptionType": "max"
                },
                "oauthAccount": { "emailAddress": "claude@example.com" }
            })
            .to_string(),
        )
        .unwrap();

        // 3. GitHub Copilot
        let copilot_dir = home.join(".config").join("github-copilot");
        std::fs::create_dir_all(&copilot_dir).unwrap();
        std::fs::write(
            copilot_dir.join("hosts.json"),
            json!({
                "github.com": {
                    "user": "bhrum-dev",
                    "oauth_token": "ghu_secret_copilot_token"
                }
            })
            .to_string(),
        )
        .unwrap();

        // 4. Gemini Code Assist
        let gemini_dir = home.join(".gemini");
        std::fs::create_dir_all(&gemini_dir).unwrap();
        std::fs::write(
            gemini_dir.join("oauth_creds.json"),
            json!({
                "access_token": "ya29.secret_gemini",
                "refresh_token": "1//refresh_gemini",
                "expiry_date": chrono::Utc::now().timestamp_millis() + 3_600_000
            })
            .to_string(),
        )
        .unwrap();

        let reg = discover_subscriptions_in_home(home, None);
        assert!(reg.has_active_subscription());
        assert_eq!(reg.accounts.len(), 4);

        // Verify raw access tokens are never serialized to JSON!
        let serialized = serde_json::to_string(&reg).unwrap();
        assert!(!serialized.contains("sk-ant-oat01-secret-token"));
        assert!(!serialized.contains("ghu_secret_copilot_token"));
        assert!(!serialized.contains("ya29.secret_gemini"));
    }

    #[test]
    fn routing_strategies_decay_cycle_guard_and_affinity_work() {
        let mut groups = HashMap::new();
        let mut main = RoutingGroup::new("smart-mix", "Smart Mix", RoutingStrategy::Rotate);
        main.members = vec![
            RoutingGroupMember::new("deepseek/deepseek-chat"),
            RoutingGroupMember::new("zhipu/glm-5"),
        ];
        main.intent_fast_model = Some("groq/llama-3.3-70b-versatile".to_string());
        main.intent_deep_model = Some("anthropic/claude-opus-4-20250514".to_string());
        groups.insert("smart-mix".to_string(), main);

        let mut metrics = RouteRuntimeMetrics::default();
        let now = 10_000_000i64;

        // Intent fast overrides first pick when prompt is short simple ask
        let fast_targets = resolve_group_targets(
            "group/smart-mix",
            &groups,
            &mut metrics,
            "deepseek",
            Some("sess1"),
            "hello",
            1,
            0,
            now,
        )
        .unwrap();
        assert_eq!(fast_targets[0].provider_id, "groq");
        assert_eq!(fast_targets[0].reason, "intent_fast");

        // Standard prompt respects session affinity and rotation
        let first_std = resolve_group_targets(
            "smart-mix",
            &groups,
            &mut metrics,
            "deepseek",
            None,
            "Please explain how connection pooling handles backpressure across services cleanly in production.",
            2,
            1,
            now,
        )
        .unwrap();
        assert_eq!(first_std[0].provider_id, "deepseek");
        let second_std = resolve_group_targets(
            "smart-mix",
            &groups,
            &mut metrics,
            "deepseek",
            None,
            "Please explain how connection pooling handles backpressure across services cleanly in production.",
            2,
            1,
            now,
        )
        .unwrap();
        assert_eq!(second_std[0].provider_id, "zhipu");

        // Half-life decay check (3600s -> half)
        metrics.record_usage("deepseek/deepseek-chat", 10_000, Some(120), now);
        let decayed = metrics
            .seats
            .get("deepseek/deepseek-chat")
            .unwrap()
            .decayed_usage(now + 3_600_000);
        assert!((decayed - 5_000.0).abs() < 1e-6, "decayed={decayed}");

        // Cycle protection
        let mut cyc_a = RoutingGroup::new("a", "A", RoutingStrategy::Order);
        cyc_a.members.push(RoutingGroupMember::new("group/b"));
        let mut cyc_b = RoutingGroup::new("b", "B", RoutingStrategy::Order);
        cyc_b.members.push(RoutingGroupMember::new("group/a"));
        groups.insert("a".to_string(), cyc_a);
        groups.insert("b".to_string(), cyc_b);

        let cyc_err = resolve_group_targets(
            "group/a",
            &groups,
            &mut metrics,
            "deepseek",
            None,
            "hello",
            1,
            0,
            now,
        )
        .unwrap_err();
        assert!(matches!(cyc_err, RoutingError::CycleDetected(_)));
    }

    #[test]
    fn masking_secrets_pii_allowlist_and_streaming_unmasker_work() {
        let mut cfg = MaskingConfig {
            enabled: true,
            mask_secrets: true,
            mask_personal: true,
            custom_words: vec!["ProjectPhoenix".to_string()],
            allow_list: vec!["safe@company.com".to_string()],
            default_action: MaskingAction::Mask,
            ..Default::default()
        };
        cfg.kind_actions
            .insert("card_number".to_string(), MaskingAction::Block);

        let engine = MaskingEngine::new(cfg);
        let sample_key = "sk-ant-api03-secret9988776655";
        let sample_jwt = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTYifQ.dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let input = format!(
            "Key={sample_key}, JWT={sample_jwt}, email=alice@realcorp.io, doc=bob@example.com, safe=safe@company.com, word=ProjectPhoenix"
        );

        let masked = engine.mask_text(&input);
        assert!(!masked.blocked);
        assert!(!masked.masked_text.contains(sample_key));
        assert!(!masked.masked_text.contains(sample_jwt));
        assert!(!masked.masked_text.contains("alice@realcorp.io"));
        assert!(!masked.masked_text.contains("ProjectPhoenix"));
        // example.com and allowlisted safe@company.com remain untouched
        assert!(masked.masked_text.contains("bob@example.com"));
        assert!(masked.masked_text.contains("safe@company.com"));
        assert_eq!(engine.unmask_text(&masked.masked_text), input);

        // Split placeholder across streaming SSE chunks
        let mut stream_unmasker = StreamingUnmasker::new();
        let bytes_text = masked.masked_text.as_str();
        let mut reconstructed = String::new();
        for chunk in bytes_text.as_bytes().chunks(5) {
            let s = std::str::from_utf8(chunk).unwrap();
            reconstructed.push_str(&stream_unmasker.push(s, &engine));
        }
        reconstructed.push_str(&stream_unmasker.finish(&engine));
        assert_eq!(reconstructed, input);

        // Luhn card blocking
        let blocked_res = engine.mask_text("charge 4111-1111-1111-1111 now");
        assert!(blocked_res.blocked);
        assert!(blocked_res.block_reason.is_some());
    }

    #[test]
    fn gemini_rosetta_protocol_and_import_links_work() {
        assert_eq!(
            resolve_gemini_endpoint_url("https://generativelanguage.googleapis.com", "gemini-2.5-flash", false),
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent"
        );
        assert_eq!(
            resolve_gemini_endpoint_url("https://generativelanguage.googleapis.com/v1beta/openai", "models/gemini-2.5-pro", true),
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:streamGenerateContent?alt=sse"
        );

        let openai_body = json!({
            "model": "gemini-2.5-flash",
            "stream": true,
            "messages": [
                {"role": "system", "content": "Be concise."},
                {"role": "user", "content": "Hello Gemini"}
            ]
        });
        let unified = inbound_chat_completions_to_unified(&openai_body).unwrap();
        let g_payload = build_gemini_generate_content_payload(&unified);
        assert_eq!(g_payload["systemInstruction"]["parts"][0]["text"], "Be concise.");
        assert_eq!(g_payload["contents"][0]["parts"][0]["text"], "Hello Gemini");

        let resp_body = json!({
            "candidates": [{
                "content": {
                    "role": "model",
                    "parts": [
                        {"thought": true, "text": "thinking step"},
                        {"text": "Hello from Gemini!"}
                    ]
                },
                "finishReason": "STOP"
            }],
            "usageMetadata": {
                "promptTokenCount": 12,
                "candidatesTokenCount": 5,
                "totalTokenCount": 17
            }
        })
        .to_string();
        let parsed = parse_gemini_response_json("google", "gemini-2.5-flash", &resp_body).unwrap();
        assert_eq!(parsed.content, "Hello from Gemini!");
        assert_eq!(parsed.reasoning.as_deref(), Some("thinking step"));
        assert_eq!(parsed.usage.total_tokens, 17);

        let sse_delta = parse_gemini_sse_data(
            "data: {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"chunk\"}]}}]}",
            0,
        )
        .unwrap()
        .unwrap();
        assert_eq!(sse_delta.text_delta, "chunk");

        let spec = parse_magpie_import_link("magpie://import?preset=zhipu&region=coding&key=sk-glm-123").unwrap();
        assert_eq!(spec.preset_id.as_deref(), Some("zhipu"));
        assert_eq!(spec.region_id.as_deref(), Some("coding"));
        assert_eq!(spec.base_url, "https://open.bigmodel.cn/api/coding/paas/v4");
        assert_eq!(spec.api_key.as_deref(), Some("sk-glm-123"));

        let web_spec = parse_magpie_import_link(
            "https://usemagpie.ai/import?name=Team%20Relay&chat=https://relay.example.com/v1&key=sk-r1&models=deepseek-chat,gpt-4o",
        )
        .unwrap();
        assert_eq!(web_spec.id, "team-relay");
        assert_eq!(web_spec.models, vec!["deepseek-chat", "gpt-4o"]);

        let dotenv_specs = parse_dotenv_providers(
            "export GEMINI_API_KEY=\"AIzaSyDummyTestKey1234567890123456\"\nDEEPSEEK_API_KEY=sk-ds-1234567890123456 # prod\n",
        );
        assert_eq!(dotenv_specs.len(), 2);
        assert!(dotenv_specs.iter().any(|s| s.id == "google"));
        assert!(dotenv_specs.iter().any(|s| s.id == "deepseek"));

        // Responses + Anthropic inbound/outbound adapters
        let resp_req = inbound_responses_to_unified(&json!({
            "model": "gpt-4o",
            "instructions": "Sys",
            "input": [{"type": "message", "role": "user", "content": "Hello"}],
            "max_output_tokens": 512
        }))
        .unwrap();
        assert_eq!(resp_req.system_prompt, "Sys");
        assert_eq!(resp_req.max_tokens, Some(512));

        let ant_req = inbound_anthropic_to_unified(&json!({
            "model": "claude-sonnet-4-20250514",
            "system": "AntSys",
            "max_tokens": 1024,
            "messages": [{"role": "user", "content": [{"type": "text", "text": "Ping"}]}]
        }))
        .unwrap();
        assert_eq!(ant_req.system_prompt, "AntSys");
        assert_eq!(ant_req.messages[0].content, "Ping");

        let chat_out = unified_to_chat_completions_json(&parsed);
        assert_eq!(chat_out["choices"][0]["message"]["content"], "Hello from Gemini!");
        let resp_out = unified_to_responses_json(&parsed);
        assert_eq!(resp_out["output_text"], "Hello from Gemini!");
        let ant_out = unified_to_anthropic_json(&parsed);
        assert_eq!(ant_out["stop_reason"], "end_turn");
    }

    #[test]
    fn routing_cheapest_pace_usage_drain_and_pem_bearer_masking_work() {
        let mut metrics = RouteRuntimeMetrics::default();
        let now = 5_000_000i64;
        let mut groups = HashMap::new();

        // Cheapest strategy prefers free / lowest cost member
        let mut cheap_g = RoutingGroup::new("cost", "Cost", RoutingStrategy::Cheapest);
        cheap_g.members = vec![
            RoutingGroupMember {
                target: "anthropic/claude-opus-4-20250514".to_string(),
                input_cost_per_1m: Some(15.0),
                output_cost_per_1m: Some(75.0),
                ..RoutingGroupMember::new("anthropic/claude-opus-4-20250514")
            },
            RoutingGroupMember {
                target: "deepseek/deepseek-chat".to_string(),
                input_cost_per_1m: Some(0.27),
                output_cost_per_1m: Some(1.10),
                ..RoutingGroupMember::new("deepseek/deepseek-chat")
            },
        ];
        groups.insert("cost".to_string(), cheap_g);
        let cheap_res = resolve_group_targets(
            "group/cost",
            &groups,
            &mut metrics,
            "deepseek",
            None,
            "Compare SQL engines cleanly across read-heavy workloads.",
            2,
            1,
            now,
        )
        .unwrap();
        assert_eq!(cheap_res[0].provider_id, "deepseek");
        assert_eq!(cheap_res[0].reason, "cheapest");

        // Drain stays on first member until in cooldown/quota exhausted
        let mut drain_g = RoutingGroup::new("drain", "Drain", RoutingStrategy::Drain);
        drain_g.members = vec![
            RoutingGroupMember::new("openai/gpt-4o"),
            RoutingGroupMember::new("zhipu/glm-5"),
        ];
        groups.insert("drain".to_string(), drain_g);
        let d1 = resolve_group_targets("drain", &groups, &mut metrics, "openai", None, "Normal task here", 2, 1, now).unwrap();
        assert_eq!(d1[0].provider_id, "openai");
        metrics.record_failure("openai/gpt-4o", Some(429), Some(60), now);
        let d2 = resolve_group_targets("drain", &groups, &mut metrics, "openai", None, "Normal task here", 2, 1, now).unwrap();
        assert_eq!(d2[0].provider_id, "zhipu");

        // Usage + Pace least-used selection
        metrics.record_usage("deepseek/deepseek-chat", 5_000, Some(100), now);
        let mut usage_g = RoutingGroup::new("bal", "Bal", RoutingStrategy::Usage);
        usage_g.members = vec![
            RoutingGroupMember::new("deepseek/deepseek-chat"),
            RoutingGroupMember::new("qwen/qwen-plus"),
        ];
        groups.insert("bal".to_string(), usage_g);
        let u_res = resolve_group_targets("bal", &groups, &mut metrics, "deepseek", None, "Normal task here", 2, 1, now).unwrap();
        assert_eq!(u_res[0].provider_id, "qwen");

        // PEM private key + Bearer token masking
        let eng = MaskingEngine::new(MaskingConfig {
            enabled: true,
            mask_secrets: true,
            mask_personal: true,
            ..Default::default()
        });
        let pem = "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASC\n-----END PRIVATE KEY-----";
        let raw = format!("Auth: Bearer supersecrettoken987654321 and key:\n{pem}");
        let res = eng.mask_text(&raw);
        assert!(!res.masked_text.contains("supersecrettoken987654321"));
        assert!(!res.masked_text.contains("MIIEvQIBADANBgkqhkiG9w0BAQEFAASC"));
        assert_eq!(eng.unmask_text(&res.masked_text), raw);
    }
}
