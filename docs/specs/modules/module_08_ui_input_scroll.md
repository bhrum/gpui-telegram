# Module 08 Slice 2 Specification: UI Inputs, Labels, Scroll & Interactive Widgets

## 1. Executive Summary & Architecture Overview

This specification establishes the deep C++ source parity for Slice 2 of Module 08 (`ui & boxes`) under **User Mandates #15, #18, #19, and #20**. Slice 2 encompasses all upstream Telegram Desktop text input fields, specialized formatters, single and cross-fading labels, marquee scrolling text, auto-scroll and selecting scroll controllers, multi-select bubble pickers, passcode strength indicators, vertical drum pickers, and resize dividers.

### 1.1 Architecture & Subsystem Boundaries
In Telegram Desktop (`tdesktop`), interactive widgets extend `Ui::RpWidget` (reactive painting widget). The text inputs, scroll controllers, and label widgets manage real-time text layout, cursor animations, middle-click autoscroll velocity integration, and smooth easing transitions.

In our GPUI reconstruction:
1. **Input Fields & Formatted Inputs**: Implement genuine GPUI elements with cursor blink, selection highlight, masked digit validation, and dynamic placeholder animations.
2. **Smooth Scroll & Autoscroll Mechanics**: Replicate upstream momentum curves, deadzones, and boundary acceleration for selecting and middle-click scrolling.
3. **Labels & Dynamic Text**: Implement cross-fading text transitions (`anim::easeOutCubic`) and continuous marquee ticker animations with boundary deceleration.
4. **Interactive Selection & Tooltips**: Implement chip bubbles with sliding animations, expander check views, and glare gradient tooltip sweeps.

## 2. Mandate #19 Pillar 2: 100% Per-File Reading Manifest

The following reading manifest documents all 43 upstream C++ and style source files assigned to Module 08 Slice 2 (`ui & boxes`), establishing full line-by-line coverage with zero omissions.

