// Source: Telegram/SourceFiles/settings/settings_privacy_controllers.cpp, Telegram/SourceFiles/settings/settings_privacy_controllers.h
// Subsystem: settings | Feature ID: TCG-N33
// Upstream: BlockPeerBoxController, lng_blocked_list_already_blocked, applySlice, unixtime::now

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrivacyKey {
    PhoneNumber,
    LastSeen,
    ProfilePhoto,
    Forwards,
    Calls,
    VoiceMessages,
    Bio,
    Birthday,
    Gifts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyOption {
    Everyone,
    Contacts,
    Nobody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivacyRule {
    pub option: PrivacyOption,
    pub always_allow: Vec<u64>,
    pub never_allow: Vec<u64>,
}

impl Default for PrivacyRule {
    fn default() -> Self {
        Self {
            option: PrivacyOption::Everyone,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsPrivacyControllers {
    pub rules: HashMap<PrivacyKey, PrivacyRule>,
    pub active_key: PrivacyKey,
}

impl SettingsPrivacyControllers {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        rules.insert(PrivacyKey::PhoneNumber, PrivacyRule {
            option: PrivacyOption::Contacts,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        });
        rules.insert(PrivacyKey::LastSeen, PrivacyRule {
            option: PrivacyOption::Everyone,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        });
        rules.insert(PrivacyKey::ProfilePhoto, PrivacyRule {
            option: PrivacyOption::Everyone,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        });
        rules.insert(PrivacyKey::Forwards, PrivacyRule {
            option: PrivacyOption::Everyone,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        });
        rules.insert(PrivacyKey::Calls, PrivacyRule {
            option: PrivacyOption::Contacts,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        });
        rules.insert(PrivacyKey::VoiceMessages, PrivacyRule {
            option: PrivacyOption::Everyone,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        });
        rules.insert(PrivacyKey::Bio, PrivacyRule {
            option: PrivacyOption::Everyone,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        });
        rules.insert(PrivacyKey::Birthday, PrivacyRule {
            option: PrivacyOption::Contacts,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        });
        rules.insert(PrivacyKey::Gifts, PrivacyRule {
            option: PrivacyOption::Everyone,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        });

        Self {
            rules,
            active_key: PrivacyKey::PhoneNumber,
        }
    }

    pub fn get_rule(&self, key: PrivacyKey) -> PrivacyRule {
        self.rules.get(&key).cloned().unwrap_or(PrivacyRule {
            option: PrivacyOption::Everyone,
            always_allow: Vec::new(),
            never_allow: Vec::new(),
        })
    }

    pub fn set_option(&mut self, key: PrivacyKey, option: PrivacyOption) {
        if let Some(rule) = self.rules.get_mut(&key) {
            rule.option = option;
        }
    }

    pub fn add_always_allow(&mut self, key: PrivacyKey, user_id: u64) {
        if let Some(rule) = self.rules.get_mut(&key) {
            rule.never_allow.retain(|&id| id != user_id);
            if !rule.always_allow.contains(&user_id) {
                rule.always_allow.push(user_id);
            }
        }
    }

    pub fn add_never_allow(&mut self, key: PrivacyKey, user_id: u64) {
        if let Some(rule) = self.rules.get_mut(&key) {
            rule.always_allow.retain(|&id| id != user_id);
            if !rule.never_allow.contains(&user_id) {
                rule.never_allow.push(user_id);
            }
        }
    }

    pub fn is_allowed(&self, key: PrivacyKey, user_id: u64, is_contact: bool) -> bool {
        let rule = self.get_rule(key);
        if rule.never_allow.contains(&user_id) {
            return false;
        }
        if rule.always_allow.contains(&user_id) {
            return true;
        }
        match rule.option {
            PrivacyOption::Everyone => true,
            PrivacyOption::Contacts => is_contact,
            PrivacyOption::Nobody => false,
        }
    }
}

impl Default for SettingsPrivacyControllers {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsPrivacyControllers {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let rule = self.get_rule(self.active_key);
        let opt_label = match rule.option {
            PrivacyOption::Everyone => "Everybody",
            PrivacyOption::Contacts => "My Contacts",
            PrivacyOption::Nobody => "Nobody",
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(20.0))
            .gap(px(16.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_lg()
                    .child(format!("Privacy Rule: {:?}", self.active_key))
            )
            .child(
                div()
                    .p(px(12.0))
                    .bg(rgb(0xf4f4f5))
                    .rounded(px(8.0))
                    .flex()
                    .justify_between()
                    .child(div().child("Who can see this"))
                    .child(div().font_weight(FontWeight::SEMIBOLD).text_color(rgb(0x2481cc)).child(opt_label))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).child("Add Exceptions"))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child(format!(
                                "Always Allow: {} users | Never Allow: {} users",
                                rule.always_allow.len(),
                                rule.never_allow.len()
                            ))
                    )
            )
    }
}
