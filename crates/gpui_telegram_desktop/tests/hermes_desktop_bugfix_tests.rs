use gpui_telegram_client::net::hermes_client::{
    HermesApprovalDto, HermesCommandDescriptor, HermesCronJobDto, HermesHeartbeatConfigDto,
    HermesHonchoStateDto, HermesInlineButtonDto, HermesKanbanBoardDto, HermesKanbanCardDto,
    HermesMemoryEntryDto, HermesMemoryStateDto, HermesProviderConfigDto, HermesRichMessageDto,
    HermesSessionDto, HermesSessionExportResponse, HermesSkillDto, HermesSoulDto,
    HermesStatusDto, HermesTerminalProcessDto, HermesUsageSummaryDto,
};
use gpui_telegram_client::ChatSummary;
use gpui_telegram_desktop::views::app_shell::shell_truth::{
    apply_authoritative_chat_list, decode_inline_keyboard_json, derive_initials,
    hermes_keyboard_to_stage, overlay_rich_message_metadata, select_authoritative_dialog,
};
use gpui_telegram_desktop::views::canonical_composer::extract_context_reference_badges;
use gpui_telegram_desktop::views::global_tasks_workspace::{
    default_28_hermes_toolsets, StudioInputTarget, TasksWorkspaceAction, TasksWorkspaceTab,
};
use gpui_telegram_desktop::views::stage::cards::{
    DeliveryStatus, InlineButtonType, MessageContent, MessageDirection, MessageItemModel,
};
use gpui_telegram_desktop::views::{
    CanonicalComposer, ChatSidebar, ContextInspector, ConversationStage, GlobalTasksWorkspace,
};
use serde_json::json;
use std::path::PathBuf;

fn desktop_src_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

#[test]
fn test_derive_initials_numeric_unicode_and_profile_inspector_sync() {
    // Regression: numeric chat title "23" previously yielded "?"
    assert_eq!(derive_initials("23"), "23");
    assert_eq!(derive_initials("404"), "40");
    // Unicode CJK titles
    assert_eq!(derive_initials("大乘"), "大");
    assert_eq!(derive_initials("Rust Core"), "RC");
    assert_eq!(derive_initials("   "), "");

    let mut sidebar = ChatSidebar::new();
    let mut stage = ConversationStage::new();
    let mut inspector = ContextInspector::new();

    let chats = vec![ChatSummary {
        chat_id: 23,
        title: "23".to_string(),
        chat_type: 4,
        last_message: Some("Ready".to_string()),
        unread_count: 1,
        is_group: Some(false),
        is_bot: Some(true),
        avatar_color: Some(0x2AABEE),
        updated_at: Some(1_700_000_000_000),
    }];
    let selected = apply_authoritative_chat_list(&mut sidebar, &mut stage, &mut inspector, &chats);
    assert_eq!(selected, None);
    assert_eq!(sidebar.dialogs.len(), 1);
    assert_eq!(sidebar.dialogs[0].folder, "Bots");
    assert_eq!(sidebar.dialogs[0].avatar_initials, "23");

    assert!(select_authoritative_dialog(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        23,
    ));
    assert_eq!(stage.chat_title, "23");
    assert_eq!(stage.chat_avatar_initials, "23");
    assert_eq!(inspector.profile.name, "23");
    assert_eq!(inspector.profile.avatar_initials, "23");
    assert_eq!(inspector.profile.status, "");

    inspector.set_peer("23", "Custom Hermes persona", "23", 0x2AABEE);
    inspector.sync_profile_details(
        "@custom_ai_bot".to_string(),
        String::new(),
        "Hermes AI Bot".to_string(),
        None,
        vec![("Engine".to_string(), "hermes-rust".to_string())],
    );
    assert_eq!(inspector.profile.username, "@custom_ai_bot");
    assert_eq!(inspector.profile.status, "Hermes AI Bot");
    assert_eq!(inspector.profile.bio, "Custom Hermes persona");
    assert_eq!(
        inspector.profile.facts,
        vec![("Engine".to_string(), "hermes-rust".to_string())]
    );
}