| # | Upstream Source Path | Lines | Read Range | Extracted C++ Logic & Constants | Target GPUI Component |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/ui/widgets/cross_fade_label.h` | 45 | L1-L45 | Declares `Ui::CrossFadeLabel` extending `RpWidget`, text transition direction, `Animations::Simple` | `components/ui/widgets/cross_fade_label.rs` |
| 2 | `Telegram/SourceFiles/ui/widgets/cross_fade_label.cpp` | 82 | L1-L82 | `kDuration = crl::time(200);`, `anim::easeOutCubic`, slide height offset, alpha cross-fade math | `components/ui/widgets/cross_fade_label.rs` |
| 3 | `Telegram/SourceFiles/ui/widgets/marquee_label.h` | 126 | L1-L126 | Declares `Ui::MarqueeLabel`, accessibility roles, text selection hooks, edge scroll producer | `components/ui/widgets/marquee_label.rs` |
| 4 | `Telegram/SourceFiles/ui/widgets/marquee_label.cpp` | 791 | L1-L791 | `kMarqueeDelay = 500ms;`, `kFrameDelta = 16ms;`, `kEdgeScrollSpeed = 3.0;`, slowdown & acceleration curves | `components/ui/widgets/marquee_label.rs` |
| 5 | `Telegram/SourceFiles/ui/widgets/marquee_label.style` | 16 | L1-L16 | `marqueeLabelGap: 16px;`, `marqueeLabelFade: 6px;`, `marqueeLabelSlowdown: 100px;`, speeds 50px/30px | `theme/ui/marquee_label_style.rs` |
| 6 | `Telegram/SourceFiles/ui/widgets/middle_click_autoscroll.h` | 57 | L1-L57 | `MiddleClickAutoscroll` state machine, `toggleOrBeginHold`, cursor modes (`Neutral`, `Up`, `Down`) | `components/ui/widgets/middle_click_autoscroll.rs` |
| 7 | `Telegram/SourceFiles/ui/widgets/middle_click_autoscroll.cpp` | 244 | L1-L244 | `kDelay = 15;`, `kHoldToToggleThreshold = 220ms;`, `Deadzone() = 6px;`, quadratic speed scale | `components/ui/widgets/middle_click_autoscroll.rs` |
| 8 | `Telegram/SourceFiles/ui/widgets/selecting_scroll.h` | 18 | L1-L18 | `SetupSelectingScroll(widget, scrollBy)` functional interface | `components/ui/widgets/selecting_scroll.rs` |
| 9 | `Telegram/SourceFiles/ui/widgets/selecting_scroll.cpp` | 109 | L1-L109 | `kSpeedInterval = 15ms;`, `kMaxSpeed = 37;`, `kMaxFrameInterval = 64ms;`, `delta * 3 / 20 + 1` formula | `components/ui/widgets/selecting_scroll.rs` |
| 10 | `Telegram/SourceFiles/ui/widgets/multi_select.h` | 77 | L1-L77 | `MultiSelect` recipient bubble picker interface, query changes, tag deletion callbacks | `components/ui/widgets/multi_select.rs` |
| 11 | `Telegram/SourceFiles/ui/widgets/multi_select.cpp` | 1147 | L1-L1147 | `kWideScale = 3;`, `Item` pill geometry, `radius = height / 2;`, `SlideAnimation` between rows | `components/ui/widgets/multi_select.rs` |
| 12 | `Telegram/SourceFiles/ui/widgets/sent_code_field.h` | 74 | L1-L74 | `SentCodeField` auto-submit length, `SentCodeCall` countdown state (`Waiting`, `Calling`, `Called`) | `components/ui/widgets/sent_code_field.rs` |
| 13 | `Telegram/SourceFiles/ui/widgets/sent_code_field.cpp` | 164 | L1-L164 | Digit filtering, hyphen preserve, countdown mm:ss formatting (`timeout / 60`, `timeout % 60`) | `components/ui/widgets/sent_code_field.rs` |
| 14 | `Telegram/SourceFiles/ui/widgets/fields/special_fields.h` | 147 | L1-L147 | `CountryCodeInput`, `PhonePartInput`, `UsernameInput`, `PhoneInput` composite widget classes | `components/ui/widgets/fields/special_fields.rs` |
| 15 | `Telegram/SourceFiles/ui/widgets/fields/special_fields.cpp` | 492 | L1-L492 | `kMaxUsernameLength = 32;`, `kMaxPhoneTailLength = 32;`, `kMaxPhoneCodeLength = 4;`, phone grouping | `components/ui/widgets/fields/special_fields.rs` |
| 16 | `Telegram/SourceFiles/ui/widgets/fields/time_part_input_with_placeholder.h` | 40 | L1-L40 | `TimePartWithPlaceholder` zero-padding and phrase alignment interface | `components/ui/widgets/fields/time_part_input_with_placeholder.rs` |
| 17 | `Telegram/SourceFiles/ui/widgets/fields/time_part_input_with_placeholder.cpp` | 63 | L1-L63 | `leftMargins = (width - placeholder.width)/2 + leftOffset;`, dynamic number phrase injection | `components/ui/widgets/fields/time_part_input_with_placeholder.rs` |
| 18 | `Telegram/SourceFiles/ui/widgets/glare_tooltip.h` | 82 | L1-L82 | `GlareTooltip` floating tooltip with sweeping metallic glare highlight | `components/ui/widgets/glare_tooltip.rs` |
| 19 | `Telegram/SourceFiles/ui/widgets/glare_tooltip.cpp` | 251 | L1-L251 | `kGlareDurationStep = 320ms;`, `kGlareTimeout = 1000ms;`, `glareSize = outer.height * 3;` | `components/ui/widgets/glare_tooltip.rs` |
| 20 | `Telegram/SourceFiles/ui/widgets/level_meter.h` | 33 | L1-L33 | `LevelMeter` audio recording / microphone volume bar array interface | `components/ui/widgets/level_meter.rs` |
| 21 | `Telegram/SourceFiles/ui/widgets/level_meter.cpp` | 49 | L1-L49 | `valueAtLine = (i + 1) / lineCount;`, `radius = lineWidth / 2;`, spacing & active brush transitions | `components/ui/widgets/level_meter.rs` |
| 22 | `Telegram/SourceFiles/ui/widgets/peer_bubble.h` | 26 | L1-L26 | `CreatePeerBubble(parent, peer)` pill avatar widget constructor | `components/ui/widgets/peer_bubble.rs` |
| 23 | `Telegram/SourceFiles/ui/widgets/peer_bubble.cpp` | 73 | L1-L73 | Combined avatar width + name width + padding, pill round rect `radius = height / 2;` | `components/ui/widgets/peer_bubble.rs` |
| 24 | `Telegram/SourceFiles/ui/widgets/participants_check_view.h` | 64 | L1-L64 | `ExpanderCheckView` animated toggle checkbox with participant count expander | `components/ui/widgets/participants_check_view.rs` |
| 25 | `Telegram/SourceFiles/ui/widgets/participants_check_view.cpp` | 141 | L1-L141 | `SetExpanderText`, `ComputeSize`, `ToggleUpDownArrowPath` 4-stroke animation, round-rect ripple | `components/ui/widgets/participants_check_view.rs` |
| 26 | `Telegram/SourceFiles/ui/widgets/passcode_strength_meter.h` | 44 | L1-L44 | `PasscodeStrengthMeter` candidate evaluation and advice display | `components/ui/widgets/passcode_strength_meter.rs` |
| 27 | `Telegram/SourceFiles/ui/widgets/passcode_strength_meter.cpp` | 126 | L1-L126 | `kSegmentCount = 4;`, `AdviceText`, 4-segment progress layout with `available * (i + 1) / 4` math | `components/ui/widgets/passcode_strength_meter.rs` |
| 28 | `Telegram/SourceFiles/ui/widgets/passcode_strength_meter.style` | 45 | L1-L45 | `height: 4px;`, `segmentSpacing: 4px;`, `segmentRadius: 2px;`, `labelSkip: 6px;`, 4 band colors | `theme/ui/passcode_strength_meter_style.rs` |
| 29 | `Telegram/SourceFiles/ui/widgets/vertical_drum_picker.h` | 116 | L1-L116 | `VerticalDrumPicker` 3D cylindrical drum selection wheel | `components/ui/widgets/vertical_drum_picker.rs` |
| 30 | `Telegram/SourceFiles/ui/widgets/vertical_drum_picker.cpp` | 321 | L1-L321 | `kAlmostIndex = 0.99;`, `kMinYScale = 0.2;`, `yScale = 0.2 + 0.8 * easeOutCubic`, 3D projection | `components/ui/widgets/vertical_drum_picker.rs` |
| 31 | `Telegram/SourceFiles/ui/widgets/horizontal_fit_container.h` | 36 | L1-L36 | `HorizontalFitContainer` dynamic proportional grid layout for buttons | `components/ui/widgets/horizontal_fit_container.rs` |
| 32 | `Telegram/SourceFiles/ui/widgets/horizontal_fit_container.cpp` | 57 | L1-L57 | `buttonWidth = (width - totalSpacing) / count;`, horizontal geometry distribution | `components/ui/widgets/horizontal_fit_container.rs` |
| 33 | `Telegram/SourceFiles/ui/widgets/expandable_peer_list.h` | 43 | L1-L43 | `ExpandablePeerListController` and `AddExpandablePeerList` helper | `components/ui/widgets/expandable_peer_list.rs` |
| 34 | `Telegram/SourceFiles/ui/widgets/expandable_peer_list.cpp` | 208 | L1-L208 | `ParticipantsExpanderText`, `ExpanderButton`, `SlideWrap` fold/unfold animation | `components/ui/widgets/expandable_peer_list.rs` |
| 35 | `Telegram/SourceFiles/ui/countryinput.h` | 67 | L1-L67 | `CountryInput` selector button with country name, phone code, and dropdown arrow | `components/ui/countryinput.rs` |
| 36 | `Telegram/SourceFiles/ui/countryinput.cpp` | 202 | L1-L202 | Border painting, `st::introCountryIcon`, country chosen callback to `CountrySelectBox` | `components/ui/countryinput.rs` |
| 37 | `Telegram/SourceFiles/ui/rows_scroll_cache.h` | 67 | L1-L67 | `RowsScrollCache` pixmap buffer for accelerated smooth list scrolling | `components/ui/rows_scroll_cache.rs` |
| 38 | `Telegram/SourceFiles/ui/rows_scroll_cache.cpp` | 43 | L1-L43 | `kStopTimeout = 120ms;`, invalidate on mutation, cache clearing on scroll stop | `components/ui/rows_scroll_cache.rs` |
| 39 | `Telegram/SourceFiles/ui/search_field_controller.h` | 57 | L1-L57 | `SearchFieldController` reactive query stream and row view builder | `components/ui/search_field_controller.rs` |
| 40 | `Telegram/SourceFiles/ui/search_field_controller.cpp` | 118 | L1-L118 | `createRowView`, `CrossButton` cancel toggle animation, `PlainShadow` bottom divider | `components/ui/search_field_controller.rs` |
| 41 | `Telegram/SourceFiles/ui/vertical_list.h` | 52 | L1-L52 | `AddSkip`, `AddDivider`, `AddDividerText`, `AddSubsectionTitle` list builder DSL | `components/ui/vertical_list.rs` |
| 42 | `Telegram/SourceFiles/ui/vertical_list.cpp` | 84 | L1-L84 | `st::defaultVerticalListSkip`, `st::boxDividerHeight`, `st::defaultSubsectionTitlePadding` | `components/ui/vertical_list.rs` |
| 43 | `Telegram/SourceFiles/ui/resize_area.h` | 79 | L1-L79 | `ResizeArea` drag splitter, `cur_sizehor`, `moveLeft` event stream, `moveFinished` callback | `components/ui/resize_area.rs` |

