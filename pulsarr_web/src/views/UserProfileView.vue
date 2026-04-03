<script setup lang="ts">
    import { onMounted, onUnmounted, ref, computed } from "vue";
    import { useRoute } from "vue-router";
    import type { Rating } from "@/apiClient";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler";
    import { useContextStore } from "@/stores/context";
    import { storeToRefs } from "pinia";
    import { User, Lock } from "lucide-vue-next";
    import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
    import RatingCard from "@/components/RatingCard.vue";
    import RatingDetailModal from "@/components/RatingDetailModal.vue";
    import EmptyState from "@/components/EmptyState.vue";
    import LoadingSpinner from "@/components/LoadingSpinner.vue";
    import { formatDate } from "@/helpers/ratingFormatters";

    interface UserProfileResponse {
        user: { pulsarr_user_id: number; name: string; email: string; join_date?: string; };
        crate_group: any | null;
        visible_group_ids: number[];
        is_own_profile: boolean;
        is_private: boolean;
    }

    const route = useRoute();
    const contextStore = useContextStore();
    const { user: currentUser } = storeToRefs(contextStore);

    const profile = ref<UserProfileResponse | null>(null);
    const ratings = ref<Rating[]>([]);
    const loading = ref(true);
    const loadingMore = ref(false);
    const hasMore = ref(true);
    const coverArtCache = ref<Record<string, string>>({});
    const groupNames = ref<Map<number, string>>(new Map());
    const groupRatingMax = ref<Map<number, string>>(new Map());
    const selectedRating = ref<Rating | null>(null);
    const showRatingModal = ref(false);
    const sentinel = ref<HTMLElement | null>(null);
    let observer: IntersectionObserver | null = null;

    const userId = computed(() => Number(route.params.userId));
    const isOwnProfile = computed(() => currentUser.value?.pulsarr_user_id === userId.value);

    const loadProfile = async () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            profile.value = data as UserProfileResponse;
        };
        drh.onErrorCallback = (error) => {
            console.error('Failed to load profile:', error);
        };
        await drh.get(`/profile/${userId.value}`);
    };

    const loadRatings = async (append = false) => {
        if (!profile.value || profile.value.is_private) return;

        if (append) loadingMore.value = true;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const newRatings = data as Rating[];
            if (append) {
                ratings.value.push(...newRatings);
            } else {
                ratings.value = newRatings;
            }
            hasMore.value = newRatings.length >= 20;
            if (append) loadingMore.value = false;
            newRatings.forEach(r => {
                fetchCoverArt(r.musicbrainz_id);
                fetchGroupName(r.pulsarr_group_id);
            });
        };
        drh.onErrorCallback = () => {
            if (append) loadingMore.value = false;
        };
        await drh.post(`/profile/${userId.value}/ratings`, {
            take_size: 20,
            offset: append ? ratings.value.length : 0,
        });
    };

    const fetchGroupName = (groupId: number) => {
        if (groupNames.value.has(groupId)) return;
        groupNames.value.set(groupId, '');
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data: any) => {
            groupNames.value.set(groupId, data.name);
            if (data.rating_system?.rating_max) {
                groupRatingMax.value.set(groupId, data.rating_system.rating_max.toString());
            }
        };
        drh.get(`/group/preview/${groupId}`);
    };

    const fetchCoverArt = (musicbrainzId: string) => {
        if (coverArtCache.value[musicbrainzId] !== undefined) return;
        coverArtCache.value[musicbrainzId] = '';
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data: any) => {
            if (data?.front_thumbnail_small) {
                coverArtCache.value[musicbrainzId] = data.front_thumbnail_small;
            } else if (data?.front) {
                coverArtCache.value[musicbrainzId] = data.front;
            }
        };
        drh.get(`/musicbrainz/release-group/${musicbrainzId}/cover-art-info`);
    };

    const onRatingClick = (rating: Rating) => {
        selectedRating.value = rating;
        showRatingModal.value = true;
    };

    onMounted(async () => {
        await loadProfile();
        if (profile.value && !profile.value.is_private) {
            await loadRatings();
        }
        loading.value = false;

        // Infinite scroll
        observer = new IntersectionObserver((entries) => {
            if (entries[0].isIntersecting && hasMore.value && !loadingMore.value) {
                loadRatings(true);
            }
        }, { rootMargin: '200px' });

        if (sentinel.value) observer.observe(sentinel.value);
    });

    onUnmounted(() => {
        observer?.disconnect();
    });
</script>

<template>
    <div class="max-w-4xl mx-auto p-4 sm:p-6 space-y-6">
        <LoadingSpinner v-if="loading" />

        <template v-else-if="profile">
            <!-- Profile Header -->
            <Card>
                <CardHeader>
                    <div class="flex items-center gap-4">
                        <div class="w-16 h-16 rounded-full bg-muted flex items-center justify-center">
                            <User class="w-8 h-8 text-muted-foreground" />
                        </div>
                        <div>
                            <CardTitle class="text-2xl">{{ profile.user.name }}</CardTitle>
                            <p v-if="profile.user.join_date" class="text-sm text-muted-foreground">
                                Joined {{ formatDate(profile.user.join_date) }}
                            </p>
                            <p v-if="isOwnProfile" class="text-xs text-muted-foreground mt-1">
                                This is your profile
                            </p>
                        </div>
                    </div>
                </CardHeader>
            </Card>

            <!-- Private profile message -->
            <Card v-if="profile.is_private && !isOwnProfile">
                <CardContent class="py-12">
                    <EmptyState
                        :icon="Lock"
                        title="This profile is private"
                        :description="`${profile.user.name} has set their profile to private.`"
                    />
                </CardContent>
            </Card>

            <!-- Ratings feed -->
            <template v-else>
                <div v-if="ratings.length === 0 && !loading">
                    <Card>
                        <CardContent class="py-12">
                            <EmptyState
                                title="No visible ratings"
                                :description="isOwnProfile ? 'You haven\'t rated anything yet.' : 'No ratings to display.'"
                            />
                        </CardContent>
                    </Card>
                </div>

                <div v-else class="space-y-3">
                    <RatingCard
                        v-for="rating in ratings"
                        :key="rating.rating_id"
                        :rating="rating"
                        :user-name="rating.user_name ?? profile.user.name"
                        :group-name="groupNames.get(rating.pulsarr_group_id)"
                        :rating-max="groupRatingMax.get(rating.pulsarr_group_id)"
                        :cover-art-url="coverArtCache[rating.musicbrainz_id]"
                        @click="onRatingClick(rating)"
                    />
                </div>

                <!-- Infinite scroll sentinel -->
                <div ref="sentinel" class="h-4" />
                <LoadingSpinner v-if="loadingMore" />
            </template>
        </template>

        <!-- Rating detail modal -->
        <RatingDetailModal
            v-if="selectedRating"
            :open="showRatingModal"
            :rating="selectedRating"
            :user-name="selectedRating.user_name ?? profile?.user.name ?? ''"
            @update:open="showRatingModal = $event"
        />
    </div>
</template>