#[test]
fn test_rich_message_overlay_resolves_peer_identity_and_preserves_bot_keyboard() {
    let mut models = vec![MessageItemModel {
        id: 501,
        sender_id: 777_001,
        sender_name: "User 777001".to_string(),
        avatar_initials: "U7".to_string(),
        avatar_color: 0x2AABEE,
        direction: MessageDirection::Incoming,
        timestamp: "14:20".to_string(),
        delivery_status: DeliveryStatus::Sent,
        reply_to: None,
        reactions: Vec::new(),
        content: MessageContent::Text {
            text: "你好，我已经准备好协助你。".to_string(),
            is_edited: false,
        },
        bot_keyboard: None,
        is_pinned: false,
    }];

    let rich = vec![HermesRichMessageDto {
        message_id: 501,
        chat_id: 23,
        sender_id: 777_001,
        sender_name: Some("Kestrel AI".to_string()),
        sender_initials: Some("KA".to_string()),
        is_bot_sender: true,
        text: "你好，我已经准备好协助你。".to_string(),
        content: None,
        date_ms: Some(1_700_000_000_000),
        reply_to: None,
        is_outgoing: Some(false),
        inline_keyboard: Some(vec![vec![
            HermesInlineButtonDto {
                text: "Launch Talk Mode".to_string(),
                callback_data: Some("talk:start".to_string()),
                url: None,
                web_app_url: None,
            },
            HermesInlineButtonDto {
                text: "Remote Computer".to_string(),
                callback_data: Some("cua:open".to_string()),
                url: None,
                web_app_url: None,
            },
        ]]),
        bot_keyboard: None,
    }];

    overlay_rich_message_metadata(&mut models, &rich);
    assert_eq!(models[0].sender_name, "Kestrel AI");
    assert_ne!(models[0].sender_name, "Peer");
    assert_eq!(models[0].avatar_initials, "KA");
    assert_ne!(models[0].avatar_initials, "PR");

    let kb = models[0]
        .bot_keyboard
        .as_ref()
        .expect("bot_keyboard preserved after rich overlay");
    assert_eq!(kb.len(), 1);
    assert_eq!(kb[0].len(), 2);
    assert_eq!(kb[0][0].text, "Launch Talk Mode");
    assert_eq!(
        kb[0][0].button_type,
        InlineButtonType::Callback("talk:start".to_string())
    );
    assert_eq!(kb[0][1].text, "Remote Computer");
    assert_eq!(
        kb[0][1].button_type,
        InlineButtonType::Callback("cua:open".to_string())
    );

    // JSON keyboard decode + URL / WebApp conversions
    let decoded = decode_inline_keyboard_json(&json!([[
        { "text": "Docs", "url": "https://example.com/docs" },
        { "text": "Mini App", "web_app_url": "https://example.com/app" }
    ]]))
    .expect("json inline keyboard decodes");
    assert_eq!(
        decoded[0][0].button_type,
        InlineButtonType::Url("https://example.com/docs".to_string())
    );
    assert_eq!(
        decoded[0][1].button_type,
        InlineButtonType::WebApp("https://example.com/app".to_string())
    );

    let converted = hermes_keyboard_to_stage(&[vec![HermesInlineButtonDto {
        text: "Run /help".to_string(),
        callback_data: Some("/help".to_string()),
        url: None,
        web_app_url: None,
    }]])
    .expect("hermes keyboard converts");
    assert_eq!(
        converted[0][0].button_type,
        InlineButtonType::Callback("/help".to_string())
    );
}

