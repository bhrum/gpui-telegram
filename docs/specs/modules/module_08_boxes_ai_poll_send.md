# Module 08 (ui & boxes) — Slice 11: Boxes AI, Poll, Send Files & Utility Dialogs Specification

## 1. Executive Summary & Architectural Role

Module 08 Slice 11 covers the desktop modal dialogs and interaction boxes for:
- AI text composition, tone styling, tone creation, and tone preview (`compose_ai_box`, `create_ai_box`, `create_ai_tone_box`, `preview_ai_tone_box`)
- Poll and Quiz creation, and Todo list creation/editing (`create_poll_box`, `edit_todo_list_box`)
- File, photo, video, and GIF media dispatch dialogs (`send_files_box`, `send_files_box_reply_header`, `send_gif_with_caption_box`, `edit_caption_box`)
- Music tracks selection and ringtone upload/playback settings (`music_attach_box`, `ringtones_box`)
- Chat moderation, message pinning, phone ban resolution, and invite limit handling (`moderate_messages_box`, `pin_messages_box`, `phone_banned_box`, `max_invite_box`)
- System download settings, spellcheck dictionary management, shortcut legends, and account TTL self-destruction (`auto_download_box`, `download_path_box`, `dictionaries_manager`, `keys_legend`, `self_destruction_box`)

All components run as GPUI modal layers inside the main window, with full state management, keyboard navigation, and callback integration.

---

