# Subsystem Shard: `statistics`

**Total Source Files**: 43  
**Total Source Lines**: 7,229  
**Primary Feature IDs**: TCG-N12, TCG-N37  
**JSON Shard Path**: `docs/specs/ledger/shards/statistics.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/statistics/chart_lines_filter_controller.cpp` | `TCG-N37` | `ChartLinesFilterController` | `crates/gpui_telegram_server/src/statistics/chart_lines_filter_controller.rs` | 71 | `tests/test_tcg_n37_chart_lines_filter_controller.rs` |
| `Telegram/SourceFiles/statistics/chart_lines_filter_controller.h` | `TCG-N37` | `ChartLinesFilterController` | `crates/gpui_telegram_server/src/statistics/chart_lines_filter_controller.rs` | 39 | `tests/test_tcg_n37_chart_lines_filter_controller.rs` |
| `Telegram/SourceFiles/statistics/chart_rulers_data.cpp` | `TCG-N12` | `ChartRulersData` | `crates/gpui_telegram_server/src/statistics/chart_rulers_data.rs` | 135 | `tests/test_tcg_n12_chart_rulers_data.rs` |
| `Telegram/SourceFiles/statistics/chart_rulers_data.h` | `TCG-N12` | `ChartRulersData` | `crates/gpui_telegram_server/src/statistics/chart_rulers_data.rs` | 44 | `tests/test_tcg_n12_chart_rulers_data.rs` |
| `Telegram/SourceFiles/statistics/chart_widget.cpp` | `TCG-N12` | `ChartWidget` | `crates/gpui_telegram_server/src/statistics/chart_widget.rs` | 1567 | `tests/test_tcg_n12_chart_widget.rs` |
| `Telegram/SourceFiles/statistics/chart_widget.h` | `TCG-N12` | `ChartWidget` | `crates/gpui_telegram_server/src/statistics/chart_widget.rs` | 184 | `tests/test_tcg_n12_chart_widget.rs` |
| `Telegram/SourceFiles/statistics/segment_tree.cpp` | `TCG-N12` | `SegmentTree` | `crates/gpui_telegram_server/src/statistics/segment_tree.rs` | 151 | `tests/test_tcg_n12_segment_tree.rs` |
| `Telegram/SourceFiles/statistics/segment_tree.h` | `TCG-N12` | `SegmentTree` | `crates/gpui_telegram_server/src/statistics/segment_tree.rs` | 71 | `tests/test_tcg_n12_segment_tree.rs` |
| `Telegram/SourceFiles/statistics/statistics.style` | `TCG-N12` | `StatisticsStyleSheet` | `crates/gpui_telegram_desktop/src/theme/statistics/statistics_style.rs` | 177 | `tests/test_tcg_n12_statistics.rs` |
| `Telegram/SourceFiles/statistics/statistics_common.h` | `TCG-N12` | `StatisticsCommon` | `crates/gpui_telegram_server/src/statistics/statistics_common.rs` | 27 | `tests/test_tcg_n12_statistics_common.rs` |
| `Telegram/SourceFiles/statistics/statistics_data_deserialize.cpp` | `TCG-N12` | `StatisticsDataDeserialize` | `crates/gpui_telegram_server/src/statistics/statistics_data_deserialize.rs` | 183 | `tests/test_tcg_n12_statistics_data_deserialize.rs` |
| `Telegram/SourceFiles/statistics/statistics_data_deserialize.h` | `TCG-N12` | `StatisticsDataDeserialize` | `crates/gpui_telegram_server/src/statistics/statistics_data_deserialize.rs` | 21 | `tests/test_tcg_n12_statistics_data_deserialize.rs` |
| `Telegram/SourceFiles/statistics/statistics_format_values.cpp` | `TCG-N12` | `StatisticsFormatValues` | `crates/gpui_telegram_server/src/statistics/statistics_format_values.rs` | 70 | `tests/test_tcg_n12_statistics_format_values.rs` |
| `Telegram/SourceFiles/statistics/statistics_format_values.h` | `TCG-N12` | `StatisticsFormatValues` | `crates/gpui_telegram_server/src/statistics/statistics_format_values.rs` | 16 | `tests/test_tcg_n12_statistics_format_values.rs` |
| `Telegram/SourceFiles/statistics/statistics_graphics.cpp` | `TCG-N12` | `StatisticsGraphics` | `crates/gpui_telegram_server/src/statistics/statistics_graphics.rs` | 42 | `tests/test_tcg_n12_statistics_graphics.rs` |
| `Telegram/SourceFiles/statistics/statistics_graphics.h` | `TCG-N12` | `StatisticsGraphics` | `crates/gpui_telegram_server/src/statistics/statistics_graphics.rs` | 20 | `tests/test_tcg_n12_statistics_graphics.rs` |
| `Telegram/SourceFiles/statistics/statistics_sheets.cpp` | `TCG-N12` | `StatisticsSheets` | `crates/gpui_telegram_server/src/statistics/statistics_sheets.rs` | 265 | `tests/test_tcg_n12_statistics_sheets.rs` |
| `Telegram/SourceFiles/statistics/statistics_sheets.h` | `TCG-N12` | `StatisticsSheets` | `crates/gpui_telegram_server/src/statistics/statistics_sheets.rs` | 23 | `tests/test_tcg_n12_statistics_sheets.rs` |
| `Telegram/SourceFiles/statistics/statistics_types.h` | `TCG-N12` | `StatisticsTypes` | `crates/gpui_telegram_server/src/statistics/statistics_types.rs` | 14 | `tests/test_tcg_n12_statistics_types.rs` |
| `Telegram/SourceFiles/statistics/statistics_xlsx.cpp` | `TCG-N12` | `StatisticsXlsx` | `crates/gpui_telegram_server/src/statistics/statistics_xlsx.rs` | 429 | `tests/test_tcg_n12_statistics_xlsx.rs` |
| `Telegram/SourceFiles/statistics/statistics_xlsx.h` | `TCG-N12` | `StatisticsXlsx` | `crates/gpui_telegram_server/src/statistics/statistics_xlsx.rs` | 41 | `tests/test_tcg_n12_statistics_xlsx.rs` |
| `Telegram/SourceFiles/statistics/view/abstract_chart_view.cpp` | `TCG-N12` | `AbstractChartViewView` | `crates/gpui_telegram_server/src/statistics/view/abstract_chart_view.rs` | 103 | `tests/test_tcg_n12_abstract_chart_view.rs` |
| `Telegram/SourceFiles/statistics/view/abstract_chart_view.h` | `TCG-N12` | `AbstractChartViewView` | `crates/gpui_telegram_server/src/statistics/view/abstract_chart_view.rs` | 127 | `tests/test_tcg_n12_abstract_chart_view.rs` |
| `Telegram/SourceFiles/statistics/view/bar_chart_view.cpp` | `TCG-N12` | `BarChartViewView` | `crates/gpui_telegram_server/src/statistics/view/bar_chart_view.rs` | 286 | `tests/test_tcg_n12_bar_chart_view.rs` |
| `Telegram/SourceFiles/statistics/view/bar_chart_view.h` | `TCG-N12` | `BarChartViewView` | `crates/gpui_telegram_server/src/statistics/view/bar_chart_view.rs` | 66 | `tests/test_tcg_n12_bar_chart_view.rs` |
| `Telegram/SourceFiles/statistics/view/chart_rulers_view.cpp` | `TCG-N12` | `ChartRulersViewView` | `crates/gpui_telegram_server/src/statistics/view/chart_rulers_view.rs` | 205 | `tests/test_tcg_n12_chart_rulers_view.rs` |
| `Telegram/SourceFiles/statistics/view/chart_rulers_view.h` | `TCG-N12` | `ChartRulersViewView` | `crates/gpui_telegram_server/src/statistics/view/chart_rulers_view.rs` | 59 | `tests/test_tcg_n12_chart_rulers_view.rs` |
| `Telegram/SourceFiles/statistics/view/chart_view_factory.cpp` | `TCG-N12` | `ChartViewFactoryView` | `crates/gpui_telegram_server/src/statistics/view/chart_view_factory.rs` | 38 | `tests/test_tcg_n12_chart_view_factory.rs` |
| `Telegram/SourceFiles/statistics/view/chart_view_factory.h` | `TCG-N12` | `ChartViewFactoryView` | `crates/gpui_telegram_server/src/statistics/view/chart_view_factory.rs` | 18 | `tests/test_tcg_n12_chart_view_factory.rs` |
| `Telegram/SourceFiles/statistics/view/linear_chart_view.cpp` | `TCG-N12` | `LinearChartViewView` | `crates/gpui_telegram_server/src/statistics/view/linear_chart_view.rs` | 227 | `tests/test_tcg_n12_linear_chart_view.rs` |
| `Telegram/SourceFiles/statistics/view/linear_chart_view.h` | `TCG-N12` | `LinearChartViewView` | `crates/gpui_telegram_server/src/statistics/view/linear_chart_view.rs` | 95 | `tests/test_tcg_n12_linear_chart_view.rs` |
| `Telegram/SourceFiles/statistics/view/stack_chart_common.cpp` | `TCG-N12` | `StackChartCommonView` | `crates/gpui_telegram_server/src/statistics/view/stack_chart_common.rs` | 87 | `tests/test_tcg_n12_stack_chart_common.rs` |
| `Telegram/SourceFiles/statistics/view/stack_chart_common.h` | `TCG-N12` | `StackChartCommonView` | `crates/gpui_telegram_server/src/statistics/view/stack_chart_common.rs` | 34 | `tests/test_tcg_n12_stack_chart_common.rs` |
| `Telegram/SourceFiles/statistics/view/stack_linear_chart_common.cpp` | `TCG-N12` | `StackLinearChartCommonView` | `crates/gpui_telegram_server/src/statistics/view/stack_linear_chart_common.rs` | 99 | `tests/test_tcg_n12_stack_linear_chart_common.rs` |
| `Telegram/SourceFiles/statistics/view/stack_linear_chart_common.h` | `TCG-N12` | `StackLinearChartCommonView` | `crates/gpui_telegram_server/src/statistics/view/stack_linear_chart_common.rs` | 39 | `tests/test_tcg_n12_stack_linear_chart_common.rs` |
| `Telegram/SourceFiles/statistics/view/stack_linear_chart_view.cpp` | `TCG-N12` | `StackLinearChartViewView` | `crates/gpui_telegram_server/src/statistics/view/stack_linear_chart_view.rs` | 1010 | `tests/test_tcg_n12_stack_linear_chart_view.rs` |
| `Telegram/SourceFiles/statistics/view/stack_linear_chart_view.h` | `TCG-N12` | `StackLinearChartViewView` | `crates/gpui_telegram_server/src/statistics/view/stack_linear_chart_view.rs` | 150 | `tests/test_tcg_n12_stack_linear_chart_view.rs` |
| `Telegram/SourceFiles/statistics/widgets/chart_header_widget.cpp` | `TCG-N12` | `ChartHeaderWidgetWidgets` | `crates/gpui_telegram_server/src/statistics/widgets/chart_header_widget.rs` | 58 | `tests/test_tcg_n12_chart_header_widget.rs` |
| `Telegram/SourceFiles/statistics/widgets/chart_header_widget.h` | `TCG-N12` | `ChartHeaderWidgetWidgets` | `crates/gpui_telegram_server/src/statistics/widgets/chart_header_widget.rs` | 36 | `tests/test_tcg_n12_chart_header_widget.rs` |
| `Telegram/SourceFiles/statistics/widgets/chart_lines_filter_widget.cpp` | `TCG-N37` | `ChartLinesFilterWidgetWidgets` | `crates/gpui_telegram_server/src/statistics/widgets/chart_lines_filter_widget.rs` | 246 | `tests/test_tcg_n37_chart_lines_filter_widget.rs` |
| `Telegram/SourceFiles/statistics/widgets/chart_lines_filter_widget.h` | `TCG-N37` | `ChartLinesFilterWidgetWidgets` | `crates/gpui_telegram_server/src/statistics/widgets/chart_lines_filter_widget.rs` | 44 | `tests/test_tcg_n37_chart_lines_filter_widget.rs` |
| `Telegram/SourceFiles/statistics/widgets/point_details_widget.cpp` | `TCG-N12` | `PointDetailsWidgetWidgets` | `crates/gpui_telegram_server/src/statistics/widgets/point_details_widget.rs` | 527 | `tests/test_tcg_n12_point_details_widget.rs` |
| `Telegram/SourceFiles/statistics/widgets/point_details_widget.h` | `TCG-N12` | `PointDetailsWidgetWidgets` | `crates/gpui_telegram_server/src/statistics/widgets/point_details_widget.rs` | 85 | `tests/test_tcg_n12_point_details_widget.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/statistics/chart_lines_filter_controller.h`
- **Classes & Structs**: LinesFilterController, Entry
- **Core Methods & Signals**: LinesFilterController, setEnabled, tick
- **Target**: `ChartLinesFilterController` in `crates/gpui_telegram_server/src/statistics/chart_lines_filter_controller.rs`