## 3. Mandate #18 Anti-Bluffing C++ Algorithm Deep-Dive

### 3.1 CrossFadeLabel Alpha & Spatial Translation Transitions
- **Source**: `ui/widgets/cross_fade_label.cpp:L13-L80`
- **Constants**: `kDuration = crl::time(200);`
- **Mathematical Model**:
  When new text arrives, the existing text slides vertically by `slide = font->height` in `_direction` (+1 downward, -1 upward) while fading out (`opacity = 1.0 - progress`). Concurrently, incoming text starts at `-direction * slide` and translates to 0 while fading in (`opacity = progress`). Interpolation strictly follows `anim::easeOutCubic`:
  $$f(t) = 1 - (1 - t)^3$$
  Exact line references: `cross_fade_label.cpp:L48-L50` and `cross_fade_label.cpp:L60-L78`.

### 3.2 MarqueeLabel Acceleration, Speed Tiers & Edge Scrolling
- **Source**: `ui/widgets/marquee_label.cpp:L27-L33`, `ui/widgets/marquee_label.cpp:L218-L246`, `ui/widgets/marquee_label.style:L10-L15`
- **Constants & Tokens**:
  - `kMarqueeDelay = 500ms;` initial pause before scrolling begins
  - `kFrameDelta = 16ms;` target ~60 FPS frame rate interval
  - `kEdgeScrollSpeed = 3.0;` speed factor per pixel when cursor is outside
  - `marqueeLabelGap = 16px;` spacing between repeated text loops
  - `marqueeLabelSlowdown = 100px;` acceleration / deceleration transition distance
  - `marqueeLabelSpeed = 50px/s;` constant cruising speed
  - `marqueeLabelSpeedSlow = 30px/s;` boundary approach speed
