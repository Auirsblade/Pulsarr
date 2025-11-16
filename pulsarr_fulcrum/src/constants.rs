// Media Types
pub const ALBUM_MEDIA_TYPE: &str = "Album";
pub const SONG_MEDIA_TYPE: &str = "Song";
pub const MEDIA_TYPE: [&str; 2] = [ALBUM_MEDIA_TYPE, SONG_MEDIA_TYPE];

// Privacy Types
pub const PUBLIC_PRIVACY_TYPE: &str = "Public";
pub const PRIVATE_PRIVACY_TYPE: &str = "Private";
pub const PERSONAL_PRIVACY_TYPE: &str = "Personal";
pub const PRIVACY_TYPE: [&str; 3] = [PUBLIC_PRIVACY_TYPE, PRIVATE_PRIVACY_TYPE, PERSONAL_PRIVACY_TYPE];

// Rating Types
pub const ABSOLUTE_RATING_TYPE: &str = "Absolute";
pub const CUMULATIVE_RATING_TYPE: &str = "Cumulative";
pub const AVERAGE_RATING_TYPE: &str = "Average";
pub const RATING_TYPE: [&str; 3] = [ABSOLUTE_RATING_TYPE, CUMULATIVE_RATING_TYPE, AVERAGE_RATING_TYPE];

// Group Member Roles
pub const OWNER_ROLE: &str = "Owner";
pub const ADMIN_ROLE: &str = "Admin";
pub const MEMBER_ROLE: &str = "Member";
pub const VIEW_ONLY_ROLE: &str = "ViewOnly";
pub const MEMBERSHIP_TYPE: [&str; 4] = [OWNER_ROLE, ADMIN_ROLE, MEMBER_ROLE, VIEW_ONLY_ROLE];