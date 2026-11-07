# Module 08 Specification: UI Core Widgets & Controls (`ui/widgets`)

## Overview

Module 08 Slice 1 (`ui/widgets`) provides the core GPUI Telegram Desktop widget toolkit, including continuous and discrete sliders, chat folder filter tabs strip and slider with drag-to-reorder, sliding tabs, gradient round button with glare sweeps, marquee label with fade ramps, cross-fade label, horizontal fit container, level meter, passcode strength meter with advice heuristics, multi-select tag editor with delete animations, expandable peer list, participants check view, peer bubble, selecting scroll accelerator, middle-click autoscroll HUD, sent code input field with auto-submit, special phone/username fields, and color picker editor.

This specification is authored in strict compliance with:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic boilerplate stubs (`{ id, label, value }` Patterns A, B, C, D) are completely replaced with genuine domain models and interactive GPUI views.
- **Mandate #17 (Dynamic Target File Architecture)**: Upstream C++ files serve as the immutable reading checklist. Target Rust/GPUI architecture decomposes Qt widgets into idiomatic GPUI view models, states, and event listeners.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component, animation curve, debounce timer, and pixel calculation cites exact upstream C++ implementation file line ranges (`.cpp:L<start>-L<end>`), header definitions (`.h:L<start>-L<end>`), and style tokens (`.style:L<start>-L<end>`), extracting anonymous-namespace `k...` constants and mathematical formulas.
- **Mandate #19 (Zero-Omitted-File Reading Guarantee)**: Every single one of the 53 assigned upstream C++ and style files in `Telegram/SourceFiles/ui/widgets/` is accounted for in the 100% Per-File Reading Manifest below (`omitted_from_design_doc == 0`).

---

## 1. 100% Per-File Reading Manifest (All 53 Upstream C++ & Style Sources)

