use sqlx::PgPool;
use crate::constants::{OWNER_ROLE, ADMIN_ROLE, MEMBER_ROLE, VIEW_ONLY_ROLE, PUBLIC_PRIVACY_TYPE};
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

/// Returns the subset of `group_ids` whose contents `user_id` may read: Public
/// groups, plus any group the user is a member of. Same rule as `GET /group/<id>`
/// (Private and Personal groups require membership), checked in one query.
pub async fn visible_group_ids(
    pool: &PgPool,
    group_ids: &[i32],
    user_id: i32,
) -> Result<Vec<i32>, PulsarrError> {
    sqlx::query_scalar::<_, i32>(
        "SELECT g.pulsarr_group_id FROM pulsarr_group g \
         WHERE g.pulsarr_group_id = ANY($1) \
           AND (g.privacy_type = $2 OR EXISTS ( \
               SELECT 1 FROM user_group ug \
               WHERE ug.pulsarr_group_id = g.pulsarr_group_id AND ug.pulsarr_user_id = $3 \
           ))"
    )
        .bind(group_ids)
        .bind(PUBLIC_PRIVACY_TYPE)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| PulsarrError::internal_error("Failed to check group visibility", e))
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