- **Velocity Curve**:
  $$\text{speed}(x) = \begin{cases}
  v_{\text{slow}} + (v_{\text{fast}} - v_{\text{slow}}) \cdot \frac{x}{d_{\text{slow}}}, & \text{if } x < d_{\text{slow}} \\
  v_{\text{fast}}, & \text{if } d_{\text{slow}} \le x < \text{total} - d_{\text{slow}} \\
  v_{\text{fast}} - (v_{\text{fast}} - v_{\text{slow}}) \cdot \frac{x - (\text{total} - d_{\text{slow}})}{d_{\text{slow}}}, & \text{if } x \ge \text{total} - d_{\text{slow}}
  \end{cases}$$
  Exact line references: `marquee_label.cpp:L225-L236`.

### 3.3 MiddleClickAutoscroll Deadzone & Speed Integration
- **Source**: `ui/widgets/middle_click_autoscroll.cpp:L16-L23`, `ui/widgets/middle_click_autoscroll.cpp:L214-L241`
- **Constants**:
  - `kDelay = 15ms;` timer dispatch interval
  - `kHoldToToggleThreshold = 220ms;` distinction between click-to-activate and hold-and-release
  - `Deadzone() = 6px;` minimum delta from starting click anchor before scrolling begins
  - `Cursor size = 27px;` with inner circular hub and dual directional arrowheads
