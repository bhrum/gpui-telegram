//! Built-in Telegram Desktop `.strings` localization packs (`en`, `zh-CN`).

use std::collections::HashMap;
use std::sync::OnceLock;

const EN_STRINGS_RAW: &str = include_str!("../../assets/langs/en.strings");
const ZH_CN_STRINGS_RAW: &str = include_str!("../../assets/langs/zh-CN.strings");

static EN_PACK: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
static ZH_CN_PACK: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn parse_static_strings(raw: &'static str) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('"') {
            continue;
        }
        let Some(key_rest) = trimmed.strip_prefix('"') else {
            continue;
        };
        let Some(key_end) = key_rest.find('"') else {
            continue;
        };
        let key = &key_rest[..key_end];
        let after_key = key_rest[key_end + 1..].trim_start();
        let Some(after_eq) = after_key.strip_prefix('=') else {
            continue;
        };
        let after_eq = after_eq.trim_start();
        let Some(val_rest) = after_eq.strip_prefix('"') else {
            continue;
        };
        let bytes = val_rest.as_bytes();
        let mut i = 0;
        let mut escaped = false;
        let mut val_end = None;
        while i < bytes.len() {
            if escaped {
                escaped = false;
            } else if bytes[i] == b'\\' {
                escaped = true;
            } else if bytes[i] == b'"' {
                val_end = Some(i);
                break;
            }
            i += 1;
        }
        if let Some(end_idx) = val_end {
            map.insert(key, &val_rest[..end_idx]);
        }
    }
    map
}

/// Normalizes locale code checking whether Simplified Chinese (`zh*`) should be selected.
pub fn is_zh_lang(lang_id: &str) -> bool {
    let lower = lang_id.trim().to_ascii_lowercase();
    lower == "zh" || lower.starts_with("zh-") || lower.starts_with("zh_")
}

/// Returns the built-in static key-value dictionary for `lang_code` (`"zh-CN"` or `"en"`).
pub fn builtin_pack(lang_code: &str) -> &'static HashMap<&'static str, &'static str> {
    if is_zh_lang(lang_code) {
        ZH_CN_PACK.get_or_init(|| parse_static_strings(ZH_CN_STRINGS_RAW))
    } else {
        EN_PACK.get_or_init(|| parse_static_strings(EN_STRINGS_RAW))
    }
}

/// Inline bilingual selector preserved for legacy calls (`zh` when Chinese, otherwise `en`).
pub fn tr_str<'a>(lang_id: &str, zh: &'a str, en: &'a str) -> &'a str {
    if is_zh_lang(lang_id) {
        zh
    } else {
        en
    }
}

/// Looks up an authentic Telegram `lng_*` localization key from the embedded language pack.
pub fn tr_key(lang_id: &str, key: &str) -> &'static str {
    if is_zh_lang(lang_id) {
        if let Some(&val) = builtin_pack("zh-CN").get(key) {
            return val;
        }
    }
    builtin_pack("en").get(key).copied().unwrap_or("")
}

/// Looks up a Telegram `lng_*` key with an explicit fallback string when absent.
pub fn tr_key_or<'a>(lang_id: &str, key: &str, fallback_en: &'a str) -> &'a str {
    if is_zh_lang(lang_id) {
        if let Some(&val) = builtin_pack("zh-CN").get(key) {
            return val;
        }
    }
    if let Some(&val) = builtin_pack("en").get(key) {
        return val;
    }
    fallback_en
}

/// Looks up a Telegram `lng_*` key and replaces `{var}` placeholders with `vars`.
pub fn tr_fmt(lang_id: &str, key: &str, vars: &[(&str, &str)]) -> String {
    let tmpl = tr_key(lang_id, key);
    let mut out = if tmpl.is_empty() {
        key.to_owned()
    } else {
        tmpl.to_owned()
    };
    for (k, v) in vars {
        let placeholder = format!("{{{k}}}");
        out = out.replace(&placeholder, v);
    }
    out
}