### `Telegram/SourceFiles/statistics/chart_rulers_data.h`
- **Classes & Structs**: ChartRulersData, Line
- **Core Methods & Signals**: computeRelative
- **Target**: `ChartRulersData` in `crates/gpui_telegram_server/src/statistics/chart_rulers_data.rs`

### `Telegram/SourceFiles/statistics/chart_widget.cpp`
- **Classes & Structs**: RpMouseWidget, State, ChartWidget, DragArea, PaintCallback, Type
- **Core Methods & Signals**: rect::right, RpMouseWidget::start, RpMouseWidget::mouseStateChanged, RpMouseWidget::mousePressEvent, RpMouseWidget::mouseMoveEvent, RpMouseWidget::mouseReleaseEvent
- **Target**: `ChartWidget` in `crates/gpui_telegram_server/src/statistics/chart_widget.rs`

### `Telegram/SourceFiles/statistics/chart_widget.h`
- **Classes & Structs**: RpMouseWidget, PointDetailsWidget, ChartLinesFilterWidget, AbstractChartView, Header, LinesFilterController, ChartWidget, BottomCaptionLineData, Footer, ChartAnimationController
- **Core Methods & Signals**: ChartWidget, setChartData, setTitle, setZoomedChartData, addRuler, resizeGetHeight
- **Target**: `ChartWidget` in `crates/gpui_telegram_server/src/statistics/chart_widget.rs`