#[test]
fn test_canonical_composer_slash_autocomplete_and_context_badges() {
    let mut composer = CanonicalComposer::new();
    assert!(!composer.hermes_slash_enabled);
    composer.hermes_slash_enabled = true;
    assert!(
        composer.hermes_commands.len() >= 50,
        "Expected 50+ Hermes slash command autocomplete entries, got {}",
        composer.hermes_commands.len()
    );
    for required in [
        "/help",
        "/memory",
        "/soul",
        "/kanban",
        "/cron",
        "/goal",
        "/moa",
        "/skills",
        "/toolsets",
        "/rollback",
        "/undo",
        "/yolo",
        "/talk",
        "/export",
    ] {
        assert!(
            composer.hermes_commands.iter().any(|c| c.slash_name == required),
            "Missing Hermes command {required} in CanonicalComposer"
        );
    }

    // Typing slash query opens autocomplete & matches
    composer.set_text("/mem");
    composer.ime_machine.cursor_index = composer.text().len();
    composer.sync_autocomplete();
    assert!(composer.hermes_ac_visible);
    assert!(composer
        .hermes_ac_matches
        .iter()
        .any(|c| c.slash_name == "/memory"));

    // Keyboard navigation and completion
    assert!(composer.handle_hermes_autocomplete_nav("down", false));
    assert!(composer.handle_hermes_autocomplete_nav("up", false));
    assert!(composer.handle_hermes_autocomplete_nav("tab", false));
    assert!(!composer.hermes_ac_visible);
    assert!(composer.text().starts_with("/"));

    // Escape hides popup cleanly
    composer.set_text("/kan");
    composer.ime_machine.cursor_index = composer.text().len();
    composer.sync_autocomplete();
    assert!(composer.hermes_ac_visible);
    assert!(composer.handle_hermes_autocomplete_nav("escape", false));
    assert!(!composer.hermes_ac_visible);

    // Custom server descriptor sync replaces command table cleanly
    composer.set_hermes_commands(vec![HermesCommandDescriptor {
        name: "kanban".to_string(),
        slash_name: "/kanban".to_string(),
        aliases: vec!["board".to_string()],
        syntax: "/kanban [list|add]".to_string(),
        category: "tasks".to_string(),
        description: "Manage Kanban boards".to_string(),
        accepts_args: true,
    }]);
    composer.set_text("/boa");
    composer.ime_machine.cursor_index = composer.text().len();
    composer.sync_autocomplete();
    assert!(composer.hermes_ac_visible);

    // @file / @dir / @diff / @url context badges
    let badges = extract_context_reference_badges(
        "Check @file:src/main.rs, @dir:crates/agent, @diff and @url:https://hermes.local/spec",
    );
    assert_eq!(badges.len(), 4);
    assert_eq!(badges[0].0, "FILE");
    assert_eq!(badges[1].0, "DIR");
    assert_eq!(badges[2].0, "GIT");
    assert_eq!(badges[3].0, "URL");
}

