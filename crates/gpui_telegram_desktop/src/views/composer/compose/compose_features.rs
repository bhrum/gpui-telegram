//! Composer Feature Flags and Permissions
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/compose/compose_features.h:L1-L41`
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeFeaturesCompose {
    pub likes: bool,
    pub send_as: bool,
    pub ttl_info: bool,
    pub attachments: bool,
    pub bot_command_send: bool,
    pub silent_broadcast_toggle: bool,
    pub attach_bots_menu: bool,
    pub inline_bots: bool,
    pub megagroup_set: bool,
    pub collectible_status: bool,
    pub stickers_settings: bool,
    pub open_sticker_sets: bool,
    pub photo_button: bool,
    pub audio_button: bool,
    pub link_button: bool,
    pub autocomplete_hashtags: bool,
    pub autocomplete_mentions: bool,
    pub autocomplete_commands: bool,
    pub suggest_stickers_by_emoji: bool,
    pub common_tabbed_panel: bool,
    pub record_media_message: bool,
    pub edit_message_stars: bool,
    pub emoji_only_panel: bool,
    pub all_custom_emoji_premium: bool,
    pub video_stream: bool,
    pub rich_editor: bool,
}

impl Default for ComposeFeaturesCompose {
    /// Default values directly mirroring `compose_features.h:L12-L39`
    fn default() -> Self {
        Self {
            likes: false,
            send_as: true,
            ttl_info: true,
            attachments: true,
            bot_command_send: true,
            silent_broadcast_toggle: true,
            attach_bots_menu: true,
            inline_bots: true,
            megagroup_set: true,
            collectible_status: false,
            stickers_settings: true,
            open_sticker_sets: true,
            photo_button: false,
            audio_button: false,
            link_button: false,
            autocomplete_hashtags: true,
            autocomplete_mentions: true,
            autocomplete_commands: true,
            suggest_stickers_by_emoji: true,
            common_tabbed_panel: true,
            record_media_message: true,
            edit_message_stars: false,
            emoji_only_panel: false,
            all_custom_emoji_premium: false,
            video_stream: false,
            rich_editor: true,
        }
    }
}

pub type ComposeFeatures = ComposeFeaturesCompose;

impl ComposeFeaturesCompose {
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct minimal features for emoji-only picker mode
    pub fn emoji_only() -> Self {
        Self {
            emoji_only_panel: true,
            attachments: false,
            record_media_message: false,
            suggest_stickers_by_emoji: false,
            ..Self::default()
        }
    }

    /// Construct features for broadcast channels (no likes, silent toggle enabled)
    pub fn broadcast_channel() -> Self {
        Self {
            likes: false,
            silent_broadcast_toggle: true,
            record_media_message: true,
            ..Self::default()
        }
    }

    /// Construct features for megagroup/supergroup chats
    pub fn supergroup() -> Self {
        Self {
            megagroup_set: true,
            autocomplete_mentions: true,
            autocomplete_commands: true,
            ..Self::default()
        }
    }

    /// Check if any autocomplete mode is active
    pub fn has_any_autocomplete(&self) -> bool {
        self.autocomplete_hashtags || self.autocomplete_mentions || self.autocomplete_commands
    }
}
