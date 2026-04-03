<script setup lang="ts">
    import type { Rating, RatingDetailResponse, RatingSystemDTO, GroupDTO } from "@/apiClient";
    import { Dialog, DialogScrollContent, DialogHeader, DialogTitle, DialogDescription } from "@/components/ui/dialog";
    import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
    import { Button } from "@/components/ui/button";
    import { Star, MessageSquare, AlertTriangle, Pencil, Users } from "lucide-vue-next";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler";
    import { formatDate, formatRatingValue } from "@/helpers/ratingFormatters";
    import { useContextStore } from "@/stores/context";
    import { storeToRefs } from "pinia";
    import { RouterLink } from "vue-router";
    import { ref, computed, watch, defineAsyncComponent } from "vue";
    import LoadingSpinner from "@/components/LoadingSpinner.vue";

    const NestedRatingDetailModal = defineAsyncComponent(() => import("@/components/RatingDetailModal.vue"));

    const props = defineProps<{
        rating: Rating | null;
        open: boolean;
        coverArtUrl?: string;
        userName: string;
        groupName?: string;
        ratingSystem?: RatingSystemDTO | null;
        outdated?: boolean;
        group?: GroupDTO | null;
        groupAverage?: string;
        groupRatingCount?: number;
        ratingMax?: string;
        initialTab?: string;
    }>();

    const emit = defineEmits<{
        'update:open': [value: boolean];
        'edit': [rating: Rating];
        'rateThis': [rating: Rating];
    }>();

    const { user } = storeToRefs(useContextStore());

    const isOwner = computed(() => {
        if (!props.rating || !user.value) return false;
        return props.rating.pulsarr_user_id === user.value.pulsarr_user_id;
    });

    const isMember = computed(() => {
        if (!props.group?.members || !user.value) return false;
        return props.group.members.some(m => m.user?.pulsarr_user_id === user.value?.pulsarr_user_id);
    });

    const onEdit = () => {
        if (props.rating) {
            emit('edit', props.rating);
            emit('update:open', false);
        }
    };

    const onRateThis = () => {
        if (props.rating) {
            emit('rateThis', props.rating);
            emit('update:open', false);
        }
    };

    const ratingDetails = ref<RatingDetailResponse[]>([]);
    const detailsLoaded = ref(false);

    const hasParameters = () => {
        return (props.ratingSystem?.parameters?.length ?? 0) > 0;
    };

    const getParameterInfo = (parameterId: number) => {
        return props.ratingSystem?.parameters?.find(p => p.rating_system_parameter_id === parameterId);
    };

    const fetchRatingDetails = (ratingId: number) => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const details = data as RatingDetailResponse[];
            details.sort((a, b) => a.rating_system_parameter_id - b.rating_system_parameter_id);
            ratingDetails.value = details;
            detailsLoaded.value = true;
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to fetch rating details:', err);
        };
        drh.get(`/rating/rating_detail/by_rating/${ratingId}`);
    };

    // Tab state
    const activeTab = ref(props.initialTab || 'rating');

    // Group ratings tab state
    const albumRatings = ref<Rating[]>([]);
    const albumRatingsLoaded = ref(false);
    const albumRatingsLoading = ref(false);

    // Nested modal for viewing individual ratings from group list
    const nestedRating = ref<Rating | null>(null);
    const showNestedModal = ref(false);

    const fetchAlbumRatings = () => {
        if (!props.rating || !props.rating.pulsarr_group_id || !props.rating.musicbrainz_id) return;
        albumRatingsLoading.value = true;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            albumRatings.value = data as Rating[];
            albumRatingsLoaded.value = true;
            albumRatingsLoading.value = false;
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to fetch album ratings:', err);
            albumRatingsLoading.value = false;
        };
        drh.get(`/rating/by-album/${props.rating.pulsarr_group_id}/${props.rating.musicbrainz_id}`);
    };

    const openNestedRating = (rating: Rating) => {
        nestedRating.value = rating;
        showNestedModal.value = true;
    };

    const getUserName = (rating: Rating): string => {
        if (rating.user_name) return rating.user_name;
        const member = props.group?.members?.find(m => m.user?.pulsarr_user_id === rating.pulsarr_user_id);
        return member?.user?.name || 'Unknown User';
    };

    watch(() => props.open, (isOpen) => {
        if (isOpen && props.rating && hasParameters() && !detailsLoaded.value) {
            fetchRatingDetails(props.rating.rating_id);
        }
        if (isOpen) {
            activeTab.value = props.initialTab || 'rating';
        }
    });

    watch(() => props.rating, () => {
        ratingDetails.value = [];
        detailsLoaded.value = false;
        albumRatings.value = [];
        albumRatingsLoaded.value = false;
        albumRatingsLoading.value = false;
    });

    // Lazy-load album ratings when switching to group-ratings tab
    watch(activeTab, (tab) => {
        if (tab === 'group-ratings' && !albumRatingsLoaded.value && !albumRatingsLoading.value) {
            fetchAlbumRatings();
        }
    });
