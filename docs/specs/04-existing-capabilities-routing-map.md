# Specification 04: Existing Capabilities Telegram UI/UX Routing Map & Integration Spec

**Document ID**: `SPEC-04-CAPABILITIES-ROUTING-MAP`  
**Revision**: `3.0.0` (Milestone 1, Explorer Gen 3)  
**Date**: `2026-11-06`  
**Status**: Authoritative Technical Specification & Integration Blueprint  
**Target Repository**: `/Users/bhrum/teamwork_projects/gpui_telegram`  
**Target Path**: `docs/specs/04-existing-capabilities-routing-map.md`  

---

## 1. Executive Summary & Integration Principles

This specification defines the seamless, native integration of all existing advanced capabilities (AI Agents & Multi-Agent ACP, Primary Global Task Center & Cron, MCP & Plugin Ecosystem, Remote Computer & CUA Control, Full-Duplex Talk Mode, and Rich Interactive Cards) into the 1:1 GPUI Telegram Desktop client interface.

### 1.1 Non-Fragmented Native Integration
Rather than isolating advanced features in disjointed secondary windows or custom panels, every capability is mapped directly to natural Telegram Desktop interaction surfaces:
- **Left NavRail (52px)**: Primary entry points for Global Tasks (`Cmd+2`), Remote Devices (`Cmd+3`), and Extensions (`Cmd+4`).
- **Conversation Stage (Transcript)**: Native message bubbles embedding collapsible `AgentThinkingBubble`, 100-sample 5-bit voice waveform players, interactive A2UI cards, and Inline Keyboards.
- **Canonical Composer Dock**: Anchored tool permission docks (`LocalToolPermissionDock`), Model/Provider pills, and hold-to-record voice gesture bars.
- **Right Context Inspector Drawer (320px)**: Multi-tab inspection of Chat Profile, Members, Shared Media (6 categories), MCP Skills, Linked Tasks, and Routines.

### 1.2 Direct Reuse of Inherited Rust Capabilities (Mandate #1)
All underlying execution logic is powered by inherited Rust crates (`source/host`, `native/mahayana-messaging`, `source/node-agent-coordinator`, `SandAgentDb`). The GPUI presentation layer communicates with these runtimes via direct in-process channels and FFI bindings without IPC serialization latency.

## 2. Spec 04 Existing Capabilities Source-File Equivalence Ledger (既有能力源码文件对等账本)

