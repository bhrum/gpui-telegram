# Subsystem Shard: `chat_helpers`

**Total Source Files**: 57  
**Total Source Lines**: 30,847  
**Primary Feature IDs**: TCG-N11, TCG-N15, TCG-N27, TCG-N40  
**JSON Shard Path**: `docs/specs/ledger/shards/chat_helpers.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/chat_helpers/bot_command.cpp` | `TCG-N27` | `BotCommand` | `crates/gpui_telegram_desktop/src/views/composer/bot_command.rs` | 49 | `tests/test_tcg_n27_bot_command.rs` |
| `Telegram/SourceFiles/chat_helpers/bot_command.h` | `TCG-N27` | `BotCommand` | `crates/gpui_telegram_desktop/src/views/composer/bot_command.rs` | 31 | `tests/test_tcg_n27_bot_command.rs` |
| `Telegram/SourceFiles/chat_helpers/bot_keyboard.cpp` | `TCG-N27` | `BotKeyboard` | `crates/gpui_telegram_desktop/src/views/composer/bot_keyboard.rs` | 452 | `tests/test_tcg_n27_bot_keyboard.rs` |
| `Telegram/SourceFiles/chat_helpers/bot_keyboard.h` | `TCG-N27` | `BotKeyboard` | `crates/gpui_telegram_desktop/src/views/composer/bot_keyboard.rs` | 103 | `tests/test_tcg_n27_bot_keyboard.rs` |
| `Telegram/SourceFiles/chat_helpers/chat_helpers.style` | `TCG-N11` | `ChatHelpersStyleSheet` | `crates/gpui_telegram_desktop/src/theme/chat_helpers/chat_helpers_style.rs` | 1907 | `tests/test_tcg_n11_chat_helpers.rs` |
| `Telegram/SourceFiles/chat_helpers/compose/compose_features.h` | `TCG-N11` | `ComposeFeaturesCompose` | `crates/gpui_telegram_desktop/src/views/composer/compose/compose_features.rs` | 41 | `tests/test_tcg_n11_compose_features.rs` |
| `Telegram/SourceFiles/chat_helpers/compose/compose_show.cpp` | `TCG-N11` | `ComposeShowCompose` | `crates/gpui_telegram_desktop/src/views/composer/compose/compose_show.rs` | 58 | `tests/test_tcg_n11_compose_show.rs` |
| `Telegram/SourceFiles/chat_helpers/compose/compose_show.h` | `TCG-N11` | `ComposeShowCompose` | `crates/gpui_telegram_desktop/src/views/composer/compose/compose_show.rs` | 70 | `tests/test_tcg_n11_compose_show.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_interactions.cpp` | `TCG-N15` | `EmojiInteractions` | `crates/gpui_telegram_desktop/src/views/composer/emoji_interactions.rs` | 546 | `tests/test_tcg_n15_emoji_interactions.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_interactions.h` | `TCG-N15` | `EmojiInteractions` | `crates/gpui_telegram_desktop/src/views/composer/emoji_interactions.rs` | 144 | `tests/test_tcg_n15_emoji_interactions.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_keywords.cpp` | `TCG-N15` | `EmojiKeywords` | `crates/gpui_telegram_desktop/src/views/composer/emoji_keywords.rs` | 759 | `tests/test_tcg_n15_emoji_keywords.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_keywords.h` | `TCG-N15` | `EmojiKeywords` | `crates/gpui_telegram_desktop/src/views/composer/emoji_keywords.rs` | 87 | `tests/test_tcg_n15_emoji_keywords.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_list_widget.cpp` | `TCG-N15` | `EmojiListWidget` | `crates/gpui_telegram_desktop/src/views/composer/emoji_list_widget.rs` | 4348 | `tests/test_tcg_n15_emoji_list_widget.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_list_widget.h` | `TCG-N15` | `EmojiListWidget` | `crates/gpui_telegram_desktop/src/views/composer/emoji_list_widget.rs` | 620 | `tests/test_tcg_n15_emoji_list_widget.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_picker_overlay.cpp` | `TCG-N15` | `EmojiPickerOverlay` | `crates/gpui_telegram_desktop/src/views/composer/emoji_picker_overlay.rs` | 790 | `tests/test_tcg_n15_emoji_picker_overlay.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_picker_overlay.h` | `TCG-N15` | `EmojiPickerOverlay` | `crates/gpui_telegram_desktop/src/views/composer/emoji_picker_overlay.rs` | 112 | `tests/test_tcg_n15_emoji_picker_overlay.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_picker_overlay.style` | `TCG-N15` | `EmojiPickerOverlayStyleSheet` | `crates/gpui_telegram_desktop/src/theme/chat_helpers/emoji_picker_overlay_style.rs` | 45 | `tests/test_tcg_n15_emoji_picker_overlay.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_sets_manager.cpp` | `TCG-N15` | `EmojiSetsManager` | `crates/gpui_telegram_desktop/src/views/composer/emoji_sets_manager.rs` | 570 | `tests/test_tcg_n15_emoji_sets_manager.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_sets_manager.h` | `TCG-N15` | `EmojiSetsManager` | `crates/gpui_telegram_desktop/src/views/composer/emoji_sets_manager.rs` | 33 | `tests/test_tcg_n15_emoji_sets_manager.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_suggestions_widget.cpp` | `TCG-N15` | `EmojiSuggestionsWidget` | `crates/gpui_telegram_desktop/src/views/composer/emoji_suggestions_widget.rs` | 1153 | `tests/test_tcg_n15_emoji_suggestions_widget.rs` |
| `Telegram/SourceFiles/chat_helpers/emoji_suggestions_widget.h` | `TCG-N15` | `EmojiSuggestionsWidget` | `crates/gpui_telegram_desktop/src/views/composer/emoji_suggestions_widget.rs` | 120 | `tests/test_tcg_n15_emoji_suggestions_widget.rs` |
| `Telegram/SourceFiles/chat_helpers/field_autocomplete.cpp` | `TCG-N11` | `FieldAutocomplete` | `crates/gpui_telegram_desktop/src/views/composer/field_autocomplete.rs` | 2163 | `tests/test_tcg_n11_field_autocomplete.rs` |
| `Telegram/SourceFiles/chat_helpers/field_autocomplete.h` | `TCG-N11` | `FieldAutocomplete` | `crates/gpui_telegram_desktop/src/views/composer/field_autocomplete.rs` | 222 | `tests/test_tcg_n11_field_autocomplete.rs` |
| `Telegram/SourceFiles/chat_helpers/field_characters_count_manager.cpp` | `TCG-N11` | `FieldCharactersCountManager` | `crates/gpui_telegram_desktop/src/views/composer/field_characters_count_manager.rs` | 36 | `tests/test_tcg_n11_field_characters_count_manager.rs` |
| `Telegram/SourceFiles/chat_helpers/field_characters_count_manager.h` | `TCG-N11` | `FieldCharactersCountManager` | `crates/gpui_telegram_desktop/src/views/composer/field_characters_count_manager.rs` | 26 | `tests/test_tcg_n11_field_characters_count_manager.rs` |
| `Telegram/SourceFiles/chat_helpers/gifs_list_widget.cpp` | `TCG-N15` | `GifsListWidget` | `crates/gpui_telegram_desktop/src/views/composer/gifs_list_widget.rs` | 1040 | `tests/test_tcg_n15_gifs_list_widget.rs` |
| `Telegram/SourceFiles/chat_helpers/gifs_list_widget.h` | `TCG-N15` | `GifsListWidget` | `crates/gpui_telegram_desktop/src/views/composer/gifs_list_widget.rs` | 227 | `tests/test_tcg_n15_gifs_list_widget.rs` |
| `Telegram/SourceFiles/chat_helpers/message_field.cpp` | `TCG-N11` | `MessageField` | `crates/gpui_telegram_desktop/src/views/composer/message_field.rs` | 1665 | `tests/test_tcg_n11_message_field.rs` |
| `Telegram/SourceFiles/chat_helpers/message_field.h` | `TCG-N11` | `MessageField` | `crates/gpui_telegram_desktop/src/views/composer/message_field.rs` | 241 | `tests/test_tcg_n11_message_field.rs` |
| `Telegram/SourceFiles/chat_helpers/rich_paste_toast.cpp` | `TCG-N11` | `RichPasteToast` | `crates/gpui_telegram_desktop/src/views/composer/rich_paste_toast.rs` | 242 | `tests/test_tcg_n11_rich_paste_toast.rs` |
| `Telegram/SourceFiles/chat_helpers/rich_paste_toast.h` | `TCG-N11` | `RichPasteToast` | `crates/gpui_telegram_desktop/src/views/composer/rich_paste_toast.rs` | 62 | `tests/test_tcg_n11_rich_paste_toast.rs` |
| `Telegram/SourceFiles/chat_helpers/share_message_phrase_factory.cpp` | `TCG-N11` | `ShareMessagePhraseFactory` | `crates/gpui_telegram_desktop/src/views/composer/share_message_phrase_factory.rs` | 83 | `tests/test_tcg_n11_share_message_phrase_factory.rs` |
| `Telegram/SourceFiles/chat_helpers/share_message_phrase_factory.h` | `TCG-N11` | `ShareMessagePhraseFactory` | `crates/gpui_telegram_desktop/src/views/composer/share_message_phrase_factory.rs` | 37 | `tests/test_tcg_n11_share_message_phrase_factory.rs` |
| `Telegram/SourceFiles/chat_helpers/spellchecker_common.cpp` | `TCG-N11` | `SpellcheckerCommon` | `crates/gpui_telegram_desktop/src/views/composer/spellchecker_common.rs` | 653 | `tests/test_tcg_n11_spellchecker_common.rs` |
| `Telegram/SourceFiles/chat_helpers/spellchecker_common.h` | `TCG-N11` | `SpellcheckerCommon` | `crates/gpui_telegram_desktop/src/views/composer/spellchecker_common.rs` | 78 | `tests/test_tcg_n11_spellchecker_common.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_dice_pack.cpp` | `TCG-N15` | `StickersDicePack` | `crates/gpui_telegram_desktop/src/views/composer/stickers_dice_pack.rs` | 198 | `tests/test_tcg_n15_stickers_dice_pack.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_dice_pack.h` | `TCG-N15` | `StickersDicePack` | `crates/gpui_telegram_desktop/src/views/composer/stickers_dice_pack.rs` | 71 | `tests/test_tcg_n15_stickers_dice_pack.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_emoji_image_loader.cpp` | `TCG-N15` | `StickersEmojiImageLoader` | `crates/gpui_telegram_desktop/src/views/composer/stickers_emoji_image_loader.rs` | 99 | `tests/test_tcg_n15_stickers_emoji_image_loader.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_emoji_image_loader.h` | `TCG-N15` | `StickersEmojiImageLoader` | `crates/gpui_telegram_desktop/src/views/composer/stickers_emoji_image_loader.rs` | 35 | `tests/test_tcg_n15_stickers_emoji_image_loader.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_emoji_pack.cpp` | `TCG-N15` | `StickersEmojiPack` | `crates/gpui_telegram_desktop/src/views/composer/stickers_emoji_pack.rs` | 581 | `tests/test_tcg_n15_stickers_emoji_pack.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_emoji_pack.h` | `TCG-N15` | `StickersEmojiPack` | `crates/gpui_telegram_desktop/src/views/composer/stickers_emoji_pack.rs` | 172 | `tests/test_tcg_n15_stickers_emoji_pack.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_gift_box_pack.cpp` | `TCG-N15` | `StickersGiftBoxPack` | `crates/gpui_telegram_desktop/src/views/composer/stickers_gift_box_pack.rs` | 153 | `tests/test_tcg_n15_stickers_gift_box_pack.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_gift_box_pack.h` | `TCG-N15` | `StickersGiftBoxPack` | `crates/gpui_telegram_desktop/src/views/composer/stickers_gift_box_pack.rs` | 66 | `tests/test_tcg_n15_stickers_gift_box_pack.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_list_footer.cpp` | `TCG-N15` | `StickersListFooter` | `crates/gpui_telegram_desktop/src/views/composer/stickers_list_footer.rs` | 1598 | `tests/test_tcg_n15_stickers_list_footer.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_list_footer.h` | `TCG-N15` | `StickersListFooter` | `crates/gpui_telegram_desktop/src/views/composer/stickers_list_footer.rs` | 349 | `tests/test_tcg_n15_stickers_list_footer.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_list_widget.cpp` | `TCG-N15` | `StickersListWidget` | `crates/gpui_telegram_desktop/src/views/composer/stickers_list_widget.rs` | 4187 | `tests/test_tcg_n15_stickers_list_widget.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_list_widget.h` | `TCG-N15` | `StickersListWidget` | `crates/gpui_telegram_desktop/src/views/composer/stickers_list_widget.rs` | 594 | `tests/test_tcg_n15_stickers_list_widget.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_lottie.cpp` | `TCG-N15` | `StickersLottie` | `crates/gpui_telegram_desktop/src/views/composer/stickers_lottie.rs` | 393 | `tests/test_tcg_n15_stickers_lottie.rs` |
| `Telegram/SourceFiles/chat_helpers/stickers_lottie.h` | `TCG-N15` | `StickersLottie` | `crates/gpui_telegram_desktop/src/views/composer/stickers_lottie.rs` | 143 | `tests/test_tcg_n15_stickers_lottie.rs` |
| `Telegram/SourceFiles/chat_helpers/tabbed_panel.cpp` | `TCG-N11` | `TabbedPanel` | `crates/gpui_telegram_desktop/src/views/composer/tabbed_panel.rs` | 526 | `tests/test_tcg_n11_tabbed_panel.rs` |
| `Telegram/SourceFiles/chat_helpers/tabbed_panel.h` | `TCG-N11` | `TabbedPanel` | `crates/gpui_telegram_desktop/src/views/composer/tabbed_panel.rs` | 132 | `tests/test_tcg_n11_tabbed_panel.rs` |
| `Telegram/SourceFiles/chat_helpers/tabbed_section.cpp` | `TCG-N11` | `TabbedSection` | `crates/gpui_telegram_desktop/src/views/composer/tabbed_section.rs` | 80 | `tests/test_tcg_n11_tabbed_section.rs` |
| `Telegram/SourceFiles/chat_helpers/tabbed_section.h` | `TCG-N11` | `TabbedSection` | `crates/gpui_telegram_desktop/src/views/composer/tabbed_section.rs` | 62 | `tests/test_tcg_n11_tabbed_section.rs` |
| `Telegram/SourceFiles/chat_helpers/tabbed_selector.cpp` | `TCG-N11` | `TabbedSelector` | `crates/gpui_telegram_desktop/src/views/composer/tabbed_selector.rs` | 1685 | `tests/test_tcg_n11_tabbed_selector.rs` |
| `Telegram/SourceFiles/chat_helpers/tabbed_selector.h` | `TCG-N11` | `TabbedSelector` | `crates/gpui_telegram_desktop/src/views/composer/tabbed_selector.rs` | 488 | `tests/test_tcg_n11_tabbed_selector.rs` |
| `Telegram/SourceFiles/chat_helpers/ttl_media_layer_widget.cpp` | `TCG-N40` | `TtlMediaLayerWidget` | `crates/gpui_telegram_desktop/src/views/composer/ttl_media_layer_widget.rs` | 400 | `tests/test_tcg_n40_ttl_media_layer_widget.rs` |
| `Telegram/SourceFiles/chat_helpers/ttl_media_layer_widget.h` | `TCG-N40` | `TtlMediaLayerWidget` | `crates/gpui_telegram_desktop/src/views/composer/ttl_media_layer_widget.rs` | 22 | `tests/test_tcg_n40_ttl_media_layer_widget.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/chat_helpers/bot_command.h`
- **Classes & Structs**: PeerData, UserData, SendCommandRequest
- **Target**: `BotCommand` in `crates/gpui_telegram_desktop/src/views/composer/bot_command.rs`

