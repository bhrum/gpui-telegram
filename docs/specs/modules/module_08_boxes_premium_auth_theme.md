# Module 08 Slice 12: Boxes Premium, Auth & Theme Subsystem Specification

## 1. Executive Summary & Scope

This specification documents the architecture, data structures, state machines, and GPUI implementations for Slice 12 of Module 08 (`ui & boxes`).
Slice 12 covers the complete set of Telegram Premium, Stars & Gift Credits, URL Authorization, Sticker & Media Creators, and all 14 Box Theme Stylesheet definitions.

### Subsystem Responsibilities
1. **Telegram Premium & Limits**: `premium_limits_box`, `premium_preview_box`, `gift_premium_box` (doubled limits engine, feature showcase, gifting durations & giveaways).
2. **Telegram Stars & Credits**: `gift_credits_box`, `send_credits_box`, `star_gift_cover_box`, `star_gift_craft_animation`, `transfer_gift_box` (balance checks, peer gifting, 3D craft cube forge animation, TON/Wallet transfers).
3. **Stickers & Creation**: `sticker_creator_box`, `stickers_box` (512px canvas, WebM/WebP constraints, archived/featured/installed sticker set management).
4. **URL Auth & Translations**: `url_auth_box`, `url_auth_box_content`, `unconfirmed_auth_denied_box`, `translate_box`, `translate_box_content`, `username_box` (bot web auth, domain verification, message translation chunks, username validity).
5. **Box Theme Stylesheets**: 14 dedicated theme style definitions implementing pixel metrics, typography, and color tokens from `Telegram/SourceFiles/boxes/*.style`.

---

## 2. 100% Per-File Upstream Reading Manifest (46 Files)