#[test]
fn test_global_tasks_workspace_6_hermes_studio_tabs_and_state_sync() {
    let default_toolsets = default_28_hermes_toolsets();
    assert_eq!(default_toolsets.len(), 28);

    let mut ws = GlobalTasksWorkspace::new();
    assert_eq!(ws.toolsets.len(), 28);
    assert_eq!(ws.status_summary.toolsets_count, 28);
    assert_eq!(ws.status_summary.tools_count, 58);

    let tab_cases = [
        (TasksWorkspaceTab::Tasks, 0usize),
        (TasksWorkspaceTab::Skills, 1usize),
        (TasksWorkspaceTab::MemorySoul, 2usize),
        (TasksWorkspaceTab::McpTools, 3usize),
        (TasksWorkspaceTab::Cron, 4usize),
        (TasksWorkspaceTab::Sessions, 5usize),
    ];
    for (tab, expected_idx) in tab_cases {
        ws.set_tab(tab);
        assert_eq!(ws.tab, tab);
        assert_eq!(ws.active_tab, expected_idx);
    }
    ws.set_tab(TasksWorkspaceTab::Approvals);
    assert_eq!(ws.tab, TasksWorkspaceTab::Approvals);
    assert_eq!(ws.active_tab, 0);

    ws.sync_studio_status(HermesStatusDto {
        healthy: true,
        engine: "hermes-rust".to_string(),
        active_model: "claude-sonnet-4".to_string(),
        active_provider: "anthropic".to_string(),
        api_mode: "anthropic_messages".to_string(),
        toolsets_count: 28,
        tools_count: 58,
        skills_count: 3,
        cron_count: 2,
        sessions_count: 4,
        pending_approvals_count: 1,
        heartbeat_enabled: true,
        ..Default::default()
    });
    assert_eq!(ws.active_api_mode, "anthropic_messages");

    ws.sync_kanban_boards(vec![HermesKanbanBoardDto {
        board_id: "b1".to_string(),
        name: "Hermes Sprint".to_string(),
        columns: vec!["Backlog".into(), "Doing".into(), "Done".into()],
        cards: vec![HermesKanbanCardDto {
            card_id: "c1".to_string(),
            board_id: "b1".to_string(),
            title: "Wire Studio".to_string(),
            description: "Wire Hermes Studio panels".to_string(),
            column: "Doing".to_string(),
            priority: "high".to_string(),
            assigned_agent: Some("hermes".to_string()),
            position: 0,
            created_at_ms: 1,
            updated_at_ms: 1,
        }],
        created_at_ms: 1,
    }]);
    assert_eq!(ws.active_board_id, "b1");

    ws.sync_hermes_approvals(vec![HermesApprovalDto {
        approval_id: "apr_1".to_string(),
        session_id: "s1".to_string(),
        tool_name: "terminal".to_string(),
        command_preview: "cargo check".to_string(),
        risk_level: "caution".to_string(),
        reason: "Confirmation needed".to_string(),
        status: "pending".to_string(),
        created_at_ms: 1,
        decided_at_ms: None,
    }]);
    assert_eq!(ws.hermes_approvals.len(), 1);

    ws.sync_skills(vec![HermesSkillDto {
        name: "wal-audit".to_string(),
        description: "Audits SQLite WAL".to_string(),
        category: "user".to_string(),
        content: "# WAL Audit".to_string(),
        enabled: true,
        approved: true,
        security_status: "safe".to_string(),
        tags: vec!["sqlite".into()],
        updated_at_ms: 1,
    }]);
    assert_eq!(ws.selected_skill_name.as_deref(), Some("wal-audit"));

    ws.sync_memory_state(HermesMemoryStateDto {
        entries: vec![HermesMemoryEntryDto {
            id: "mem_0".into(),
            key: "mem_0".into(),
            content: "Keep debug=0".into(),
            category: "memory".into(),
            scope: "memory".into(),
            char_count: 12,
            updated_at_ms: 1,
        }],
        memory_markdown: "Keep debug=0".into(),
        user_markdown: "Prefers Rust".into(),
        total_chars: 12,
        char_limit: 2200,
    });
    assert_eq!(ws.memory_edit_draft, "Keep debug=0");
    assert_eq!(ws.user_edit_draft, "Prefers Rust");

    ws.sync_soul_state(HermesSoulDto {
        bot_id: None,
        chat_id: None,
        persona_name: "Kestrel".into(),
        content: "Zero legacy persona".into(),
        system_prompt: None,
        welcome_message: None,
        model: Some("claude-sonnet-4".into()),
        updated_at_ms: 1,
    });
    assert_eq!(ws.soul_persona_draft, "Kestrel");

    ws.sync_honcho_state(HermesHonchoStateDto {
        enabled: true,
        peer_name: Some("Alice".into()),
        session_notes: Vec::new(),
        profile_summary: "Systems engineer".into(),
        updated_at_ms: 1,
    });
    assert!(ws.honcho_state.enabled);

    ws.sync_terminal_processes(vec![HermesTerminalProcessDto {
        process_id: "proc_1".into(),
        session_id: Some("s1".into()),
        command: "cat".into(),
        status: "running".into(),
        pid: Some(4242),
        exit_code: None,
        started_at_ms: 1,
        output_tail: Some("ready".into()),
    }]);
    assert_eq!(ws.terminal_processes.len(), 1);

    ws.sync_cron_jobs(vec![HermesCronJobDto {
        job_id: "cron_1".into(),
        name: "Nightly".into(),
        schedule: "0 2 * * *".into(),
        prompt: "Summarize".into(),
        chat_id: Some(23),
        bot_id: None,
        enabled: true,
        last_run_ms: None,
        next_run_ms: Some(2),
        last_status: None,
    }]);
    ws.sync_heartbeat(HermesHeartbeatConfigDto {
        enabled: true,
        interval_secs: 600,
        chat_id: Some(23),
        bot_id: None,
        prompt: "Check tasks".into(),
        last_beat_ms: None,
    });
    assert_eq!(ws.cron_jobs.len(), 1);
    assert!(ws.heartbeat.enabled);

    ws.sync_sessions(vec![HermesSessionDto {
        session_id: "s1".into(),
        chat_id: Some(23),
        bot_id: None,
        title: Some("Primary".into()),
        model: "claude-sonnet-4".into(),
        provider: "anthropic".into(),
        parent_session_id: None,
        branch_name: None,
        message_count: 4,
        token_count: 100,
        created_at_ms: 1,
        updated_at_ms: 2,
    }]);
    ws.sync_session_search_hits(vec![json!({ "session_id": "s1", "snippet": "Primary" })]);
    ws.sync_session_export(HermesSessionExportResponse {
        session_id: "s1".into(),
        format: "sharegpt".into(),
        content: "{\"conversations\":[]}".into(),
        exported_at_ms: 2,
    });
    ws.sync_providers(vec![HermesProviderConfigDto {
        provider_id: "openai".into(),
        name: "OpenAI".into(),
        api_mode: "codex_responses".into(),
        base_url: None,
        default_model: "gpt-5".into(),
        models: vec!["gpt-5".into()],
        enabled: true,
        has_api_key: true,
        is_active: true,
    }]);
    ws.sync_usage_summary(HermesUsageSummaryDto {
        total_turns: 5,
        prompt_tokens: 500,
        completion_tokens: 300,
        total_tokens: 800,
        estimated_cost_usd: 0.02,
        tool_calls: 2,
        by_model: Vec::new(),
    });
    assert_eq!(ws.active_api_mode, "codex_responses");
    assert_eq!(ws.subagent_lineage.len(), 1);

    // Interactive input emission
    ws.active_input = StudioInputTarget::NewTask;
    ws.task_draft = "Audit WAL lock".to_string();
    assert!(ws.apply_text_key_to_active_input("enter", None));
    assert_eq!(
        ws.take_action(),
        Some(TasksWorkspaceAction::CreateTask("Audit WAL lock".to_string()))
    );
}

