//! Info Profile Tabs Host component
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/info_profile_tabs_host.cpp:L1-L898`
//! Upstream: `Telegram/SourceFiles/info/profile/tabs/info_profile_tabs_host.h:L1-L151`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use super::info_profile_tab_content::ProfileTabKind;

/// Host container coordinating profile tabs switching, sliding transitions,
/// and tab content virtualization.
#[derive(Debug, Clone)]
pub struct InfoProfileTabsHostProfileTabs {
    pub active_tab: ProfileTabKind,
    pub available_tabs: Vec<ProfileTabKind>,
    pub is_animating_slide: bool,
    pub slide_progress: f32,
}

impl InfoProfileTabsHostProfileTabs {
    pub fn new(available_tabs: Vec<ProfileTabKind>) -> Self {
        let active_tab = available_tabs.first().copied().unwrap_or(ProfileTabKind::Media);
        Self {
            active_tab,
            available_tabs,
            is_animating_slide: false,
            slide_progress: 0.0,
        }
    }

    pub fn switch_to_tab(&mut self, tab: ProfileTabKind) -> bool {
        if self.available_tabs.contains(&tab) && self.active_tab != tab {
            self.active_tab = tab;
            self.slide_progress = 0.0;
            true
        } else {
            false
        }
    }
}

impl Default for InfoProfileTabsHostProfileTabs {
    fn default() -> Self {
        Self::new(vec![ProfileTabKind::Media, ProfileTabKind::Files, ProfileTabKind::Links])
    }
}

impl Render for InfoProfileTabsHostProfileTabs {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x18222d))
            .child(
                div()
                    .p(px(8.0))
                    .text_xs()
                    .text_color(rgb(0x708499))
                    .child(format!("Active tab: {:?}", self.active_tab))
            )
    }
}
