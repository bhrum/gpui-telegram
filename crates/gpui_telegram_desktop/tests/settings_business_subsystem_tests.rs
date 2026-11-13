//! Settings Business & Global TTL Subsystem Tests
//! Covers: SettingsAwayMessageBusiness, SettingsChatIntroBusiness, SettingsChatLinksBusiness,
//! SettingsChatbotsBusiness, SettingsGreetingBusiness, SettingsLocationBusiness,
//! SettingsQuickRepliesBusiness, SettingsRecipientsHelperBusiness, SettingsShortcutMessagesBusiness,
//! SettingsWorkingHoursBusiness, SettingsGlobalTtlSections

use gpui_telegram_desktop::views::settings::business::*;
use gpui_telegram_desktop::views::settings::business::settings_away_message::{AwayScheduleType, K_START_TIME_MIN, K_END_TIME_MIN};
use gpui_telegram_desktop::views::settings::business::settings_chatbots::*;
use gpui_telegram_desktop::views::settings::business::settings_quick_replies::is_valid_shortcut_name;
use gpui_telegram_desktop::views::settings::business::settings_recipients_helper::*;
use gpui_telegram_desktop::views::settings::sections::settings_global_ttl::*;

#[test]
fn test_settings_away_message() {
    let mut away = SettingsAwayMessageBusiness::new();
    assert!(!away.enabled);
    assert_eq!(away.schedule_type, AwayScheduleType::Always);
    assert_eq!(away.shortcut_name, "away");

    away.toggle_enabled();
    assert!(away.enabled);

    away.set_schedule_type(AwayScheduleType::OutsideWorkingHours);
    assert_eq!(away.schedule_type, AwayScheduleType::OutsideWorkingHours);

    // Custom interval validation
    assert!(away.set_custom_interval(K_START_TIME_MIN + 10, K_END_TIME_MIN + 10).is_ok());
    assert!(away.set_custom_interval(100, 200).is_err()); // Before 2013 launch

    assert!(away.save().is_ok());
}

#[test]
fn test_settings_chat_intro() {
    let mut intro = SettingsChatIntroBusiness::new();
    assert!(intro.is_empty());

    intro.set_title("Welcome!".into());
    assert_eq!(intro.title_text, "Welcome!");

    // Max limit enforcement
    let long_desc = "d".repeat(100);
    intro.set_description(long_desc);
    assert!(intro.description_text.is_empty()); // Exceeds 70

    intro.set_description("Short intro message".into());
    assert_eq!(intro.description_text, "Short intro message");

    intro.choose_sticker(42);
    assert_eq!(intro.sticker_id, Some(42));
    assert!(!intro.is_empty());

    intro.reset();
    assert!(intro.is_empty());
}

#[test]
fn test_settings_chat_links() {
    let mut links = SettingsChatLinksBusiness::new();
    assert_eq!(links.links.len(), 1);
    assert!(links.can_create());

    let created = links.create_link("Sales".into(), "Hello sales".into());
    assert!(created.is_ok());
    assert_eq!(links.links.len(), 2);
    assert_eq!(links.links[0].title, "Sales");

    let link_url = links.links[0].link.clone();
    links.delete_link(&link_url);
    assert_eq!(links.links.len(), 1);
}

#[test]
fn test_settings_chatbots() {
    let mut chatbots = SettingsChatbotsBusiness::new();
    assert!(chatbots.bot.is_none());
    assert!(chatbots.has_permission(PERM_VIEW_MESSAGES));

    chatbots.set_search_query("@my_sales_bot".into());
    assert_eq!(chatbots.search_query, "my_sales_bot");

    chatbots.add_bot(12345, "Support Bot".into(), "support_bot".into());
    assert!(chatbots.bot.is_some());

    chatbots.toggle_permission(PERM_TRANSFER_STARS);
    assert!(chatbots.has_permission(PERM_TRANSFER_STARS));
    assert!(chatbots.warning_alert.is_some());

    chatbots.remove_bot();
    assert!(chatbots.bot.is_none());
    assert_eq!(chatbots.permissions, PERM_VIEW_MESSAGES);
}