#[test]
fn test_desktop_bot_call_inline_routing_and_zero_legacy_persona_contract() {
    let root = desktop_src_root();
    let stage_actions_src =
        std::fs::read_to_string(root.join("views/app_shell/shell_stage_actions.rs")).unwrap();
    let calls_rtc_src =
        std::fs::read_to_string(root.join("views/app_shell/shell_calls_rtc.rs")).unwrap();

    let mut shell_paths = vec![root.join("views/shell.rs")];
    let mut app_shell_paths: Vec<_> = std::fs::read_dir(root.join("views/app_shell"))
        .expect("Failed to read views/app_shell")
        .filter_map(|e| e.ok().map(|entry| entry.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("shell_") && n.ends_with(".rs"))
                .unwrap_or(false)
        })
        .collect();
    app_shell_paths.sort();
    assert!(!app_shell_paths.is_empty(), "Expected views/app_shell/shell_*.rs files");
    shell_paths.extend(app_shell_paths);

    let mut all_shell_src = String::new();
    for path in &shell_paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("shell");
        let src = std::fs::read_to_string(path).unwrap();
        assert!(
            !src.contains("大乘 AI 助手"),
            "{name} must never hardcode legacy persona string"
        );
        assert!(
            !src.contains("\"PR\".to_string()"),
            "{name} must never hardcode fallback PR initials"
        );
        all_shell_src.push('\n');
        all_shell_src.push_str(&src);
    }

    let mut studio_paths = vec![root.join("views/hermes_studio_panels.rs")];
    let mut studio_subpaths: Vec<_> = std::fs::read_dir(root.join("views/hermes_studio_panels"))
        .expect("Failed to read views/hermes_studio_panels")
        .filter_map(|e| e.ok().map(|entry| entry.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rs"))
        .collect();
    studio_subpaths.sort();
    assert!(
        !studio_subpaths.is_empty(),
        "Expected views/hermes_studio_panels/*.rs files"
    );
    studio_paths.extend(studio_subpaths);
    for path in studio_paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("studio");
        let src = std::fs::read_to_string(&path).unwrap();
        assert!(
            !src.contains("大乘 AI 助手"),
            "{name} must never hardcode legacy persona string"
        );
    }

    // Incoming NewMessage uses resolve_incoming_sender_identity rather than hardcoded Peer/PR
    assert!(all_shell_src.contains("resolve_incoming_sender_identity"));
    assert!(!all_shell_src.contains("\"PR\".to_string()"));

    // Bot inline callback + TopBar AI Bot call route through Hermes call/Talk/CUA handlers
    assert!(stage_actions_src.contains("execute_hermes_inline_callback"));
    assert!(calls_rtc_src.contains("pub fn is_chat_bot"));
    assert!(calls_rtc_src.contains("pub fn is_active_chat_bot"));
    assert!(calls_rtc_src.contains("pub fn current_peer_user_id"));
    assert!(calls_rtc_src.contains("hermes_call_invite") || stage_actions_src.contains("hermes_call_invite"));
}

