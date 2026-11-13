# Module 03 Specification: Data Peers & Session Subsystem (`data_peers`)

## 1. Module Overview & Architectural Objectives

Module 03 (`storage & data`) contains the core domain data structures of Telegram Desktop. Specifically, the **Data Peers & Session subsystem** represents the heart of Telegram's object graph:
- **Peer Identity (`PeerId`)**: A unified 64-bit integer encoding users, group chats, broadcast channels, and synthetic/fake peers with bit-level discriminator shifts.
- **Peer Entities (`PeerData`, `UserData`, `ChatData`, `ChannelData`)**: Full polymorphic domain models tracking profile information, names, photos, access hashes, custom emoji statuses, stars ratings, birthday, wallet card, member counts, and detailed bitflags.
- **Admin Rights & Restrictions (`ChatAdminRights`, `ChatRestrictions`)**: Precise bitfield representations of administrative powers and channel/group participant restrictions matching Telegram MTProto schemas.
- **Histories & Messages Management (`Histories`, `History`, `HistoryMessages`, `MsgId`, `UnreadValue`)**: Session-wide index of message streams, read state tracking (unread counts, mentions, marks), and message life cycle.
- **Folders & Chat Filters (`Folder`, `ChatFilter`, `ChatFilterFlags`)**: Custom folder filters, rule-based inclusion/exclusion (contacts, non-contacts, groups, channels, bots, muted, read), pinned chats per filter, and folder hierarchy.
- **Data Session Management (`Data::Session`)**: Central repository and lifecycle manager for all peer entities, coordinating user updates, participant changes, group calls, credits, and session-level caches.

This Rust implementation in `crates/gpui_telegram_client/src/data/` replaces all dummy boilerplate structures (`{ id, label, value }`, dummy timestamp setters) with authentic, C++-faithful domain types and algorithms.

---

## 2. Upstream C++ Source Mapping & Target Symbols

The following table details the primary upstream C++ files in `Telegram/SourceFiles/data/` and their corresponding Rust targets in `gpui_telegram_client`:

| Upstream C++ Source | Key C++ Classes / Structs | Target Rust File | Target Rust Symbol | Subsystem / Responsibility |
|---|---|---|---|---|
| `data_peer_id.h`, `.cpp` | `PeerId`, `UserId`, `ChatId`, `ChannelId`, `FakeChatId`, `BareId` | `crates/gpui_telegram_client/src/data/data_peer_id.rs` | `DataPeerId` | 64-bit PeerId bitshift discriminator, legacy/modern serializer |
| `data_peer.h`, `.cpp` | `PeerData`, `RestrictionCheckResult`, `UnavailableReason`, `AllowedReactions` | `crates/gpui_telegram_client/src/data/data_peer.rs` | `DataPeer` | Base peer model, color indices, emoji status, stories state |
| `data_user.h`, `.cpp` | `UserData`, `UserDataFlag`, `BotInfo`, `StarRefProgram` | `crates/gpui_telegram_client/src/data/data_user.rs` | `DataUser` | User domain model, names, phone, bot info, flags, access hash |
| `data_user_names.h`, `.cpp` | `Usernames`, `UsernamesInfo` | `crates/gpui_telegram_client/src/data/data_user_names.rs` | `DataUserNames` | Multiple usernames, active/editable username resolver |
| `data_user_photos.h`, `.cpp` | `UserPhotos`, `PhotoData` | `crates/gpui_telegram_client/src/data/data_user_photos.rs` | `DataUserPhotos` | Profile photos history, photo list pagination |
| `data_user_wallet_card.h`, `.cpp` | `WalletCard`, `WalletCardType` | `crates/gpui_telegram_client/src/data/data_user_wallet_card.rs` | `DataUserWalletCard` | TON Wallet card and crypto address binding |
| `data_chat.h`, `.cpp` | `ChatData`, `ChatDataFlag`, `ChatParticipants` | `crates/gpui_telegram_client/src/data/data_chat.rs` | `DataChat` | Legacy small group chat model, participants, creator, invite link |
| `data_chat_participant_status.h`, `.cpp` | `ChatAdminRight`, `ChatRestriction`, `ChatAdminRightsInfo`, `ChatRestrictionsInfo` | `crates/gpui_telegram_client/src/data/data_chat_participant_status.rs` | `DataChatParticipantStatus` | MTProto admin bitmask and restricted permissions bitmask |
| `data_channel.h`, `.cpp` | `ChannelData`, `ChannelDataFlag`, `MegagroupInfo` | `crates/gpui_telegram_client/src/data/data_channel.rs` | `DataChannel` | Channel / Supergroup model, member counts, slowmode, bot commands |
| `data_channel_admins.h`, `.cpp` | `ChannelAdmins`, `Admin` | `crates/gpui_telegram_client/src/data/data_channel_admins.rs` | `DataChannelAdmins` | Channel admin list manager and permission lookup |
| `data_channel_earn.h`, `.cpp` | `ChannelEarn`, `EarnInt` | `crates/gpui_telegram_client/src/data/data_channel_earn.rs` | `DataChannelEarn` | Channel ad revenue sharing & star monetization |
| `data_histories.h`, `.cpp` | `Histories`, `History` | `crates/gpui_telegram_client/src/data/data_histories.rs` | `DataHistories` | Session-wide peer message history manager, read inbox |
| `data_history_messages.h`, `.cpp` | `HistoryMessages`, `HistoryItem` | `crates/gpui_telegram_client/src/data/data_history_messages.rs` | `DataHistoryMessages` | Ordered message collection for a peer, bidirectional lookup |
| `data_messages.h`, `.cpp` | `Messages`, `MessageIdsList` | `crates/gpui_telegram_client/src/data/data_messages.rs` | `DataMessages` | Message ID list operations, batch selection, delete ops |
| `data_msg_id.h`, `.cpp` | `MsgId`, `FullMsgId`, `ServerMsgId` | `crates/gpui_telegram_client/src/data/data_msg_id.rs` | `DataMsgId` | Message identifier types, client vs server IDs |
| `data_unread_value.rs` | `UnreadValue`, `Dialogs::UnreadState` | `crates/gpui_telegram_client/src/data/data_unread_value.rs` | `DataUnreadValue` | Unread badge counts, muted flag, mention counts |
| `data_folder.h`, `.cpp` | `Folder`, `FolderId` | `crates/gpui_telegram_client/src/data/data_folder.rs` | `DataFolder` | Archive and user folder containers for chats |
| `data_chat_filters.h`, `.cpp` | `ChatFilter`, `ChatFilterFlags`, `ChatFilterTitle` | `crates/gpui_telegram_client/src/data/data_chat_filters.rs` | `DataChatFilters` | Chat tab folder filters, rules evaluation, inclusion/exclusion |
| `data_session.h`, `.cpp` | `Data::Session`, `SessionSettings` | `crates/gpui_telegram_client/src/data/data_session.rs` | `DataSession` | Central domain coordinator storing peer entities, histories, folders |
| `data_lastseen_status.h`, `.cpp` | `LastseenStatus`, `LastseenType` | `crates/gpui_telegram_client/src/data/data_lastseen_status.rs` | `DataLastseenStatus` | Online / last seen time, privacy obfuscation |
| `data_birthday.h`, `.cpp` | `Birthday`, `BirthdayFormat` | `crates/gpui_telegram_client/src/data/data_birthday.rs` | `DataBirthday` | User birth date with day/month/year parsing and formatted strings |
| `data_boosts.h`, `.cpp` | `BoostsStatus`, `MyBoost` | `crates/gpui_telegram_client/src/data/data_boosts.rs` | `DataBoosts` | Channel and supergroup level boosts, unrestrict perks |
| `data_credits.h`, `.cpp` | `CreditsStatus`, `CreditsHistoryEntry` | `crates/gpui_telegram_client/src/data/data_credits.rs` | `DataCredits` | Telegram Stars balance, transaction ledger |
| `data_credits_earn.h`, `.cpp` | `CreditsEarnStatistics` | `crates/gpui_telegram_client/src/data/data_credits_earn.rs` | `DataCreditsEarn` | Bot & channel stars revenue statistics and withdrawal state |
| `data_passkey_deserialize.h`, `.cpp` | `PasskeyData`, `PasskeyCredential` | `crates/gpui_telegram_client/src/data/data_passkey_deserialize.rs` | `DataPasskeyDeserialize` | WebAuthn passkey registration payload deserializer |
| `data_report.h`, `.cpp` | `ReportReason`, `ReportInput` | `crates/gpui_telegram_client/src/data/data_report.rs` | `DataReport` | Spam / abuse reporting categories and commentary |
| `data_pts_waiter.h`, `.cpp` | `PtsWaiter`, `PtsState` | `crates/gpui_telegram_client/src/data/data_pts_waiter.rs` | `DataPtsWaiter` | Updates PTS gap detection and synchronization waiter |
| `data_cloud_themes.h`, `.cpp` | `CloudThemes`, `CloudTheme` | `crates/gpui_telegram_client/src/data/data_cloud_themes.rs` | `DataCloudThemes` | Cloud chat themes, daylight and night mode palettes |
| `data_search_calendar.h`, `.cpp` | `SearchCalendar`, `CalendarDay` | `crates/gpui_telegram_client/src/data/data_search_calendar.rs` | `DataSearchCalendar` | Date-based chat history message search navigator |
| `data_search_controller.h`, `.cpp` | `SearchController`, `SearchQuery` | `crates/gpui_telegram_client/src/data/data_search_controller.rs` | `DataSearchController` | Chat and global message search controller |
| `data_community.h`, `.cpp` | `CommunityData`, `CommunityTopic` | `crates/gpui_telegram_client/src/data/data_community.rs` | `DataCommunity` | Telegram Communities and multi-chat federations |
| `data_groups.h`, `.cpp` | `Groups`, `GroupInfo` | `crates/gpui_telegram_client/src/data/data_groups.rs` | `DataGroups` | Message album groupings and clustered media |
| `data_subscriptions.h`, `.cpp` | `Subscription`, `SubscriptionPlan` | `crates/gpui_telegram_client/src/data/data_subscriptions.rs` | `DataSubscriptions` | Channel paid subscription plans and active subscriber status |
| `data_premium_limits.h`, `.cpp` | `PremiumLimits`, `PremiumLimitType` | `crates/gpui_telegram_client/src/data/data_premium_limits.rs` | `DataPremiumLimits` | Telegram Premium quotas (channels, pinned, bio, upload size) |
| `data_premium_subscription_option.h`, `.cpp` | `PremiumSubscriptionOption` | `crates/gpui_telegram_client/src/data/data_premium_subscription_option.rs` | `DataPremiumSubscriptionOption` | Premium subscription pricing tiers (monthly, yearly, gift) |

