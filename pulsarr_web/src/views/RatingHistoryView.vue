<script setup lang="ts">
    import { onMounted, onUnmounted, ref, watch } from "vue";
    import type { Rating } from "@/apiClient";
    import { useFeedStore } from "@/stores/feed";
    import { storeToRefs } from "pinia";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler";
    import { Star, Music, BarChart3, User, TrendingUp } from "lucide-vue-next";
    import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
    import RatingCard from "@/components/RatingCard.vue";
    import RatingDetailModal from "@/components/RatingDetailModal.vue";
    import EmptyState from "@/components/EmptyState.vue";
    import LoadingSpinner from "@/components/LoadingSpinner.vue";
    import { formatRatingValue } from "@/helpers/ratingFormatters";

    interface CoverArtInfo {
        front?: string;
        front_thumbnail_small?: string;
        front_thumbnail_large?: string;
    }

    interface ArtistCount {
        artist_name: string;
        count: number;
    }

    interface UserRatingStats {
        total_ratings: number;
        average_rating: string | null;
        top_artists: ArtistCount[];
    }

    const feedStore = useFeedStore();
    const { myGroups } = storeToRefs(feedStore);

    const ratings = ref<Rating[]>([]);
    const stats = ref<UserRatingStats | null>(null);
    const loading = ref(true);
    const loadingMore = ref(false);
    const hasMore = ref(true);
    const coverArtCache = ref<Record<string, string>>({});
    const selectedRating = ref<Rating | null>(null);
    const showRatingModal = ref(false);
    const sentinel = ref<HTMLElement | null>(null);
    let observer: IntersectionObserver | null = null;

    const PAGE_SIZE = 20;

    const fetchCoverArt = (releaseId: string) => {
        if (coverArtCache.value[releaseId]) return;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const coverArt = data as CoverArtInfo;
            if (coverArt.front_thumbnail_small || coverArt.front) {
                coverArtCache.value[releaseId] = coverArt.front_thumbnail_small || coverArt.front || '';
            }
        };
        drh.onErrorCallback = () => {};
        drh.get(`/musicbrainz/release-group/${releaseId}/cover-art-info`);
    };

    const fetchAllCoverArt = (items: Rating[]) => {
        items.forEach(rating => {
            if (rating.media_type === 'album' && rating.musicbrainz_id) {
                fetchCoverArt(rating.musicbrainz_id);
            }
        });
    };

    const openRatingModal = (rating: Rating) => {
        selectedRating.value = rating;
        showRatingModal.value = true;
    };

    const fetchRatings = (offset: number) => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const fetched = data as Rating[];
            if (offset === 0) {
                ratings.value = fetched;
            } else {
                ratings.value = [...ratings.value, ...fetched];
            }
            hasMore.value = fetched.length >= PAGE_SIZE;
            fetchAllCoverArt(fetched);
            if (offset === 0) loading.value = false;
            loadingMore.value = false;
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to fetch ratings:', err);
            if (offset === 0) loading.value = false;
            loadingMore.value = false;
        };
        drh.get(`/rating/byUser?take_size=${PAGE_SIZE}&offset=${offset}`);
    };

    const loadMore = () => {
        if (loadingMore.value || !hasMore.value) return;
        loadingMore.value = true;
        fetchRatings(ratings.value.length);
    };

    const setupObserver = () => {
        if (observer) observer.disconnect();
        const el = sentinel.value;
        if (!el) return;
        observer = new IntersectionObserver(
            (entries) => {
                if (entries[0].isIntersecting) {
                    loadMore();
                }
            },
            { rootMargin: '200px' }
        );
        observer.observe(el);
    };

    const maxArtistCount = (artists: ArtistCount[]) => {
        return Math.max(...artists.map(a => a.count), 1);
    };

    onMounted(() => {
        // Load feed store if needed (for group name resolution)
        if (myGroups.value.length === 0) {
            feedStore.loadFeed();
        }

        // Fetch stats
        const statsDrh = new DataRequestHandler();
        statsDrh.onSuccessCallback = (data) => {
            stats.value = data as UserRatingStats;
        };
        statsDrh.onErrorCallback = (err) => {
            console.error('Failed to fetch stats:', err);
        };
        statsDrh.get('/rating/stats');

        // Fetch initial ratings
        fetchRatings(0);
    });

    watch(sentinel, () => setupObserver());
    watch(ratings, () => setupObserver());

    onUnmounted(() => {
        if (observer) observer.disconnect();
    });