| 核心能力域 (6 Domains) | 上游 OpenClaw 源码锚点 (`openclaw/`) | 上游 Telegram Desktop 源码锚点 (`tdesktop/`) | 已继承的成熟 Rust 模块路径 (`fabushi-unified-design-20261105`) | 目标 GPUI 客户端/运行时实现路径 (`crates/gpui_telegram_desktop/` & `crates/gpui_telegram_agent/`) | UI/UX 放置位置与交互组件 |
|---|---|---|---|---|---|
| **1. AI Multi-Agent ACP (智能体与多代理协作)** | `packages/acp-core/`<br>`src/agents/harness/acp.ts`<br>`src/agents/multi-agent/lineage.ts`<br>`packages/agent-core/` | `data/data_thread.cpp`<br>`data/data_forum_topic.cpp` | `source/host/src/runner/production_turn_agent_owner.rs`<br>`source/host/src/runner/subagent_runtime.rs`<br>`source/host/src/runner/sand_agent_runner.rs`<br>`source/node-agent-coordinator/src/inference_router.rs` | `crates/gpui_telegram_agent/src/acp/runner.rs`<br>`crates/gpui_telegram_agent/src/acp/lineage.rs`<br>`crates/gpui_telegram_desktop/src/components/thinking_bubble.rs`<br>`crates/gpui_telegram_desktop/src/components/composer_model_picker.rs` | 会话舞台气泡顶部 `AgentThinkingBubble`（折叠思考链）；Composer 顶条 `ModelSelectorPill`；抽屉 `SubagentLineageTree`。 |
| **2. Global Task Center & Cron (一级全局任务中心)** | `src/process/worker-state.ts`<br>`src/cron/schedule.ts`<br>`src/cron/schedule-identity.ts`<br>`src/security/approvals.ts`<br>`packages/worker-runtime/` | `api/api_sending.cpp` (`SEND-SCHEDULED`)<br>`history/view/controls/history_view_scheduled_box.cpp`<br>`storage/storage_account.cpp` | `source/host/src/runner/background_work.rs`<br>`source/host/src/runner/sand_auto_review.rs`<br>`source/host/src/automations/automation.rs`<br>`source/host/src/automations/automation_schedule.rs`<br>`source/host/src/automations/automation_trigger.rs` | `crates/gpui_telegram_agent/src/tasks/fsm.rs`<br>`crates/gpui_telegram_agent/src/tasks/cron_engine.rs`<br>`crates/gpui_telegram_agent/src/tasks/approval_gate.rs`<br>`crates/gpui_telegram_desktop/src/views/global_tasks_workspace.rs` | 左侧 NavRail 52px 导航项（快捷键 `Cmd+2`）；中央主舞台展示 3 栏任务板（`Waiting Approval`、`In Progress`、`Terminal`）及 Cron 编辑器。 |
| **3. MCP & Plugin Ecosystem (MCP 与插件工具生态)** | `src/mcp/`<br>`packages/plugin-sdk/`<br>`packages/tool-call-repair/`<br>`packages/net-policy/`<br>`extensions/code-mode-quickjs/` | `Telegram/SourceFiles/inline_bots/bot_attach_web_view.cpp`<br>`boxes/web_view_box.cpp`<br>`core/sandbox_rules.cpp` | `source/host/src/extensions/mcp/coordinator_relay.rs`<br>`source/host/src/extensions/mcp/plugin_skills_cache.rs`<br>`source/node-agent-coordinator/src/routed_mcp_bridge.rs`<br>`source/box-exec-daemon/src/main.rs`<br>`native/mahayana-messaging/src/miniapp.rs` | `crates/gpui_telegram_agent/src/mcp/registry.rs`<br>`crates/gpui_telegram_agent/src/mcp/permission_dock.rs`<br>`crates/gpui_telegram_desktop/src/components/tool_permission_dock.rs`<br>`crates/gpui_telegram_desktop/src/views/miniapp_sandbox.rs` | Composer 顶部门禁挂架 `LocalToolPermissionDock`；NavRail / 设置“扩展中心”；附件菜单与 Bot 按钮拉起独立沙箱 `MiniAppWindow`。 |
| **4. Remote Device & Cloud PC (远程设备与 CUA 控制)** | `extensions/cua-computer/`<br>`src/node-host/`<br>`crates/openclaw-node-host/` | `calls/calls_controller_webrtc.cpp` (Screen share) | `source/host/src/runner/computer_use.rs`<br>`source/host/src/remote_computer_target.rs`<br>`source/node-agent-coordinator/src/gateway/box_vnc_proxy.rs`<br>`source/host/src/network_enrollment_owner.rs` | `crates/gpui_telegram_agent/src/computer/cua_executor.rs`<br>`crates/gpui_telegram_agent/src/computer/control_lease.rs`<br>`crates/gpui_telegram_desktop/src/views/computer_workbench.rs`<br>`crates/gpui_telegram_desktop/src/components/vnc_canvas.rs` | 会话触发或 NavRail“设备”打开 `ComputerWorkbench`；实时 VNC Canvas；顶部高危指示黄条与 `Esc` / 一键紧急接管红按钮（Kill-Switch）。 |
| **5. Full-Duplex Talk Mode (全双工语音与 5-Bit 波形)** | `src/talk/talk-session-controller.ts`<br>`src/talk/exact-speech-protocol.ts`<br>`src/tts/`<br>`src/realtime-transcription/` | `Telegram/SourceFiles/media/audio/media_audio_waveform.cpp`<br>`history/view/media/history_view_voice.cpp`<br>`calls/calls_call.cpp` | `source/host/tests/openclaw_audio_talk_contract.rs` (12 test oracles)<br>`native/mahayana-messaging/src/realtime.rs`<br>`native/mahayana-messaging/src/message.rs` (`waveform: Vec<u8>`) | `crates/gpui_telegram_media/src/waveform_5bit.rs`<br>`crates/gpui_telegram_agent/src/talk/coordinator.rs`<br>`crates/gpui_telegram_agent/src/talk/vad_bargein.rs`<br>`crates/gpui_telegram_desktop/src/components/talk_mode_orb.rs`<br>`crates/gpui_telegram_desktop/src/components/voice_bubble_player.rs` | 会话顶栏电话/波形图标拉起 160px Canvas 动态能量光球 `TalkModeOrbModal`（支持最小化 PIP 胶囊）；Composer 按住麦克风滑动锁定；100 点 5-bit 波形播放气泡。 |
| **6. Rich Interactive Cards (富交互卡片与动作台)** | `src/session-cards/task-card.ts`<br>`src/interactive/a2ui.ts`<br>`packages/workboard-contract/` | `Telegram/SourceFiles/history/history_item.cpp`<br>`Telegram/SourceFiles/data/data_poll.cpp`<br>`Telegram/SourceFiles/payments/` | `source/node-agent-coordinator/src/tdrp/polls_todo.rs`<br>`source/node-agent-coordinator/src/tdrp/export_payments_bots.rs`<br>`native/mahayana-messaging/src/payment.rs`<br>`native/mahayana-messaging/src/wallet.rs` | `crates/gpui_telegram_proto/src/cards.rs`<br>`crates/gpui_telegram_desktop/src/components/card_poll.rs`<br>`crates/gpui_telegram_desktop/src/components/card_todo.rs`<br>`crates/gpui_telegram_desktop/src/components/card_invoice.rs`<br>`crates/gpui_telegram_desktop/src/components/inline_keyboard.rs` | 消息流行内渲染：A2UI 声明式卡片、投票选择条、Todo 交互复选框条目、Stars / USD 发票结账模态弹窗与 1-8 列 Inline Keyboard 按钮网格。 |