### `Telegram/SourceFiles/chat_helpers/bot_keyboard.cpp`
- **Classes & Structs**: Style, Color, Radius, Corner
- **Core Methods & Signals**: Style::Style, ReplyKeyboard::Style, Style::paintButtonStart, Style::textStyle, Style::repaint, Style::buttonRounding
- **Target**: `BotKeyboard` in `crates/gpui_telegram_desktop/src/views/composer/bot_keyboard.rs`

### `Telegram/SourceFiles/chat_helpers/bot_keyboard.h`
- **Classes & Structs**: ReplyKeyboard, BotKeyboardButton, SessionController, BotKeyboard
- **Core Methods & Signals**: RpWidget::resizeToWidth, BotKeyboard, updateMarkup, step_selected, resizeToWidth, tooltipText
- **Target**: `BotKeyboard` in `crates/gpui_telegram_desktop/src/views/composer/bot_keyboard.rs`

### `Telegram/SourceFiles/chat_helpers/chat_helpers.style`
- **Core Methods & Signals**: align, point, margins, TextStyle, font, Menu
- **Style Tokens**: size: pixels, shift: pixels, stroke: pixels, align: align, outer: color, bg: color
- **Target**: `ChatHelpersStyleSheet` in `crates/gpui_telegram_desktop/src/theme/chat_helpers/chat_helpers_style.rs`

