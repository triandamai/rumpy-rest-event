pub const KIND_THREAD_ATTACHMENT: &str = "PRODUCT-KIND_THREAD_ATTACHMENT";

pub const KIND_PUBLIC: &str = "PUBLIC";
pub const KIND_DISCUSSION: &str = "DISCUSSION";

pub const KIND_UP_VOTE_THREAD: &str = "UP-VOTE-THREAD";
pub const KIND_DOWN_VOTE_THREAD: &str = "DOWN-VOTE-THREAD";

pub const PROVIDER_BASIC: &str = "BASIC";
pub const PROVIDER_OTP: &str = "OTP";
pub const PROVIDER_GOOGLE: &str = "GOOGLE";

pub const BUCKET_THREAD: &str = "thread";
pub const BUCKET_PROFILE_PICTURE: &str = "thread";

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

pub const REDIS_SESSION_OTP_SIGN_UP: &str = "sign-up-otp";
pub const REDIS_SESSION_OTP_SIGN_IN: &str = "sign-in-otp";
pub const REDIS_SESSION_SIGN_IN: &str = "sign-in";

pub const USER_STATUS_WAITING_ACTIVATION: &str = "WAITING_ACTIVATION";
pub const USER_STATUS_ACTIVE: &str = "ACTIVE";
pub const USER_STATUS_SUSPENDED: &str = "SUSPENDED";
pub const USER_STATUS_INACTIVE: &str = "INACTIVE";

pub const DEFAULT_ID_NON_MEMBER: &str = "6742c74a15e68b0e7ee06145";

pub const COLLECTION_USERS: &str = "users";
pub const COLLECTION_FOLLOWER: &str = "mutuals";
pub const COLLECTION_USER_PROFILE: &str = "user-profile";
pub const COLLECTION_THREAD: &str = "thread";
pub const COLLECTION_THREAD_VOTE: &str = "thread-vote";
pub const COLLECTION_RESERVE_ATTACHMENT: &str = "reserve-attachment";
