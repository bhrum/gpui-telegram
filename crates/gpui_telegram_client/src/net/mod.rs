pub mod api_acp;
pub mod api_auth;
pub mod api_chats;
pub mod api_features;
pub mod hermes_client;
pub mod http_client;
pub mod http_helpers;
pub mod ws_channel;
pub mod ws_transport;

pub use gpui_telegram_proto::contracts::{
    AuthResponse as ProtoAuthResponse, CallInviteRequest, CallRecordDto, CallSignalRequest,
    ChannelStatisticsDto, ChatDetailDto, ChatFolderDto, ContactDto, CreateChatRequest,
    CreatePollHttpRequest, DailyStatPoint, DraftDto, EditMessageHttpRequest, ForumTopicDto,
    GifResultDto, InlineKeyboardButton, InviteLinkDto, MediaUploadCompleteResponse,
    MediaUploadInitResponse, MemberDto, PhoneSendCodeRequest, PhoneSendCodeResponse,
    PhoneVerifyRequest, PinnedMessageDto, PollHttpResponse, QrPairAcceptRequest,
    QrPairInitResponse, QrPairPollResponse, ReactionHttpRequest, ReactionHttpResponse,
    ReadChatHttpRequest, ReadChatHttpResponse, SearchHitDto, SearchResponse,
    SendMessageHttpRequest as RichSendMessageRequest,
    SendMessageHttpResponse as RichSendMessageResponse, SessionDto, StarGiftRequest,
    StatusResponse, StickerItemDto, StickerSetDto, StoryDto, TwoFactorSetRequest,
    TwoFactorStatusResponse, TwoFactorVerifyRequest,
    UpdateProfileRequest as ProtoUpdateProfileRequest, UserProfileDto, VotePollHttpRequest,
    WalletBalanceDto, WalletTransactionDto,
};
pub use http_client::{
    AddContactHttpRequest, AddContactHttpResponse, AgentRunRequest, AgentRunResponse,
    AlipayPollResponse, AlipayStartResponse, AlipayUserDto, AuthResponse, ChatDetails,
    ChatMemberItem, ChatSummary, CreateChatPayload, CreateChatResponse, EmailRegisterRequest,
    EmailSendCodeRequest, EmailSendCodeResponse, ForumTopicItem, HistoricalMessage,
    HistoricalMessageItem, HttpClient, HttpClientError, LoginRequest, RegisterRequest,
    SearchResultItemDto, SendMessageHttpRequest, SendMessageHttpResponse, SendMessageResponse,
    ServerStatusResponse, SyncDifferenceResponse, UpdateProfileRequest, UserDto, UserMeResponse,
};
pub use hermes_client::{
    HermesApprovalDecisionRequest, HermesApprovalDto, HermesCallInviteRequest,
    HermesCallInviteResponse, HermesClient, HermesCommandDescriptor, HermesCommandRequest,
    HermesCommandResponse, HermesCreateKanbanBoardRequest, HermesCreateKanbanCardRequest,
    HermesCronJobDto, HermesHeartbeatConfigDto, HermesHonchoRequest, HermesHonchoStateDto,
    HermesInlineButtonDto, HermesKanbanBoardDto, HermesKanbanCardDto, HermesMemoryEntryDto,
    HermesMemoryMutationRequest, HermesMemoryStateDto, HermesMoveKanbanCardRequest,
    HermesProviderConfigDto, HermesRichChatDto, HermesRichMessageDto, HermesRollbackRequest,
    HermesRollbackResponse, HermesSaveCronRequest, HermesSaveProviderRequest,
    HermesSaveSkillRequest, HermesSessionBranchRequest, HermesSessionDetailDto,
    HermesSessionDto, HermesSessionExportResponse, HermesSessionSearchRequest, HermesSkillDto,
    HermesSoulDto, HermesStatusDto, HermesTalkTurnRequest, HermesTalkTurnResponse,
    HermesTerminalProcessDto, HermesToolsetDto, HermesTurnRequest, HermesTurnResponse,
    HermesUpdateToolsetsRequest, HermesUsageSummaryDto, OpenAiChatCompletionsRequest,
    OpenAiChatMessage,
};
pub use ws_channel::WsChannel;
pub use ws_transport::{ConnectionStatus, WsTransport};
