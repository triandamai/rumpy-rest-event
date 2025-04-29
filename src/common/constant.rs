pub const KIND_THREAD_ATTACHMENT: &str = "PRODUCT-KIND_THREAD_ATTACHMENT";

pub const KIND_PUBLIC: &str = "PUBLIC";
pub const KIND_DISCUSSION: &str = "DISCUSSION";

pub const KIND_UP_VOTE_THREAD: &str = "UP-VOTE-THREAD";
pub const KIND_DOWN_VOTE_THREAD: &str = "DOWN-VOTE-THREAD";

pub const PROVIDER_BASIC: &str = "BASIC";
pub const PROVIDER_OTP: &str = "OTP";
pub const PROVIDER_GOOGLE: &str = "GOOGLE";

pub const BUCKET_EVENT: &str = "event";
pub const BUCKET_THREAD: &str = "thread";
pub const BUCKET_PROFILE_PICTURE: &str = "thread";

pub const PATH_PROFILE_PICTURE: &str = "profile";

pub const REDIS_KEY_USER_TOKEN: &str = "token";
pub const REDIS_KEY_OTP: &str = "otp";
pub const REDIS_KEY_OTP_AT: &str = "otp_at";
pub const REDIS_KEY_OTP_EXPIRED: &str = "expired_at";
pub const REDIS_KEY_OTP_ATTEMPT: &str = "otp_attempt";
pub const REDIS_KEY_OTP_TYPE: &str = "otp_type";
pub const REDIS_KEY_OTP_PHONE_NUMBER: &str = "otp_phone_number";
pub const REDIS_KEY_USER_PHONE_NUMBER: &str = "user_phone_number";
pub const REDIS_KEY_USER_EMAIL: &str = "email";
pub const REDIS_KEY_USER_ID: &str = "id";
pub const REDIS_KEY_USER_DISPLAY_NAME: &str = "display_name";

pub const REDIS_SESSION_OTP_SIGN_UP: &str = "sign-up-otp";
pub const REDIS_SESSION_OTP_SIGN_IN: &str = "sign-in-otp";
pub const REDIS_SESSION_SIGN_IN: &str = "sign-in";

pub const USER_STATUS_WAITING_ACTIVATION: &str = "WAITING_ACTIVATION";
pub const USER_STATUS_ACTIVE: &str = "ACTIVE";
pub const USER_STATUS_SUSPENDED: &str = "SUSPENDED";
pub const USER_STATUS_INACTIVE: &str = "INACTIVE";

pub const EVENT_STATUS_INACTIVE: &str = "INACTIVE";
pub const EVENT_STATUS_DRAFT: &str = "DRAFT";
pub const EVENT_STATUS_PUBLISHED: &str = "PUBLISHED";

pub const DEFAULT_ID_NON_MEMBER: &str = "6742c74a15e68b0e7ee06145";

pub const EVENT_GUEST_ROLE_HOST:&str="host";
pub const EVENT_GUEST_ROLE_CO_HOST:&str="co-host";
pub const EVENT_GUEST_ROLE_GUEST:&str="guest";

pub const NOTIFICATION_TYPE_INVITATION:&str="invitation";

pub const COLLECTION_USERS: &str = "users";
pub const COLLECTION_EVENTS: &str = "events";
pub const COLLECTION_EVENT_INVITATION: &str = "invitation-event";
pub const COLLECTION_CONFIGURATION: &str = "configuration";
pub const COLLECTION_MUTUAL: &str = "mutual";
pub const COLLECTION_EVENT_IMAGES: &str = "event-images";
pub const COLLECTION_EVENT_GUEST: &str = "event-host";
pub const COLLECTION_EVENT_THEMES: &str = "event-themes";
pub const COLLECTION_NOTIFICATION: &str = "notifications";
pub const COLLECTION_NOTIFICATION_LOG: &str = "notification-logs";


pub const COLLECTION_USER_PROFILE: &str = "user-profile";
pub const COLLECTION_THREAD: &str = "thread";
pub const COLLECTION_THREAD_VOTE: &str = "thread-vote";
pub const COLLECTION_RESERVE_ATTACHMENT: &str = "reserve-attachment";


pub const SSE_EVENT_UPDATE_EVENT_IMAGE: &str = "update-event-image";
pub const SSE_EVENT_UPDATE_EVENT_DATA: &str = "update-event-data";
pub const SSE_EVENT_UPDATE_EVENT_VENUE: &str = "update-event-venue";
pub const SSE_EVENT_UPDATE_EVENT_CONFIG: &str = "update-event-config";
pub const SSE_EVENT_UPDATE_EVENT_HOST: &str = "update-event-host";

pub const INVITATION_TYPE_PUBLIC: &str = "invitation-public";
pub const INVITATION_TYPE_USER: &str = "invitation-user";