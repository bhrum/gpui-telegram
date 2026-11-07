# Module 03 Specification: Data Media & Content Subsystem (`data_media_content`)

## 1. Module Overview & Architectural Objectives

The Data Media & Content subsystem (`crates/gpui_telegram_client/src/data/`) forms the rich multimedia, interactive content, and business layer of Telegram Desktop. In the upstream C++ implementation (`Telegram/SourceFiles/data/`), this subsystem is responsible for:

1. **Document & Media Pipeline**:
   - `DocumentData` and `DocumentMedia`: Encapsulates documents, audio tracks, voice notes, round video messages, animations (GIF/MP4), and custom stickers.
   - MIME type resolution and format detection (audio, video, PDF, archives, images).
   - Audio waveform representation (5-bit quantized amplitude bars), duration, and playback metadata.
   - Video qualities list, streaming preload offsets, and dimensions.
2. **Photo Management & Progressive Sizing**:
   - `PhotoData` and `PhotoMedia`: Multi-resolution image pipeline supporting progressive loading across `Small` (100px), `Thumbnail` (320px), and `Large` (1280px) levels.
   - Animated video avatars (looping MP4s with playback timestamps and inline thumbnails).
   - Local disk cache keys, cloud file references, and upload/download progress state machines.
3. **Forum & Threaded Topics**:
   - `Forum`, `ForumTopic`, and `ForumTopicIcons`: Hierarchical topic threads for supergroups.
   - `kGeneralId = 1` reserved General topic, custom icon document IDs, and Telegram color palettes (`colorId` 0..5).
   - Per-topic unread counters, mention tracking, reaction badges, and pinned topic ordering.
4. **Ephemeral Stories & Media Overlays**:
   - `Stories`, `StoryData`, and `StoryViews`: Ephemeral status media with 24-hour expiration timers (`expires <= now`).
   - Granular privacy tiers (`Public`, `CloseFriends`, `Contacts`, `SelectedContacts`).
   - Interactive media areas (`LocationArea`, `ReactionArea`, `ChannelPostArea`, `UrlArea`, `WeatherArea`).
   - Viewer analytics: view counts, reaction counts, forward tallies, and viewer user lists.
5. **Interactive Polls & Quizzes**:
   - `PollData` and `PollAnswer`: Public/anonymous voting, single/multi-choice, quiz modes with correct answer indices and explanations.
   - Dynamic vote tallies, percentage distribution, and automated close timers.
6. **Web Previews & Instant View**:
   - `WebPageData`: URL preview extraction, site name, rich author/title metadata, media embeds, and collages.
   - Instant View (IV) article reader structures and auctions/gift links.
7. **Star Gifts & Collectibles**:
   - `StarGift` and `UniqueGift`: Digital collectibles, Telegram Stars pricing, limited edition counts, rarity attributes (`Default`, `Uncommon`, `Rare`, `Epic`, `Legendary`), TON blockchain resale, and transfer timers.
8. **Saved Messages & Sublists**:
   - `SavedMessages`, `SavedSublist`, and `SavedMusic`: Monoforum sublists partitioned by original author peer, pinned sublists, and tag indexing.
9. **Stickers & Custom Emojis**:
   - `Stickers`, `StickersSet`, and `CustomEmojiManager`: Sticker packs, WebP/TGS (Lottie)/WebM formats, cloud recents, faved sets, custom emoji status packs.
10. **Business Features & Quick Replies**:
    - `Chatbots`: Connected business AI bots, permission flags (read, reply, mark read, delete), recipient filters.
    - `ShortcutMessages`: Business quick replies (`/shortcut` slash commands), shortcut message bundles, and opening hours.

---

## 2. Upstream C++ Class Hierarchy & Rust GPUI Target Mapping

The 58 assigned files in `crates/gpui_telegram_client/src/data/` map 1:1 to upstream `Telegram/SourceFiles/data/` as follows:

| Upstream C++ Source Files | Upstream C++ Classes / Types | Target Rust File | Target Rust Symbol | Responsibilities |
|---|---|---|---|---|
| `data/data_document.h`, `.cpp` | `DocumentData`, `StickerData`, `SongData`, `VoiceData`, `VideoData` | `data/data_document.rs` | `DataDocument` | Document metadata, dimensions, MIME type, audio/video attributes, streaming flags. |
| `data/data_document_media.h`, `.cpp` | `DocumentMedia`, `DocumentMedia::View` | `data/data_document_media.rs` | `DataDocumentMedia` | Document render cache, thumbnail image holder, video playback handle. |
| `data/data_document_resolver.h`, `.cpp` | `DocumentResolver`, `ResolveDocument` | `data/data_document_resolver.rs` | `DataDocumentResolver` | MIME-to-type resolution, file extension mapping, viewer association. |
| `data/data_photo.h`, `.cpp` | `PhotoData`, `PhotoSize`, `PhotoSizes` | `data/data_photo.rs` | `DataPhoto` | Progressive photo container (small, thumb, large), video photo loops, dimensions. |
| `data/data_photo_media.h`, `.cpp` | `PhotoMedia`, `PhotoMedia::View` | `data/data_photo_media.rs` | `DataPhotoMedia` | Cached image surfaces, texture loading, blurhash preview decoding. |
| `data/data_media_types.h`, `.cpp` | `Media`, `SharedMediaType`, `ItemPreview` | `data/data_media_types.rs` | `DataMediaTypes` | Shared media enum variants (13 types), message preview builders. |
| `data/data_media_preload.h`, `.cpp` | `MediaPreload`, `PreloadTask` | `data/data_media_preload.rs` | `DataMediaPreload` | Automatic media preloading manager based on network settings and file sizes. |
| `data/data_media_rotation.h`, `.cpp` | `MediaRotation`, `RotationAngle` | `data/data_media_rotation.rs` | `DataMediaRotation` | EXIF and user orientation adjustments (0, 90, 180, 270 degrees). |
| `data/data_download_manager.h`, `.cpp` | `DownloadManager`, `DownloadItem` | `data/data_download_manager.rs` | `DataDownloadManager` | User-initiated file downloads tracker, bandwidth limiter, pause/resume. |
| `data/data_auto_download.h`, `.cpp` | `AutoDownload`, `AutoDownloadSettings` | `data/data_auto_download.rs` | `DataAutoDownload` | Data-saving presets (Low, Medium, High), cellular/Wi-Fi/roaming auto-download rules. |
| `data/data_file_origin.h`, `.cpp` | `FileOrigin`, `FileOriginType` | `data/data_file_origin.rs` | `DataFileOrigin` | Security context for file requests (User, Chat, Channel, Story, WebPage). |
| `data/data_file_click_handler.h`, `.cpp` | `FileClickHandler`, `DocumentClickHandler` | `data/data_file_click_handler.rs` | `DataFileClickHandler` | Click action dispatcher (Open, Save As, Show in Folder, Stream). |
| `data/data_streaming.h`, `.cpp` | `Streaming`, `StreamPlayback` | `data/data_streaming.rs` | `DataStreaming` | Progressive streaming controller, buffer watermark management. |
| `data/data_audio_msg_id.h`, `.cpp` | `AudioMsgId`, `AudioMsgType` | `data/data_audio_msg_id.rs` | `DataAudioMsgId` | Global unique audio/voice message identifier for the media player. |
| `data/data_forum.h`, `.cpp` | `Forum`, `ForumOffsets` | `data/data_forum.rs` | `DataForum` | Group forum controller, topic cache, topic list ordering, unread aggregator. |
| `data/data_forum_topic.h`, `.cpp` | `ForumTopic`, `TopicIconDescriptor` | `data/data_forum_topic.rs` | `DataForumTopic` | Forum topic thread, root message ID, icon, color ID, unread counts, pinned state. |
| `data/data_forum_icons.h`, `.cpp` | `ForumIcons`, `ForumIconColor` | `data/data_forum_icons.rs` | `DataForumIcons` | Palette resolver for forum topic icons (0x6FB9F0, 0xFFD67E, etc.). |
| `data/data_thread.h`, `.cpp` | `Thread`, `ThreadId` | `data/data_thread.rs` | `DataThread` | Abstract conversation thread model shared by History, ForumTopic, and SavedSublist. |
| `data/data_saved_messages.h`, `.cpp` | `SavedMessages`, `SavedMessagesOffsets` | `data/data_saved_messages.rs` | `DataSavedMessages` | Saved Messages container, author-partitioned sublists, pinned topics. |
| `data/data_saved_sublist.h`, `.cpp` | `SavedSublist` | `data/data_saved_sublist.rs` | `DataSavedSublist` | Sub-conversation within Saved Messages filtered by original peer. |
| `data/data_saved_music.h`, `.cpp` | `SavedMusic`, `SavedMusicList` | `data/data_saved_music.rs` | `DataSavedMusic` | Dedicated audio playlist extracted from Saved Messages. |
| `data/data_replies_list.h`, `.cpp` | `RepliesList` | `data/data_replies_list.rs` | `DataRepliesList` | Message comment replies thread list and pagination tracker. |
| `data/data_reply_preview.h`, `.cpp` | `ReplyPreview`, `ReplyMarkup` | `data/data_reply_preview.rs` | `DataReplyPreview` | Replied message preview generator with thumbnail and sender summary. |
| `data/data_shared_media.h`, `.cpp` | `SharedMedia`, `SharedMediaCache` | `data/data_shared_media.rs` | `DataSharedMedia` | Peer shared media indexer across photos, videos, files, music, voice, links. |
| `data/data_stories.h`, `.cpp` | `Stories`, `StoriesSource` | `data/data_stories.rs` | `DataStories` | Peer stories collection, unread story ring tracking, archive lists. |
| `data/data_stories_ids.h`, `.cpp` | `StoriesIds`, `StoryId` | `data/data_stories_ids.rs` | `DataStoriesIds` | Story ID set tracking, read/unread status bitsets. |
| `data/data_story.h`, `.cpp` | `Story`, `StoryViews`, `StoryArea` | `data/data_story.rs` | `DataStory` | Individual story item, media attachment, expiration timer, privacy, view analytics. |
| `data/data_message_reaction_id.h`, `.cpp` | `ReactionId`, `ReactionType` | `data/data_message_reaction_id.rs` | `DataMessageReactionId` | Custom emoji vs standard emoji reaction identifier. |
| `data/data_message_reactions.rs` | `MessageReactions`, `ReactionCount` | `data/data_message_reactions.rs` | `DataMessageReactions` | Aggregated message reactions, recent reactors, my chosen reaction. |
| `data/data_star_gift.h`, `.cpp` | `StarGift`, `UniqueGift`, `GiftRarity` | `data/data_star_gift.rs` | `DataStarGift` | Telegram Star gifts, collectible gifts, upgrade variants, marketplace resale. |
| `data/data_poll.h`, `.cpp` | `PollData`, `PollAnswer`, `PollFlags` | `data/data_poll.rs` | `DataPoll` | Voting poll model, quiz answer validation, options distribution, close timers. |
| `data/data_poll_messages.h`, `.cpp` | `PollMessages` | `data/data_poll_messages.rs` | `DataPollMessages` | Message tracker referencing active polls for live result broadcasts. |
| `data/data_web_page.h`, `.cpp` | `WebPageData`, `WebPageType`, `WebPageCollage`| `data/data_web_page.rs` | `DataWebPage` | Rich link preview model, Instant View article cache, collage embeds. |
| `data/data_wall_paper.h`, `.cpp` | `WallPaper`, `WallPaperPattern` | `data/data_wall_paper.rs` | `DataWallPaper` | Chat background wallpaper definition (colors, gradients, patterns). |
| `data/data_drafts.h`, `.cpp` | `Draft`, `DraftKey` | `data/data_drafts.rs` | `DataDrafts` | Text draft persistence with reply/thread context and markdown entities. |
| `data/data_emoji_statuses.h`, `.cpp` | `EmojiStatus`, `EmojiStatusCollectibles`| `data/data_emoji_statuses.rs` | `DataEmojiStatuses` | Telegram Premium custom emoji status badge and expiration timestamp. |
| `data/data_game.h`, `.cpp` | `GameData`, `HighScore` | `data/data_game.rs` | `DataGame` | In-chat HTML5 game model, score table, bot provider. |
| `data/data_group_call.h`, `.cpp` | `GroupCall`, `CallParticipant` | `data/data_group_call.rs` | `DataGroupCall` | Group voice chat / video broadcast state, participants, mute status. |
| `data/data_location.h`, `.cpp` | `LocationPoint`, `LocationVenue` | `data/data_location.rs` | `DataLocation` | Geo coordinates, live location sharing duration, venue details. |
| `data/data_send_action.h`, `.cpp` | `SendAction`, `SendActionType` | `data/data_send_action.rs` | `DataSendAction` | Typing, recording voice, uploading photo status indicators. |
| `data/data_statistics.h` | `StatisticsData` | `data/data_statistics.rs` | `DataStatistics` | Channel and group growth analytics metrics. |
| `data/data_statistics_chart.h`, `.cpp` | `StatisticsChart`, `ChartPoint` | `data/data_statistics_chart.rs` | `DataStatisticsChart` | Chart coordinate curves, zooming, stacked area / line graph models. |
| `data/data_statistics_lists.h` | `StatisticsLists` | `data/data_statistics_lists.rs` | `DataStatisticsLists` | Top referring channels, recent message interactions table. |
| `data/data_todo_list.h`, `.cpp` | `TodoList`, `TodoItem` | `data/data_todo_list.rs` | `DataTodoList` | Collaborative group checklist / task list data model. |
| `data/data_ai_compose_tones.h`, `.cpp`| `AiComposeTones`, `AiTone` | `data/data_ai_compose_tones.rs` | `DataAiComposeTones` | AI message rewriting tone selector (Formal, Casual, Poetic, etc.). |
| `data/data_bot_app.h`, `.cpp` | `BotApp`, `BotAppShortName` | `data/data_bot_app.rs` | `DataBotApp` | Telegram Mini App WebApp definition, launch parameters, permissions. |
| `data/data_cloud_file.h`, `.cpp` | `CloudFile`, `CloudFileLocation` | `data/data_cloud_file.rs` | `DataCloudFile` | Abstract remote cloud file reference, DC ID, and access hash. |
| `data/data_compose_stash.h` | `ComposeStash` | `data/data_compose_stash.rs` | `DataComposeStash` | Temporary stashed message state when navigating across multiple chats. |
| `data/business/data_business_chatbots.h`, `.cpp` | `Chatbots`, `ChatbotsSettings` | `data/business/data_business_chatbots.rs` | `DataBusinessChatbotsBusiness` | Business bot integration, automatic response permissions, exclusion list. |
| `data/business/data_business_common.h`, `.cpp` | `BusinessRecipients`, `BusinessHours` | `data/business/data_business_common.rs` | `DataBusinessCommonBusiness` | Working hours schedule, away message rules, greeting messages. |
| `data/business/data_business_info.h`, `.cpp` | `BusinessInfo`, `BusinessLocation` | `data/business/data_business_info.rs` | `DataBusinessInfoBusiness` | Business profile details, physical address, business introduction. |
| `data/business/data_shortcut_messages.h`, `.cpp` | `ShortcutMessages`, `Shortcut` | `data/business/data_shortcut_messages.rs` | `DataShortcutMessagesBusiness` | Business quick reply shortcuts (`/order`, `/info`), message bundles. |
| `data/notify/data_notify_settings.h`, `.cpp` | `NotifySettings` | `data/notify/data_notify_settings.rs` | `DataNotifySettingsNotify` | Global notification presets (private chats, groups, channels). |
| `data/notify/data_peer_notify_settings.h`, `.cpp`| `PeerNotifySettings` | `data/notify/data_peer_notify_settings.rs` | `DataPeerNotifySettingsNotify` | Per-chat mute intervals, sound, preview toggles, exception flags. |
| `data/notify/data_peer_notify_volume.h`, `.cpp` | `PeerNotifyVolume` | `data/notify/data_peer_notify_volume.rs` | `DataPeerNotifyVolumeNotify` | Sound volume overrides and custom ringtone document IDs. |
| `data/stickers/data_custom_emoji.h`, `.cpp` | `CustomEmojiManager`, `CustomEmoji` | `data/stickers/data_custom_emoji.rs` | `DataCustomEmojiStickers` | Custom emoji document cache, colorable emoji rendering, pack IDs. |
| `data/stickers/data_stickers.h`, `.cpp` | `Stickers`, `StickerType` | `data/stickers/data_stickers.rs` | `DataStickersStickers` | Sticker catalog, installed sets, recent stickers, faved stickers. |
| `data/stickers/data_stickers_set.h`, `.cpp` | `StickersSet`, `StickersSetFlags` | `data/stickers/data_stickers_set.rs` | `DataStickersSetStickers` | Individual sticker pack model, short name, title, count, cover document. |

