//! Embedded vector SVG icons and localized `.strings` resource tables.

macro_rules! define_icons {
    ($($variant:ident => $file:literal),* $(,)?) => {
        /// Every crisp vector SVG icon embedded in the desktop theme asset pack.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum IconName {
            $($variant,)*
        }

        impl IconName {
            /// All 62 icon variants in canonical registration order.
            pub const ALL: &'static [IconName] = &[
                $(IconName::$variant,)*
            ];

            /// Relative asset path resolved by [`super::assets::TelegramAssetSource`].
            pub const fn path(self) -> &'static str {
                match self {
                    $(IconName::$variant => concat!("icons/", $file),)*
                }
            }

            /// Bare SVG filename within the `icons/` directory.
            pub const fn filename(self) -> &'static str {
                match self {
                    $(IconName::$variant => $file,)*
                }
            }

            /// Compiled-in UTF-8 SVG bytes embedded via `include_bytes!`.
            pub const fn embedded_bytes(self) -> &'static [u8] {
                match self {
                    $(IconName::$variant => include_bytes!(concat!("../../assets/icons/", $file)),)*
                }
            }
        }
    };
}

define_icons! {
    HamburgerMenu => "hamburger_menu.svg",
    Search => "search.svg",
    Close => "close.svg",
    BackArrow => "back_arrow.svg",
    ComposePen => "compose_pen.svg",
    Lock => "lock.svg",
    Pin => "pin.svg",
    Mute => "mute.svg",
    Verified => "verified.svg",
    PremiumStar => "premium_star.svg",
    SavedMessages => "saved_messages.svg",
    ArchivedChats => "archived_chats.svg",
    FolderAll => "folder_all.svg",
    FolderChat => "folder_chat.svg",
    FolderBots => "folder_bots.svg",
    FolderWork => "folder_work.svg",
    Paperclip => "paperclip.svg",
    SmileEmoji => "smile_emoji.svg",
    Sticker => "sticker.svg",
    Gif => "gif.svg",
    Mic => "mic.svg",
    VideoRound => "video_round.svg",
    SendPlane => "send_plane.svg",
    CheckSingle => "check_single.svg",
    CheckDouble => "check_double.svg",
    ClockSending => "clock_sending.svg",
    Reply => "reply.svg",
    Forward => "forward.svg",
    Edit => "edit.svg",
    DeleteTrash => "delete_trash.svg",
    Copy => "copy.svg",
    SelectCheck => "select_check.svg",
    Translate => "translate.svg",
    PhoneCall => "phone_call.svg",
    VideoCall => "video_call.svg",
    SidebarPanel => "sidebar_panel.svg",
    MoreVertical => "more_vertical.svg",
    Play => "play.svg",
    Pause => "pause.svg",
    Speaker => "speaker.svg",
    ScreenShare => "screen_share.svg",
    Hangup => "hangup.svg",
    Camera => "camera.svg",
    Poll => "poll.svg",
    LocationPin => "location_pin.svg",
    FileDoc => "file_doc.svg",
    HeartReaction => "heart_reaction.svg",
    BubbleTailIn => "bubble_tail_in.svg",
    BubbleTailOut => "bubble_tail_out.svg",
    ChevronDown => "chevron_down.svg",
    ChevronRight => "chevron_right.svg",
    NightMoon => "night_moon.svg",
    SunDay => "sun_day.svg",
    SettingsGear => "settings_gear.svg",
    ContactsUsers => "contacts_users.svg",
    GroupUsers => "group_users.svg",
    ChannelMegaphone => "channel_megaphone.svg",
    BotRobot => "bot_robot.svg",
    WalletStars => "wallet_stars.svg",
    TerminalMcp => "terminal_mcp.svg",
    ComputerDesktop => "computer_desktop.svg",
    TelegramPlaneLogo => "telegram_plane_logo.svg",
}

pub const EMBEDDED_EN_STRINGS: &[u8] = include_bytes!("../../assets/langs/en.strings");
pub const EMBEDDED_ZH_CN_STRINGS: &[u8] = include_bytes!("../../assets/langs/zh-CN.strings");

pub fn lookup_embedded_asset(normalized: &str) -> Option<&'static [u8]> {
    match normalized {
        "langs/en.strings" => Some(EMBEDDED_EN_STRINGS),
        "langs/zh-CN.strings" => Some(EMBEDDED_ZH_CN_STRINGS),
        _ => IconName::ALL
            .iter()
            .copied()
            .find(|icon| icon.path() == normalized)
            .map(IconName::embedded_bytes),
    }
}