---

## 3. Exact Telegram 1:1 UI/UX Placement & Routing Mapping Table

| Capability | Domain | Telegram UI Surface Location | Specific Component / Widget | Trigger / Route | Visual & Interaction Specifications |
|---|---|---|---|---|---|
| **Global Task Center** | Tasks | Left NavRail (Primary Rail, 52px) | `NavRailItem::Tasks` (`GlobalTasksWorkspace`) | Click NavRail "Tasks" icon (Shortcut: `Cmd+2`) | 52px rail item with unread badge counter. Replaces main stage with 3-column task board (`Waiting Approval`, `In Progress`, `Completed`). |
| **Routine & Cron Hub** | Tasks | Inside Task Center / Context Inspector | `GlobalTasksWorkspace::RoutinesTab` & `Inspector::RoutinesSection` | Tab inside Task Center or Right Inspector | 5-field cron syntax editor, next 7-day schedule preview, execution history log, enable/disable toggle. |
| **Task Approval Gate** | Tasks | Message Stage / Modal Overlay | `ApprovalGateCard` & `ApprovalModal` | Agent requests sensitive tool execution | Amber banner in transcript & floating card in Task Center. "Approve Once", "Always Allow", "Deny". 10-min countdown. |
| **Agent Thinking Bubble** | AI Agent | Message Stage (Transcript) | `AgentThinkingBubble` inside `MessageBubble` | Live generation stream | Collapsible bubble above response. Shimmering pulse while reasoning. Displays elapsed time (e.g. "Thought for 4.2s"). |
| **Subagent Lineage Tree** | AI Agent | Message Stage / Context Inspector | `SubagentLineageWidget` | Click "View Execution Tree" on Task Card | Tree view showing root task, child workers, execution status badges, and intermediate artifact receipts. |
| **Model & Thinking Effort Selector** | AI Agent | Canonical Composer Dock | `ComposerToolbar::ModelSelectorPill` | Pill button above rich text editor | Dropdown showing active provider, model name, and reasoning budget (Low/Med/High). Hidden in pure human chats. |
| **MCP Server Marketplace** | MCP/Plugins | Left NavRail / Settings | `NavRailItem::Extensions` (`MarketplaceView`) | Click NavRail "Extensions" icon | Catalog of installed & discoverable MCP servers. Server health status, tool counts, enable/disable toggles. |
| **MCP Dynamic Permission Dock** | MCP/Plugins | Canonical Composer Dock | `LocalToolPermissionDock` | Agent requests local shell / filesystem access | Anchored banner directly above the composer input. Highlights command arguments, directory path, risk level. |
| **Installed Mini Apps** | MCP/Plugins | Attachment Toolbar / Bot Menu | `AttachMenu::MiniApps` & Bot Inline Button | Click paperclip "+" -> Mini App, or click Bot inline button | Opens sandboxed `MiniAppWindow` with native titlebar, refresh, permission status, and strict CSP fencing. |
| **Remote Computer Workbench** | Remote PC | Context Inspector / Fullscreen Modal | `ComputerWorkbench` & `LocalComputerWorkbench` | Agent triggers Computer Use tool, or via NavRail "Devices" | Live screen canvas with remote mouse cursor, injected keyboard events, resolution selector, and prominent "Takeover" banner. |
| **Device & Node Network** | Remote PC | Left NavRail / Settings | `NavRailItem::Devices` (`DevicesAndNodesView`) | Click NavRail "Devices" icon | List of paired workstations, remote VPS nodes, enrollment QR code generator, and active authorization tokens. |
| **Talk Mode Floating Energy Orb** | Voice | Stage Header / Floating Overlay | `TalkModeOrbModal` | Click Telephone/Waveform icon in top header | 160px canvas-rendered energy orb (Idle/Listening/Speaking). Can minimize to 48px floating PIP capsule in bottom-right. |
| **Voice Message Recording Gesture** | Voice | Canonical Composer Dock | `ComposerDock::VoiceGestureBar` | Hold Mic icon on right side of composer | Slide up to lock hands-free, slide left to cancel with shredder animation. Live audio waveform meter (0..31 dB). |
| **5-Bit Voice Message Bubble** | Voice | Message Stage (Transcript) | `VoiceMessagePlayer` | Inbound / Outbound voice note | 100-peak quantized waveform canvas, play/pause toggle, playback scrubber, 1x/1.5x/2x speed selector, inline STT transcript. |
| **Inline Button Keyboards** | Rich Cards | Message Stage (Transcript) | `InlineKeyboardGrid` attached to `MessageBubble` | Attached to Bot / Agent messages | Telegram standard 1 to 8 column inline button matrix with callback query spinner feedback. |
| **Interactive Polls & Todos** | Rich Cards | Attachment Toolbar / Transcript | `AttachMenu::PollTodo` -> `PollCard` / `TodoCard` | Paperclip "+" -> "Create Poll" or "Create Todo" | Poll: single/multi choice percentage bars. Todo: interactive checkboxes with assignee avatar and due date pill. |
| **Invoice Checkout Card** | Rich Cards | Message Stage (Transcript) | `InvoiceCard` -> `InvoiceCheckoutModal` | Agent / Merchant sends bill | Styled invoice container with line items, total amount in Stars/USD, and "Pay Now" button triggering secure checkout. |
| **Context Inspector Drawer** | Framework | Right Slide-Out Drawer (320px) | `ContextInspector` (Dynamic Tabs) | Click Profile Avatar or Top Header info button | Unified drawer: Profile, Members, Shared Media (6 categories), MCP Skills, Linked Tasks, Routines. |

