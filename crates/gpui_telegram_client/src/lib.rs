pub mod cache;
pub mod client;
pub mod client_acp;
pub mod client_auth;
pub mod client_dialogs;
pub mod client_dispatch;
pub mod client_features;
pub mod client_messaging;
pub mod config;
pub mod net;
pub mod session_persistence;
pub mod sync;

pub use cache::{CachedMediaEntry, LocalStore};
pub use client::TelegramClient;
pub use client_dispatch::ClientDomainEvent;
pub use config::{ClientConfig, ConfigSource, StoredSession};
pub use net::{
    AddContactHttpRequest, AddContactHttpResponse, AgentRunRequest, AgentRunResponse,
    AlipayPollResponse, AlipayStartResponse, AlipayUserDto, AuthResponse, CallInviteRequest,
    CallRecordDto, CallSignalRequest, ChannelStatisticsDto, ChatDetailDto, ChatDetails,
    ChatFolderDto, ChatMemberItem, ChatSummary, ConnectionStatus, ContactDto, CreateChatPayload,
    CreateChatRequest, CreateChatResponse, CreatePollHttpRequest, DailyStatPoint, DraftDto,
    EditMessageHttpRequest, EmailRegisterRequest, EmailSendCodeRequest, EmailSendCodeResponse,
    ForumTopicDto, ForumTopicItem, GifResultDto, HistoricalMessage, HistoricalMessageItem,
    HttpClient, HttpClientError, InlineKeyboardButton, InviteLinkDto, LoginRequest,
    MediaUploadCompleteResponse, MediaUploadInitResponse, MemberDto, PhoneSendCodeRequest,
    PhoneSendCodeResponse, PhoneVerifyRequest, PinnedMessageDto, PollHttpResponse,
    ProtoAuthResponse, ProtoUpdateProfileRequest, QrPairAcceptRequest, QrPairInitResponse,
    QrPairPollResponse, ReactionHttpRequest, ReactionHttpResponse, ReadChatHttpRequest,
    ReadChatHttpResponse, RegisterRequest, RichSendMessageRequest, RichSendMessageResponse,
    SearchHitDto, SearchResponse, SearchResultItemDto, SendMessageHttpRequest,
    SendMessageHttpResponse, SendMessageResponse, ServerStatusResponse, SessionDto,
    StarGiftRequest, StatusResponse, StickerItemDto, StickerSetDto, StoryDto,
    SyncDifferenceResponse, TwoFactorSetRequest, TwoFactorStatusResponse, TwoFactorVerifyRequest,
    UpdateProfileRequest, UserDto, UserMeResponse, UserProfileDto, VotePollHttpRequest,
    WalletBalanceDto, WalletTransactionDto, WsChannel, WsTransport,
};
pub use session_persistence::{
    AccountSessionRecord, ActiveDeviceSession, PersistedClientState, ProxyEndpointRecord,
    SessionPersistenceBridge, WalletTransactionRecord,
};
pub use sync::{ClientSyncCoordinator, SyncOutcome};

pub mod data;
pub use data::*;
pub mod storage;
pub use storage::*;
pub mod export;
pub use export::*;

#[allow(special_module_name)]
pub mod r#main;
pub use r#main::*;
pub mod dialogs;
pub mod history;
pub use history::*;
pub mod wallet;
