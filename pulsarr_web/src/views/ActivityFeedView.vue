<script setup lang="ts">
    import { onMounted, onUnmounted, ref, watch } from "vue";
    import type { Rating } from "@/apiClient";
    import { useFeedStore } from "@/stores/feed";
    import { useContextStore } from "@/stores/context";
    import { storeToRefs } from "pinia";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler";
    import { Star, Music } from "lucide-vue-next";
    import { Button } from "@/components/ui/button";
    import RatingCard from "@/components/RatingCard.vue";
    import RatingDetailModal from "@/components/RatingDetailModal.vue";
    import EmptyState from "@/components/EmptyState.vue";
    import LoadingSpinner from "@/components/LoadingSpinner.vue";

    interface CoverArtInfo {
        front?: string;
        front_thumbnail_small?: string;
        front_thumbnail_large?: string;
    }

    type FeedMode = 'all' | 'myGroups';

    const feedStore = useFeedStore();
    const { ratings, loading, loadingMore, hasMore, error } = storeToRefs(feedStore);
    const { isLoggedIn } = storeToRefs(useContextStore());

    const feedMode = ref<FeedMode>('all');
    const coverArtCache = ref<Record<string, string>>({});
    const selectedRating = ref<Rating | null>(null);
    const showRatingModal = ref(false);
    const sentinel = ref<HTMLElement | null>(null);
    let observer: IntersectionObserver | null = null;

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

    const fetchAllCoverArt = () => {
        ratings.value.forEach(rating => {
            if (rating.media_type === 'album' && rating.musicbrainz_id) {
                fetchCoverArt(rating.musicbrainz_id);
            }
        });
    };

    const openRatingModal = (rating: Rating) => {
        selectedRating.value = rating;
        showRatingModal.value = true;
    };

    const loadData = () => {
        if (!isLoggedIn.value) {
            feedStore.loadPublicFeed();
        } else if (feedMode.value === 'all') {
            feedStore.loadAllFeed();
        } else {
            feedStore.loadFeed();
        }
    };

    const setupObserver = () => {
        if (observer) observer.disconnect();
        const el = sentinel.value;
        if (!el) return;
        observer = new IntersectionObserver(
            (entries) => {
                if (entries[0].isIntersecting) {
                    feedStore.loadMoreRatings();
                }
            },
            { rootMargin: '200px' }
        );
        observer.observe(el);
    };

    onMounted(loadData);

    watch(isLoggedIn, loadData);
    watch(feedMode, loadData);

    watch(sentinel, () => {
        setupObserver();
    });

    // Re-setup observer and fetch cover art when ratings change
    watch(ratings, () => {
        setupObserver();
        fetchAllCoverArt();
    });

    onUnmounted(() => {
        if (observer) observer.disconnect();
    });
</script>

<template>
    <div class="p-4 max-w-4xl mx-auto">
        <div class="flex items-center justify-between mb-4">
            <h1 class="text-2xl font-bold flex items-center gap-2">
                <Star class="w-6 h-6" />
                Activity Feed
            </h1>
            <div v-if="isLoggedIn" class="flex rounded-md border border-input overflow-hidden">
                <Button
                    variant="ghost"
                    size="sm"
                    class="rounded-none border-0"
                    :class="feedMode === 'all' ? 'bg-secondary' : ''"
                    @click="feedMode = 'all'"
                >
                    All
                </Button>
                <Button
                    variant="ghost"
                    size="sm"
                    class="rounded-none border-0"
                    :class="feedMode === 'myGroups' ? 'bg-secondary' : ''"
                    @click="feedMode = 'myGroups'"
                >
                    My Groups
                </Button>
            </div>
        </div>

        <LoadingSpinner v-if="loading" text="Loading feed..." />

        <div v-else-if="error" class="flex justify-center py-8">
            <span class="text-destructive">{{ error }}</span>
        </div>

        <EmptyState
            v-else-if="ratings.length === 0"
            :icon="Music"
            :title="isLoggedIn ? 'No activity yet' : 'No public ratings yet'"
            :description="isLoggedIn ? 'Join a group and start rating music to see activity here' : 'Sign in to discover what others are listening to'"
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
    </div>
</template>