#[test]
fn test_rich_text_multiline_and_cjk_inline_wrapping_regression() {
    use gpui_telegram_desktop::views::stage::{
        parse_rich_blocks, split_wrappable_inline_tokens, EntityKind, RichBlock, RichEntity,
    };

    let sample = "你好！我是 1 (@1_bot).\n\n你可以做什么\n使用 `read file <path>` 查看文件";
    let blocks = parse_rich_blocks(sample, &[]);

    assert!(blocks.len() >= 4, "expected line-per-block paragraphs, got {blocks:?}");
    for block in &blocks {
        if let RichBlock::Paragraph(segs) = block {
            for seg in segs {
                assert!(
                    !seg.text.contains('\n') && !seg.text.contains('\r'),
                    "paragraph segment must not contain embedded newlines: {:?}",
                    seg.text
                );
            }
        }
    }

    let first_line = match &blocks[0] {
        RichBlock::Paragraph(segs) => segs,
        other => panic!("expected Paragraph first block, got {other:?}"),
    };
    assert!(
        first_line.iter().any(|s| s.flags.mention.as_deref() == Some("@1_bot")),
        "expected @1_bot mention in first paragraph line: {first_line:?}"
    );
    assert!(
        first_line.last().map(|s| s.text.as_str()) == Some(")."),
        "expected trailing ').' to stay on first paragraph line: {first_line:?}"
    );

    // Explicit UTF-16 entity parsing must also split lines without embedded newlines
    let explicit_sample = "第一行 (@1_bot).\n\n第二行 你可以做什么";
    let utf16_len = explicit_sample.encode_utf16().count();
    let mention_utf16_offset = "第一行 (".encode_utf16().count();
    let explicit_entities = vec![RichEntity {
        offset_utf16: mention_utf16_offset,
        length_utf16: "@1_bot".encode_utf16().count(),
        kind: EntityKind::Mention,
    }];
    assert!(utf16_len > mention_utf16_offset);
    let explicit_blocks = parse_rich_blocks(explicit_sample, &explicit_entities);
    assert_eq!(explicit_blocks.len(), 3, "expected 2 content lines + 1 blank spacer, got {explicit_blocks:?}");
    for block in &explicit_blocks {
        if let RichBlock::Paragraph(segs) = block {
            for seg in segs {
                assert!(!seg.text.contains('\n') && !seg.text.contains('\r'));
            }
        }
    }

    // CJK wrapping keeps closing punctuation glued to preceding token and strips newlines
    let tokens = split_wrappable_inline_tokens("我可以帮你执行。\n(Session turns: 3)");
    assert!(tokens.iter().all(|t| !t.contains('\n') && !t.contains('\r')));
    assert!(
        tokens.iter().any(|t| t == "行。"),
        "closing punctuation should attach to preceding CJK character: {tokens:?}"
    );
}