---

## 4. State Machine Transition Equations & Diagrams

### 4.1 Domain 2: 7-State Global Task Lifecycle FSM
$$\text{TaskStatus} \in \{\text{Pending}, \text{Running}, \text{WaitingAuth}, \text{WaitingConnection}, \text{Completed}, \text{Failed}, \text{Cancelled}\}$$

```
                +-------------------+
                |      Pending      |
                +---------+---------+
                          | (Host Spawns Worker)
                          v
                +-------------------+ <-------+ (Auth Approved / Reconnected)
                |      Running      |         |
                +----+----+----+----+         |
                     |    |    |              |
     +---------------+    |    +---------+    |
     | (Needs Auth)       | (Lost Conn)  |    |
     v                    v              |    |
+------------+   +-------------------+   |    |
|WaitingAuth |   | WaitingConnection |   |    |
+----+---+---+   +---------+---------+   |    |
     |   |                 |             |    |
     |   +--(Approved)-----+             |    |
     |   |                               |    |
     |   +--(Denied / Timeout 10m)-------+----+----> [ Cancelled ]
     |                                   |
     +-----> (Final Error) ------------->+---------> [ Failed ]
     |                                   |
     +-----> (User Abort) -------------->+---------> [ Cancelled ]
     |                                   |
     +-----> (Turn Success) ------------>+---------> [ Completed ]
```

### 4.2 Domain 5: Full-Duplex Talk Mode & Barge-In State Machine
$$\text{TalkState} \in \{\text{Idle}, \text{Listening}, \text{Thinking}, \text{Speaking}, \text{Interrupted}, \text{Reconnecting}, \text{Closed}\}$$

- $\text{Idle} \xrightarrow{\text{User clicks Orb or Wake Word}} \text{Listening}$ (Microphone active, VAD primed).
- $\text{Listening} \xrightarrow{\text{VAD silence detected (>500ms)}} \text{Thinking}$ (Send audio chunk to Whisper STT).
- $\text{Thinking} \xrightarrow{\text{TTS stream arrives}} \text{Speaking}$ (Output audio plays through speaker, energy orb undulates).
- $\text{Speaking} \xrightarrow{\text{VAD user speech detected (<60ms)}} \text{Interrupted} \xrightarrow{\le 50\text{ms}} \text{Listening}$
  - **Immediate Barge-in Actions**:
    1. Send `barge_in_abort` signal to server.
    2. Flush client audio playback buffer instantly.
    3. Truncate previous AI turn transcript in SandAgentDb to the exact uttered duration.
    4. Begin buffering new user speech stream.

### 4.3 Domain 4: Remote Computer Control Lease State Machine
$$\text{LeaseState} \in \{\text{Available}, \text{Acquiring}, \text{HeldByAgent}, \text{HumanTakeover}, \text{Revoked}\}$$

1. **Acquisition**: When an Agent receives a CUA tool call, it issues `acquire_control_lease(agent_id, session_id)`.
2. **Lease Grant**: Host assigns `ComputerControlLease { generation, agent_id, expires_at }`.
3. **UI Indication**: The conversation displays an active yellow/amber lease banner: `Agent [Researcher] is controlling remote computer #node-01`.
4. **Emergency Takeover**: Clicking "Takeover" or pressing `Esc` sends `release_control_lease(generation)`. Host immediately discards incoming simulated events, locks agent input, and switches to `HumanTakeover` in $<10\text{ms}$.