- **Algorithm**:
  $$\text{effective\_delta} = \max(0, |\Delta y| - \text{Deadzone})$$
  $$\text{velocity} = \min(\text{effective\_delta} \cdot \text{scale}, v_{\text{max}})$$
  $$\text{step} = \text{round}\left(\frac{\text{velocity} \cdot \Delta t}{1000}\right)$$
  Exact line references: `middle_click_autoscroll.cpp:L226-L240`.

### 3.4 SelectingScroll Frame Accumulation & Velocity Clamping
- **Source**: `ui/widgets/selecting_scroll.cpp:L18-L43`, `ui/widgets/selecting_scroll.cpp:L63-L80`
- **Constants**:
  - `kSpeedInterval = 15ms;` base tick interval
  - `kMaxSpeed = 37px;` maximum allowable scroll speed per tick
  - `kMaxFrameInterval = 64ms;` threshold guarding against background latency spikes
- **Delta-to-Speed Formula**:
  $$\text{speed}(\Delta) = \begin{cases}
  \min\left(\lfloor\Delta \cdot \frac{3}{20}\rfloor + 1, 37\right), & \text{if } \Delta > 0 \\
  \max\left(\lfloor\Delta \cdot \frac{3}{20}\rfloor - 1, -37\right), & \text{if } \Delta < 0
  \end{cases}$$
  Exact line references: `selecting_scroll.cpp:L38-L42` and `selecting_scroll.cpp:L73-L79`.

### 3.5 SentCodeField & Countdown State Machine
- **Source**: `ui/widgets/sent_code_field.cpp:L21-L98`, `ui/widgets/sent_code_field.cpp:L100-L162`
- **State FSM**:
  - `State::Waiting` (timer ticks down each 1000ms; mm:ss formatted text)
  - `State::Calling` (triggering external call handler)
  - `State::Called` (call complete)
- **Formatting Rules**:
  Preserves existing `-` hyphens while strictly enforcing digit count. When `digitCount == autoSubmitLength`, invokes submit callback immediately without requiring manual Return/Enter.
  Exact line references: `sent_code_field.cpp:L59-L97`.

### 3.6 SpecialFields (CountryCode, PhonePart, Username)
- **Source**: `ui/widgets/fields/special_fields.cpp:L18-L28`, `ui/widgets/fields/special_fields.cpp:L63-L108`, `ui/widgets/fields/special_fields.cpp:L145-L248`, `ui/widgets/fields/special_fields.cpp:L315-L343`
- **Constants**:
  - `kMaxUsernameLength = 32;`
  - `kMaxPhoneTailLength = 32;`
  - `kMaxPhoneCodeLength = 4;`
- **Behavior**:
  - `CountryCodeInput`: prepends `+`, filters out non-digits, strips leading `+` on change notifications.
  - `PhonePartInput`: dynamically spaces digits matching country phone number pattern chunks, paints custom unicode dash `\u{2212}` placeholder glyphs.
  - `UsernameInput`: trims surrounding whitespace, handles `@` prefix, enforces 32-character maximum length.

### 3.7 PasscodeStrengthMeter Segment Math
- **Source**: `ui/widgets/passcode_strength_meter.cpp:L18-L58`, `ui/widgets/passcode_strength_meter.cpp:L100-L123`, `passcode_strength_meter.style:L24-L44`
- **Constants**:
  - `kSegmentCount = 4;`
  - `height = 4px; segmentSpacing = 4px; segmentRadius = 2px; labelSkip = 6px;`