---

## 3. Core Data Models & State Machines

### 3.1 Document & Media Model (`DataDocument`)
```
  MTP Document ────────► DataDocument
                            │
                            ├── id: u64
                            ├── size: i64
                            ├── mime: String ("audio/ogg", "video/mp4", "image/webp")
                            ├── dimensions: (u32, u32)
                            ├── dc_id: i32, access_hash: u64, file_reference: Vec<u8>
                            ├── kind: DocumentKind
                            │     ├── File { filename: String }
                            │     ├── Sticker { alt: String, set_id: u64, format: StickerFormat }
                            │     ├── Song { title: String, performer: String, duration: u32 }
                            │     ├── Voice { waveform: Vec<u8>, duration: u32 }
                            │     └── Video { duration: u32, codec: String, qualities: Vec<VideoQuality> }
                            └── loading_state: FileLoadingState (Idle, Loading(0.0..1.0), Ready, Failed)
```

### 3.2 Progressive Photo Pipeline (`DataPhoto`)
```
  PhotoData (id, date, has_video)
     │
     ├── small:     PhotoSizeVariant (100x100 max, thumbnail preview, instant load)
     ├── thumbnail: PhotoSizeVariant (320x320 max, bubble preview)
     └── large:     PhotoSizeVariant (1280x1280 max, full screen viewer)
```