### `Telegram/SourceFiles/chat_helpers/compose/compose_features.h`
- **Classes & Structs**: ComposeFeatures
- **Target**: `ComposeFeaturesCompose` in `crates/gpui_telegram_desktop/src/views/composer/compose/compose_features.rs`

### `Telegram/SourceFiles/chat_helpers/compose/compose_show.h`
- **Classes & Structs**: PhotoData, DocumentData, FileOrigin, SessionController, Details, FileChosen, PauseReason, Show, PauseReasons, ResolveWindow
- **Core Methods & Signals**: is_flag_type, activate, showMediaPreview, processChosenSticker, rpl::pauseChanged, rpl::adjustShadowLeft
- **Target**: `ComposeShowCompose` in `crates/gpui_telegram_desktop/src/views/composer/compose/compose_show.rs`

### `Telegram/SourceFiles/chat_helpers/emoji_interactions.h`
- **Classes & Structs**: PeerData, HistoryItem, DocumentData, DocumentMedia, Session, Element, EmojiInteractionPlayRequest, EmojiInteractionsBunch, Single, EmojiInteractionSeen, EmojiInteractions, Animation
- **Core Methods & Signals**: EmojiInteractions, startOutgoing, startAutoplay, startIncoming, seenOutgoing, playStarted
- **Target**: `EmojiInteractions` in `crates/gpui_telegram_desktop/src/views/composer/emoji_interactions.rs`

