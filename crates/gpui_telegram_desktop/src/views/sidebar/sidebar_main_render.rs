//! Main GPUI `Render` implementation for [`ChatSidebar`].

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::chat_sidebar::{ChatSidebar, DialogItemModel};
use crate::views::sidebar::dialogs_overlays::{
    render_dialog_context_menu, render_folder_tabs_strip, render_forum_topics_subtree,
};
use crate::views::sidebar::dialogs_renderer::{
    render_archived_header_row, render_plus_dropdown, render_stories_strip, render_suggestion_bar,
};
use crate::views::sidebar::dialogs_row_view::{render_dialog_row_item, SidebarDialogMeta};
use crate::views::sidebar::gemini_unified_sidebar::{
    render_collapsed_gemini_sidebar, render_expanded_gemini_footer,
    render_expanded_gemini_search_bar, render_expanded_gemini_top_bar, UnifiedNavSnapshot,
};
use crate::views::sidebar::sidebar_search_view::{
    render_apps_catalog_section_with_recent, render_search_panel, render_search_results_list,
};

fn render_sidebar_dialog_row(
    item: &DialogItemModel,
    idx: usize,
    active_id: Option<u64>,
    is_narrow: bool,
    meta: &SidebarDialogMeta,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<ChatSidebar>,
) -> AnyElement {
    let did = item.id;
    let is_sel = active_id == Some(did);
    render_dialog_row_item(
        item,
        idx,
        is_sel,
        is_narrow,
        meta,
        palette,
        lang_id,
        cx,
        move |this, cx| {
            this.context_menu = None;
            this.search_focused = false;
            this.select_dialog_from_user(did);
            cx.notify();
        },
        move |this, pos, cx| {
            this.context_menu = Some((did, pos));
            cx.notify();
        },
        move |this, cx| {
            this.toggle_forum_topics(did);
            cx.notify();
        },
        move |this, uri, cx| {
            this.context_menu = None;
            this.search_focused = false;
            this.pending_launch_dialog_miniapp = Some((did, uri));
            cx.notify();
        },
    )
}