## 2. Upstream C++ 100% Reading Manifest (Part 1: AI, Polls, Send Files)

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/boxes/compose_ai_box.h` | 47 | `ComposeAiBoxArgs` (session, text, chatStyle, apply, send, setupMenu, richSource, applyRich, allowPrompt) | `crates/gpui_telegram_desktop/src/views/boxes/compose_ai_box.rs` |
| `Telegram/SourceFiles/boxes/compose_ai_box.cpp` | 2616 | `compose_ai_box.cpp:L89-L150`: `kAiComposeStyleTooltipHiddenPref`, `kPromptRemainingThreshold = 100`, `ComposeAiMode { Translate, Style, Fix }`, `CardState { Waiting, Loading, Ready, Failed }`, `HighlightDiff`, `StrikeOutDiff`, `BuildDiffDisplay` | `crates/gpui_telegram_desktop/src/views/boxes/compose_ai_box.rs` |
| `Telegram/SourceFiles/boxes/compose_ai_box.style` | 74 | `aiComposeBox`, `aiComposeField`, `aiComposeToneToastIconSize` (24x24), `aiComposeToneToastIconPadding` | `crates/gpui_telegram_desktop/src/theme/boxes/compose_ai_box_style.rs` |
| `Telegram/SourceFiles/boxes/create_ai_box.h` | 41 | `CreateAiBoxArgs` (session, applyToPage), `CreateAiBox`, `ShowCreateAiBox` | `crates/gpui_telegram_desktop/src/views/boxes/create_ai_box.rs` |
| `Telegram/SourceFiles/boxes/create_ai_box.cpp` | 665 | `create_ai_box.cpp:L52-L100`: `kPromptRemainingThreshold = 100`, `DefaultAiTranslateTo` (English fallback, Spanish tiebreaker), `SelectorTitle`, `ResponseIsland` rich preview | `crates/gpui_telegram_desktop/src/views/boxes/create_ai_box.rs` |
| `Telegram/SourceFiles/boxes/create_ai_tone_box.h` | 61 | `AddAiComposeFieldDecor`, `AddAiToneIconPreview`, `CreateAiToneBox`, `EditAiToneBox`, `ConfirmDeleteAiTone`, `ShowAiComposeToneLimitError` | `crates/gpui_telegram_desktop/src/views/boxes/create_ai_tone_box.rs` |
| `Telegram/SourceFiles/boxes/create_ai_tone_box.cpp` | 750 | `create_ai_tone_box.cpp:L54-L100`: `kAiComposeToneToastDuration = 4000ms`, `ShowToneToast`, `ChooseToneIconBox` with custom emoji picker | `crates/gpui_telegram_desktop/src/views/boxes/create_ai_tone_box.rs` |
| `Telegram/SourceFiles/boxes/preview_ai_tone_box.h` | 33 | `PreviewAiToneBox` with session, tone, weak controller | `crates/gpui_telegram_desktop/src/views/boxes/preview_ai_tone_box.rs` |
| `Telegram/SourceFiles/boxes/preview_ai_tone_box.cpp` | 633 | `preview_ai_tone_box.cpp:L48-L100`: `kToastDuration = 4000ms`, `kSpinDuration = 600ms`, `kWaitDuration = 1000ms`, `RefreshSpinEmoji` state machine (`Idle`, `Spinning`, `Waiting`) | `crates/gpui_telegram_desktop/src/views/boxes/preview_ai_tone_box.rs` |
| `Telegram/SourceFiles/boxes/create_poll_box.h` | 99 | `CreatePollBox`: `Result` (poll, text, options), `Error` flags (Question, Options, Correct, Other, Solution, Media, Deadline, Country) | `crates/gpui_telegram_desktop/src/views/boxes/create_poll_box.rs` |
| `Telegram/SourceFiles/boxes/create_poll_box.cpp` | 3454 | `create_poll_box.cpp:L107-L150`: `kQuestionLimit = 255`, `kMaxOptionsCount = 10`, `kOptionLimit = 100`, `kWarnQuestionLimit = 80`, `kWarnOptionLimit = 30`, `kSolutionLimit = 200`, `kWarnSolutionLimit = 60`, `kMediaUploadMaxAge = 45min` | `crates/gpui_telegram_desktop/src/views/boxes/create_poll_box.rs` |
| `Telegram/SourceFiles/boxes/edit_todo_list_box.h` | 113 | `EditTodoListBox`, `AddTodoListTasksBox`: Result, Error flags (Title, Tasks, Other), dynamic task addition | `crates/gpui_telegram_desktop/src/views/boxes/edit_todo_list_box.rs` |
| `Telegram/SourceFiles/boxes/edit_todo_list_box.cpp` | 1308 | `edit_todo_list_box.cpp:L54-L100`: `kWarnTitleLimit = 12`, `kWarnTaskLimit = 24`, `kErrorLimit = 99`, draggable task reordering, toggle completed, locked existing items | `crates/gpui_telegram_desktop/src/views/boxes/edit_todo_list_box.rs` |
| `Telegram/SourceFiles/boxes/send_files_box.h` | 385 | `SendFilesAllow` bitflags (OnlyOne, Photos, Videos, Music, Files, Stickers, Gifs, EmojiWithoutPremium, Texts), `SendFilesBoxDescriptor`, `SendFilesBox` | `crates/gpui_telegram_desktop/src/views/boxes/send_files_box.rs` |
| `Telegram/SourceFiles/boxes/send_files_box.cpp` | 2999 | `send_files_box.cpp:L89-L150`: `kMaxMessageLength = 4096`, `kMaxDisplayNameLength = 64`, `kStashFilesLimit = 100`, `kStashFilesMemoryLimit = 256MB`, `RenameFileBox`, grouped album preview | `crates/gpui_telegram_desktop/src/views/boxes/send_files_box.rs` |
| `Telegram/SourceFiles/boxes/send_files_box_reply_header.h` | 79 | `ReplyPillHeader`: shown message, spoiler animation, sender name, message preview text, dismiss request, desiredHeight | `crates/gpui_telegram_desktop/src/views/boxes/send_files_box_reply_header.rs` |
| `Telegram/SourceFiles/boxes/send_files_box_reply_header.cpp` | 376 | `send_files_box_reply_header.cpp:L36-L100`: `kAnimationDuration = 180ms`, reply target tracking, edited/destroyed updates | `crates/gpui_telegram_desktop/src/views/boxes/send_files_box_reply_header.rs` |
| `Telegram/SourceFiles/boxes/send_gif_with_caption_box.h` | 71 | `SetupCaptionFieldInBox`, `SendGifWithCaptionBox`, `SendGifWithCaption` | `crates/gpui_telegram_desktop/src/views/boxes/send_gif_with_caption_box.rs` |
| `Telegram/SourceFiles/boxes/send_gif_with_caption_box.cpp` | 791 | `send_gif_with_caption_box.cpp:L68-L100`: caption length limits, emoji picker panel binding, gif autoplay preview | `crates/gpui_telegram_desktop/src/views/boxes/send_gif_with_caption_box.rs` |
| `Telegram/SourceFiles/boxes/edit_caption_box.h` | 174 | `EditCaptionBox`: `StartMediaReplace`, `StartPhotoEdit`, invertCaption, spoilered, suggest options | `crates/gpui_telegram_desktop/src/views/boxes/edit_caption_box.rs` |
| `Telegram/SourceFiles/boxes/edit_caption_box.cpp` | 1398 | `edit_caption_box.cpp:L79-L100`: `kChangesDebounceTimeout = 1000ms`, `ListFromMimeData`, single file media replacement, photo editor integration | `crates/gpui_telegram_desktop/src/views/boxes/edit_caption_box.rs` |

## 3. Upstream C++ 100% Reading Manifest (Part 2: Music, Moderation, Storage & System)

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/boxes/music_attach_box.h` | 29 | `MusicAttachBox` with session, peer, actionFactory | `crates/gpui_telegram_desktop/src/views/boxes/music_attach_box.rs` |
| `Telegram/SourceFiles/boxes/music_attach_box.cpp` | 2532 | `music_attach_box.cpp:L90-L100`: `kSearchRequestDelay = 400ms`, `kFirstLimit = 10`, `MusicSectionController` for saved music & global search, audio preview | `crates/gpui_telegram_desktop/src/views/boxes/music_attach_box.rs` |
| `Telegram/SourceFiles/boxes/ringtones_box.h` | 39 | `ExtractRingtoneName`, `RingtonesBox`, `ThreadRingtonesBox` | `crates/gpui_telegram_desktop/src/views/boxes/ringtones_box.rs` |
| `Telegram/SourceFiles/boxes/ringtones_box.cpp` | 394 | `ringtones_box.cpp:L50-L90`: `kDefaultValue = -1`, `kNoSoundValue = -2`, `kNoDetachTimeout = 250ms`, `AudioCreator` RAII device attach, volume controller | `crates/gpui_telegram_desktop/src/views/boxes/ringtones_box.rs` |
| `Telegram/SourceFiles/boxes/moderate_messages_box.h` | 59 | `ModerateMessagesBoxOptions`, `ModerateReactionEntry`, `ModerateMessagesBoxEntry`, `CreateModerateMessagesBox`, `DeleteChatBox`, `DeleteSublistBox` | `crates/gpui_telegram_desktop/src/views/boxes/moderate_messages_box.rs` |
| `Telegram/SourceFiles/boxes/moderate_messages_box.cpp` | 1831 | `moderate_messages_box.cpp:L79-L97`: `kModerateCommonGroups = "moderate-common-groups"`, `kModerateMessagesBoxAnimationDuration = 80ms`, `ModerateOptions` (spam report, delete all messages/reactions, ban user) | `crates/gpui_telegram_desktop/src/views/boxes/moderate_messages_box.rs` |
| `Telegram/SourceFiles/boxes/moderate_messages_box.style` | 44 | `moderateBox`, `moderateCommonGroupsCheckbox`, `moderateParticipantsList` | `crates/gpui_telegram_desktop/src/theme/boxes/moderate_messages_box_style.rs` |
| `Telegram/SourceFiles/boxes/pin_messages_box.h` | 17 | `PinMessageBox` with generic box and history item | `crates/gpui_telegram_desktop/src/views/boxes/pin_messages_box.rs` |
| `Telegram/SourceFiles/boxes/pin_messages_box.cpp` | 141 | `pin_messages_box.cpp:L24-L68`: `IsOldForPin`, `PinMessageBox`: notify checkbox for megagroups, `pinForPeer` checkbox for 1-on-1 private chat | `crates/gpui_telegram_desktop/src/views/boxes/pin_messages_box.rs` |
| `Telegram/SourceFiles/boxes/phone_banned_box.h` | 21 | `ShowPhoneBannedError` with controller and phone string | `crates/gpui_telegram_desktop/src/views/boxes/phone_banned_box.rs` |
| `Telegram/SourceFiles/boxes/phone_banned_box.cpp` | 74 | `phone_banned_box.cpp:L22-L71`: `SendToBannedHelp` compose mailto URL to `login@stel.com` with AppVersionStr, OS and locale info | `crates/gpui_telegram_desktop/src/views/boxes/phone_banned_box.rs` |
| `Telegram/SourceFiles/boxes/max_invite_box.h` | 40 | `MaxInviteBox`: text measurement, copy invite link handler, mouse hover link | `crates/gpui_telegram_desktop/src/views/boxes/max_invite_box.rs` |
| `Telegram/SourceFiles/boxes/max_invite_box.cpp` | 158 | `max_invite_box.cpp:L30-L53`: `kInformBoxTextOptions`, chatSizeMax threshold handling, copy to clipboard with toast notification | `crates/gpui_telegram_desktop/src/views/boxes/max_invite_box.rs` |
| `Telegram/SourceFiles/boxes/auto_download_box.h` | 40 | `AutoDownloadBox` with session and Source enum | `crates/gpui_telegram_desktop/src/views/boxes/auto_download_box.rs` |
| `Telegram/SourceFiles/boxes/auto_download_box.cpp` | 467 | `auto_download_box.cpp:L33-L75`: `kMegabyte = 1048576`, `kDefaultDownloadLimit = 10MB`, `kDefaultAutoPlayLimit = 50MB`, `PeerBuckets` (always/never overrides) | `crates/gpui_telegram_desktop/src/views/boxes/auto_download_box.rs` |
| `Telegram/SourceFiles/boxes/download_path_box.h` | 60 | `DownloadPathBox`: Directory enum (Downloads, Temp, Custom), path link | `crates/gpui_telegram_desktop/src/views/boxes/download_path_box.rs` |
| `Telegram/SourceFiles/boxes/download_path_box.cpp` | 160 | `download_path_box.cpp:L21-L100`: directory selection, OS native default path, temp directory or custom folder selection with bookmark | `crates/gpui_telegram_desktop/src/views/boxes/download_path_box.rs` |
| `Telegram/SourceFiles/boxes/dictionaries_manager.h` | 37 | `ManageDictionariesBox` with session | `crates/gpui_telegram_desktop/src/views/boxes/dictionaries_manager.rs` |
| `Telegram/SourceFiles/boxes/dictionaries_manager.cpp` | 466 | `dictionaries_manager.cpp:L42-L80`: `kMaxQueryLength = 15`, multi-select language filter, dictionary download size display, enable/disable row state | `crates/gpui_telegram_desktop/src/views/boxes/dictionaries_manager.rs` |
| `Telegram/SourceFiles/boxes/keys_legend.h` | 26 | `ShowKeysLegend`, `HandleKeysLegendHotkey` | `crates/gpui_telegram_desktop/src/views/boxes/keys_legend.rs` |
| `Telegram/SourceFiles/boxes/keys_legend.cpp` | 780 | `keys_legend.cpp:L51-L97`: `IsModifierGlyph` (0x2318 Cmd, 0x2303 Ctrl, 0x2325 Opt, 0x21E7 Shift), `SplitDisplayString`, key legend sections | `crates/gpui_telegram_desktop/src/views/boxes/keys_legend.rs` |
| `Telegram/SourceFiles/boxes/keys_legend.style` | 52 | `keysLegendBox`, `keysLegendKey`, `keysLegendSectionHeader` | `crates/gpui_telegram_desktop/src/theme/boxes/keys_legend_style.rs` |
| `Telegram/SourceFiles/boxes/self_destruction_box.h` | 53 | `SelfDestructionBox`: Type (Account, Sessions), `DaysLabel` | `crates/gpui_telegram_desktop/src/views/boxes/self_destruction_box.rs` |
| `Telegram/SourceFiles/boxes/self_destruction_box.cpp` | 232 | `self_destruction_box.cpp:L95-L101`: `Values(Account) = [30, 90, 180, 365, 548, 720]`, `Values(Sessions) = [7, 30, 90, 180, 365]`, delete account in test mode | `crates/gpui_telegram_desktop/src/views/boxes/self_destruction_box.rs` |