### `Telegram/SourceFiles/chat_helpers/emoji_keywords.cpp`
- **Classes & Structs**: LangPackEmoji, LangPackData, EmojiKeywords, State, Result, Delegate, Entry
- **Core Methods & Signals**: internal::CacheFileFolder, views::all, views::filter, ranges::find, views::transform, ranges::any_of
- **Target**: `EmojiKeywords` in `crates/gpui_telegram_desktop/src/views/composer/emoji_keywords.rs`

### `Telegram/SourceFiles/chat_helpers/emoji_keywords.h`
- **Classes & Structs**: ApiWrap, EmojiKeywordsLangPackDelegate, EmojiKeywords, Result, LangPack
- **Core Methods & Signals**: langPackRefreshed, EmojiKeywords, refresh, delegate, handleSessionChanges, apiChanged
- **Target**: `EmojiKeywords` in `crates/gpui_telegram_desktop/src/views/composer/emoji_keywords.rs`

### `Telegram/SourceFiles/chat_helpers/emoji_list_widget.cpp`
- **Classes & Structs**: EmojiColorPicker, EmojiListWidget, Flag, FooterDescriptor, SizeTag, Section
- **Core Methods & Signals**: EmojiColorPicker::EmojiColorPicker, EmojiColorPicker::showEmoji, EmojiColorPicker::createAllLabel, tr::lng_emoji_color_all, EmojiColorPicker::updateSize, EmojiColorPicker::paintEvent
- **Target**: `EmojiListWidget` in `crates/gpui_telegram_desktop/src/views/composer/emoji_list_widget.rs`

