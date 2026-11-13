# Module 06 — History View Media Rich (Slice 6) Specification

## Executive Summary & Architecture

Module 06 Slice 6 covers the **Rich Media Views** within Telegram Desktop's history bubble system:
`Telegram/SourceFiles/history/view/media/` spanning 23 component pairs (**46 files**, totaling **17,543 lines** of C++ upstream code).

These components render complex, interactive and transaction-oriented message cards:
1. **Interactive Polls & Quizzes**: `Poll`, `PercentCounterItem`, `AdjustPercentCount`, `CountNicePercent` rounding algorithm, solution explanation cards, and win fireworks animations.
2. **Rich Web Page & Instant View Previews**: `WebPage`, article collage layout, Instant View (`IvClickHandler`), photo/video preview banners, site name, author, description, and fact-checking badge with collapsible states.
3. **Invoices & Payments**: `Invoice`, Star credits and fiat payments, receipt states, test invoice labels, payment provider buttons, and photo attachments.
4. **TON & Gram Transfers**: `GramTransfer`, `BumpCurve`, glare and spin animations, star burst particles, address group layout (4 characters x 6 groups), amount typography, and blockchain state tracker.
5. **Interactive Todo Lists**: `TodoList`, task completion checkboxes, radial check animations, userpic tags for "completed by", completion status indicators, and fireworks on 100% completion.
6. **Unique & Premium Star Gifts**: `UniqueGift`, `PremiumGift`, dynamic 3D backdrops, collectible rarity numbers, star particles (`ColoredMiniStars`), and gift transfer / open buttons.
7. **Locations & Live Streaming Locations**: `Location`, static map coordinates pin, live location countdown timer (`RemainingTimeText`), accuracy radius ring, and live status pulsation.
8. **Games, Dices & Slot Machines**: `Game`, `Dice`, `SlotMachine`, 1..64 random outcomes decoding (`ComputeComplexIndex`, `ComputePartValue` into 3 reels: BAR, Berries, Lemon, Seven), TON stake win/loss overlays.
9. **Custom & Large Emoji**: `CustomEmoji`, `LargeEmoji`, dimension scaling (1x, 0.7x, 0.52x, Isolated, Large), animated sticker player delegation.
10. **Suggestions & Service Plates**: `UserpicSuggestion`, `BirthdaySuggestion`, `EphemeralPlate`, `CommunityAdded`, `StoryMention`, `SuggestDecision`.

---

## 100% Reading Manifest (Mandate #19 Pillar 2)

All 46 upstream C++ files in Slice 6 are enumerated below with exact line counts, physical SHA256 hashes, symbols, constants, and target Rust paths:

| # | Upstream Source Path | Lines | SHA256 (64 hex) | Core Symbols & Extracted Constants | Target Rust Path | Disposition |
|---|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/history/view/media/history_view_poll.h` | 142 | `7c7933e61e5f12902527faa380fc6fd6386d0501fda2446f068537dbd2823595` | `Poll`, `Header`, `Options`, `AddOption`, `Footer`, `showVotes`, `canVote` | `crates/gpui_telegram_desktop/src/views/stage/media/poll.rs` | `gpui_ui_1to1` |
| 2 | `Telegram/SourceFiles/history/view/media/history_view_poll.cpp` | 4656 | `a72bf316a73e7b8a8867529a7381ae298084dbe39ee92490a9f3224a2c4855fa` | `CountNicePercent:L173-L200`, `AdjustPercentCount:L144-L171`, `HashPollShuffleValue:L202-L210`, `kShowRecentVotersCount = 3`, `kRotateSegments = 8`, `kRotateAmplitude = 3.`, `kScaleSegments = 2`, `kScaleAmplitude = 0.03`, `kRollDuration = 400ms` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_poll.rs` | `gpui_ui_1to1` |
| 3 | `Telegram/SourceFiles/history/view/media/history_view_web_page.h` | 229 | `c164f015d4e059d0311aa0b82dbd5c9dac52d6f84d1f80c3ece727fb8281584d` | `WebPage`, `WebPageType`, `Factcheck`, `OpenContext`, `draw`, `textState` | `crates/gpui_telegram_desktop/src/views/stage/media/web_page.rs` | `gpui_ui_1to1` |
| 4 | `Telegram/SourceFiles/history/view/media/history_view_web_page.cpp` | 1916 | `9ceee3a53cba5672b8c65bc4a4d20f888e93d6cc9cd30dbd36be7d156d4d4948` | `PageToPhrase:L162-L200`, `ArticleThumbWidth:L56-L61`, `PrepareCollageMedia:L72-L98`, `IvClickHandler:L100-L115`, `kMaxOriginalEntryLines = 8192`, `kFactcheckCollapsedLines = 3`, `kStickerSetLines = 3`, `kFactcheckAboutDuration = 5000ms`, `kSponsoredUserpicLines = 2`, `kLogEntryPreviewLines = 2` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_web_page.rs` | `gpui_ui_1to1` |
| 5 | `Telegram/SourceFiles/history/view/media/history_view_contact.h` | 99 | `09abb614dac277f21e03c95770784f66bfef2d6f09f3c81eb08680569363fc33` | `Contact`, `SendMessageClickHandler`, `AddContactClickHandler`, `countOptimalSize`, `draw` | `crates/gpui_telegram_desktop/src/views/stage/media/location.rs` | `gpui_ui_1to1` |
| 6 | `Telegram/SourceFiles/history/view/media/history_view_contact.cpp` | 657 | `4ac15028b0b8d0b49b17c4438ed8f35a9a7a686e7cef43f4db1bdcf9d4c34d74` | `ContactClickHandler:L39-L54`, `SendMessageClickHandler:L56-L76`, `AddContactClickHandler:L78-L100`, vCard v3 parsing, phone formatting, userpic circle cache | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_contact.rs` | `gpui_ui_1to1` |
| 7 | `Telegram/SourceFiles/history/view/media/history_view_location.h` | 135 | `b1801b915f9c82b8b720aba824dd5bff97af357931e910b077eb09006fec7e28` | `Location`, `LocationPoint`, `Live`, `countOptimalSize`, `draw`, `updateLiveStatus` | `crates/gpui_telegram_desktop/src/views/stage/media/location.rs` | `gpui_ui_1to1` |
| 8 | `Telegram/SourceFiles/history/view/media/history_view_location.cpp` | 846 | `371651eaa0cd723a78c20d5c9c1dc6abaa159ad2e90d4ea78a12788245e27c3b` | `RemainingTimeText:L48-L66`, `RemainingTimeProgress:L68-L79`, `kUntilOffPeriod = INT_MAX`, `kLiveElapsedPartOpacity = 0.2`, live pin pulse, accuracy ring | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_location.rs` | `gpui_ui_1to1` |
| 9 | `Telegram/SourceFiles/history/view/media/history_view_game.h` | 120 | `8416f799837b2545cddbafdf0e0b1e3fb69dd1110d7a1554424c62304a46ef30` | `Game`, `GameData`, `countOptimalSize`, `draw`, `textState`, `_openl` | `crates/gpui_telegram_desktop/src/views/stage/media/web_page.rs` | `gpui_ui_1to1` |
| 10 | `Telegram/SourceFiles/history/view/media/history_view_game.cpp` | 549 | `8804dc358c5c5099fa9954367e17995863ee757c5478baa24672be255470a1ff` | `countOptimalSize:L54-L100`, `CreateAttach`, score badge layout, Play Game button | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_game.rs` | `gpui_ui_1to1` |
| 11 | `Telegram/SourceFiles/history/view/media/history_view_invoice.h` | 106 | `c60fc53c64a64ee3ed96f06be8be31807174a9eb3a7480651ecaaef9adf8a49c` | `Invoice`, `fillFromData`, `countOptimalSize`, `draw`, `_statusVisible`, `_receiptMsgId` | `crates/gpui_telegram_desktop/src/views/stage/media/invoice.rs` | `gpui_ui_1to1` |
| 12 | `Telegram/SourceFiles/history/view/media/history_view_invoice.cpp` | 468 | `c8abd57c8dd2280bcc2c43beb25598f52547f9b9507874d87b16ffc7c313520a` | `IsOrdinaryUnpaidCreditsInvoice:L26-L31`, `ClampSelection:L33-L40`, `fillFromData:L56-L112`, `descMaxLines = 4096`, `st::msgMinWidth` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_invoice.rs` | `gpui_ui_1to1` |
| 13 | `Telegram/SourceFiles/history/view/media/history_view_gram_transfer.h` | 41 | `2e07cc402e9849c92071e621d517fa185075bcb58aed8d2de8ef584a5df9065e` | `GramReadLine`, `CreateGramTransferMedia`, `Turn`, `schedule` | `crates/gpui_telegram_desktop/src/views/stage/media/transfer.rs` | `inherited_rust_reused` |
| 14 | `Telegram/SourceFiles/history/view/media/history_view_gram_transfer.cpp` | 2328 | `faa1f1a7faaa902149f039bcdb70358fba17335f4b77e80768606a74c906f900` | `BumpCurve:L53-L60`, `kAddressGroupSize = 4`, `kAddressGroupsPerLine = 6`, `kGlareDuration = 1100`, `kGlareTimeout = 400`, `kSettleBump:L81-L88`, `kBumpAmplitude = 0.07`, `kReadRollDuration = 1000`, `kBurstStarsPerSide = 48` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_gram_transfer.rs` | `inherited_rust_reused` |
| 15 | `Telegram/SourceFiles/history/view/media/history_view_todo_list.h` | 152 | `493efca4573a71b0102018a95d6dbb342eea153c17cd2b9e25e3346a65900df2` | `TodoList`, `Task`, `canComplete`, `toggleCompletion`, `maybeStartFireworks` | `crates/gpui_telegram_desktop/src/views/stage/media/todo.rs` | `gpui_ui_1to1` |
| 16 | `Telegram/SourceFiles/history/view/media/history_view_todo_list.cpp` | 912 | `2a6f9450fab3076fbe4c575c74b05aad4bdd19510e5158ad1b63bb267adab46b` | `fillData:L76-L93`, `setCompletedBy:L95-L101`, `setupPreviousState:L119-L130`, `st::historyPollAnswerStyle`, completed userpic rendering | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_todo_list.rs` | `gpui_ui_1to1` |
| 17 | `Telegram/SourceFiles/history/view/media/history_view_unique_gift.h` | 161 | `a690c2b5b3f5045c43c7bff32747c363df2ff459a1c677dc2cb46db9c1506107` | `UniqueGift`, `ButtonPart`, `TextBubblePart`, `UniqueGiftMessagePart`, `countOptimalSize`, `draw` | `crates/gpui_telegram_desktop/src/views/stage/media/gift.rs` | `gpui_ui_1to1` |
| 18 | `Telegram/SourceFiles/history/view/media/history_view_unique_gift.cpp` | 1032 | `b1387bf06b3125926bd1c4a52416346e1da34f8308507a2d2bb34ddaae7b029c` | `ButtonPart:L52-L92`, `TextBubblePart:L94-L122`, `UniqueGiftMessagePart:L124-L158`, collectible rarity, slug, star patterns | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_unique_gift.rs` | `gpui_ui_1to1` |
| 19 | `Telegram/SourceFiles/history/view/media/history_view_premium_gift.h` | 78 | `2c77531b65254c8d9aa7864c3761cbe166106c387f644d9ba7a8668aae798a87` | `PremiumGift`, `GeneratePremiumGiftMedia`, `GenerateStargiftMedia` | `crates/gpui_telegram_desktop/src/views/stage/media/gift.rs` | `gpui_ui_1to1` |
| 20 | `Telegram/SourceFiles/history/view/media/history_view_premium_gift.cpp` | 613 | `1a3a3317b56ac2c47b04d2e168c36b78b1034618b2cdf12ab2cb79814bd794b4` | `GeneratePremiumGiftMedia:L45-L95`, months badge, Star credits conversion rate, gift ribbon | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_premium_gift.rs` | `gpui_ui_1to1` |
| 21 | `Telegram/SourceFiles/history/view/media/history_view_giveaway.h` | 35 | `e3ec68de6eb9726bb7355f1d9a1b21fe64057f12066c0bb0d0dff026b8ca49a8` | `GenerateGiveawayStart`, `GenerateGiveawayResults` | `crates/gpui_telegram_desktop/src/views/stage/media/gift.rs` | `gpui_ui_1to1` |
| 22 | `Telegram/SourceFiles/history/view/media/history_view_giveaway.cpp` | 314 | `2202076237562c63bbbf91ec86b067fdcc51535cb820d44231926cbb9b6f4604` | `kOutlineRatio = 0.85:L30`, `GenerateGiveawayStart:L32-L60`, channels list, prize months, random winner picking date | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_giveaway.rs` | `gpui_ui_1to1` |
| 23 | `Telegram/SourceFiles/history/view/media/history_view_similar_channels.h` | 104 | `bdf657486da8b8b28bbd685387be7544bbb6270649d1120b88e90a346bd4f820` | `SimilarChannels`, `Channel`, `ensureCacheReady`, `consumeHorizontalScroll` | `crates/gpui_telegram_desktop/src/views/stage/media/web_page.rs` | `gpui_ui_1to1` |
| 24 | `Telegram/SourceFiles/history/view/media/history_view_similar_channels.cpp` | 651 | `93353fdc724d8f31de9e247e49a9fe8607ef0b978aacf34261856b5f3f5b17d0` | `countOptimalSize:L79-L81`, subscriber count, subscriber pill badge, horizontal carousel scroll | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_similar_channels.rs` | `gpui_ui_1to1` |
| 25 | `Telegram/SourceFiles/history/view/media/history_view_call.h` | 59 | `f355b1da82943c2fdd3bb92c25eb8b68eb7255c86dc0479611a65a7dae7340c7` | `Call`, `CallState`, `countOptimalSize`, `draw`, `textState` | `crates/gpui_telegram_desktop/src/views/stage/media/invoice.rs` | `gpui_ui_1to1` |
| 26 | `Telegram/SourceFiles/history/view/media/history_view_call.cpp` | 138 | `2b0770e0ec93af97850ac5b25708b372b22a06358c2225e8d8283869029c9255` | `ComputeDuration:L33-L37`, `st::historyCallWidth`, `st::historyCallHeight`, missed arrow vs outgoing arrow, video/conference icon | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_call.rs` | `gpui_ui_1to1` |
| 27 | `Telegram/SourceFiles/history/view/media/history_view_dice.h` | 69 | `7eeec7fd1c4ac3780c9c07b94d45a7787100655c61d43402ded916276de55e4d` | `Dice`, `updateOutcomeMessage`, `countOptimalSize`, `link` | `crates/gpui_telegram_desktop/src/views/stage/media/poll.rs` | `gpui_ui_1to1` |
| 28 | `Telegram/SourceFiles/history/view/media/history_view_dice.cpp` | 179 | `0fcb980029302159b55af30c8410564148a14ca442bf23016c9533e82ffed435` | `Lookup:L23-L29`, `updateOutcomeMessage:L58-L106`, `nanoTon`, `stakeNanoTon`, TON stake winning banner | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_dice.rs` | `gpui_ui_1to1` |
| 29 | `Telegram/SourceFiles/history/view/media/history_view_slot_machine.h` | 79 | `7be47eb61df632590d9ad1bc21d2dd7c7864a32897f0ca8ff7a25af7d4beb236` | `SlotMachine`, `resolveStarts`, `draw` | `crates/gpui_telegram_desktop/src/views/stage/media/poll.rs` | `gpui_ui_1to1` |
| 30 | `Telegram/SourceFiles/history/view/media/history_view_slot_machine.cpp` | 213 | `ed4b6f3b8488e6e0034ce9092073ea6458b299faa28b2d6ef4d78bdf5b8dcd5e` | `ComputeComplexIndex:L53-L68`, `ComputePartValue:L49-L51`, `kWinValue = 64`, `kShifts = {3, 9, 15}`, BAR (0), Berries (1), Lemon (2), Seven (3) | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_slot_machine.rs` | `gpui_ui_1to1` |
| 31 | `Telegram/SourceFiles/history/view/media/history_view_custom_emoji.h` | 104 | `673f0d79084dec606b7dfe9ad7aa1ac1880a93e3dfc141752ab9bf16eba438ac` | `CustomEmoji`, `SizesInfo`, `EmojiSize`, `countOptimalSize`, `draw` | `crates/gpui_telegram_desktop/src/views/stage/media/poll.rs` | `gpui_ui_1to1` |
| 32 | `Telegram/SourceFiles/history/view/media/history_view_custom_emoji.cpp` | 332 | `218769f7a36333fcf2e4087a714aa8cc6e0857c9ef9dd51cd184bb09309c7c2b` | `SizesInfo:L38-L48` (1->1.0, 2->0.7, 3->0.52), `EmojiSize:L50-L56` (4..5->Isolated, 6..7->Large, else Normal) | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_custom_emoji.rs` | `gpui_ui_1to1` |
| 33 | `Telegram/SourceFiles/history/view/media/history_view_large_emoji.h` | 67 | `917e1da3807b89fa7df5201c86063d7e937498ca16374beb96787f0db388ef20` | `LargeEmoji`, `countOptimalSize`, `draw` | `crates/gpui_telegram_desktop/src/views/stage/media/poll.rs` | `gpui_ui_1to1` |
| 34 | `Telegram/SourceFiles/history/view/media/history_view_large_emoji.cpp` | 187 | `302008d953f62e13d6a43327c457943f4c4a6311ce189550286f978e9d90cbfa` | `ResolveImages:L30-L51`, `kIsolatedEmojiLimit = 3`, `st::largeEmojiSkip`, `st::largeEmojiOutline`, `st::largeEmojiPadding` | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_large_emoji.rs` | `gpui_ui_1to1` |
| 35 | `Telegram/SourceFiles/history/view/media/history_view_userpic_suggestion.h` | 58 | `14bf4ab6b0fae0c732eac61bf4201ae463fd8ca7d0e180ae84508700934979de` | `UserpicSuggestion`, `ShowUserpicSuggestion` | `crates/gpui_telegram_desktop/src/views/stage/media/location.rs` | `gpui_ui_1to1` |
| 36 | `Telegram/SourceFiles/history/view/media/history_view_userpic_suggestion.cpp` | 278 | `bebd745d245d0fa64621243bacf47d14754cfe417b0183a58c065beb84d83c85` | `kToastDuration = 5000ms:L39`, `ShowUserpicSuggestion:L41-L60`, user photo crop, set as profile picture | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_userpic_suggestion.rs` | `gpui_ui_1to1` |
| 37 | `Telegram/SourceFiles/history/view/media/history_view_birthday_suggestion.h` | 70 | `0e642c3f6445cd9cf2a8f56b06f69f0e1d46dc475c150844c6b9f5e599b695d3` | `GenerateSuggetsBirthdayMedia` | `crates/gpui_telegram_desktop/src/views/stage/media/gift.rs` | `gpui_ui_1to1` |
| 38 | `Telegram/SourceFiles/history/view/media/history_view_birthday_suggestion.cpp` | 201 | `2c3b96de51aafbfa318decc6a84263b8c904ccece3808fcb9460328e16562d2a` | `GenerateSuggetsBirthdayMedia:L43-L60`, `cake` sticker generator, Send Gift button | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_birthday_suggestion.rs` | `gpui_ui_1to1` |
| 39 | `Telegram/SourceFiles/history/view/media/history_view_ephemeral_plate.h` | 54 | `b4f248bd2cf80dd91bb31bef29328b148ed978544a9926dfd751f1605ac9d415` | `RefreshEphemeralPlate`, `EphemeralPlateMaxWidth` | `crates/gpui_telegram_desktop/src/views/stage/media/location.rs` | `gpui_ui_1to1` |
| 40 | `Telegram/SourceFiles/history/view/media/history_view_ephemeral_plate.cpp` | 180 | `5df957a54515b423882067aa76e357479b454bea8dda50328feb97519b48dcee` | `kMaxLines = 3:L22`, `kFitSlack = 6:L23`, `RefreshEphemeralPlate:L27-L58`, self-destruct timer plate | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_ephemeral_plate.rs` | `gpui_ui_1to1` |
| 41 | `Telegram/SourceFiles/history/view/media/history_view_community_added.h` | 25 | `10f9d1fac1b3863d0c861c1a4e85083652dcfb443b583f415f290213be797fd2` | `GenerateCommunityAddedMedia` | `crates/gpui_telegram_desktop/src/views/stage/media/web_page.rs` | `gpui_ui_1to1` |
| 42 | `Telegram/SourceFiles/history/view/media/history_view_community_added.cpp` | 247 | `7bc20bf9269a856cc7f5bbc2c3084273ad803ee567006426bc2050963355f35d` | `CommunityServiceEmptyIcon:L34-L36`, `CommunityServiceUserpic:L42-L60`, group add invitation banner | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_community_added.rs` | `gpui_ui_1to1` |
| 43 | `Telegram/SourceFiles/history/view/media/history_view_story_mention.h` | 69 | `9cd74dd52edf626fa91a62ec2f0be7983650f3dc2ea8f496053b9fad1e1d8bbb` | `StoryMention`, `countOptimalSize`, `draw` | `crates/gpui_telegram_desktop/src/views/stage/media/gift.rs` | `gpui_ui_1to1` |
| 44 | `Telegram/SourceFiles/history/view/media/history_view_story_mention.cpp` | 188 | `940e0a3de1f4bbbab5ef4898de0e7bfeb685152214121d29b18df9f2cfa04161` | `kReadOutlineAlpha = 0.5:L44`, story thumbnail round outline, unread gradient border | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_story_mention.rs` | `gpui_ui_1to1` |
| 45 | `Telegram/SourceFiles/history/view/media/history_view_suggest_decision.h` | 33 | `d96dcc89d8302eda23856b650c57ab5a47269b092f10d5aa900438358cfc03c4` | `GenerateSuggestDecisionMedia` | `crates/gpui_telegram_desktop/src/views/stage/media/invoice.rs` | `gpui_ui_1to1` |
| 46 | `Telegram/SourceFiles/history/view/media/history_view_suggest_decision.cpp` | 369 | `3ed5d227fab807894fe75d13a861d7bc4c0cd081c5f42d0549e071327148d575` | `kFadedOpacity = 0.85:L28`, `EmojiType:L30-L53` (Agreement, Calendar, Money, Hourglass, Reload, Decline, Discard, Warning), suggestion acceptance / decline actions | `crates/gpui_telegram_desktop/src/views/stage/view/media/history_view_suggest_decision.rs` | `gpui_ui_1to1` |

---

## Detailed Algorithm Implementations (Mandate #18 Anti-Bluffing)

### 1. Poll Voting Percentages Algorithm (`CountNicePercent` & `AdjustPercentCount`)
- Upstream C++ citations: `history_view_poll.cpp:L144-L200`.
- Problem: Straight integer division `(votes * 100) / total` truncates fractions, resulting in sums like 98% or 99%, while standard floating point rounding can sum to 101%.
- Telegram Solution:
  1. Compute base integer percentages: `item.percent = (votes * 100) / total`.
  2. Compute remainders: `item.remainder = (votes * 100) - (item.percent * total)`.
  3. Track leftover points: `left = 100 - sum(percents)`.
  4. Stable Sort by `(remainder DESC, percent ASC)`.
  5. Distribute +1% to the options with highest remainders until `left == 0`.
  6. Guard against zero-remainder options receiving spurious increments (edge case: 4 items with 3 votes for 3 different items).

### 2. Slot Machine Reel Decoding (`ComputeComplexIndex` & `ComputePartValue`)
- Upstream C++ citations: `history_view_slot_machine.cpp:L24-L68`.
- Random value space: 1..64.
- Jackpot rule: Value `64` represents "777 Jackpot" where all 3 reels show `kSevenWinIndex` (0).
- Reel value derivation for values 1..63:
  - `part_value = ((value - 1) >> (part_index * 2)) & 0x03` where `part_index in [0, 1, 2]`.
  - Mapping: `0 -> BAR`, `1 -> Berries`, `2 -> Lemon`, `3 -> Seven`.
  - Complex index: `kShifts[part_index] + in_part_index` where `kShifts = [3, 9, 15]`.

### 3. TON / Gram Transfer Animations & Formatting
- Upstream C++ citations: `history_view_gram_transfer.cpp:L53-L125`.
- Address formatting: 24 characters displayed in 6 groups of 4 characters (`kAddressGroupSize = 4`, `kAddressGroupsPerLine = 6`).
- Star burst particle physics: 48 stars per side (`kBurstStarsPerSide = 48`), delay 90ms, spread 250ms, lifespan 850ms–1100ms.
- Physics curves (`BumpCurve`): Rise, hold, fall, settle intervals with `fallEase = 1.2` and undershoot elasticity.

### 4. Live Location Remaining Time Calculation
- Upstream C++ citations: `history_view_location.cpp:L48-L79`.
- When period is `kUntilOffPeriod` (infinity), returns `∞`.
- Formats remaining duration into granular units:
  - `< 10s`: seconds
  - `< 10m`: minutes
  - `< 1h`: minute integer
  - `< 24h`: hours
  - `>= 24h`: days
- Progress fraction computed as `std::clamp(period - elapsed, 0, period) / float64(period)`.

---

## Target Rust Architecture

The GPUI rich media components are structured into two complementary layers:
1. **`crates/gpui_telegram_desktop/src/views/stage/media/`**
   - High-level, ergonomic, idiomatic GPUI element builders:
     - `poll.rs`: `RichPollCard`, `PollOptionModel`, `CountNicePercent`, quiz state & explanation card.
     - `web_page.rs`: `RichWebPageCard`, `WebPageType`, `InstantViewBanner`, `FactcheckBadge`.
     - `invoice.rs`: `RichInvoiceCard`, `PaymentReceiptCard`, currency formatter, Call history card (`RichCallCard`).
     - `transfer.rs`: `RichTonTransferCard`, address chunking, burst animation particles, wallet link.
     - `todo.rs`: `RichTodoListCard`, todo task model, completion toggles, progress bar.
     - `gift.rs`: `RichStarGiftCard`, `UniqueCollectibleCard`, `GiveawayCard`, rarity badge.
     - `location.rs`: `RichLocationCard`, live location countdown tracker, contact card (`RichContactCard`), ephemeral self-destruct plate (`RichEphemeralPlate`).
     - `mod.rs`: Clean exports of all rich media card elements.
2. **`crates/gpui_telegram_desktop/src/views/stage/view/media/`**
   - 23 historical component units matching the exact shard entries:
     - All 23 files overwritten to completely eradicate boilerplate patterns (A/B/C/D).
     - Full authentic state models, GPUI `Render` implementations, and delegation to domain logic.