---

## 3. PeerId Architecture & Discriminator Logic

In Telegram Desktop, a `PeerId` is a 64-bit integer packing both the entity type and the entity's 48-bit identifier:
```
 63       56 55                   48 47                                0
┌───────────┬───────────────────────┬───────────────────────────────────┐
│ Reserved  │   Type Shift (8-bit)  │       Bare ID (48-bit Mask)       │
└───────────┴───────────────────────┴───────────────────────────────────┘
```
- **Type Shifts**:
  - `UserId`: `Shift = 0` (User IDs occupy `0x0000_0000_0000` .. `0x0000_FFFF_FFFF_FFFF`)
  - `ChatId`: `Shift = 1` (`0x0001_0000_0000_0000` .. `0x0001_FFFF_FFFF_FFFF`)
  - `ChannelId`: `Shift = 2` (`0x0002_0000_0000_0000` .. `0x0002_FFFF_FFFF_FFFF`)
  - `FakeChatId`: `Shift = 0x7F` (`0x007F_0000_0000_0000` .. `0x007F_FFFF_FFFF_FFFF`)
- **Masks**:
  - `kChatTypeMask = 0x0000_FFFF_FFFF_FFFF` (48-bit bare identifier mask)
  - `kReservedBit = 0x80` (bit 63 used for serialization mode disambiguation)
- **Serialization**:
  - **Modern**: `id.value | (0x80 << 48)`
  - **Legacy**: Encoded using 32-bit masks where Chat has `0x1000_0000_0000`, Channel has `0x2000_0000_0000`, User has `0x0000_0000_0000`, and Fake has `0xF000_0000_0000`.

---

## 4. User, Chat, and Channel Domain Models

### 4.1 UserData
- `id: PeerId`, `access_hash: u64`
- `first_name: String`, `last_name: String`, `phone: String`, `usernames: Vec<String>`
- `flags: UserDataFlags`:
  - `Contact`, `MutualContact`, `Deleted`, `Verified`, `Scam`, `Fake`, `BotInlineGeo`, `Blocked`
  - `HasPhoneCalls`, `PhoneCallsPrivate`, `Support`, `SelfUser`, `Premium`, `VoiceMessagesForbidden`
- `lastseen: LastseenStatus`: `OnlineUntil(i32)`, `Recently`, `LastWeek`, `LastMonth`, `Hidden`
- `birthday: Option<Birthday>`
- `bot_info: Option<BotInfo>`: `commands: Vec<BotCommand>`, `description: String`, `reads_all_history: bool`

### 4.2 ChatData (Small Basic Group)
- `id: PeerId` (Shift = 1)
- `title: String`, `photo_id: u64`, `creator_id: UserId`, `date: i64`, `member_count: usize`
- `participants: Vec<UserId>`, `admins: Vec<UserId>`
- `flags: ChatDataFlags`: `Left`, `Creator`, `Deactivated`, `Forbidden`, `CallActive`, `NoForwards`
- `admin_rights: ChatAdminRights`, `default_restrictions: ChatRestrictions`
- `migrated_to: Option<ChannelId>` (when upgraded to supergroup)