impl Render for ChatSidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = self.palette.clone();
        let lang_id = self.language_id.clone();
        let active_folder = self.active_folder.clone();
        let active_id = self.active_chat_id;
        let query = self.search_query.clone();
        let is_search_focused = self.search_focused;
        let is_plus_open = self.is_plus_menu_open;
        let creation_modal = self.active_creation_modal;
        let contact_kind = self.contact_kind;
        let active_field = self.active_creation_field;
        let primary_val = self.creation_primary_input.clone();
        let secondary_val = self.creation_secondary_input.clone();
        let tertiary_val = self.creation_tertiary_input.clone();
        let active_search_cat = self.active_search_category.clone();
        let active_search_tag = self.active_search_tag.clone();
        let is_narrow = self.width < 252.0;

        let total_dialog_unread: u32 = self.total_unread_badge();
        let nav_snapshot = UnifiedNavSnapshot::from_rail_and_unread(
            self.nav_rail.as_ref().map(|r| r.read(cx)),
            self.standalone_active_destination,
            total_dialog_unread,
        );
        if self.is_collapsed {
            let mut rail_dialogs: Vec<&DialogItemModel> = self
                .dialogs
                .iter()
                .filter(|d| !self.archived_ids.contains(&d.id) && d.is_pinned)
                .collect();
            rail_dialogs.extend(
                self.dialogs
                    .iter()
                    .filter(|d| !self.archived_ids.contains(&d.id) && !d.is_pinned),
            );
            return render_collapsed_gemini_sidebar(
                &nav_snapshot,
                &rail_dialogs,
                active_id,
                self.hovered_rail_item,
                self.hovered_rail_dialog_id,
                &palette,
                &lang_id,
                cx,
                |this, action, cx| this.handle_unified_action(action, cx),
            );
        }

        let categorized_results = if is_search_focused {
            self.categorized_search_results()
        } else {
            Vec::new()
        };

        let top_bar = render_expanded_gemini_top_bar(
            &nav_snapshot,
            is_plus_open || creation_modal.is_some(),
            self.hovered_rail_item,
            &palette,
            &lang_id,
            cx,
            |this, action, cx| this.handle_unified_action(action, cx),
        );
        let search_bar = render_expanded_gemini_search_bar(
            &query,
            is_search_focused,
            &palette,
            &lang_id,
            cx,
            |this, action, cx| this.handle_unified_action(action, cx),
        );
        let footer = render_expanded_gemini_footer(
            &nav_snapshot,
            &palette,
            &lang_id,
            cx,
            |this, action, cx| this.handle_unified_action(action, cx),
        );

        let plus_dropdown = if is_plus_open {
            Some(render_plus_dropdown(
                &palette,
                &lang_id,
                cx,
                |this, cx| {
                    this.is_plus_menu_open = false;
                    cx.notify();
                },
                |this, kind, cx| {
                    this.open_add_contact_modal(kind);
                    cx.notify();
                },
                |this, cx| {
                    this.open_create_group_modal();
                    cx.notify();
                },
                |this, cx| {
                    this.is_plus_menu_open = false;
                    this.pending_open_new_channel = true;
                    cx.notify();
                },
                |this, cx| {
                    this.open_contacts_modal();
                    cx.notify();
                },
            ))
        } else {
            None
        };

        let search_panel = if is_search_focused {
            let peers_list = self.top_peers.entries.clone();
            let scope_ui = self.search_in_ui.clone();
            Some(render_search_panel(
                &active_search_cat,
                &active_search_tag,
                &peers_list,
                &scope_ui,
                &palette,
                &lang_id,
                cx,
                |this, cid, cx| {
                    this.select_search_category(cid);
                    cx.notify();
                },
                |this, tid, cx| {
                    this.select_search_tag(tid);
                    cx.notify();
                },
                |this, pid, cx| {
                    this.search_focused = false;
                    this.select_dialog_from_user(pid);
                    cx.notify();
                },
                |this, cx| {
                    this.search_in_ui.clear_scope();
                    cx.notify();
                },
            ))
        } else {
            None
        };

        let suggestion_bar = {
            let sug_ui = self.suggestions.clone();
            render_suggestion_bar(&sug_ui, &palette, cx, |this, cx| {
                this.suggestions.dismiss_current(&mut this.suggestion_manager);
                cx.notify();
            })
        };

        let stories_strip = {
            let stories_ui = self.stories.clone();
            render_stories_strip(
                &stories_ui,
                &palette,
                &lang_id,
                cx,
                |this, cx| {
                    this.stories.toggle_expanded();
                    cx.notify();
                },
                |this, sid, peer, cx| {
                    this.story_clicked_peer = Some(peer);
                    this.stories.active_story_id = Some(sid);
                    this.select_dialog_from_user(sid);
                    cx.notify();
                },
            )
        };

        let folder_tabs = if !is_search_focused && self.folders.len() > 1 {
            let folders_list = self.folders.clone();
            Some(render_folder_tabs_strip(
                &folders_list,
                &active_folder,
                &palette,
                &lang_id,
                cx,
                |this, fname, cx| {
                    this.select_folder(fname);
                    cx.notify();
                },
            ))
        } else {
            None
        };

        let scroll_children: Vec<AnyElement> = if is_search_focused {
            let q_lower = query.trim().to_lowercase();
            let show_apps_catalog = active_search_cat.eq_ignore_ascii_case("Apps")
                || q_lower.contains("botfather")
                || query.contains("机器人之父")
                || query.contains("小程序")
                || q_lower.contains("miniapp");
            let mut items: Vec<AnyElement> = Vec::new();
            if show_apps_catalog {
                items.extend(render_apps_catalog_section_with_recent(
                    &query,
                    &self.recent_mini_apps,
                    &self.catalog_mini_apps,
                    &palette,
                    &lang_id,
                    cx,
                    |this, cx| {
                        this.search_focused = false;
                        this.pending_open_botfather = true;
                        cx.notify();
                    },
                    |this, app, cx| {
                        this.search_focused = false;
                        this.pending_open_bot_profile = Some(app.bot_username);
                        cx.notify();
                    },
                ));
            }
            if !categorized_results.is_empty() || items.is_empty() {
                items.extend(render_search_results_list(
                    &query,
                    categorized_results,
                    &palette,
                    &lang_id,
                    cx,
                    |this, cid, cx| {
                        this.search_focused = false;
                        if this.dialogs.iter().any(|d| d.id == cid) {
                            this.select_dialog_from_user(cid);
                        } else if let Some(hit) = this
                            .remote_search_results
                            .iter()
                            .find(|r| r.chat_id == cid)
                            .cloned()
                        {
                            if hit.title.eq_ignore_ascii_case("BotFather")
                                || hit.subtitle.to_lowercase().contains("@botfather")
                            {
                                this.pending_open_botfather = true;
                            } else {
                                this.pending_open_search_hit = Some(hit);
                            }
                        } else {
                            this.select_dialog_from_user(cid);
                        }
                        cx.notify();
                    },
                ));
            }
            items
        } else {
            let is_default_all = active_folder == "All" && query.trim().is_empty();
            let active_cf_pinned: Vec<i64> = self
                .custom_folders
                .iter()
                .find(|cf| cf.title == active_folder)
                .map(|cf| cf.pinned_chats.clone())
                .unwrap_or_default();
            let filtered: Vec<DialogItemModel> =
                self.filtered_dialogs().into_iter().cloned().collect();
            let mut archived: Vec<DialogItemModel> = Vec::new();
            let mut pinned: Vec<DialogItemModel> = Vec::new();
            let mut regular: Vec<DialogItemModel> = Vec::new();
            for mut item in filtered {
                let folder_pinned = active_cf_pinned.contains(&(item.id as i64));
                if folder_pinned {
                    item.is_pinned = true;
                }
                if is_default_all && self.archived_ids.contains(&item.id) {
                    archived.push(item);
                } else if item.is_pinned {
                    pinned.push(item);
                } else {
                    regular.push(item);
                }
            }
            let archived_with_meta: Vec<(DialogItemModel, SidebarDialogMeta)> = archived
                .iter()
                .map(|d| (d.clone(), self.dialog_meta(d)))
                .collect();
            let main_with_meta: Vec<(DialogItemModel, SidebarDialogMeta)> = pinned
                .into_iter()
                .chain(regular.into_iter())
                .map(|d| (d.clone(), self.dialog_meta(&d)))
                .collect();

            let mut list: Vec<AnyElement> = Vec::new();
            if let Some(arch_header) = render_archived_header_row(
                &archived,
                self.is_archived_expanded,
                &palette,
                &lang_id,
                cx,
                |this, cx| {
                    this.is_archived_expanded = !this.is_archived_expanded;
                    cx.notify();
                },
            ) {
                list.push(arch_header);
            }
            if self.is_archived_expanded {
                for (idx, (item, meta)) in archived_with_meta.iter().enumerate() {
                    list.push(render_sidebar_dialog_row(
                        item,
                        idx,
                        active_id,
                        is_narrow,
                        meta,
                        &palette,
                        &lang_id,
                        cx,
                    ));
                }
            }

            let selected_topic = self.selected_topic_id;
            for (idx, (item, meta)) in main_with_meta.iter().enumerate() {
                list.push(render_sidebar_dialog_row(
                    item,
                    idx,
                    active_id,
                    is_narrow,
                    meta,
                    &palette,
                    &lang_id,
                    cx,
                ));
                if meta.is_topics_expanded {
                    if let Some(ref topics_ui) = meta.topics {
                        list.push(render_forum_topics_subtree(
                            topics_ui,
                            selected_topic,
                            &palette,
                            cx,
                            |this, fid, tid, cx| {
                                this.selected_topic_id = Some((fid, tid));
                                this.select_dialog_from_user(fid);
                                cx.notify();
                            },
                        ));
                    }
                }
            }

            if list.is_empty() {
                list.push(
                    div()
                        .id("sidebar-empty-dialogs-state")
                        .w_full()
                        .py(px(40.0))
                        .px(px(16.0))
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(10.0))
                        .child(tg_icon(IconName::FolderChat, 32.0, palette.text_secondary))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(palette.text_primary))
                                .child(tr_str(&lang_id, "暂无对话", "No chats yet")),
                        )
                        .child(
                            div()
                                .text_size(px(11.5))
                                .text_color(rgb(palette.text_secondary))
                                .child(tr_str(
                                    &lang_id,
                                    "点击右上角图标添加联系人或创建群组",
                                    "Start a chat or connect your workspace",
                                )),
                        )
                        .into_any_element(),
                );
            }
            list
        };

        let ctx_menu_overlay = if let Some((ctx_id, anchor)) = self.context_menu {
            let (is_pinned, is_muted, has_unread) = self
                .dialogs
                .iter()
                .find(|d| d.id == ctx_id)
                .map(|d| {
                    (
                        d.is_pinned,
                        d.is_muted,
                        d.unread_count > 0 || self.unread_marked_ids.contains(&ctx_id),
                    )
                })
                .unwrap_or((false, false, false));
            let is_archived = self.archived_ids.contains(&ctx_id);
            Some(render_dialog_context_menu(
                ctx_id,
                anchor,
                is_pinned,
                is_muted,
                is_archived,
                has_unread,
                &palette,
                &lang_id,
                cx,
                |this, action, cx| {
                    this.apply_context_action(action);
                    cx.notify();
                },
                |this, cx| {
                    this.context_menu = None;
                    cx.notify();
                },
            ))
        } else {
            None
        };

        let _ = (contact_kind, active_field, &primary_val, &secondary_val, &tertiary_val);
        let accent = rgb(palette.accent);
        let scaled_sidebar_width = palette.scaled_px(self.width).min(460.0);
        div()
            .relative()
            .w(px(scaled_sidebar_width))
            .flex_shrink_0()
            .h_full()
            .overflow_hidden()
            .bg(rgb(palette.dialogs_bg))
            .border_r_1()
            .border_color(rgb(palette.border_fg))
            .flex()
            .flex_col()
            .on_mouse_move(cx.listener(|this, ev: &MouseMoveEvent, _w, cx| {
                if this.is_resizing {
                    if let Some(start_x) = this.resize_drag_start_x {
                        let dx = f32::from(ev.position.x) - start_x;
                        this.set_width(this.resize_start_width + dx);
                        cx.notify();
                    }
                }
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _ev: &MouseUpEvent, _w, cx| {
                if this.is_resizing {
                    this.is_resizing = false;
                    this.resize_drag_start_x = None;
                    cx.notify();
                }
            }))
            .child(
                div()
                    .id("sidebar-resize-handle")
                    .absolute()
                    .top_0()
                    .right_0()
                    .w(px(4.0))
                    .h_full()
                    .cursor_col_resize()
                    .hover(move |s| s.bg(accent))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, ev: &MouseDownEvent, _w, cx| {
                        this.is_resizing = true;
                        this.resize_drag_start_x = Some(f32::from(ev.position.x));
                        this.resize_start_width = this.width;
                        cx.notify();
                    })),
            )
            .child(top_bar)
            .child(search_bar)
            .when_some(search_panel, |el, panel| el.child(panel))
            .when_some(suggestion_bar, |el, bar| el.child(bar))
            .when_some(stories_strip, |el, strip| el.child(strip))
            .when_some(folder_tabs, |el, tabs| el.child(tabs))
            .child(
                div()
                    .id("sidebar-dialogs-scroll")
                    .flex_1()
                    .min_h(px(0.0))
                    .w_full()
                    .overflow_y_scroll()
                    .when(palette.mode.is_fabushi_style(), |el| {
                        el.px(px(palette.scaled_px(6.0))).gap(px(1.0))
                    })
                    .flex()
                    .flex_col()
                    .children(scroll_children),
            )
            .child(footer)
            .when_some(plus_dropdown, |el, panel| el.child(panel))
            .when_some(ctx_menu_overlay, |el, overlay| el.child(overlay))
            .into_any_element()
    }
}