### `Telegram/SourceFiles/statistics/segment_tree.h`
- **Classes & Structs**: SegmentTree, Node, PendingVal
- **Core Methods & Signals**: build, propagate, change
- **Target**: `SegmentTree` in `crates/gpui_telegram_server/src/statistics/segment_tree.rs`

### `Telegram/SourceFiles/statistics/statistics.style`
- **Core Methods & Signals**: margins, TextStyle, font, RoundButton, FlatLabel, SettingsButton
- **Style Tokens**: statisticsLayerOverviewMargins: margins, statisticsLayerMargins: margins, statisticsChartHeight: 200px, statisticsChartEntryPadding: margins, statisticsDetailsArrowShift: 3px, statisticsDetailsArrowStroke: 1
- **Target**: `StatisticsStyleSheet` in `crates/gpui_telegram_desktop/src/theme/statistics/statistics_style.rs`

### `Telegram/SourceFiles/statistics/statistics_common.h`
- **Classes & Structs**: Limits, ChartViewType
- **Target**: `StatisticsCommon` in `crates/gpui_telegram_server/src/statistics/statistics_common.rs`

### `Telegram/SourceFiles/statistics/statistics_data_deserialize.h`
- **Classes & Structs**: StatisticalChart, QByteArray
- **Target**: `StatisticsDataDeserialize` in `crates/gpui_telegram_server/src/statistics/statistics_data_deserialize.rs`

### `Telegram/SourceFiles/statistics/statistics_format_values.h`
- **Classes & Structs**: namespace Statistic
- **Target**: `StatisticsFormatValues` in `crates/gpui_telegram_server/src/statistics/statistics_format_values.rs`

### `Telegram/SourceFiles/statistics/statistics_graphics.h`
- **Classes & Structs**: StatisticalChart
- **Target**: `StatisticsGraphics` in `crates/gpui_telegram_server/src/statistics/statistics_graphics.rs`

