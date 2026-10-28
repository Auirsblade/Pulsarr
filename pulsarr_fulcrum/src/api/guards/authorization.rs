use sqlx::PgPool;
use crate::constants::{OWNER_ROLE, ADMIN_ROLE, MEMBER_ROLE, VIEW_ONLY_ROLE, PUBLIC_PRIVACY_TYPE, PERSONAL_PRIVACY_TYPE};
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

/// Fails with 403 unless `user_id` may read rating `rating_id` (and its details and
/// reactions). Same rule as `visible_group_ids`, plus two cases `/profile/<id>/ratings`
/// already exposes: the author can always see their own rating (even after leaving the
/// group), and a crate owner's ratings are readable when they've made their crate
/// public on a public profile. A missing rating is also 403, so ids can't be probed.
pub async fn require_rating_visible(
    pool: &PgPool,
    rating_id: i32,
    user_id: i32,
) -> Result<(), PulsarrError> {
    let visible = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS ( \
             SELECT 1 FROM rating r \
             JOIN pulsarr_group g ON g.pulsarr_group_id = r.pulsarr_group_id \
             LEFT JOIN pulsarr_user owner ON owner.pulsarr_user_id = g.created_by_user_id \
             WHERE r.rating_id = $1 AND ( \
                 r.pulsarr_user_id = $2 \
                 OR g.privacy_type = $3 \
                 OR EXISTS ( \
                     SELECT 1 FROM user_group ug \
                     WHERE ug.pulsarr_group_id = g.pulsarr_group_id AND ug.pulsarr_user_id = $2 \
                 ) \
                 OR (g.privacy_type = $4 AND r.pulsarr_user_id = g.created_by_user_id \
                     AND owner.crate_visibility = 'public' AND owner.profile_visibility <> 'private') \
             ))"
    )
        .bind(rating_id)
        .bind(user_id)
        .bind(PUBLIC_PRIVACY_TYPE)
        .bind(PERSONAL_PRIVACY_TYPE)
        .fetch_one(pool)
        .await
        .map_err(|e| PulsarrError::internal_error("Failed to check rating visibility", e))?;

    if visible {
        Ok(())
    } else {
        Err(PulsarrError::forbidden("This rating is in a private group"))
    }
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