#[test]
fn test_markdown_heading_divider_list_blockquote_and_link_parsing() {
    use gpui_telegram_desktop::views::stage::{parse_rich_blocks, plain_text_of_segments, RichBlock};

    let sample = concat!(
        "### 2. 💻 本地电脑控制\n",
        "- **文件与目录操作**: 查看日志 (`free -h`)\n",
        "> 💡 **关于安装 Hermes**: 说明文本\n",
        "---\n",
        "参考 [NousResearch](https://github.com/NousResearch/hermes-agent) 项目"
    );
    let blocks = parse_rich_blocks(sample, &[]);

    assert!(matches!(blocks.first(), Some(RichBlock::Heading { level: 3, .. })));
    if let RichBlock::Heading { segments, .. } = &blocks[0] {
        assert_eq!(plain_text_of_segments(segments), "2. 💻 本地电脑控制");
    }

    match &blocks[1] {
        RichBlock::ListItem { bullet, segments, .. } => {
            assert_eq!(bullet, "•");
            assert!(segments.iter().any(|s| s.flags.bold && s.text.contains("文件与目录操作")));
            assert!(segments.iter().any(|s| s.flags.code && s.text == "free -h"));
        }
        other => panic!("expected ListItem block, got {other:?}"),
    }

    match &blocks[2] {
        RichBlock::Blockquote { lines, .. } => {
            assert_eq!(lines.len(), 1);
            assert!(
                lines[0].iter().any(|s| s.flags.bold && s.text == "关于安装 Hermes"),
                "expected bold segment inside blockquote without literal asterisks: {lines:?}"
            );
            assert!(!plain_text_of_segments(&lines[0]).contains("**"));
        }
        other => panic!("expected Blockquote block, got {other:?}"),
    }

    assert!(matches!(blocks.get(3), Some(RichBlock::Divider)));

    match &blocks[4] {
        RichBlock::Paragraph(segs) => {
            assert!(
                segs.iter().any(|s| {
                    s.text == "NousResearch"
                        && s.flags.link_url.as_deref() == Some("https://github.com/NousResearch/hermes-agent")
                }),
                "expected parsed markdown link segment: {segs:?}"
            );
        }
        other => panic!("expected Paragraph block with link, got {other:?}"),
    }
}

#[test]
fn magpie_ai_providers_settings_search_main_and_studio_sync_work() {
    use gpui_telegram_client::net::hermes_client::{
        MagpieProviderDto, MagpieStateDto, MagpieSubscriptionDto,
    };
    use gpui_telegram_desktop::views::settings::{
        sections::SettingsMainSections, settings_search::SettingsSearch,
        settings_type::SettingsSectionType,
    };

    assert_eq!(SettingsSectionType::AiProviders.id_str(), "ai_providers");
    assert_eq!(
        SettingsSectionType::AiProviders.display_title_localized(true),
        "AI 订阅与模型供应商"
    );
    assert_eq!(
        SettingsSectionType::AiProviders.breadcrumb_path(),
        vec![SettingsSectionType::Main, SettingsSectionType::AiProviders]
    );

    let mut search = SettingsSearch::with_default_entries();
    let zh_hits = search.search("供应商");
    assert!(
        zh_hits.iter().any(|r| r.section == SettingsSectionType::AiProviders),
        "expected Chinese query 供应商 to match AiProviders"
    );
    let en_hits = search.search("magpie");
    assert!(
        en_hits.iter().any(|r| r.section == SettingsSectionType::AiProviders),
        "expected English query magpie to match AiProviders"
    );

    let mut main = SettingsMainSections::new();
    main.sync_ai_providers_status(Some("请选择供应商".to_string()), true);
    assert!(main.needs_provider_selection);
    assert_eq!(main.ai_providers_summary.as_deref(), Some("请选择供应商"));

    let mut ws = GlobalTasksWorkspace::new();
    ws.sync_magpie_state(MagpieStateDto {
        active_provider: "deepseek".to_string(),
        active_model: "deepseek-chat".to_string(),
        needs_provider_selection: false,
        has_configured_provider: true,
        has_active_subscription: true,
        providers: vec![MagpieProviderDto {
            provider_id: "deepseek".to_string(),
            name: "DeepSeek".to_string(),
            api_mode: "chat_completions".to_string(),
            default_model: "deepseek-chat".to_string(),
            is_active: true,
            enabled: true,
            ..Default::default()
        }],
        subscriptions: vec![MagpieSubscriptionDto {
            id: "sub-codex".to_string(),
            kind: "codex_chatgpt".to_string(),
            label: "Codex ChatGPT".to_string(),
            active: true,
            ..Default::default()
        }],
        routing_strategy: "smart".to_string(),
        ..Default::default()
    });
    assert_eq!(ws.active_api_mode, "chat_completions");
    assert!(!ws.status_summary.needs_provider_selection);
    assert!(ws.status_summary.has_active_subscription);
    assert!(ws.status_summary.has_configured_provider);
}