| # | Upstream C++ Source Path | Exact Line Range Read | Upstream Symbols & Classes | Extracted Constants & Logic | Dynamic Rust Target Module & Disposition |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/ui/widgets/chat_filters_tabs_mode.h` | `L1-L34` | `ChatsFiltersTabsMode`, `HorizontalChatsFiltersTabsMode`, `VerticalChatsFiltersTabsMode` | `Default = 0`, `TextAndIcons = 1`, `IconsOnly = 2`, `TextOnly = 3`; Horizontal mode maps Default -> TextOnly; Vertical mode maps Default -> TextAndIcons | `crates/gpui_telegram_desktop/src/components/ui/widgets/chat_filters_tabs_mode.rs` (`ChatFiltersTabsModeWidgets`, `gpui_ui_1to1`) |
| 2 | `Telegram/SourceFiles/ui/widgets/chat_filters_tabs_slider.cpp` | `L1-L529` | `ChatsFiltersTabs`, `setSectionsAndCheckChanged`, `setUnreadCount`, `cacheUnreadCount`, `calculateLockedFromX` | `_unreadMaxString(u"99+")`, `_unreadSkip(st::lineWidth * 5)`, `_iconSkip(st::dialogsFiltersTabsIconSkip)`, badge widths `{one, 99, 999}`, locked tab separator styling | `crates/gpui_telegram_desktop/src/components/ui/widgets/chat_filters_tabs_slider.rs` (`ChatFiltersTabsSliderWidgets`, `gpui_ui_1to1`) |
| 3 | `Telegram/SourceFiles/ui/widgets/chat_filters_tabs_slider.h` | `L1-L118` | `ChatsFiltersTabs`, `ShiftedSection`, `setTabsMode`, `setSectionIcons`, `setLockedFrom`, `reorderSections` | SettingsSlider specialization for chat folder tab strip; horizontal shift per tab, badge caching, locked folder click stream | `crates/gpui_telegram_desktop/src/components/ui/widgets/chat_filters_tabs_slider.rs` (`ChatFiltersTabsSliderWidgets`, `gpui_ui_1to1`) |
| 4 | `Telegram/SourceFiles/ui/widgets/chat_filters_tabs_slider_reorder.cpp` | `L1-L376` | `ChatsFiltersTabsReorder`, `cancel`, `start`, `mouseMove`, `finishReordering` | `kScrollFactor = 0.05` (`chat_filters_tabs_slider_reorder.cpp:L20`), auto-scroll edge thresholds, animated tab displacement during drag-and-drop | `crates/gpui_telegram_desktop/src/components/ui/widgets/chat_filters_tabs_slider_reorder.rs` (`ChatFiltersTabsSliderReorderWidgets`, `gpui_ui_1to1`) |
| 5 | `Telegram/SourceFiles/ui/widgets/chat_filters_tabs_slider_reorder.h` | `L1-L98` | `ChatsFiltersTabsReorder`, `Entry`, `indexOf`, `updateScrollCallback` | Drag-to-reorder controller with horizontal shift tracking and edge scrolling animation | `crates/gpui_telegram_desktop/src/components/ui/widgets/chat_filters_tabs_slider_reorder.rs` (`ChatFiltersTabsSliderReorderWidgets`, `gpui_ui_1to1`) |
| 6 | `Telegram/SourceFiles/ui/widgets/chat_filters_tabs_strip.cpp` | `L1-L571` | `ChatsFiltersTabsStrip`, `State`, `rebuildFilters`, `refreshTabs`, `filterContextMenu` | Context menu actions (Mark as read, Edit folder, Remove folder), suggested folders fetching, tab click routing | `crates/gpui_telegram_desktop/src/components/ui/widgets/chat_filters_tabs_strip.rs` (`ChatFiltersTabsStripWidgets`, `gpui_ui_1to1`) |
| 7 | `Telegram/SourceFiles/ui/widgets/chat_filters_tabs_strip.h` | `L1-L37` | `ChatsFiltersTabsStrip`, `FilterId`, `reorder` | Container widget hosting `ChatsFiltersTabs` slider with scroll area integration | `crates/gpui_telegram_desktop/src/components/ui/widgets/chat_filters_tabs_strip.rs` (`ChatFiltersTabsStripWidgets`, `gpui_ui_1to1`) |
| 8 | `Telegram/SourceFiles/ui/widgets/color_editor.cpp` | `L1-L1282` | `ColorEditor`, `Picker`, `preparePaletteRGBA`, `preparePaletteHSL`, `updateCurrentPoint` | HSV/RGB coordinate mapping, crosshair picker cursor, hex input validation with `#` prefix, alpha slider | `crates/gpui_telegram_desktop/src/components/ui/widgets/color_editor.rs` (`ColorEditorWidgets`, `gpui_ui_1to1`) |
| 9 | `Telegram/SourceFiles/ui/widgets/color_editor.h` | `L1-L106` | `ColorEditor`, `Mode`, `HSB`, `Picker`, `setRGB`, `setHSB` | Color picker widget supporting RGB, HSB and Hex modes with reactive color changed producer | `crates/gpui_telegram_desktop/src/components/ui/widgets/color_editor.rs` (`ColorEditorWidgets`, `gpui_ui_1to1`) |
| 10 | `Telegram/SourceFiles/ui/widgets/color_editor.style` | `L1-L28` | `colorEditorField`, `colorEditorPipette`, `colorEditorSize` | Hex input field style, pipette icon button, color preview swatch size | `crates/gpui_telegram_desktop/src/theme/ui/color_editor_style.rs` (`ColorEditorWidgetsStyleSheet`, `gpui_ui_1to1`) |
| 11 | `Telegram/SourceFiles/ui/widgets/continuous_sliders.cpp` | `L1-L570` | `ContinuousSlider`, `FilledSlider`, `MediaSlider`, `computeValue`, `mousePressEvent`, `wheelEvent` | `kByWheelFinishedTimeout = 1000` (`continuous_sliders.cpp:L20`), wheel delta step 120, coef 1/1200, Mac wheel delta inversion, key stepping | `crates/gpui_telegram_desktop/src/components/ui/widgets/continuous_sliders.rs` (`ContinuousSlidersWidgets`, `gpui_ui_1to1`) |
| 12 | `Telegram/SourceFiles/ui/widgets/continuous_sliders.h` | `L1-L286` | `ContinuousSlider`, `Direction`, `FilledSlider`, `MediaSlider`, `ColorOverrides`, `setPseudoDiscrete` | Continuous floating point range [0.0, 1.0], hover opacity animation, progress and finished callbacks | `crates/gpui_telegram_desktop/src/components/ui/widgets/continuous_sliders.rs` (`ContinuousSlidersWidgets`, `gpui_ui_1to1`) |
| 13 | `Telegram/SourceFiles/ui/widgets/cross_fade_label.cpp` | `L1-L81` | `CrossFadeLabel`, `setText`, `paintEvent` | `kDuration = 200ms` (`cross_fade_label.cpp:L15`), `anim::easeOutCubic`, vertical sliding delta `st.font->height` | `crates/gpui_telegram_desktop/src/components/ui/widgets/cross_fade_label.rs` (`CrossFadeLabelWidgets`, `gpui_ui_1to1`) |
| 14 | `Telegram/SourceFiles/ui/widgets/cross_fade_label.h` | `L1-L44` | `CrossFadeLabel`, `setText`, `setDirection` | Animated transition label cross-fading old and new text with vertical sliding displacement | `crates/gpui_telegram_desktop/src/components/ui/widgets/cross_fade_label.rs` (`CrossFadeLabelWidgets`, `gpui_ui_1to1`) |
| 15 | `Telegram/SourceFiles/ui/widgets/discrete_sliders.cpp` | `L1-L529` | `DiscreteSlider`, `SettingsSlider`, `activateCallback`, `setSelectedSection`, `getIndexFromPosition` | Precise timer debounce, safe round integer index mapping, ripple start on press, section range interpolation | `crates/gpui_telegram_desktop/src/components/ui/widgets/discrete_sliders.rs` (`DiscreteSlidersWidgets`, `gpui_ui_1to1`) |
| 16 | `Telegram/SourceFiles/ui/widgets/discrete_sliders.h` | `L1-L173` | `DiscreteSlider`, `Section`, `Range`, `SettingsSlider`, `setActiveSection`, `setSelectOnPress` | Stepped slider widget with discrete segments, active pill pill animation, ripple effect on each section | `crates/gpui_telegram_desktop/src/components/ui/widgets/discrete_sliders.rs` (`DiscreteSlidersWidgets`, `gpui_ui_1to1`) |
| 17 | `Telegram/SourceFiles/ui/widgets/expandable_peer_list.cpp` | `L1-L207` | `AddExpandablePeerList`, `ParticipantsExpanderText` | Expander button with count formatting, vertical slide-down list of peers, select-all toggling | `crates/gpui_telegram_desktop/src/components/ui/widgets/expandable_peer_list.rs` (`ExpandablePeerListWidgets`, `gpui_ui_1to1`) |
| 18 | `Telegram/SourceFiles/ui/widgets/expandable_peer_list.h` | `L1-L43` | `ExpandablePeerListController`, `AddExpandablePeerList`, `ControllerData` | Controller managing collapsible peer list with count expander button and member checkboxes | `crates/gpui_telegram_desktop/src/components/ui/widgets/expandable_peer_list.rs` (`ExpandablePeerListWidgets`, `gpui_ui_1to1`) |
| 19 | `Telegram/SourceFiles/ui/widgets/fields/special_fields.cpp` | `L1-L491` | `CountryCodeInput`, `PhoneInput`, `UsernameInput`, `codeSelected` | `kMaxUsernameLength = 32`, `kMaxPhoneTailLength = 32`, `kMaxPhoneCodeLength = 4` (`fields/special_fields.cpp:L19-L26`) | `crates/gpui_telegram_desktop/src/components/ui/widgets/fields/special_fields.rs` (`SpecialFieldsWidgetsFields`, `gpui_ui_1to1`) |
| 20 | `Telegram/SourceFiles/ui/widgets/fields/special_fields.h` | `L1-L147` | `CountryCodeInput`, `PhoneInput`, `UsernameInput`, `chooseCountry` | Specialized text input fields for international phone codes, dialable phone tails, and usernames | `crates/gpui_telegram_desktop/src/components/ui/widgets/fields/special_fields.rs` (`SpecialFieldsWidgetsFields`, `gpui_ui_1to1`) |
| 21 | `Telegram/SourceFiles/ui/widgets/fields/time_part_input_with_placeholder.cpp` | `L1-L62` | `TimePartWithPlaceholder`, `maybeUpdatePlaceholder`, `paintAdditionalPlaceholder` | Dynamic time string offset calculation, zero padding, centered text alignment | `crates/gpui_telegram_desktop/src/components/ui/widgets/fields/time_part_input_with_placeholder.rs` (`TimePartInputWithPlaceholderWidgetsFields`, `gpui_ui_1to1`) |
| 22 | `Telegram/SourceFiles/ui/widgets/fields/time_part_input_with_placeholder.h` | `L1-L40` | `TimePartWithPlaceholder`, `setPhrase`, `PlaceholderInfo` | Input field for time duration parts (hours, minutes, seconds) with responsive localized placeholder | `crates/gpui_telegram_desktop/src/components/ui/widgets/fields/time_part_input_with_placeholder.rs` (`TimePartInputWithPlaceholderWidgetsFields`, `gpui_ui_1to1`) |
| 23 | `Telegram/SourceFiles/ui/widgets/glare_tooltip.cpp` | `L1-L250` | `GlareTooltip`, `showGlare`, `fade`, `paintEvent` | `kGlareDurationStep = 320ms`, `kGlareTimeout = 1000ms` (`glare_tooltip.cpp:L19-L20`), `_glareSize = outer.height() * 3`, `_skip = 2 * stroke` | `crates/gpui_telegram_desktop/src/components/ui/widgets/glare_tooltip.rs` (`GlareTooltipWidgets`, `gpui_ui_1to1`) |
| 24 | `Telegram/SourceFiles/ui/widgets/glare_tooltip.h` | `L1-L82` | `GlareTooltip`, `GlareTooltipColors`, `fade`, `toggle` | Premium feature promotional tooltip with periodic metallic light beam glare sweep animation | `crates/gpui_telegram_desktop/src/components/ui/widgets/glare_tooltip.rs` (`GlareTooltipWidgets`, `gpui_ui_1to1`) |
| 25 | `Telegram/SourceFiles/ui/widgets/gradient_round_button.cpp` | `L1-L131` | `GradientButton`, `paintEvent`, `paintGlare`, `setStops` | Ripple color `rgba(0, 0, 0, 36)` (`gradient_round_button.cpp:L45`), linear gradient background cache, glare shine pass | `crates/gpui_telegram_desktop/src/components/ui/widgets/gradient_round_button.rs` (`GradientRoundButtonWidgets`, `gpui_ui_1to1`) |
| 26 | `Telegram/SourceFiles/ui/widgets/gradient_round_button.h` | `L1-L41` | `GradientButton`, `setStops`, `setFullRadius`, `startGlare` | Rounded button rendered with multi-stop color gradients and animated glare highlight effect | `crates/gpui_telegram_desktop/src/components/ui/widgets/gradient_round_button.rs` (`GradientRoundButtonWidgets`, `gpui_ui_1to1`) |
| 27 | `Telegram/SourceFiles/ui/widgets/horizontal_fit_container.cpp` | `L1-L56` | `HorizontalFitContainer`, `add`, `remove`, `updateLayout` | Even spacing calculation: `buttonWidth = (size.width() - spacing * (count - 1)) / count` (`horizontal_fit_container.cpp:L47`) | `crates/gpui_telegram_desktop/src/components/ui/widgets/horizontal_fit_container.rs` (`HorizontalFitContainerWidgets`, `gpui_ui_1to1`) |
| 28 | `Telegram/SourceFiles/ui/widgets/horizontal_fit_container.h` | `L1-L36` | `HorizontalFitContainer`, `add`, `remove` | Layout container distributing horizontal space equally among child buttons | `crates/gpui_telegram_desktop/src/components/ui/widgets/horizontal_fit_container.rs` (`HorizontalFitContainerWidgets`, `gpui_ui_1to1`) |
| 29 | `Telegram/SourceFiles/ui/widgets/level_meter.cpp` | `L1-L48` | `LevelMeter`, `setValue`, `paintEvent` | `radius = lineWidth / 2`, line offset `(lineWidth + lineSpacing) * i`, active vs inactive brush selection | `crates/gpui_telegram_desktop/src/components/ui/widgets/level_meter.rs` (`LevelMeterWidgets`, `gpui_ui_1to1`) |
| 30 | `Telegram/SourceFiles/ui/widgets/level_meter.h` | `L1-L33` | `LevelMeter`, `setValue`, `style::LevelMeter` | Segmented audio volume/VU level meter displaying active volume bars | `crates/gpui_telegram_desktop/src/components/ui/widgets/level_meter.rs` (`LevelMeterWidgets`, `gpui_ui_1to1`) |
| 31 | `Telegram/SourceFiles/ui/widgets/marquee_label.cpp` | `L1-L790` | `MarqueeLabel`, `marqueeStep`, `edgeScrollStep`, `setText` | `kMarqueeDelay = 500ms`, `kFrameDelta = 16ms`, `kEdgeScrollSpeed = 3.0` (`marquee_label.cpp:L29-L32`), fade ramp opacity curves | `crates/gpui_telegram_desktop/src/components/ui/widgets/marquee_label.rs` (`MarqueeLabelWidgets`, `gpui_ui_1to1`) |
| 32 | `Telegram/SourceFiles/ui/widgets/marquee_label.h` | `L1-L126` | `MarqueeLabel`, `setText`, `setTextColorOverride`, `setSelectable` | Auto-scrolling marquee label for long song titles or peer names exceeding available width | `crates/gpui_telegram_desktop/src/components/ui/widgets/marquee_label.rs` (`MarqueeLabelWidgets`, `gpui_ui_1to1`) |
| 33 | `Telegram/SourceFiles/ui/widgets/marquee_label.style` | `L1-L15` | `marqueeLabelFade`, `marqueeLabelGap`, `marqueeLabelSpeed` | Gap 16px, fade width 24px, scrolling speed 40px/sec | `crates/gpui_telegram_desktop/src/theme/ui/marquee_label_style.rs` (`MarqueeLabelWidgetsStyleSheet`, `gpui_ui_1to1`) |
| 34 | `Telegram/SourceFiles/ui/widgets/middle_click_autoscroll.cpp` | `L1-L243` | `MiddleClickAutoscroll`, `PaintCursorImage`, `paintEvent` | `kDelay = 15ms`, `kHoldToToggleThreshold = 220ms` (`middle_click_autoscroll.cpp:L17-L18`), `Deadzone() = 6px`, outer stroke 1.5, center skip 10.5 | `crates/gpui_telegram_desktop/src/components/ui/widgets/middle_click_autoscroll.rs` (`MiddleClickAutoscrollWidgets`, `gpui_ui_1to1`) |
| 35 | `Telegram/SourceFiles/ui/widgets/middle_click_autoscroll.h` | `L1-L57` | `MiddleClickAutoscroll`, `start`, `stop`, `isActive` | Middle mouse button auto-scrolling HUD overlay with directional arrows and velocity calculation | `crates/gpui_telegram_desktop/src/components/ui/widgets/middle_click_autoscroll.rs` (`MiddleClickAutoscrollWidgets`, `gpui_ui_1to1`) |
| 36 | `Telegram/SourceFiles/ui/widgets/multi_select.cpp` | `L1-L1146` | `MultiSelect`, `Item`, `paintDeleteButton`, `paintCached` | `kWideScale = 3` (`multi_select.cpp:L25`), cross delete animation, backspace to delete item, query text filtering | `crates/gpui_telegram_desktop/src/components/ui/widgets/multi_select.rs` (`MultiSelectWidgets`, `gpui_ui_1to1`) |
| 37 | `Telegram/SourceFiles/ui/widgets/multi_select.h` | `L1-L77` | `MultiSelect`, `addItem`, `removeItem`, `entitySelected` | Token/tag selection input bar with userpic pills, active delete crosses, and embedded search query field | `crates/gpui_telegram_desktop/src/components/ui/widgets/multi_select.rs` (`MultiSelectWidgets`, `gpui_ui_1to1`) |
| 38 | `Telegram/SourceFiles/ui/widgets/participants_check_view.cpp` | `L1-L140` | `ExpanderCheckView`, `ComputeSize`, `SetExpanderText` | Moderate box expand metrics: `height + skip * 4 + labelWidth + toggleSize`, rotating chevron arrow | `crates/gpui_telegram_desktop/src/components/ui/widgets/participants_check_view.rs` (`ParticipantsCheckViewWidgets`, `gpui_ui_1to1`) |
| 39 | `Telegram/SourceFiles/ui/widgets/participants_check_view.h` | `L1-L64` | `ExpanderCheckView`, `ComputeSize`, `getSize` | Custom check view embedding an expandable member count label with animated check mark | `crates/gpui_telegram_desktop/src/components/ui/widgets/participants_check_view.rs` (`ParticipantsCheckViewWidgets`, `gpui_ui_1to1`) |
| 40 | `Telegram/SourceFiles/ui/widgets/passcode_strength_meter.cpp` | `L1-L125` | `PasscodeStrengthMeter`, `AdviceText`, `BandColor` | `kSegmentCount = 4` (`passcode_strength_meter.cpp:L20`), Advice categories (TooShort, CommonWord, LeetWord, Sequence, Repeat, Keyboard, DateLike, AddVariety, Fine) | `crates/gpui_telegram_desktop/src/components/ui/widgets/passcode_strength_meter.rs` (`PasscodeStrengthMeterWidgets`, `gpui_ui_1to1`) |
| 41 | `Telegram/SourceFiles/ui/widgets/passcode_strength_meter.h` | `L1-L44` | `PasscodeStrengthMeter`, `PasscodeStrengthAdvice`, `PasscodeStrengthBand` | Strength meter displaying 4 colored segments and dynamic advice for local passcode setup | `crates/gpui_telegram_desktop/src/components/ui/widgets/passcode_strength_meter.rs` (`PasscodeStrengthMeterWidgets`, `gpui_ui_1to1`) |
| 42 | `Telegram/SourceFiles/ui/widgets/passcode_strength_meter.style` | `L1-L44` | `passcodeStrengthMeter`, `passcodeStrengthSegments`, `passcodeStrengthHeight` | 4 segments, 3px height, colors: veryWeak (#E53935), weak (#FB8C00), good (#43A047), strong (#00897B) | `crates/gpui_telegram_desktop/src/theme/ui/passcode_strength_meter_style.rs` (`PasscodeStrengthMeterWidgetsStyleSheet`, `gpui_ui_1to1`) |
| 43 | `Telegram/SourceFiles/ui/widgets/peer_bubble.cpp` | `L1-L72` | `CreatePeerBubble`, `UploadUserpicButton`, `FlatLabel` | Compact pill displaying peer avatar, display name, and background pill with rounded corners | `crates/gpui_telegram_desktop/src/components/ui/widgets/peer_bubble.rs` (`PeerBubbleWidgets`, `gpui_ui_1to1`) |
| 44 | `Telegram/SourceFiles/ui/widgets/peer_bubble.h` | `L1-L26` | `CreatePeerBubble`, `RpWidget` | Factory function creating small pill widget representing selected chat or channel peer | `crates/gpui_telegram_desktop/src/components/ui/widgets/peer_bubble.rs` (`PeerBubbleWidgets`, `gpui_ui_1to1`) |
| 45 | `Telegram/SourceFiles/ui/widgets/selecting_scroll.cpp` | `L1-L108` | `SetupSelectingScroll`, `DeltaOutside`, `SpeedForDelta` | `kSpeedInterval = 15ms`, `kMaxSpeed = 37`, `kMaxFrameInterval = 64ms` (`selecting_scroll.cpp:L20-L22`), speed formula: `min(delta * 3 / 20 + 1, 37)` | `crates/gpui_telegram_desktop/src/components/ui/widgets/selecting_scroll.rs` (`SelectingScrollWidgets`, `gpui_ui_1to1`) |
| 46 | `Telegram/SourceFiles/ui/widgets/selecting_scroll.h` | `L1-L18` | `SetupSelectingScroll`, `scrollBy` | Auto-scrolling helper when mouse drag selection extends outside visible viewport boundaries | `crates/gpui_telegram_desktop/src/components/ui/widgets/selecting_scroll.rs` (`SelectingScrollWidgets`, `gpui_ui_1to1`) |
| 47 | `Telegram/SourceFiles/ui/widgets/sent_code_field.cpp` | `L1-L163` | `SentCodeField`, `setAutoSubmit`, `getDigitsOnly`, `fix` | Digits-only sanitization, automatic submission when target code length (e.g. 5 or 6 digits) is filled | `crates/gpui_telegram_desktop/src/components/ui/widgets/sent_code_field.rs` (`SentCodeFieldWidgets`, `gpui_ui_1to1`) |
| 48 | `Telegram/SourceFiles/ui/widgets/sent_code_field.h` | `L1-L74` | `SentCodeField`, `setAutoSubmit`, `setChangedCallback` | Specialized SMS / Telegram login verification code input field | `crates/gpui_telegram_desktop/src/components/ui/widgets/sent_code_field.rs` (`SentCodeFieldWidgets`, `gpui_ui_1to1`) |
| 49 | `Telegram/SourceFiles/ui/widgets/slider_natural_width.h` | `L1-L21` | `CustomWidthSlider`, `SettingsSlider`, `setNaturalWidth` | Custom width provider specialization for discrete settings sliders | `crates/gpui_telegram_desktop/src/components/ui/widgets/slider_natural_width.rs` (`SliderNaturalWidthWidgets`, `gpui_ui_1to1`) |
| 50 | `Telegram/SourceFiles/ui/widgets/sliding_tabs.cpp` | `L1-L138` | `SlidingTabs`, `showTab`, `finishAnimating` | Tab slide transition using horizontal displacement and cross-fade caching between adjacent vertical layouts | `crates/gpui_telegram_desktop/src/components/ui/widgets/sliding_tabs.rs` (`SlidingTabsWidgets`, `gpui_ui_1to1`) |
| 51 | `Telegram/SourceFiles/ui/widgets/sliding_tabs.h` | `L1-L45` | `SlidingTabs`, `tab`, `showTab` | Tab container managing animated transitions across multiple vertical layout child sections | `crates/gpui_telegram_desktop/src/components/ui/widgets/sliding_tabs.rs` (`SlidingTabsWidgets`, `gpui_ui_1to1`) |
| 52 | `Telegram/SourceFiles/ui/widgets/vertical_drum_picker.cpp` | `L1-L320` | `VerticalDrumPicker`, `DefaultPaintCallback`, `PickerAnimation` | `kAlmostIndex = 0.99`, `kMinYScale = 0.2` (`vertical_drum_picker.cpp:L18-L19`), 3D drum perspective scaling `yScale = 0.2 + 0.8 * easeOutCubic(revProgress)` | `crates/gpui_telegram_desktop/src/components/ui/widgets/vertical_drum_picker.rs` (`VerticalDrumPickerWidgets`, `gpui_ui_1to1`) |
| 53 | `Telegram/SourceFiles/ui/widgets/vertical_drum_picker.h` | `L1-L116` | `VerticalDrumPicker`, `scrollTo`, `jumpTo`, `indexSelected` | Cylindrical drum wheel picker for numbers, dates, and times with inertial scroll physics | `crates/gpui_telegram_desktop/src/components/ui/widgets/vertical_drum_picker.rs` (`VerticalDrumPickerWidgets`, `gpui_ui_1to1`) |

---

## 2. Component Architecture & Mathematical Formulations

### 2.1 Continuous & Discrete Sliders
- **ContinuousSlider (`continuous_sliders.cpp:L20-L160`)**:
  - `kByWheelFinishedTimeout = 1000ms`: debounce timer for mouse wheel interaction.
  - Wheel delta coefficient: `coef = 1.0 / (step * 10.0) = 1.0 / 1200.0`.
  - Platform delta inversion: on macOS `deltaY *= -1`, on Windows/Linux `deltaX *= -1`.
  - Clamping: `std::clamp(value + delta * coef, 0.0, 1.0)`.
  - Step keys (Left/Right/Up/Down): incremental steps with `_adjustCallback` snapping.
- **DiscreteSlider (`discrete_sliders.cpp:L15-L260`)**:
  - Step index mapping: `getIndexFromPosition(pos)` divides track into equal or content-sized sections.
  - Active pill animation: `_a_left` and `_a_width` with 150ms-200ms ease curve to smooth segment transition.
  - Ripple effect triggered on section press.
### 2.2 Chat Filters Tabs Strip & Slider Reordering
- **ChatsFiltersTabs (`chat_filters_tabs_slider.cpp:L20-L100`)**:
  - Unread badge format: `u"99+"` cap for high counters (`chat_filters_tabs_slider.cpp:L29`).
  - Badge width caching for 1-digit, 2-digit, and 3-digit counters (`_cachedBadgeWidths`).
  - Unread skip offset: `_unreadSkip = lineWidth * 5`.
  - Icon skip: `dialogsFiltersTabsIconSkip`.
  - Locked folder separator positioning (`calculateLockedFromX`).
- **ChatsFiltersTabsReorder (`chat_filters_tabs_slider_reorder.cpp:L20-L60`)**:
  - Edge auto-scroll factor: `kScrollFactor = 0.05` (`chat_filters_tabs_slider_reorder.cpp:L20`).
  - Drag-and-drop horizontal shift calculation animating unpinned tabs away from dragged item.

### 2.3 Buttons, Labels & Visual Meters
- **GradientRoundButton (`gradient_round_button.cpp:L17-L70`)**:
  - Ripple overlay color: `rgba(0, 0, 0, 36)` (`gradient_round_button.cpp:L45`).
  - Glare sweep: edge width `edgeWidth = glare.width + st::roundRadiusLarge`.
- **MarqueeLabel (`marquee_label.cpp:L29-L370`)**:
  - `kMarqueeDelay = 500ms`: pause before starting scrolling cycle.
  - `kFrameDelta = 16ms`: ~60fps step timer.
  - `kEdgeScrollSpeed = 3.0`: accelerated edge displacement.
  - Fade ramps on leading and trailing text edges (`st::marqueeLabelFadeRamp`).
- **CrossFadeLabel (`cross_fade_label.cpp:L15-L50`)**:
  - `kDuration = 200ms` with `anim::easeOutCubic`.
  - Slide displacement: `st.font->height`.
- **LevelMeter (`level_meter.cpp:L20-L45`)**:
  - `radius = lineWidth / 2`.
  - Line coordinate: `x = (lineWidth + lineSpacing) * i`.
- **PasscodeStrengthMeter (`passcode_strength_meter.cpp:L20-L55`)**:
  - `kSegmentCount = 4`: VeryWeak, Weak, Good, Strong.
  - Heuristics: TooShort, CommonWord, LeetWord, Sequence, Repeat, Keyboard, DateLike, AddVariety, Fine.

### 2.4 Selection, Scroll Helpers & Inputs
- **MultiSelect (`multi_select.cpp:L25-L280`)**:
  - Scale factor: `kWideScale = 3`.
  - Cross delete animation with cross-fade opacity and scale.
- **SelectingScroll (`selecting_scroll.cpp:L20-L42`)**:
  - `kSpeedInterval = 15ms`, `kMaxSpeed = 37`, `kMaxFrameInterval = 64ms`.
  - Velocity formula: `SpeedForDelta(delta) = (delta > 0) ? min(delta * 3 / 20 + 1, 37) : max(delta * 3 / 20 - 1, -37)`.
- **MiddleClickAutoscroll (`middle_click_autoscroll.cpp:L17-L22`)**:
  - `kDelay = 15ms`, `kHoldToToggleThreshold = 220ms`.
  - `Deadzone = 6px`.
  - Arrow geometry: `outerStroke = 1.5`, `centerSkip = 10.5`, `arrowGap = 5.0`, `headHalfWidth = 4.5`.
- **VerticalDrumPicker (`vertical_drum_picker.cpp:L18-L45`)**:
  - `kAlmostIndex = 0.99`, `kMinYScale = 0.2`.
  - Perspective scaling: `yScale = 0.2 + 0.8 * easeOutCubic(revProgress)`.
- **SpecialFields (`fields/special_fields.cpp:L19-L26`)**:
  - Phone & Username constants: `kMaxUsernameLength = 32`, `kMaxPhoneTailLength = 32`, `kMaxPhoneCodeLength = 4`.
- **SentCodeField (`sent_code_field.cpp:L21-L60`)**:
  - Digit extraction and auto-submission trigger on matching expected code length.