</script>

<template>
    <div class="p-4 max-w-4xl mx-auto">
        <h1 class="text-2xl font-bold flex items-center gap-2 mb-4">
            <BarChart3 class="w-6 h-6" />
            My Ratings
        </h1>

        <LoadingSpinner v-if="loading" text="Loading your ratings..." />

        <template v-else>
            <!-- Stats Cards -->
            <div v-if="stats" class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-6">
                <Card>
                    <CardHeader class="pb-2">
                        <CardTitle class="text-sm font-medium text-muted-foreground">Total Ratings</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="text-2xl font-bold flex items-center gap-2">
                            <Music class="w-5 h-5 text-muted-foreground" />
                            {{ stats.total_ratings }}
                        </div>
                    </CardContent>
                </Card>
                <Card>
                    <CardHeader class="pb-2">
                        <CardTitle class="text-sm font-medium text-muted-foreground">Average Rating</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="text-2xl font-bold flex items-center gap-2">
                            <Star class="w-5 h-5 text-muted-foreground" />
                            {{ stats.average_rating ? formatRatingValue(stats.average_rating) : 'N/A' }}
                        </div>
                    </CardContent>
                </Card>
                <Card>
                    <CardHeader class="pb-2">
                        <CardTitle class="text-sm font-medium text-muted-foreground">Most Rated Artist</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="text-2xl font-bold flex items-center gap-2 truncate">
                            <User class="w-5 h-5 text-muted-foreground flex-shrink-0" />
                            <span class="truncate">{{ stats.top_artists.length > 0 ? stats.top_artists[0].artist_name : 'N/A' }}</span>
                        </div>
                    </CardContent>
                </Card>
            </div>

            <!-- Top Artists Breakdown -->
            <Card v-if="stats && stats.top_artists.length > 0" class="mb-6">
                <CardHeader>
                    <CardTitle class="text-base flex items-center gap-2">
                        <TrendingUp class="w-4 h-4" />
                        Top Artists
                    </CardTitle>
                </CardHeader>
                <CardContent class="space-y-3">
                    <div v-for="artist in stats.top_artists" :key="artist.artist_name" class="flex items-center gap-3">
                        <span class="text-sm w-20 sm:w-32 truncate flex-shrink-0">{{ artist.artist_name }}</span>
                        <div class="flex-1 bg-muted rounded-full h-4 overflow-hidden">
                            <div
                                class="bg-primary h-full rounded-full transition-all"
                                :style="{ width: `${(artist.count / maxArtistCount(stats!.top_artists)) * 100}%` }"
                            />
                        </div>
                        <span class="text-sm text-muted-foreground w-8 text-right">{{ artist.count }}</span>
                    </div>
                </CardContent>
            </Card>

            <!-- Rating Timeline -->
            <EmptyState
                v-if="ratings.length === 0"
                :icon="Music"
                title="No ratings yet"
                description="Start rating music in your groups to build your history"
            />

            <div v-else class="space-y-3">
                <RatingCard
                    v-for="rating in ratings"
                    :key="rating.rating_id"
                    :rating="rating"
                    :cover-art-url="coverArtCache[rating.musicbrainz_id]"
                    :user-name="feedStore.getUserName(rating.pulsarr_user_id)"
                    :group-name="feedStore.getGroupName(rating.pulsarr_group_id)"
                    @click="openRatingModal(rating)"
                />

                <div ref="sentinel" v-if="hasMore" />
                <LoadingSpinner v-if="loadingMore" text="Loading more..." />
            </div>

            <RatingDetailModal
                :rating="selectedRating"
                :open="showRatingModal"
                :cover-art-url="selectedRating ? coverArtCache[selectedRating.musicbrainz_id] : undefined"
                :user-name="selectedRating ? feedStore.getUserName(selectedRating.pulsarr_user_id) : ''"
                :group-name="selectedRating ? feedStore.getGroupName(selectedRating.pulsarr_group_id) : undefined"
                :rating-system="selectedRating ? feedStore.getGroupForRating(selectedRating.pulsarr_group_id)?.rating_system : null"
                @update:open="showRatingModal = $event"
            />
        </template>
    </div>
</template>