- **Segment Partitioning**:
  $$\text{available} = \text{width} - \text{spacing} \cdot (\text{segments} - 1)$$
  $$\text{segment}(i) = \left\lfloor\frac{\text{available} \cdot (i + 1)}{\text{segments}}\right\rfloor - \left\lfloor\frac{\text{available} \cdot i}{\text{segments}}\right\rfloor$$
  Each segment $i < \text{filled}$ receives band color (`VeryWeak`, `Weak`, `Good`, `Strong`), remainder receives `emptyFg`.
  Exact line references: `passcode_strength_meter.cpp:L106-L122`.

### 3.8 VerticalDrumPicker 3D Projection
- **Source**: `ui/widgets/vertical_drum_picker.cpp:L16-L50`
- **Constants**:
  - `kAlmostIndex = 0.99;`
  - `kMinYScale = 0.2;`
- **Scale & Opacity Formula**:
  $$\text{rev\_progress} = 1.0 - |\text{distanceFromCenter}|$$
  $$y_{\text{scale}} = 0.2 + 0.8 \cdot \text{easeOutCubic}(\text{rev\_progress})$$
  $$\text{opacity} = \text{rev\_progress}$$
  Exact line references: `vertical_drum_picker.cpp:L36-L45`.

## 4. Subsystem Verification & Oracle Test Matrix

To fulfill Mandate #18 Layer 3 (Adversarial Oracle Tests), the test suite `crates/gpui_telegram_desktop/tests/ui_input_scroll_subsystem_tests.rs` validates the exact mathematical bounds, timing constants, and state transitions extracted from the upstream `.cpp` implementations.

### 4.1 Oracle Test Cases
1. `oracle_cross_fade_label_cubic_interpolation`:
   - Validates `easeOutCubic` curve: $f(0.5) = 1 - 0.5^3 = 0.875$.
   - Confirms spatial translation $dx = \text{height} \cdot \text{direction} \cdot f(t)$.
2. `oracle_marquee_label_speed_slowdown_curve`:
   - Validates $v = 30$ at offset 0, ramp to $v = 50$ at 100px, deceleration back to $v = 30$ at $(\text{total} - 100\text{px})$.
   - Confirms `kMarqueeDelay` = 500ms initial hold.
3. `oracle_middle_click_autoscroll_deadzone_and_threshold`:
   - Validates that mouse deltas within $\pm 6\text{px}$ deadzone yield 0 scroll velocity and `CursorMode::Neutral`.
   - Validates that hold duration $< 220\text{ms}$ engages toggle mode, while $\ge 220\text{ms}$ releases immediately on button up.
4. `oracle_selecting_scroll_formula_and_max_clamp`:
   - Tests delta calculations: delta 20 -> $\min(20 \cdot 3 / 20 + 1, 37) = 4\text{px}$; delta 400 -> clamped at 37px. Negative delta -20 -> $\max(-20 \cdot 3 / 20 - 1, -37) = -4\text{px}$.
5. `oracle_sent_code_formatting_and_auto_submit`:
   - Tests digit filtering with hyphen preservation: `"12-34"` formatted as `"12-34"`, non-digits rejected.
   - Tests auto-submit triggers when digit count hits specified length.
   - Tests `SentCodeCall` mm:ss formatting: 65s -> `"1:05"`, 3661s -> `"1:01:01"`.
6. `oracle_special_fields_country_code_and_username`:
   - Tests country code always prefixes `+`, ignores non-digits.
   - Tests username whitespace trim and 32-character maximum clamp.
7. `oracle_passcode_strength_meter_segment_geometry`:
   - Tests that 4 segments with 4px spacing sum exactly to the parent available width without pixel drift.
   - Tests band color selection (`VeryWeak`, `Weak`, `Good`, `Strong`).
8. `oracle_vertical_drum_picker_y_scale_curve`:
   - Tests that center item has `yScale = 1.0`, items at distance 1.0 have `yScale = 0.2`, and opacity drops to 0.
9. `oracle_rows_scroll_cache_timeout`:
   - Tests that scrolling caches row bitmaps and 120ms timeout flushes the buffer.
10. `oracle_reusable_widgets_integration`:
    - Tests `InputField`, `ScrollArea`, `Dropdown`, `LabelSimple`, and `Callout` state and layout.
