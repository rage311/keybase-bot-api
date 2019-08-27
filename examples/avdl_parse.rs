use avro::parse_idl;
use pest::Parser;
use rusty_keybase_bot::avdl::{self, AVDLParser};

// @namespace("keybase.1")
fn main() {
  let input = r#"@namespace("chat.1") protocol common {

  import idl "../gregor1" as gregor1;
  import idl "../keybase1" as keybase1;

  @typedef("bytes")  record ThreadID {}
  @typedef("uint") @lint("ignore") record MessageID {}
  @typedef("uint") @lint("ignore") record TLFConvOrdinal {}
  @typedef("bytes")  record TopicID {}
  @typedef("bytes")  record ConversationID {}
  @typedef("bytes")  record TLFID {}
  @typedef("bytes")  record Hash {}
  @typedef("uint64") @lint("ignore") record InboxVers {}
  @typedef("uint64") @lint("ignore") record LocalConversationVers {}
  @typedef("uint64") @lint("ignore") record ConversationVers {}
  @typedef("bytes")  record OutboxID {}
  @typedef("bytes")  record TopicNameState {}
  @typedef("bytes")  record FlipGameID {}

  record InboxVersInfo {
    gregor1.UID uid;
    InboxVers vers;
  }

  enum ConversationExistence {
    ACTIVE_0,
    ARCHIVED_1,
    DELETED_2,
    ABANDONED_3
  }

  enum ConversationMembersType {
    KBFS_0,
    TEAM_1,
    IMPTEAMNATIVE_2,
    IMPTEAMUPGRADE_3
  }

  enum SyncInboxResType {
    CURRENT_0,
    INCREMENTAL_1,
    CLEAR_2
  }

  @go("nostring")
  enum MessageType {
    NONE_0,
    TEXT_1,
    ATTACHMENT_2,
    EDIT_3,
    DELETE_4,
    METADATA_5,
    TLFNAME_6, // Only used as the very first message in conversations whose topic name is not set when created
    HEADLINE_7,
    ATTACHMENTUPLOADED_8,  // sent after upload completes to modify ATTACHMENT message
    JOIN_9, // sent when joining a channel
    LEAVE_10,
    SYSTEM_11, // messages that get autogenerated by people's clients
    DELETEHISTORY_12,
    REACTION_13,
    SENDPAYMENT_14,
    REQUESTPAYMENT_15,
    UNFURL_16,
    FLIP_17,
    PIN_18 // sent when pinning a message
  }

  @go("nostring")
  enum TopicType {
    NONE_0,
    CHAT_1,
    DEV_2,
    KBFSFILEEDIT_3
  }

  enum TeamType {
    NONE_0,
    SIMPLE_1,
    COMPLEX_2
  }

  @go("nostring")
  enum NotificationKind {
    GENERIC_0,
    ATMENTION_1
  }

  // Global notification settings
  enum GlobalAppNotificationSetting {
    NEWMESSAGES_0,
    PLAINTEXTMOBILE_1,
    PLAINTEXTDESKTOP_2,
    DEFAULTSOUNDMOBILE_3,
    DISABLETYPING_4
  }
  record GlobalAppNotificationSettings {
    map<GlobalAppNotificationSetting, bool> settings;
  }

  enum ConversationStatus {
    // Default status of the conversation
    UNFILED_0,

    // Not used yet.
    FAVORITE_1,

    // This status is useful for temporarily muting a conversation. Unless told
    // otherwise in GetInboxQuery, gregor doesn't include conversation with
    // this status in results from GetInboxRemote. Whenever some post activity
    // (post, edit, delete, etc.) happens for the conversation with this
    // status, it's automatically changed back to UNFILED.
    IGNORED_2,

    // The conversation is blocked (i.e. not included in GetInboxRemote results
    // by default), until SetConversationStatus is called with a different
    // status.
    BLOCKED_3,

    // The conversation appears in the inbox with no snippet,
    // and does not emit notifications nor badges.
    MUTED_4,

    // The conversation has been reported by the user, behaves essentially the same
    // as blocked.
    REPORTED_5

  }

  record ConversationMember {
    gregor1.UID uid;
    ConversationID convID;
    TopicType topicType;
  }

  record ConversationIDMessageIDPair {
    ConversationID convID;
    MessageID msgID;
  }
  record ConversationIDMessageIDPairs {
    array<ConversationIDMessageIDPair> pairs;
  }

  record ChannelNameMention {
    ConversationID convID;
    string topicName;
  }

  enum ConversationMemberStatus {
    ACTIVE_0,      // in the channel
    REMOVED_1,     // removed from channel forcibly
    LEFT_2,        // voluntarily left conversation
    PREVIEW_3,     // use is previewing the channel from an @mention
    RESET_4,       // status of having an account reset in an impteam
    NEVER_JOINED_5 // member of the team but never joined the conv
  }

  record Pagination {
    bytes next;
    bytes previous;
    int num; // Number of items requested when argument, and number returned when result
    boolean last; // Will be true if the number of results is less than requested
    boolean forceFirstPage; // Let downstream consumers of this Pagination know this is an initial request
  }

  record RateLimit {
    string name;
    int callsRemaining; // Number of calls remaining for the given RPC in the current window
    int windowReset; // Amount of time (in seconds) until the window resets for this rate limit bucket
    int maxCalls; // Max amount of calls allowed in a window for the given RPC
  }

  record GetInboxQuery {
    union { null, ConversationID } convID;
    union { null, TopicType } topicType;
    union { null, TLFID } tlfID;
    union { null, keybase1.TLFVisibility } tlfVisibility;
    union { null, gregor1.Time } before;
    union { null, gregor1.Time } after;
    union { null, boolean } oneChatTypePerTLF;
    union { null, string } topicName; // used only by local inbox cache

    // If left empty, default is to show unfiled and favorite
    array<ConversationStatus> status;
    // If left empty, default is to return active, preview, and reset status
    array<ConversationMemberStatus> memberStatus;
    // If left empty, default is active
    array<ConversationExistence> existences;
    // If left empty, default is any type
    array<ConversationMembersType> membersTypes;

    // Extended list of conversation IDs to fetch (don't need to set convID, if convID is set then
    // this it will be like appending it to this list)
    array<ConversationID> convIDs;

    boolean unreadOnly;
    boolean readOnly;
    boolean computeActiveList;
    boolean summarizeMaxMsgs; // if true, resulting conversation will only have summaries of max msgs
    boolean skipBgLoads; // optionally skip queuing the conversation in the background loader.
  }

  record ConversationIDTriple {
    @lint("ignore")
    TLFID tlfid;
    TopicType topicType;
    TopicID topicID;
  }

  record ConversationFinalizeInfo {
    string resetUser;
    string resetDate;
    string resetFull;
    gregor1.Time resetTimestamp;
  }

  record ConversationResolveInfo {
    string newTLFName;
  }

  record Expunge {
    // Delete upto this message ID (exclusive)
    MessageID upto;
    // The message that justifies this deletion.
    // Can point to a DeleteHistory message or be 0 in the case of a retention sweep.
    MessageID basis;
  }

  record ConversationMetadata  {
    ConversationIDTriple idTriple;
    ConversationID conversationID;
    keybase1.TLFVisibility visibility;
    ConversationStatus status;
    ConversationMembersType membersType;
    TeamType teamType;
    ConversationExistence existence;
    ConversationVers version;
    LocalConversationVers localVersion;

    // Finalize info for underlying TLF (only makes sense for KBFS convos)
    union { null, ConversationFinalizeInfo } finalizeInfo;

    array<ConversationMetadata> supersedes; // metadata about the conversations this supersedes from a TLF finalize (if any).
    array<ConversationMetadata> supersededBy; // metadata about the conversations that superseded this conversation from a TLF finalize.

    // List of users sorted by recency of last [intentional] post.
    // Most recent first. May be incomplete or empty.
    // *** Empty for TEAM chats. ***
    array<gregor1.UID> activeList;

    array<gregor1.UID> allList;   // all of the users in the conversation
    array<gregor1.UID> resetList; // all of the reset users in the conversation (only for TEAM and IMPTEAM chats)
  }

  record ConversationNotificationInfo {
    boolean channelWide;
    map<keybase1.DeviceType, map<NotificationKind, boolean>> settings;
  }

  record ConversationReaderInfo {
    gregor1.Time mtime; // The last time the convo was modified from the user perspective
    MessageID readMsgid; // The message ID the user has read up to in the convo
    MessageID maxMsgid; // The max message ID in the convo
    ConversationMemberStatus status; // The status of the membership to the convo
  }

  record ConversationCreatorInfo {
    gregor1.Time ctime;
    gregor1.UID uid;
  }

  record ConversationCreatorInfoLocal {
    gregor1.Time ctime;
    string username;
  }

  record ConversationMinWriterRoleInfo {
    gregor1.UID uid;
    keybase1.TeamRole role;
  }

  record ConversationSettings {
    @mpackkey("mwr") @jsonkey("mwr")
    union { null, ConversationMinWriterRoleInfo } minWriterRoleInfo;
  }

  record Conversation {
    ConversationMetadata metadata;
    union { null, ConversationReaderInfo } readerInfo; // information about the convo from a user perspective
    union { null, ConversationNotificationInfo } notifications; // user notification settings for the convo, will be null if it is just the default. Otherwise contains entries to modify default setting.

    // maxMsgs is the maximum message for each messageType in the conversation
    array<MessageBoxed> maxMsgs;

    // maxMsgSummaries contains a subset of the full MessageBoxed for the maximum message for
    // each messageType in the conversation
    array<MessageSummary> maxMsgSummaries;

    // creator info for the conversation
    union { null, ConversationCreatorInfo } creatorInfo;

    // Latest pinned message
    union { null, MessageID } pinnedMsg;

    // The latest history deletion. Defaults to zeroes.
    // The client keeps this synced for retention but not for delete-history messages.
    Expunge expunge;
    union { null, RetentionPolicy } convRetention;
    union { null, RetentionPolicy } teamRetention;
    @mpackkey("cs") @jsonkey("cs")
    union { null, ConversationSettings } convSettings;
  }

  record MessageSummary {
    MessageID msgID;
    MessageType messageType;
    string tlfName;
    boolean tlfPublic;
    gregor1.Time ctime;
  }

  record Reaction {
    gregor1.Time ctime;
    MessageID reactionMsgID;
  }

  record ReactionMap {
    // { reactionText (:+1:) -> { username -> Reaction } }
    map<string, map<string, Reaction>> reactions;
  }

  record MessageServerHeader {
    MessageID messageID;
    MessageID supersededBy;
    @mpackkey("r") @jsonkey("r")
    array<MessageID> reactionIDs;
    @mpackkey("u") @jsonkey("u")
    array<MessageID> unfurlIDs;
    array<MessageID> replies;
    gregor1.Time ctime;
    // server's view of now(), used to calculate ephemeral lifetimes
    @mpackkey("n") @jsonkey("n")
    gregor1.Time now;
    // clients mark the received time as soon as they pull the message from the
    // server
    @mpackkey("rt") @jsonkey("rt")
    union { null, gregor1.Time } rtime;
  }

  record MessagePreviousPointer {
    MessageID id;
    Hash hash;
  }

  record OutboxInfo {
    MessageID prev; // This is the message ID the sending client device saw as the previous
    gregor1.Time composeTime;
  }

  record MsgEphemeralMetadata {
    @mpackkey("l") @jsonkey("l")
    gregor1.DurationSec lifetime; // used to computed etime
    @mpackkey("g") @jsonkey("g")
    keybase1.EkGeneration generation;
    @mpackkey("u") @jsonkey("u")
    union { null, string } explodedBy;
  }

  record EphemeralPurgeInfo {
    @mpackkey("c") @jsonkey("c")
    ConversationID convID;
    @mpackkey("a") @jsonkey("a")
    boolean isActive;
    @mpackkey("n") @jsonkey("n")
    gregor1.Time nextPurgeTime;
    @mpackkey("e") @jsonkey("e")
    MessageID minUnexplodedID;
  }

  // The Boxer's compareHeaders* functions checks each of these fields.
  // If we add a field here, that method needs to be updated.
  record MessageClientHeader {
    // This type is attached to MessageBoxed.
    // When on a received message these fields are server-set and have not been verified.
    // If adding fields, consider whether they should be signed,
    // and if so add them to MessageClientHeaderVerified as well.

    ConversationIDTriple conv;
    string tlfName;
    boolean tlfPublic;
    MessageType messageType;
    MessageID supersedes;
    union { null, boolean } kbfsCryptKeysUsed;

    // These 3 fields are hints for the server.
    // They can be derived from the message body, and are not signed.
    array<MessageID> deletes;
    array<MessagePreviousPointer> prev;
    union {null, MessageDeleteHistory} deleteHistory;

    gregor1.UID sender;
    gregor1.DeviceID senderDevice;

    // Latest merkle root when sent.
    // Can be nil in MBv1 messages, ignored either way since not signed.
    // Non-nil in MBv2 messages.
    union { null, MerkleRoot } merkleRoot;

    union { null, OutboxID } outboxID;
    union { null, OutboxInfo } outboxInfo;

    @mpackkey("em") @jsonkey("em")
    union { null, MsgEphemeralMetadata } ephemeralMetadata;

    // [V1, V2]: Missing
    // [V3, V4]: Optional map of pairwise MACs, used for exploding messages on
    //           small teams. In V4, if MACs are present, the verifyKey is a dummy.
    @mpackkey("pm") @jsonkey("pm")
    map<keybase1.KID, bytes> pairwiseMacs;

    // [V1, V2]: Missing
    // [V3, V4]: Specifies which member is the bot recipient, if any, to select
    //           appropriate crypt keys.
    @mpackkey("b") @jsonkey("b")
    union { null, gregor1.UID } botUID;
  }

  record MessageClientHeaderVerified {
    // This type is the result of unboxing.
    // And to be used locally to the client only.
    // All fields have been verified signed by the sender.
    // If adding fields, consider updating Boxer's compareHeaders methods
    // to check invariants early.

    ConversationIDTriple conv;
    string tlfName;
    boolean tlfPublic;
    MessageType messageType;
    array<MessagePreviousPointer> prev;
    gregor1.UID sender;
    gregor1.DeviceID senderDevice;
    union { null, boolean } kbfsCryptKeysUsed;

    // Latest merkle root when sent.
    // Nil from v1 messages. Non-nil from v2 messages.
    union { null, MerkleRoot } merkleRoot;
    union { null, OutboxID } outboxID;
    union { null, OutboxInfo } outboxInfo;

    @mpackkey("em") @jsonkey("em")
    union { null, MsgEphemeralMetadata } ephemeralMetadata;

    // When putting a message in local storage we set the receivedTime
    // ephemeral lifetime calculations
    @mpackkey("rt") @jsonkey("rt")
    gregor1.Time rtime;

    @mpackkey("pm") @jsonkey("pm")
    boolean hasPairwiseMacs;

    @mpackkey("b") @jsonkey("b")
    union { null, gregor1.UID } botUID;
  }

  // The same format as in KBFS (see libkbfs/data_types.go)
  record EncryptedData {
    int   v;  // version = 1
    bytes e;  // encryptedData (output of secret box)
    bytes n;  // nonce
  }

  record SignEncryptedData {
    int   v; // version = 1
    bytes e; // signEncryptedData (output of signencrypt.SealWhole)
    bytes n; // nonce
  }

  // Encrypted or SignEncrypted. Must know which from context.
  record SealedData {
    int   v;  // version = 1
    // Encrypted: output of secret box
    // SignEncrypted: output of signencrypt.SealWhole
    bytes e;
    bytes n;  // nonce
  }

  record SignatureInfo {
    int   v; // version = 1
    bytes s; // signature; output of EdDSA
    bytes k; // Verifying key
  }

  record MerkleRoot {
    long seqno;
    bytes hash;
  }

  enum InboxResType {
    VERSIONHIT_0,
    FULL_1
  }

  record InboxViewFull {
    InboxVers vers;
    array<Conversation> conversations;
    union { null, Pagination } pagination;
  }

  variant InboxView switch (InboxResType rtype) {
    case VERSIONHIT: void;
    case FULL: InboxViewFull;
  }

  enum RetentionPolicyType {
    NONE_0,
    RETAIN_1, // Keep messages forever
    EXPIRE_2, // Delete after a while
    INHERIT_3, // Use the team's policy
    EPHEMERAL_4 // Force all messages to be exploding.
  }

  variant RetentionPolicy switch (RetentionPolicyType typ){
    case RETAIN: RpRetain;
    case EXPIRE: RpExpire;
    case INHERIT: RpInherit;
    case EPHEMERAL: RpEphemeral;
  }

  record RpRetain {}

  record RpExpire {
    // Delete messages older than this.
    gregor1.DurationSec age;
  }

  record RpInherit {}

  record RpEphemeral {
    // Messages must be exploding and have at most this lifetime.
    gregor1.DurationSec age;
  }

  enum GetThreadReason {
    GENERAL_0,
    PUSH_1,
    FOREGROUND_2,
    BACKGROUNDCONVLOAD_3,
    FIXRETRY_4,
    PREPARE_5,
    SEARCHER_6,
    INDEXED_SEARCH_7,
    KBFSFILEACTIVITY_8,
    COINFLIP_9,
    BOTCOMMANDS_10
  }

  enum ReIndexingMode {
    NONE_0,
    PRESEARCH_SYNC_1,
    POSTSEARCH_SYNC_2
  }

  record SearchOpts {
    boolean isRegex;

    // message filters
    string sentBy;
    string sentTo;
    // only set to true if sentTo is the current user.
    boolean matchMentions;
    gregor1.Time sentBefore;
    gregor1.Time sentAfter;

    // search parameters
    int maxHits;
    // only used by regexp search
    int maxMessages;
    int beforeContext;
    int afterContext;
    union { null, Pagination } initialPagination;
    // only used by in inbox search
    ReIndexingMode reindexMode;
    int maxConvsSearched;
    int maxConvsHit;
    union { null, ConversationID } convID;
    // only used by conversation name search
    int maxNameConvs;
  }

  // The search indexer uses the EmptyStruct to implement a set using a map
  // with a zero sized value.
  record EmptyStruct{}

  record ChatSearchMatch {
    int startIndex;
    int endIndex;
    string match;
  }

  record ChatSearchHit {
    array<UIMessage> beforeMessages;
    UIMessage hitMessage;
    array<UIMessage> afterMessages;
    array<ChatSearchMatch>  matches;
  }

  record ChatSearchInboxHit {
    ConversationID convID;
    TeamType teamType;
    string convName;
    string query;
    gregor1.Time time;
    array<ChatSearchHit> hits;
  }

  record ChatSearchInboxResults {
    array<ChatSearchInboxHit> hits;
    int percentIndexed;
  }

  record ChatSearchInboxDone {
    int numHits;
    int numConvs;
    int percentIndexed;
    boolean delegated;
  }

  record ChatSearchIndexStatus {
    int percentIndexed;
  }

  record AssetMetadataImage {
    int width;
    int height;
  }

  record AssetMetadataVideo {
    int width;
    int height;
    int durationMs;
  }

  record AssetMetadataAudio {
    int durationMs;
  }

  @go("nostring")
  enum AssetMetadataType {
    NONE_0,
    IMAGE_1,
    VIDEO_2,
    AUDIO_3
  }

  variant AssetMetadata switch (AssetMetadataType assetType) {
    case IMAGE: AssetMetadataImage;
    case VIDEO: AssetMetadataVideo;
    case AUDIO: AssetMetadataAudio;
  }

  @go("nostring")
  enum AssetTag {
    PRIMARY_0
  }

  record Asset {
    string filename;           // original filename of the object
    string region;             // storage region name
    string endpoint;           // storage endpoint
    string bucket;             // storage bucket
    string path;               // path to the object in bucket
    long size;                 // size of the object
    string mimeType;           // mime type of the object
    Hash encHash;              // hash of ciphertext object
    bytes key;                 // encryption key
    bytes verifyKey;           // signature verification key
    string title;              // title of the asset (defaults to filename if not provided)
    bytes nonce;               // encryption nonce
    AssetMetadata metadata;    // type-specific metadata
    AssetTag tag;              // for multiple previews, a tag to differentiate
  }

  // Bot Commands
  enum BotCommandsAdvertisementTyp {
    PUBLIC_0,
    TLFID_MEMBERS_1,
    TLFID_CONVS_2
  }

  record TeamMember {
    gregor1.UID uid;
    keybase1.TeamRole role;
    keybase1.TeamMemberStatus status;
  }
}
"#;
  let parsed = AVDLParser::parse(avdl::Rule::avdl_protocol, input);
  if let Err(e) = parsed {
    println!("{}", e);
  } else {
    println!("{:#?}", parsed.unwrap());
  }

  // let input2 = r#"union { foo }"#;
  // let parsed = AVDLParser::parse(avdl::Rule::ty, input2);
  // if let Err(e) = parsed {
  //   println!("{}", e);
  // } else {
  //   println!("{:#?}", parsed.unwrap());
  // }

  // let parsed2 = parse_idl(input);
  // println!("{:#?}", parsed2);
}