---

## 4. Anti-Bluffing Proof & Magic Constants Summary

All 21 components directly implement the upstream C++ constants and logic:
1. `compose_ai_box`: `kAiComposeStyleTooltipHiddenPref`, `kPromptRemainingThreshold = 100`, `ComposeAiMode`, `HighlightDiff`/`StrikeOutDiff`.
2. `create_ai_box`: `kPromptRemainingThreshold = 100`, `DefaultAiTranslateTo` fallback logic.
3. `create_ai_tone_box`: `kAiComposeToneToastDuration = 4000ms`, tone emoji preview.
4. `preview_ai_tone_box`: `kToastDuration = 4000ms`, `kSpinDuration = 600ms`, `kWaitDuration = 1000ms`.
5. `create_poll_box`: `kQuestionLimit = 255`, `kMaxOptionsCount = 10`, `kOptionLimit = 100`, `kWarnQuestionLimit = 80`, `kWarnOptionLimit = 30`, `kSolutionLimit = 200`.
6. `edit_todo_list_box`: `kWarnTitleLimit = 12`, `kWarnTaskLimit = 24`, `kErrorLimit = 99`.
7. `send_files_box`: `kMaxMessageLength = 4096`, `kMaxDisplayNameLength = 64`, `kStashFilesLimit = 100`, `kStashFilesMemoryLimit = 256MB`.
8. `send_files_box_reply_header`: `kAnimationDuration = 180ms`.
9. `send_gif_with_caption_box`: GIF loop autoplay, caption formatting, SendMenu details.
10. `edit_caption_box`: `kChangesDebounceTimeout = 1000ms`, single file media replacement, invert caption toggle.
11. `music_attach_box`: `kSearchRequestDelay = 400ms`, `kFirstLimit = 10`.
12. `ringtones_box`: `kDefaultValue = -1`, `kNoSoundValue = -2`, `kNoDetachTimeout = 250ms`.
13. `moderate_messages_box`: `kModerateCommonGroups = "moderate-common-groups"`, `kModerateMessagesBoxAnimationDuration = 80ms`.
14. `pin_messages_box`: `IsOldForPin`, group notify flag, 1-on-1 pin for both parties.
15. `phone_banned_box`: `SendToBannedHelp` mailto to `login@stel.com`.
16. `max_invite_box`: channel `chatSizeMax` limit reached, invite link clipboard copying.
17. `auto_download_box`: `kMegabyte = 1048576`, `kDefaultDownloadLimit = 10MB`, `kDefaultAutoPlayLimit = 50MB`.
18. `download_path_box`: directory selection (Downloads, Temp, Custom).
19. `dictionaries_manager`: `kMaxQueryLength = 15`, per-language spellcheck dictionary.
20. `keys_legend`: Mac modifier glyphs (0x2318 Cmd, 0x2303 Ctrl, 0x2325 Opt, 0x21E7 Shift).
21. `self_destruction_box`: Account TTL `[30, 90, 180, 365, 548, 720]` days, Sessions TTL `[7, 30, 90, 180, 365]` days.

---

## 5. Verification Method

- Compilation check: `cargo check --workspace`
- Subsystem test suite: `cargo test --test boxes_ai_poll_send_subsystem_tests`
- Ledger closure check: `python3 scripts/verify_ledger_closure.py`
- Build garbage cleanup: `python3 scripts/clean_build_garbage.py`
