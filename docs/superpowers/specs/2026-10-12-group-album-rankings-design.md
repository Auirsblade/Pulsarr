# Group Album Rankings — Design

**Date:** 2026-10-12
**Branch:** `feature/group-album-rankings` (off `develop`)

## Goal

Let a group see every album it has rated, ranked by the group's average score, and narrow the list to albums released within a custom date range (e.g. "Dec 1 2024 – Nov 30 2025" for an end-of-year list).

## Decisions

| Question | Decision |
|---|---|
| What does the date filter apply to? | The album's **release date**, as a custom From/To date range (both ends optional) |
| Which ratings count toward an album's score? | Only ratings made under the group's **current** rating system (consistent scale) |
| Where does it live? | A new **Albums** tab next to the existing ratings feed on the group page |
| How is it displayed? | A compact **ranked list** (leaderboard rows) |
| Where are sorting and filtering done? | **In the browser**: one endpoint returns all albums for the group |
| Does it work for My Crate? | Yes. A Crate is a Personal group, so the same view works and each score is the owner's own rating |

### Known, accepted limitation

MusicBrainz `first-release-date` can be partial. `"2024"` is stored as `2024-01-01` and `"2024-05"` as `2024-05-01`. For a range that doesn't start on Jan 1, an album whose only known date is the year can land on the wrong side of the boundary. Users will handle these cases manually.

## Backend

### Endpoint: `GET /rating/group-albums/<group_id>`

Lives in `pulsarr_fulcrum/src/api/rating.rs` and is added to `get_routes_and_docs` so it appears in the OpenAPI spec.

**Access**
- Requires an `ApiKey`, i.e. a signed-in user.
- Loads the group. If its `privacy_type` is Private or Personal and `authorization::get_membership(pool, group_id, user_id)` returns `None`, it returns `PulsarrError::forbidden`. This is the same check as `GET /group/<id>` (`group.rs`).
- Returns not-found if the group doesn't exist, as in `GET /group/<id>`.

**Query**

```sql
SELECT r.musicbrainz_id,
       MAX(r.media_title)                    AS media_title,
       MAX(r.artist_name)                    AS artist_name,
       MIN(r.release_date)                   AS release_date,
       AVG(r.rating_value)::numeric(8,3)     AS average_score,
       COUNT(*)::bigint                      AS rating_count
FROM rating r
INNER JOIN pulsarr_group g ON g.pulsarr_group_id = r.pulsarr_group_id
WHERE r.pulsarr_group_id = $1
  AND r.rating_system_id = g.rating_system_id
  AND r.musicbrainz_id IS NOT NULL
GROUP BY r.musicbrainz_id
```

- `MIN(release_date)`: when MusicBrainz had no date, `AddRatingModal` saved the rating's creation time instead, which is always later than the real release date. Taking the earliest value means any one rating with the real date fixes the album's date.
- `MAX(media_title)` and `MAX(artist_name)` pick one copy deterministically. Each rating stores its own copy, and the copies are expected to be identical.
- No `ORDER BY` and no pagination; the client sorts and filters.

**Response DTO.** Defined in `rating.rs` next to `GroupAverageResponse`:

```rust
#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub struct GroupAlbumResponse {
    pub musicbrainz_id: String,
    pub media_title: Option<String>,
    pub artist_name: Option<String>,
    pub release_date: Option<NaiveDateTime>,
    pub average_score: Decimal,
    pub rating_count: i64,
}
```

The text and date fields are `Option` because the DB columns are nullable, even though the `Rating` model treats some of them as required.

### Migration

`pulsarr_fulcrum/db/migrations/<timestamp>_rating_group_album_index.sql`:

```sql
CREATE INDEX IF NOT EXISTS idx_rating_group_musicbrainz
    ON rating (pulsarr_group_id, musicbrainz_id);
```

Supports the new grouping query and also speeds up the existing `group-averages` and `by-album` lookups. The existing unique constraint starts with `pulsarr_user_id`, so it doesn't help these queries.

## Frontend

### `composables/useCoverArt.ts` (new; extracted from `GroupView.vue`)

- Moves `GroupView`'s `coverArtCache` and `fetchCoverArt` logic into a composable.
- The cache is a reactive object at module level, so every component shares it and the two tabs never fetch the same cover twice.
- API: `const { coverArtCache, fetchCoverArt } = useCoverArt()`, where `fetchCoverArt(mbids: string[])` skips any MBID that is already cached or in flight.
- `GroupView.vue` switches to the composable. Its behavior doesn't change.

### `components/GroupAlbumRankings.vue` (new)

**Props:** `groupId: number` and `ratingMax: string | undefined`. `ratingMax` is the same value `GroupView` already passes to `RatingCard`: `group.rating_system.rating_max`.

**Emits:** `openAlbum(rating: Rating)`. The component does not render `RatingDetailModal` itself; `GroupView` opens its existing instance (see below), so the edit, "Rate this", reactions and member-name handling are reused as they are.

**Exposes:** `refresh()`, which refetches the album list.

**State**
- `albums`: the full response from `GET /rating/group-albums/<groupId>`.
- `loading` and `error`.
- `fromDate` and `toDate` (`YYYY-MM-DD` strings; empty means no limit).
- `sortDir`: `'desc'` (default) or `'asc'`.
- `visibleCount`: starts at 50.