### 3.3 Forum Topic Model (`DataForumTopic`)
```
  ForumTopic
     │
     ├── root_id: i32 (1 = General Topic)
     ├── title: String
     ├── icon_id: u64 (Custom emoji DocumentId)
     ├── color_id: i32 (0..5 color palette index)
     ├── is_closed: bool, is_hidden: bool, is_pinned: bool
     ├── unread_count: u32, unread_mentions: u32, unread_reactions: u32
     └── notify_settings: PeerNotifySettings
```

### 3.4 Story Expiration & Analytics (`DataStory`)
```
  Story
     │
     ├── id: i32, peer_id: u64
     ├── date: i64, expires: i64 (typically date + 86400s)
     ├── privacy: StoryPrivacy (Public, CloseFriends, Contacts)
     ├── expired(now) := now >= expires
     ├── media: StoryMedia (Photo, Video, Call)
     └── views: StoryViews (views: u32, reactions: u32, forwards: u32, viewers: Vec<PeerId>)
```

### 3.5 Interactive Poll & Quiz Engine (`DataPoll`)
```
  PollData
     │
     ├── id: u64, question: String
     ├── answers: Vec<PollAnswer>
     │     ├── option: Vec<u8>
     │     ├── text: String
     │     ├── votes: u32
     │     ├── is_chosen: bool
     │     └── is_correct: bool (Quiz mode)
     ├── flags: PollFlags (Closed, PublicVotes, MultiChoice, Quiz)
     ├── vote(options) -> validates limits & tallies counts
     └── close() -> marks closed and finalizes percentages
```

---

## 4. Testing & Verification

1. **Subsystem Integration Tests**:
   - `crates/gpui_telegram_client/tests/data_media_content_subsystem_tests.rs`:
     * Document MIME type resolution, song duration, voice waveform decoding.
     * Progressive photo dimensions resolution and sizing limits.
     * Forum topic creation, icon color lookup, unread state calculations.
     * Story expiration timestamps and privacy checks.
     * Poll voting mechanics, multi-choice restrictions, quiz solution verification.
     * Star gift rarity badges, resale calculations, and business chatbot permission masks.
2. **Boilerplate Burndown Verification**:
   - Ensure 0 instances of Pattern A (`pub label: String, pub value: i64`), Pattern B, Pattern C, Pattern D in all 58 files.
   - Run `python3 scripts/verify_ledger_closure.py` to assert Gate 6.2 passes cleanly.