</script>

<template>
    <Dialog :open="open" @update:open="$emit('update:open', $event)">
        <DialogScrollContent v-if="rating">
            <DialogHeader class="text-left">
                <DialogTitle class="flex items-center gap-2">
                    {{ rating.media_title }}
                    <span class="inline-flex items-center gap-1 px-2 py-0.5 bg-primary text-primary-foreground rounded font-bold text-sm">
                        <Star class="w-3 h-3" />
                        {{ formatRatingValue(rating.rating_value) }}
                    </span>
                </DialogTitle>
                <DialogDescription>
                    {{ rating.artist_name }}
                    <span v-if="rating.release_date"> · {{ formatDate(rating.release_date) }}</span>
                    <RouterLink v-if="groupName && rating" :to="`/group/${rating.pulsarr_group_id}`" class="ml-1 px-1.5 py-0.5 bg-secondary rounded text-xs hover:bg-secondary/80 transition-colors">{{ groupName }}</RouterLink>
                </DialogDescription>
            </DialogHeader>

            <!-- With tabs (group context) -->
            <Tabs v-if="groupAverage" v-model="activeTab" class="w-full">
                <TabsList class="w-full">
                    <TabsTrigger value="rating" class="flex-1">Rating</TabsTrigger>
                    <TabsTrigger value="group-ratings" class="flex-1">
                        <span class="inline-flex items-center">
                            <Users class="w-3.5 h-3.5 mr-1 flex-shrink-0" />
                            Group Ratings
                        </span>
                    </TabsTrigger>
                </TabsList>

                <TabsContent value="rating">
                    <div class="space-y-4 pt-2">
                        <div v-if="outdated" class="flex items-start gap-2 p-3 bg-amber-500/10 border border-amber-500/20 rounded-md text-sm">
                            <AlertTriangle class="w-4 h-4 text-amber-500 flex-shrink-0 mt-0.5" />
                            <span class="text-amber-700 dark:text-amber-400">This rating was made under a previous rating system.</span>
                        </div>

                        <div v-if="coverArtUrl" class="w-full max-w-[250px] mx-auto aspect-square bg-muted rounded-md overflow-hidden">
                            <img :src="coverArtUrl" class="w-full h-full object-cover" :alt="rating.media_title" />
                        </div>

                        <div v-if="ratingDetails.length > 0" class="space-y-2">
                            <h4 class="text-sm font-semibold">Rating Breakdown</h4>
                            <div class="p-3 bg-muted/50 rounded-md space-y-2">
                                <div v-for="detail in ratingDetails" :key="detail.rating_detail_id" class="flex justify-between text-sm">
                                    <span class="text-muted-foreground">
                                        {{ getParameterInfo(detail.rating_system_parameter_id)?.name || detail.parameter_name || 'Unknown' }}
                                        <span v-if="parseFloat(getParameterInfo(detail.rating_system_parameter_id)?.weight || '1') !== 1" class="text-xs">
                                            ({{ getParameterInfo(detail.rating_system_parameter_id)?.weight }}x)
                                        </span>
                                    </span>
                                    <span class="font-mono">
                                        {{ formatRatingValue(detail.rating_value) }}
                                        <span v-if="getParameterInfo(detail.rating_system_parameter_id)?.parameter_rating_max" class="text-muted-foreground">/ {{ formatRatingValue(getParameterInfo(detail.rating_system_parameter_id)!.parameter_rating_max!) }}</span>
                                    </span>
                                </div>
                            </div>
                        </div>

                        <div v-if="rating.comments" class="space-y-2">
                            <h4 class="text-sm font-semibold flex items-center gap-1">
                                <MessageSquare class="w-4 h-4" />
                                Review
                            </h4>
                            <p class="text-sm text-muted-foreground whitespace-pre-wrap break-words">{{ rating.comments }}</p>
                        </div>

                        <div class="flex items-center justify-between text-xs text-muted-foreground pt-2 border-t">
                            <span>by <RouterLink :to="`/user/${rating.pulsarr_user_id}`" class="hover:underline">{{ userName }}</RouterLink></span>
                            <span>{{ formatDate(rating.rating_date) }}</span>
                        </div>

                        <Button v-if="isOwner && group" variant="outline" class="w-full" @click="onEdit">
                            <Pencil class="w-4 h-4 mr-1" />
                            Edit Rating
                        </Button>

                        <Button v-if="!isOwner && isMember && group" variant="default" class="w-full" @click="onRateThis">
                            <Star class="w-4 h-4 mr-1" />
                            Rate This
                        </Button>
                    </div>
                </TabsContent>

                <TabsContent value="group-ratings">
                    <div class="space-y-4 pt-2">
                        <div class="flex items-center gap-3 p-3 bg-muted/50 rounded-md">
                            <Users class="w-5 h-5 text-muted-foreground" />
                            <div>
                                <div class="font-bold text-lg">
                                    {{ formatRatingValue(groupAverage) }}<span v-if="ratingMax" class="text-muted-foreground font-normal text-sm">/{{ formatRatingValue(ratingMax) }}</span>
                                </div>
                                <div class="text-xs text-muted-foreground">Group average from {{ groupRatingCount }} {{ groupRatingCount === 1 ? 'rating' : 'ratings' }}</div>
                            </div>
                        </div>

                        <LoadingSpinner v-if="albumRatingsLoading" text="Loading group ratings..." />

                        <div v-else-if="albumRatings.length > 0" class="space-y-1">
                            <button
                                v-for="ar in albumRatings"
                                :key="ar.rating_id"
                                class="w-full flex items-center justify-between p-2.5 rounded-md hover:bg-muted/50 transition-colors text-left"
                                @click="openNestedRating(ar)"
                            >
                                <span class="text-sm font-medium truncate">{{ getUserName(ar) }}</span>
                                <div class="flex items-center gap-2 flex-shrink-0">
                                    <span class="inline-flex items-center gap-1 px-1.5 py-0.5 bg-primary text-primary-foreground rounded text-xs font-bold">
                                        <Star class="w-2.5 h-2.5" />
                                        {{ formatRatingValue(ar.rating_value) }}<span v-if="ratingMax" class="font-normal text-primary-foreground/70">/{{ formatRatingValue(ratingMax) }}</span>
                                    </span>
                                    <span class="text-xs text-muted-foreground">{{ formatDate(ar.rating_date) }}</span>
                                </div>
                            </button>
                        </div>

                        <div v-else-if="albumRatingsLoaded" class="text-center text-sm text-muted-foreground py-4">
                            No ratings found for this album.
                        </div>
                    </div>
                </TabsContent>
            </Tabs>

            <!-- Without tabs (non-group context, e.g. activity feed) -->
            <div v-else class="space-y-4">
                <div v-if="outdated" class="flex items-start gap-2 p-3 bg-amber-500/10 border border-amber-500/20 rounded-md text-sm">
                    <AlertTriangle class="w-4 h-4 text-amber-500 flex-shrink-0 mt-0.5" />
                    <span class="text-amber-700 dark:text-amber-400">This rating was made under a previous rating system.</span>
                </div>

                <div v-if="coverArtUrl" class="w-full max-w-[250px] mx-auto aspect-square bg-muted rounded-md overflow-hidden">
                    <img :src="coverArtUrl" class="w-full h-full object-cover" :alt="rating.media_title" />
                </div>

                <div v-if="ratingDetails.length > 0" class="space-y-2">
                    <h4 class="text-sm font-semibold">Rating Breakdown</h4>
                    <div class="p-3 bg-muted/50 rounded-md space-y-2">
                        <div v-for="detail in ratingDetails" :key="detail.rating_detail_id" class="flex justify-between text-sm">
                            <span class="text-muted-foreground">
                                {{ getParameterInfo(detail.rating_system_parameter_id)?.name || detail.parameter_name || 'Unknown' }}
                                <span v-if="parseFloat(getParameterInfo(detail.rating_system_parameter_id)?.weight || '1') !== 1" class="text-xs">
                                    ({{ getParameterInfo(detail.rating_system_parameter_id)?.weight }}x)
                                </span>
                            </span>
                            <span class="font-mono">
                                {{ formatRatingValue(detail.rating_value) }}
                                <span v-if="getParameterInfo(detail.rating_system_parameter_id)?.parameter_rating_max" class="text-muted-foreground">/ {{ formatRatingValue(getParameterInfo(detail.rating_system_parameter_id)!.parameter_rating_max!) }}</span>
                            </span>
                        </div>
                    </div>
                </div>

                <div v-if="rating.comments" class="space-y-2">
                    <h4 class="text-sm font-semibold flex items-center gap-1">
                        <MessageSquare class="w-4 h-4" />
                        Review
                    </h4>
                    <p class="text-sm text-muted-foreground whitespace-pre-wrap break-words">{{ rating.comments }}</p>
                </div>

                <div class="flex items-center justify-between text-xs text-muted-foreground pt-2 border-t">
                    <span>by <RouterLink :to="`/user/${rating.pulsarr_user_id}`" class="hover:underline">{{ userName }}</RouterLink></span>
                    <span>{{ formatDate(rating.rating_date) }}</span>
                </div>

                <Button v-if="isOwner && group" variant="outline" class="w-full" @click="onEdit">
                    <Pencil class="w-4 h-4 mr-1" />
                    Edit Rating
                </Button>

                <Button v-if="!isOwner && isMember && group" variant="default" class="w-full" @click="onRateThis">
                    <Star class="w-4 h-4 mr-1" />
                    Rate This
                </Button>
            </div>
        </DialogScrollContent>
    </Dialog>

    <!-- Nested Rating Detail Modal (no groupAverage = no tabs = no further nesting) -->
    <NestedRatingDetailModal
        v-if="showNestedModal"
        :rating="nestedRating"
        :open="showNestedModal"
        :cover-art-url="nestedRating ? coverArtUrl : undefined"
        :user-name="nestedRating ? getUserName(nestedRating) : ''"
        :rating-system="ratingSystem"
        :outdated="nestedRating && group ? nestedRating.rating_system_id !== group.rating_system_id : false"
        :group="group"
        @update:open="showNestedModal = $event"
        @edit="(r) => { showNestedModal = false; emit('update:open', false); emit('edit', r); }"
        @rateThis="(r) => { showNestedModal = false; emit('update:open', false); emit('rateThis', r); }"
    />
</template>