**Derived list (computed)**
1. **Filter.** With no From and no To, keep every album, including those with a null `release_date`. Otherwise keep only albums with a non-null `release_date` where the date is on or after From and on or before To. Compare dates only, ignoring time, so To includes the whole day.
2. **Sort.** By `average_score` in `sortDir` order. Ties go to the higher `rating_count`, then to `media_title` alphabetically.
3. **Rank** is the 1-based position in the sorted list.
4. **Render** only the first `visibleCount` items.

**Controls row**
- "From" and "To" fields: `Input type="date"`.
- A "Clear" button, shown only when a date is set.
- Sort: a `Select` with "Highest score" / "Lowest score".
- A count label: "N albums" (the count after filtering).

**Row**
- Shows the rank, a cover thumbnail (placeholder icon on miss or failure), title, and "artist · year".
- Shows the score as `formatRatingValue(average_score) / ratingMax` and "N rating(s)".
- Title and artist truncate, with a `title` tooltip on hover (same treatment as #48).
- The whole row is clickable.

**Pagination:** a "Show more" button adds 50 to `visibleCount`. Cover art is requested for the MBIDs in the visible slice whenever that slice changes.

**Clicking a row**
1. `GET /rating/by-album/<groupId>/<mbid>`.
2. Choose the caller's own rating if one exists (`pulsarr_user_id === context user id`), otherwise the first (most recent) one.
3. Emit `openAlbum(rating)`.
4. `GroupView` handles the event:
   - Make sure `groupAverages` contains that album, using `fetchGroupAverages`. It must finish before the modal opens, because the modal only renders the Group Ratings tab when `groupAverage` is set. So `fetchGroupAverages` gets a `Promise` return so it can be awaited.
   - Call the existing `openRatingModalGroupTab(rating)`, which sets `initialTab = 'group-ratings'`.
5. The modal therefore shows the same all-ratings average and list as the rating cards do, with outdated ratings labelled "Outdated". The ranking row shows the current-system-only average. This difference is accepted.

**States**
- Loading: `LoadingSpinner`.
- Error: inline message plus a "Retry" button.
- No albums at all: `EmptyState` with the message "No albums rated yet".
- Filters exclude everything: `EmptyState` with the message "No albums released in this date range" and a "Clear dates" action.

### `views/GroupView.vue` changes

- Wrap the ratings section body in `Tabs` with the triggers "Ratings" and "Albums". The section heading and the "Add Rating" button stay above the tabs, so they show on both.
- The Ratings tab keeps the existing feed markup exactly as it is.
- The Albums tab renders `<GroupAlbumRankings>`, mounted lazily on first activation and kept mounted afterwards.
- After `onRatingCreated`, `onRatingUpdated`, a rating delete, or `onRatingSystemChanged`, set an `albumsStale` flag. When the Albums tab is next activated and the flag is set, call the component's `refresh()` and clear the flag.
- Handle `@open-album` as described above. `fetchGroupAverages` returns a `Promise<void>` that resolves when its request settles, or immediately if nothing needs fetching. Existing callers ignore the return value, so their behavior doesn't change.
- Use the `useCoverArt` composable instead of the local cache.

### API client

Run `npm run generateModels` after the backend change so `GroupAlbumResponse` is generated. Calls go through `DataRequestHandler`, as elsewhere in the views.

## Error handling

| Case | Behavior |
|---|---|
| 403 (non-member, private or Crate) | Not reachable in normal use, because `GroupView` already blocks these users. If it happens, the generic error state is shown. |
| Network or 5xx | Inline error and a Retry button. |
| Cover art missing or failed | Placeholder icon. |
| Null `release_date` | Included when no dates are set; excluded once any date bound is set. |
| `by-album` fetch fails or returns nothing on click | Inline error under the controls row; the modal does not open. |

## Verification

There is no automated test suite in either project, so verification is manual.

1. `cargo build` succeeds in `pulsarr_fulcrum/`.
2. `npm run build` (includes `vue-tsc`) succeeds in `pulsarr_web/`.
3. The endpoint, tested against the local DB:
   - Averages and counts match a hand-written SQL check for a sample group.
   - A group that has changed rating systems excludes its outdated ratings.
   - A non-member requesting a Private group gets 403, and so does someone other than the owner requesting a Crate.
   - The owner of a Crate gets their albums.
4. In the browser preview (`/group/:id`, Albums tab):
   - Sorting high→low and low→high works, and the tie order is correct.
   - From-only, To-only, and a range crossing a year boundary (Dec → Nov) all filter correctly.
   - Clear restores the full list, and the empty state appears for a range with no albums.
   - "Show more" appends rows and cover art loads for them.
   - Clicking a row opens `RatingDetailModal` on Group Ratings.
   - After adding a rating on the Ratings tab, the Albums tab reflects it.
   - The Ratings tab behaves exactly as before, including infinite scroll, cover art, and group averages.

## Out of scope

- Persisting filter state in the URL or local storage.
- Date-range presets.
- Sorting by anything other than score.
- Making the rating card group-average badge use current-system ratings only.
- Membership checks on the existing `byGroups`, `by-album` and `group-averages` endpoints. These currently only require sign-in; this is noted as a separate issue.