Every assigned upstream file has been read in its entirety in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/boxes/`.

| # | Upstream Source Path | Lines | Key Extracted Logic & Constants | Target GPUI Component |
|---|----------------------|-------|---------------------------------|-----------------------|
| 1 | `Telegram/SourceFiles/boxes/gift_credits_box.h` | 21 | `ShowGiftCreditsBox` entry point, session & callback | `views::boxes::gift_credits_box::GiftCreditsBox` |
| 2 | `Telegram/SourceFiles/boxes/gift_credits_box.cpp` | 250 | `FrequentController` (L44-L76), `GiftCreditsBox` (L110-L183), `st::creditsGiftBox`, `boxWideWidth` (420px), mini stars effect | `views::boxes::gift_credits_box::GiftCreditsBox` |
| 3 | `Telegram/SourceFiles/boxes/gift_premium_box.h` | 115 | `GiftCodeBox`, `GiftDuration`, `AddStarGiftTable`, `AddTransferGiftTable`, `AddCreditsHistoryEntryTable` | `views::boxes::gift_premium_box::GiftPremiumBox` |
| 4 | `Telegram/SourceFiles/boxes/gift_premium_box.cpp` | 2346 | `kTooltipDuration` (3s), `kHorizontalBar` (0x2015), `kSpinnerRows` (6), `kSpinDuration` (120ms), `GiftDurationPhrase` (L300-L330) | `views::boxes::gift_premium_box::GiftPremiumBox` |
| 5 | `Telegram/SourceFiles/boxes/premium_limits_box.h` | 86 | `ChannelsLimitBox`, `PinsLimitBox`, `FileSizeLimitBox`, `CaptionLimitBox`, `AccountsLimitBox`, `FiltersLimitBox` | `views::boxes::premium_limits_box::PremiumLimitsBox` |
| 6 | `Telegram/SourceFiles/boxes/premium_limits_box.cpp` | 1199 | `InactiveController` (L63-L84), `ChannelsLimitBox` (L544), `FileSizeLimitBox` (L1009), `AccountsLimitBox` (L1067), 2x limit formulas | `views::boxes::premium_limits_box::PremiumLimitsBox` |
| 7 | `Telegram/SourceFiles/boxes/premium_preview_box.h` | 121 | `enum class PremiumFeature` (L52-L93, 27 premium + 8 business features), `DoubledLimitsPreviewBox`, `CreateUnlockButton` | `views::boxes::premium_preview_box::PremiumPreviewBox` |
| 8 | `Telegram/SourceFiles/boxes/premium_preview_box.cpp` | 1856 | `kToggleStickerTimeout` (2s), `kStarPeriod` (3s), `DoubledLimitsPreviewBox` (L1558-L1620) comparison vector | `views::boxes::premium_preview_box::PremiumPreviewBox` |
| 9 | `Telegram/SourceFiles/boxes/send_credits_box.h` | 67 | `SendCreditsBox`, `CreditsEmoji`, `SendStarsForm`, `SmallBalanceSourceFromForm` | `views::boxes::send_credits_box::SendCreditsBox` |
| 10 | `Telegram/SourceFiles/boxes/send_credits_box.cpp` | 604 | `LookupPaidMediaData` (L63-L97), `AddTerms` (L99-L130), paid photo/video extendedMedia, Stars balance validation | `views::boxes::send_credits_box::SendCreditsBox` |
| 11 | `Telegram/SourceFiles/boxes/star_gift_cover_box.h` | 147 | `UniqueGiftCoverWidget`, `UniqueGiftCoverArgs`, `CraftContext`, backdrop/pattern/model views | `views::boxes::star_gift_cover_box::StarGiftCoverBox` |
| 12 | `Telegram/SourceFiles/boxes/star_gift_cover_box.cpp` | 1774 | `kCrossfadeDuration` (400ms), `kSpinnerBackdrops` (6), `kBackdropStopsAt` (2.5s), `kPatternStopsAt` (4s), `kModelStopsAt` (5.5s) | `views::boxes::star_gift_cover_box::StarGiftCoverBox` |
| 13 | `Telegram/SourceFiles/boxes/star_gift_craft_animation.h` | 224 | `CraftState`, `CraftResult`, `CraftAnimationState`, `FacePlacement`, 3D cube forge state | `views::boxes::star_gift_craft_animation::StarGiftCraftAnimation` |
| 14 | `Telegram/SourceFiles/boxes/star_gift_craft_animation.cpp` | 1996 | `kFrameDuration` (16.66ms), `kFlightDuration` (400ms), `kLoadingFadeInDuration` (150ms), `kSuccessExpandDuration` (400ms), 3D cube rotation | `views::boxes::star_gift_craft_animation::StarGiftCraftAnimation` |
| 15 | `Telegram/SourceFiles/boxes/sticker_creator_box.h` | 48 | `OpenCreateStickerFlow`, `OpenCreateEmojiFlow`, `AdaptStickerToEmoji`, `AdaptGifToSet` | `views::boxes::sticker_creator_box::StickerCreatorBox` |
| 16 | `Telegram/SourceFiles/boxes/sticker_creator_box.cpp` | 914 | `kStickerSide` (512), `kPreviewSide` (256), `kWebpQuality` (95), `kMaxEmojis` (7), `kMaxStickerWebmBytes` (256KB), `kMaxEmojiWebmBytes` (64KB) | `views::boxes::sticker_creator_box::StickerCreatorBox` |
| 17 | `Telegram/SourceFiles/boxes/stickers_box.h` | 165 | `StickersBox::Section` (Installed, Featured, Archived, Attached, Masks), Megagroup sets | `views::boxes::stickers_box::StickersBox` |
| 18 | `Telegram/SourceFiles/boxes/stickers_box.cpp` | 2901 | `kArchivedLimitFirstRequest` (10), `kArchivedLimitPerPage` (30), `kHandleMegagroupSetAddressChangeTimeout` (1s) | `views::boxes::stickers_box::StickersBox` |
| 19 | `Telegram/SourceFiles/boxes/transfer_gift_box.h` | 98 | `ShowTransferToBox`, `ShowTransferGiftBox`, `ShowBuyResaleGiftBox`, `SetThemeFromUniqueGift` | `views::boxes::transfer_gift_box::TransferGiftBox` |
| 20 | `Telegram/SourceFiles/boxes/transfer_gift_box.cpp` | 2091 | `kPlateGroupsPerLine` (4), `kMovedToWalletToastDuration` (5s), `kTmpPasswordPeriod` (600s), `kPasswordStateTimeout` (10s) | `views::boxes::transfer_gift_box::TransferGiftBox` |
| 21 | `Telegram/SourceFiles/boxes/translate_box.h` | 47 | `TranslateBox` entry point, original vs translated text, copy action | `views::boxes::translate_box::TranslateBox` |
| 22 | `Telegram/SourceFiles/boxes/translate_box.cpp` | 607 | `kSkipAtLeastOneDuration` (3s), `kMaxLines` (3), `kFirstChunk` (100), rich translation links & format | `views::boxes::translate_box::TranslateBox` |
| 23 | `Telegram/SourceFiles/boxes/translate_box_content.h` | 67 | `TranslateBoxContent` container widget, block layout, language toggle | `views::boxes::translate_box_content::TranslateBoxContent` |
| 24 | `Telegram/SourceFiles/boxes/translate_box_content.cpp` | 243 | `kMaxLines` (3), scroll area embedding, provider dispatch | `views::boxes::translate_box_content::TranslateBoxContent` |
| 25 | `Telegram/SourceFiles/boxes/unconfirmed_auth_denied_box.h` | 22 | `ShowAuthDeniedBox` declaration, unconfirmed session security alert | `views::boxes::unconfirmed_auth_denied_box::UnconfirmedAuthDeniedBox` |
| 26 | `Telegram/SourceFiles/boxes/unconfirmed_auth_denied_box.cpp` | 167 | `kTimer` (5s countdown), security denial reason, IP / location alert formatting | `views::boxes::unconfirmed_auth_denied_box::UnconfirmedAuthDeniedBox` |
| 27 | `Telegram/SourceFiles/boxes/url_auth_box.h` | 37 | `UrlAuthBox` confirmation dialog, domain verification, write permission consent | `views::boxes::url_auth_box::UrlAuthBox` |
| 28 | `Telegram/SourceFiles/boxes/url_auth_box.cpp` | 946 | `kHintedRequestTimeout` (3s), `RequestButton`, `RequestUrl`, `ActivateButton`, `ActivateUrl` | `views::boxes::url_auth_box::UrlAuthBox` |
| 29 | `Telegram/SourceFiles/boxes/url_auth_box_content.h` | 106 | `UrlAuthBoxContent`, bot userpic, authorization scopes, match code display | `views::boxes::url_auth_box_content::UrlAuthBoxContent` |
| 30 | `Telegram/SourceFiles/boxes/url_auth_box_content.cpp` | 659 | `kEmojiAnimationActiveFor` (250ms), `ShowMatchCodesBox`, OAuth token exchange callback | `views::boxes::url_auth_box_content::UrlAuthBoxContent` |
| 31 | `Telegram/SourceFiles/boxes/username_box.h` | 38 | `UsernameBox` editor dialog, username availability verification | `views::boxes::username_box::UsernameBox` |
| 32 | `Telegram/SourceFiles/boxes/username_box.cpp` | 449 | Length check 5..32, regex `[a-zA-Z0-9_]`, Fragment auction link, editable state | `views::boxes::username_box::UsernameBox` |
| 33 | `Telegram/SourceFiles/boxes/add_contact_box.style` | 28 | `contactUserIcon`, `contactPhoneIcon`, `contactIconPosition: point(17px, 12px)`, `contactPadding` | `theme::boxes::add_contact_box_style::AddContactBoxStyle` |
| 34 | `Telegram/SourceFiles/boxes/background_preview_box.style` | 56 | `localStorageLimitMargin`, `backgroundCheckbox`, `backgroundDimLabel`, `backgroundBlurredLabel` | `theme::boxes::background_preview_box_style::BackgroundPreviewBoxStyle` |
| 35 | `Telegram/SourceFiles/boxes/boxes.style` | 773 | `boxWideWidth: 420px`, `boxTitleFont`, `boxRowPadding`, `boxButtonPadding`, `boxScroll`, `boxDivider` | `theme::boxes::boxes_style::BoxesStyle` |
| 36 | `Telegram/SourceFiles/boxes/compose_ai_box.style` | 190 | `aiComposeBoxStyleTabsSkip: 8px`, `aiComposeTabsBg: boxBg`, `aiComposeTabsHeight: 58px`, `aiComposeTabsRadius: 29px` | `theme::boxes::compose_ai_box_style::ComposeAiBoxStyle` |
| 37 | `Telegram/SourceFiles/boxes/connection_box.style` | 89 | `connectionHostInputField: width 160px`, `connectionPortInputField: width 55px`, `connectionProxyRowHeight: 40px` | `theme::boxes::connection_box_style::ConnectionBoxStyle` |
| 38 | `Telegram/SourceFiles/boxes/keys_legend.style` | 52 | `keysLegendMenuSkip: 6px`, `keysLegendMenu: PopupMenu`, `keysLegendItemPadding`, `keysLegendKeyFont` | `theme::boxes::keys_legend_style::KeysLegendStyle` |
| 39 | `Telegram/SourceFiles/boxes/moderate_messages_box.style` | 46 | `moderateBoxExpand: IconEmoji`, `moderateBoxBanCheckbox`, `moderateBoxReportSpam`, `moderateBoxDeleteAll` | `theme::boxes::moderate_messages_box_style::ModerateMessagesBoxStyle` |
| 40 | `Telegram/SourceFiles/boxes/passcode_box.style` | 39 | `passcodePadding: margins(24px, 16px, 24px, 16px)`, `passcodeTextLine: 28px`, `passcodeLittleSkip: 5px` | `theme::boxes::passcode_box_style::PasscodeBoxStyle` |
| 41 | `Telegram/SourceFiles/boxes/peer_list_section_index.style` | 17 | `peerListIndexWidth: 22px`, `peerListIndexApproach: 32px`, `peerListIndexSlotHeight: 15px` | `theme::boxes::peer_list_section_index_style::PeerListSectionIndexStyle` |
| 42 | `Telegram/SourceFiles/boxes/peers/edit_peer_members.style` | 127 | `editPeerMembersPadding`, `localStorageLimitSlider`, `rightsCheckbox`, `bannedRightsList`, `adminRightsTitle` | `theme::boxes::edit_peer_members_style::EditPeerMembersStyle` |
| 43 | `Telegram/SourceFiles/boxes/polls.style` | 308 | `historyPollQuestionFont`, `historyPollQuestionStyle`, `historyPollAnswerStyle`, `pollOptionPadding` | `theme::boxes::polls_style::PollsStyle` |
| 44 | `Telegram/SourceFiles/boxes/share_box.style` | 35 | `shareRowsTop: 12px`, `shareRowHeight: 108px`, `sharePhotoTop: 6px`, `shareBoxList`, `shareCommentInput` | `theme::boxes::share_box_style::ShareBoxStyle` |
| 45 | `Telegram/SourceFiles/boxes/stickers_box.style` | 24 | `contactsNameStyle`, `groupStickersRemove`, `groupStickersRemovePosition: point`, `stickersBoxRowHeight` | `theme::boxes::stickers_box_style::StickersBoxStyle` |
| 46 | `Telegram/SourceFiles/boxes/url_auth_box_content.style` | 54 | `urlAuthBox: Box`, `urlAuthButtonPadding`, `urlAuthDetailsBox`, `urlAuthDomainLabel`, `urlAuthBotLabel` | `theme::boxes::url_auth_box_content_style::UrlAuthBoxContentStyle` |

---

## 3. Upstream Constants, Limits Comparison & State Machines

### 3.1 Anonymous-Namespace `k...` Constants Citation

Under Mandate #18, the following constants extracted directly from upstream `.cpp` anonymous namespaces govern component logic:

| Upstream File Citation | Constant Name | Type & Upstream Value | Functional Semantics |
|------------------------|---------------|-----------------------|----------------------|
| `gift_premium_box.cpp:L90` | `kTooltipDuration` | `crl::time(3000)` (3s) | Duration of tooltip for copyable gift links & codes |
| `gift_premium_box.cpp:L91` | `kHorizontalBar` | `QChar(0x2015)` (Horizontal Bar) | Fixed separator replaced by `-` in transaction IDs |
| `gift_premium_box.cpp:L92` | `kSpinnerRows` | `int = 6` | Row count for gift upgrade probability spinner matrix |
| `gift_premium_box.cpp:L93` | `kSpinDuration` | `crl::time(120)` (120ms) | Interval between random slot rolls in gift upgrade reel |
| `star_gift_cover_box.cpp:L50` | `kCrossfadeDuration` | `crl::time(400)` (400ms) | Crossfade duration when switching backdrop covers |
| `star_gift_cover_box.cpp:L56` | `kBackdropStopsAt` | `crl::time(2500)` (2.5s) | Spinner backdrop wheel deceleration stop point |
| `star_gift_cover_box.cpp:L58` | `kPatternStopsAt` | `crl::time(4000)` (4.0s) | Spinner pattern wheel deceleration stop point |
| `star_gift_cover_box.cpp:L60` | `kModelStopsAt` | `crl::time(5500)` (5.5s) | Spinner 3D model wheel deceleration stop point |
| `star_gift_craft_animation.cpp:L35` | `kFrameDuration` | `1000. / 60.` (16.66ms) | 60 FPS animation ticker interval |
| `star_gift_craft_animation.cpp:L36` | `kFlightDuration` | `crl::time(400)` (400ms) | Flight duration of gift from corner into cube forge face |
| `star_gift_craft_animation.cpp:L37` | `kLoadingFadeInDuration` | `crl::time(150)` (150ms) | Fade-in transition duration for forge progress spinner |
| `star_gift_craft_animation.cpp:L42` | `kSuccessExpandDuration` | `crl::time(400)` (400ms) | Success modal expansion duration after successful craft |
| `sticker_creator_box.cpp:L51` | `kStickerSide` | `int = 512` | Telegram standard sticker canvas dimension (512x512) |
| `sticker_creator_box.cpp:L52` | `kPreviewSide` | `int = 256` | Quick preview sticker thumbnail dimension (256x256) |
| `sticker_creator_box.cpp:L53` | `kWebpQuality` | `int = 95` | WebP encoder quality level for static stickers |
| `sticker_creator_box.cpp:L54` | `kMaxEmojis` | `int = 7` | Maximum emoji associations per sticker |
| `sticker_creator_box.cpp:L55` | `kVideoStickerMaxDuration` | `crl::time(3000)` (3s) | Video sticker max duration limit |
| `sticker_creator_box.cpp:L58` | `kMaxStickerWebmBytes` | `256 * 1024` (256 KB) | Maximum file size for WebM animated sticker |
| `sticker_creator_box.cpp:L59` | `kMaxEmojiWebmBytes` | `64 * 1024` (64 KB) | Maximum file size for custom emoji WebM stream |
| `transfer_gift_box.cpp:L61` | `kPlateGroupsPerLine` | `int = 4` | Number of TON wallet address chunks per line |
| `transfer_gift_box.cpp:L62` | `kMovedToWalletToastDuration` | `crl::time(5000)` (5s) | Notification duration when gift moves to blockchain |
| `transfer_gift_box.cpp:L63` | `kTmpPasswordPeriod` | `10 * 60` (600s) | Temporary 2FA export password valid period |
| `transfer_gift_box.cpp:L64` | `kPasswordStateTimeout` | `crl::time(10000)` (10s) | 2FA password verification roundtrip timeout |
| `translate_box.cpp:L40` | `kSkipAtLeastOneDuration` | `crl::time(3000)` (3s) | Translation cache bypass debounce window |
| `translate_box.cpp:L41` | `kMaxLines` | `int = 3` | Maximum lines rendered before inline expand toggle |
| `unconfirmed_auth_denied_box.cpp:L30` | `kTimer` | `int = 5` (5s) | Countdown delay before auto-dismissing security alert |
| `url_auth_box.cpp:L34` | `kHintedRequestTimeout` | `crl::time(3000)` (3s) | Timeout for OAuth fast-redirect response |

---
### 3.2 Premium Doubled Limits Comparison Table

Derived from `premium_limits_box.cpp:L544-L1080` and `premium_preview_box.cpp:L1558-L1620`:

| Limit Resource Dimension | Free Tier Default | Premium Tier Doubled | Enforcement Point |
|--------------------------|-------------------|----------------------|-------------------|
| Channels / Supergroups Joined | 500 | 1,000 | `ChannelsLimitBox` (L544) |
| Dialogs Pinned to Top | 5 | 10 | `PinsLimitBox` (L898) |
| Public Username Links Created | 10 | 20 | `PublicLinksLimitBox` (L632) |
| Chat Folders Created | 10 | 20 | `FiltersLimitBox` (L785) |
| Chats Contained per Folder | 100 | 200 | `FilterChatsLimitBox` (L695) |
| Pinned Chats in Folder | 100 | 200 | `FilterPinsLimitBox` (L871) |
| Shareable Chat Folder Links | 3 | 20 | `ShareableFiltersLimitBox` (L826) |
| Saved Favorite GIFs | 200 | 400 | `DoubledLimitsPreviewBox` (L1580) |
| Saved Favorite Stickers | 500 | 1,000 | `DoubledLimitsPreviewBox` (L1590) |
| Connected Accounts in App | 3 | 4 | `AccountsLimitBox` (L1067) |
| File Upload Maximum Size | 2,147,483,648 B (2 GB) | 4,294,967,296 B (4 GB) | `FileSizeLimitBox` (L1009) |
| Bio Text Character Count | 70 chars | 140 chars | `DoubledLimitsPreviewBox` (L1605) |
| Media Caption Character Count | 1,024 chars | 2,048 chars | `CaptionLimitBox` (L944) |

### 3.3 Telegram Stars & Credit Conversion Rates

From `gift_credits_box.cpp:L166-L180`, `send_credits_box.cpp:L63-L97`, and `data_credits.h`:
1. **Conversion Metric**: 1 Telegram Star = 1000 millistars (internal precision).
2. **Preset Gifting Tiers**: 50, 100, 250, 500, 1000, 2500, 5000 Stars.
3. **Paid Media Checkout**: `LookupPaidMediaData` evaluates `InvoiceMessage`, counts attached photos & videos, and verifies that `user_stars_balance >= invoice.stars_amount`.
4. **Unique Gift Resale / Transfer**: TON network fee conversion with 2FA password authorization valid within `kTmpPasswordPeriod` (600s).

### 3.4 OAuth URL Authorization & Domain Matching

From `url_auth_box.cpp:L30-L240` and `url_auth_box_content.cpp:L50-L200`:
1. **Domain Extraction**: Host extracted from requested URL with punycode normalization.
2. **Bot Identity Verification**: Confirms verified badge, username match, and token scopes.
3. **Permission Granularity**:
   - `AllowWrite`: Permission for bot to send direct messages to user (`tr::lng_url_auth_allow_messages`).
   - `SharePhoneNumber`: Optional phone scope if bot requests profile contact verification.
4. **Fast-Redirect Mode**: If user previously approved bot domain with `remember_choice`, `kHintedRequestTimeout` (3s) executes auto-redirect.
5. **Session Guard**: `unconfirmed_auth_denied_box` blocks auth if current session is flagged as newly logged in without 2FA confirmation (timer: `kTimer = 5s`).

---

## 4. GPUI Component Implementation Architecture (16 Views)

Each target view component in `crates/gpui_telegram_desktop/src/views/boxes/` is implemented as an authentic GPUI view struct adhering to upstream class layouts and behavior:

1. `gift_credits_box.rs` (`GiftCreditsBox`):
   - Implements `GiftCreditsBoxView` with frequent peer list selection, userpic header, star particle effects, preset selection chips (50..5000), and custom credits counter.
   - Upstream ref: `gift_credits_box.cpp:L110-L183`.
2. `gift_premium_box.rs` (`GiftPremiumBox`):
   - Implements `GiftPremiumBoxView` with gift duration tier cards (3 months, 6 months, 12 months with discount tags), gift codes resolution table, and giveaway claim verification.
   - Upstream ref: `gift_premium_box.cpp:L300-L360`.
3. `premium_limits_box.rs` (`PremiumLimitsBox`):
   - Implements `PremiumLimitsBoxView` presenting current limit usage vs doubled Premium limit, infographic progress bar with gradient fill, and inactive chats cleanup trigger.
   - Upstream ref: `premium_limits_box.cpp:L544-L1080`.
4. `premium_preview_box.rs` (`PremiumPreviewBox`):
   - Implements `PremiumPreviewBoxView` supporting all 27 `PremiumFeature` and 8 Business feature enum variants, animated star badge header, and gradient CTA unlock button.
   - Upstream ref: `premium_preview_box.cpp:L1558-L1620`.
5. `send_credits_box.rs` (`SendCreditsBox`):
   - Implements `SendCreditsBoxView` for paying stars on bot invoices, paid media unlock (photo/video counters), and channel subscription checkout with terms consent.
   - Upstream ref: `send_credits_box.cpp:L63-L130`.
6. `star_gift_cover_box.rs` (`StarGiftCoverBox`):
   - Implements `StarGiftCoverBoxView` with customizable backdrops, pattern custom emoji overlays, rarity attributes table, and spinner reel rotation state.
   - Upstream ref: `star_gift_cover_box.cpp:L50-L120`.
7. `star_gift_craft_animation.rs` (`StarGiftCraftAnimation`):
   - Implements `StarGiftCraftAnimationView` managing 3D isometric cube forge rotation, flight trajectories for up to 4 corner gifts, success permille calculation, and burst particles.
   - Upstream ref: `star_gift_craft_animation.cpp:L35-L210`.
8. `sticker_creator_box.rs` (`StickerCreatorBox`):
   - Implements `StickerCreatorBoxView` with 512x512 canvas bounds, WebM/WebP validation, emoji tag association (up to 7 emojis), and upload progress.
   - Upstream ref: `sticker_creator_box.cpp:L50-L120`.
9. `stickers_box.rs` (`StickersBox`):
   - Implements `StickersBoxView` with tabbed navigation (`Installed`, `Featured`, `Archived`, `Attached`, `Masks`), sticker grid preview, reorder drag handles, and megagroup sticker binding.
   - Upstream ref: `stickers_box.cpp:L60-L180`.
10. `transfer_gift_box.rs` (`TransferGiftBox`):
    - Implements `TransferGiftBoxView` for transferring unique star gifts to other users or exporting to TON wallet addresses, featuring 2FA password verification timeout.
    - Upstream ref: `transfer_gift_box.cpp:L60-L150`.
11. `translate_box.rs` (`TranslateBox`):
    - Implements `TranslateBoxView` displaying side-by-side or stacked source and translated text, language picker dropdown, copy-to-clipboard button, and TTS playback trigger.
    - Upstream ref: `translate_box.cpp:L40-L120`.
12. `translate_box_content.rs` (`TranslateBoxContent`):
    - Implements `TranslateBoxContentView` managing scrollable text containers, expandable preview when exceeding `kMaxLines` (3 lines), and formatting preservation.
    - Upstream ref: `translate_box_content.cpp:L30-L90`.
13. `unconfirmed_auth_denied_box.rs` (`UnconfirmedAuthDeniedBox`):
    - Implements `UnconfirmedAuthDeniedBoxView` security alert showing unconfirmed login attempt warning, originating IP / device location, and 5-second countdown timer.
    - Upstream ref: `unconfirmed_auth_denied_box.cpp:L30-L80`.
14. `url_auth_box.rs` (`UrlAuthBox`):
    - Implements `UrlAuthBoxView` presenting bot OAuth consent dialog, verified bot domain badges, permission checkboxes, and fast-redirect bypass.
    - Upstream ref: `url_auth_box.cpp:L34-L120`.
15. `url_auth_box_content.rs` (`UrlAuthBoxContent`):
    - Implements `UrlAuthBoxContentView` with bot avatar, formatted URL domain display, scope selection list, and security token match confirmation code.
    - Upstream ref: `url_auth_box_content.cpp:L50-L140`.
16. `username_box.rs` (`UsernameBox`):
    - Implements `UsernameBoxView` for editing and validating public @username handle (5..32 chars, regex matching), link preview (`t.me/<username>`), and Fragment purchase option.
    - Upstream ref: `username_box.cpp:L40-L120`.

---

## 5. Theme & Style Architecture (14 Style Sheets)

The 14 theme stylesheets in `crates/gpui_telegram_desktop/src/theme/boxes/` translate upstream `.style` metric and color tokens into strongly typed Rust structs:
- `add_contact_box_style.rs`: `AddContactBoxStyle` (`contactIconPosition: point(17, 12)`, `contactPadding: margins(56, 16, 16, 16)`).
- `background_preview_box_style.rs`: `BackgroundPreviewBoxStyle` (`localStorageLimitMargin`, `backgroundCheckbox`).
- `boxes_style.rs`: `BoxesStyle` (`boxWideWidth: 420px`, `boxTitleFont`, `boxRowPadding`, `boxButtonPadding`).
- `compose_ai_box_style.rs`: `ComposeAiBoxStyle` (`aiComposeTabsHeight: 58px`, `aiComposeTabsRadius: 29px`, `aiComposeTabsSkip: 8px`).
- `connection_box_style.rs`: `ConnectionBoxStyle` (`connectionHostInputField: 160px`, `connectionPortInputField: 55px`, `connectionProxyRowHeight: 40px`).
- `keys_legend_style.rs`: `KeysLegendStyle` (`keysLegendMenuSkip: 6px`, `keysLegendItemPadding`, `keysLegendKeyFont`).
- `moderate_messages_box_style.rs`: `ModerateMessagesBoxStyle` (`moderateBoxExpand: IconEmoji`, `moderateBoxBanCheckbox`).
- `passcode_box_style.rs`: `PasscodeBoxStyle` (`passcodePadding: margins(24, 16, 24, 16)`, `passcodeTextLine: 28px`, `passcodeLittleSkip: 5px`).
- `peer_list_section_index_style.rs`: `PeerListSectionIndexStyle` (`peerListIndexWidth: 22px`, `peerListIndexApproach: 32px`, `peerListIndexSlotHeight: 15px`).
- `edit_peer_members_style.rs`: `EditPeerMembersStyle` (`editPeerMembersPadding`, `localStorageLimitSlider`, `rightsCheckbox`).
- `polls_style.rs`: `PollsStyle` (`historyPollQuestionFont`, `historyPollQuestionStyle`, `historyPollAnswerStyle`).
- `share_box_style.rs`: `ShareBoxStyle` (`shareRowsTop: 12px`, `shareRowHeight: 108px`, `sharePhotoTop: 6px`).
- `stickers_box_style.rs`: `StickersBoxStyle` (`groupStickersRemove`, `groupStickersRemovePosition: point`, `stickersBoxRowHeight`).
- `url_auth_box_content_style.rs`: `UrlAuthBoxContentStyle` (`urlAuthBox: Box`, `urlAuthButtonPadding`, `urlAuthDetailsBox`).
