use sqlx::PgPool;
use crate::constants::{OWNER_ROLE, ADMIN_ROLE, MEMBER_ROLE, VIEW_ONLY_ROLE};
use crate::data::models::user_group::{self, UserGroup};
use crate::error::PulsarrError;

pub fn role_level(role: &str) -> i32 {
    match role {
        r if r == OWNER_ROLE => 4,
        r if r == ADMIN_ROLE => 3,
        r if r == MEMBER_ROLE => 2,
        r if r == VIEW_ONLY_ROLE => 1,
        _ => 0,
    }
}

pub async fn get_membership(pool: &PgPool, group_id: i32, user_id: i32) -> Option<UserGroup> {
    user_group::get_by_group_and_user(group_id, user_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

pub async fn require_role(
    pool: &PgPool,
    group_id: i32,
    user_id: i32,
    min_role: &str,
) -> Result<UserGroup, PulsarrError> {
    let membership = get_membership(pool, group_id, user_id).await;
    match membership {
        Some(m) if role_level(&m.group_role) >= role_level(min_role) => Ok(m),
        Some(_) => Err(PulsarrError::forbidden("Insufficient permissions for this action")),
        None => Err(PulsarrError::forbidden("You are not a member of this group")),
    }
}

pub fn can_act_on(actor_role: &str, target_role: &str) -> bool {
    role_level(actor_role) > role_level(target_role)
}