#[test]
fn test_settings_greeting() {
    let mut greeting = SettingsGreetingBusiness::new();
    assert!(!greeting.enabled);
    assert_eq!(greeting.no_activity_days, 7);
    assert_eq!(greeting.shortcut_name, "hello");

    greeting.toggle_enabled();
    assert!(greeting.enabled);

    greeting.set_no_activity_days(14);
    assert_eq!(greeting.no_activity_days, 14);

    greeting.set_no_activity_days(99); // Invalid
    assert_eq!(greeting.no_activity_days, 14);

    assert!(greeting.save().is_ok());
}

#[test]
fn test_settings_location() {
    let mut loc = SettingsLocationBusiness::new();
    assert!(loc.point.is_none());

    loc.set_address(" 123 Main St, Springfield  ".into());
    loc.save();
    assert_eq!(loc.address, "123 Main St, Springfield");

    loc.set_point(37.7749, -122.4194);
    assert!(loc.point.is_some());
    assert_eq!(loc.point.unwrap().lat, 37.7749);

    loc.clear_point();
    assert!(loc.point.is_none());
}

#[test]
fn test_settings_quick_replies_validation() {
    assert!(is_valid_shortcut_name("pricing"));
    assert!(is_valid_shortcut_name("hello_world"));
    assert!(is_valid_shortcut_name("faq123"));
    assert!(!is_valid_shortcut_name("")); // empty
    assert!(!is_valid_shortcut_name("name with spaces"));
    assert!(!is_valid_shortcut_name("invalid!char"));

    let mut replies = SettingsQuickRepliesBusiness::new();
    assert_eq!(replies.shortcuts.len(), 2);

    replies.set_input("support".into());
    let res = replies.add_shortcut();
    assert!(res.is_ok());
    assert_eq!(replies.shortcuts.len(), 3);
}

#[test]
fn test_settings_recipients_helper() {
    let mut helper = SettingsRecipientsHelperBusiness::new();
    assert_eq!(helper.rule.mode, K_ALL_EXCEPT);
    assert!(helper.has_flag(FLAG_CONTACTS));

    helper.toggle_flag(FLAG_CONTACTS);
    assert!(!helper.has_flag(FLAG_CONTACTS));

    helper.add_excluded(999);
    assert!(helper.rule.excluded_peers.contains(&999));

    helper.add_included(999);
    assert!(helper.rule.included_peers.contains(&999));
    assert!(!helper.rule.excluded_peers.contains(&999));
}

#[test]
fn test_settings_shortcut_messages() {
    let mut sc = SettingsShortcutMessagesBusiness::new();
    assert_eq!(sc.messages.len(), 2);
    assert!(sc.can_send_more());

    sc.set_input("New automated reply".into());
    let msg_id = sc.send_message();
    assert!(msg_id.is_ok());
    assert_eq!(sc.messages.len(), 3);

    sc.toggle_select(sc.messages[0].id);
    assert!(sc.is_multi_select);

    sc.delete_selected();
    assert_eq!(sc.messages.len(), 2);
    assert!(!sc.is_multi_select);
}

#[test]
fn test_settings_working_hours() {
    let mut hours = SettingsWorkingHoursBusiness::new();
    assert_eq!(hours.days.len(), 7);
    assert!(hours.days[0].is_open); // Monday
    assert!(!hours.days[5].is_open); // Saturday

    hours.toggle_day(5);
    assert!(hours.days[5].is_open);

    hours.set_timezone("EST".into(), "Eastern Standard Time".into(), -5);
    assert_eq!(hours.timezone_id, "EST");
    assert_eq!(hours.utc_offset_hours, -5);
}

#[test]
fn test_settings_global_ttl() {
    let mut ttl = SettingsGlobalTtlSections::new();
    assert_eq!(ttl.current_ttl, TTL_OFF);
    assert_eq!(SettingsGlobalTtlSections::format_ttl(TTL_OFF), "Off");
    assert_eq!(SettingsGlobalTtlSections::format_ttl(TTL_ONE_DAY), "1 Day");

    // Setting from Off triggers confirmation
    ttl.set_ttl(TTL_ONE_DAY);
    assert!(ttl.is_confirming);
    assert_eq!(ttl.pending_ttl, Some(TTL_ONE_DAY));
    assert_eq!(ttl.current_ttl, TTL_OFF);

    ttl.confirm_pending();
    assert!(!ttl.is_confirming);
    assert_eq!(ttl.current_ttl, TTL_ONE_DAY);

    ttl.apply_to_existing();
    assert!(ttl.status_message.is_some());
}