### 4.3 ChannelData (Supergroup or Broadcast Channel)
- `id: PeerId` (Shift = 2)
- `title: String`, `username: Option<String>`, `access_hash: u64`
- `flags: ChannelDataFlags`:
  - `Left`, `Creator`, `Forbidden`, `Megagroup`, `Broadcast`, `Gigagroup`, `Verified`, `Scam`, `Fake`
  - `Signatures`, `PreHistoryHidden`, `SlowmodeEnabled`, `NoForwards`, `Forum`, `AntiSpam`
- `members_count: i32`, `admins_count: i32`, `slowmode_seconds: i32`
- `admin_rights: ChatAdminRights`, `default_restrictions: ChatRestrictions`
- `megagroup_info: Option<MegagroupInfo>`: tracks last authors, custom member ranks, bot commands

---

## 5. Admin Rights & Participant Restrictions Bitfields

### ChatAdminRight (22 bits):
```rust
ChangeInfo             = 1 << 0,
PostMessages           = 1 << 1,
EditMessages           = 1 << 2,
DeleteMessages         = 1 << 3,
BanUsers               = 1 << 4,
InviteByLinkOrAdd      = 1 << 5,
PinMessages            = 1 << 7,
AddAdmins              = 1 << 9,
Anonymous              = 1 << 10,
ManageCall             = 1 << 11,
Other                  = 1 << 12,
ManageTopics           = 1 << 13,
PostStories            = 1 << 14,
EditStories            = 1 << 15,
DeleteStories          = 1 << 16,
ManageDirect           = 1 << 17,
ManageRanks            = 1 << 18,
ProcessJoinRequests    = 1 << 19,
ManageLinkedPeers      = 1 << 20,
ManageWelcomeMessages  = 1 << 21,
```

### ChatRestriction (29 bits):
```rust
ViewMessages       = 1 << 0,
SendStickers       = 1 << 3,
SendGifs           = 1 << 4,
SendGames          = 1 << 5,
SendInline         = 1 << 6,
EmbedLinks         = 1 << 7,
SendPolls          = 1 << 8,
ChangeInfo         = 1 << 10,
AddParticipants    = 1 << 15,
PinMessages        = 1 << 17,
CreateTopics       = 1 << 18,
SendPhotos         = 1 << 19,
SendVideos         = 1 << 20,
SendVideoMessages  = 1 << 21,
SendMusic          = 1 << 22,
SendVoiceMessages  = 1 << 23,
SendFiles          = 1 << 24,
SendOther          = 1 << 25,
EditRank           = 1 << 26,
SendReactions      = 1 << 27,
ManageLinkedPeers  = 1 << 28,
```

---

## 6. Chat Filters & Folders Subsystem

### ChatFilter Evaluation Algorithm:
A chat with `History` matches `ChatFilter` if:
1. It is explicitly listed in `always` list $\implies$ **Match (True)**.
2. It is explicitly listed in `never` list $\implies$ **Exclude (False)**.
3. If `flags` contains `NoArchived` and the chat is archived $\implies$ **Exclude (False)**.
4. If `flags` contains `NoRead` and the chat has 0 unread $\implies$ **Exclude (False)**.
5. If `flags` contains `NoMuted` and the chat is muted $\implies$ **Exclude (False)**.
6. The chat entity type matches filter flags:
   - User contact $\land$ `Contacts` flag $\implies$ Match
   - User non-contact $\land$ `NonContacts` flag $\implies$ Match
   - Group chat $\land$ `Groups` flag $\implies$ Match
   - Channel $\land$ `Channels` flag $\implies$ Match
   - Bot user $\land$ `Bots` flag $\implies$ Match

---

## 7. Verification Strategy & Test Plan

1. **Unit Tests**:
   - `test_peer_id_encoding_and_shifts`: Validates `UserId`, `ChatId`, `ChannelId`, `FakeChatId`, bit extraction, modern and legacy serialization.
   - `test_user_data_flags_and_name_formatting`: Validates full name formatting, username handling, phone number, and bitflags.
   - `test_chat_and_channel_admin_rights`: Validates admin rights bitfields, privilege checking (`can_delete_messages`, `can_ban_users`, `can_post_messages`), and restrictions.
   - `test_chat_filter_matching`: Validates `always`, `never`, unread rules, muted rules, and entity type matching.
   - `test_histories_and_unread_accounting`: Validates message insertion, inbox read pointer updates, and unread counts.
   - `test_data_session_peer_repository`: Validates peer lookup, user and chat insertion, channel migration, and lifecycle caching.
2. **Integration Gate**:
   - Execute `cargo test -p gpui_telegram_client --test data_peers_subsystem_tests`.
   - Run `python3 scripts/clean_build_garbage.py`.
   - Ensure all boilerplate patterns in the assigned files are eradicated (zero instances of Pattern A/B/C/D).
