<script setup lang="ts">
    import type { Rating, RatingDetail, RatingSystemDTO, GroupDTO } from "@/apiClient";
    import { Dialog, DialogScrollContent, DialogHeader, DialogTitle, DialogDescription } from "@/components/ui/dialog";
    import { Button } from "@/components/ui/button";
    import { Star, MessageSquare, AlertTriangle, Pencil } from "lucide-vue-next";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler";
    import { formatDate, formatRatingValue } from "@/helpers/ratingFormatters";
    import { useContextStore } from "@/stores/context";
    import { storeToRefs } from "pinia";
    import { RouterLink } from "vue-router";
    import { ref, computed, watch } from "vue";

    const props = defineProps<{
        rating: Rating | null;
        open: boolean;
        coverArtUrl?: string;
        userName: string;
        groupName?: string;
        ratingSystem?: RatingSystemDTO | null;
        outdated?: boolean;
        group?: GroupDTO | null;
    }>();

    const emit = defineEmits<{
        'update:open': [value: boolean];
        'edit': [rating: Rating];
    }>();

    const { user } = storeToRefs(useContextStore());

    const isOwner = computed(() => {
        if (!props.rating || !user.value) return false;
        return props.rating.pulsarr_user_id === user.value.pulsarr_user_id;
    });

    const onEdit = () => {
        if (props.rating) {
            emit('edit', props.rating);
            emit('update:open', false);
        }
    };

    const ratingDetails = ref<RatingDetail[]>([]);
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
            ratingDetails.value = data as RatingDetail[];
            detailsLoaded.value = true;
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to fetch rating details:', err);
        };
        drh.get(`/rating/rating_detail/by_rating/${ratingId}`);
    };

    watch(() => props.open, (isOpen) => {
        if (isOpen && props.rating && hasParameters() && !detailsLoaded.value) {
            fetchRatingDetails(props.rating.rating_id);
        }
    });

    watch(() => props.rating, () => {
        ratingDetails.value = [];
        detailsLoaded.value = false;
    });
</script>

<template>
    <Dialog :open="open" @update:open="$emit('update:open', $event)">
        <DialogScrollContent v-if="rating">
            <DialogHeader>
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

            <div class="space-y-4">
                <!-- Outdated Banner -->
                <div v-if="outdated" class="flex items-start gap-2 p-3 bg-amber-500/10 border border-amber-500/20 rounded-md text-sm">
                    <AlertTriangle class="w-4 h-4 text-amber-500 flex-shrink-0 mt-0.5" />
                    <span class="text-amber-700 dark:text-amber-400">This rating was made under a previous rating system.</span>
                </div>

                <!-- Album Art (larger in modal) -->
                <div v-if="coverArtUrl" class="w-full max-w-[250px] mx-auto aspect-square bg-muted rounded-md overflow-hidden">
                    <img
                        :src="coverArtUrl"
                        class="w-full h-full object-cover"
                        :alt="rating.media_title"
                    />
                </div>

                <!-- Rating Breakdown -->
                <div v-if="ratingDetails.length > 0" class="space-y-2">
                    <h4 class="text-sm font-semibold">Rating Breakdown</h4>
                    <div class="p-3 bg-muted/50 rounded-md space-y-2">
                        <div
                            v-for="detail in ratingDetails"
                            :key="detail.rating_detail_id"
                            class="flex justify-between text-sm"
                        >
                            <span class="text-muted-foreground">
                                {{ getParameterInfo(detail.rating_system_parameter_id)?.name || detail.parameter_name || 'Unknown' }}
                                <span v-if="parseFloat(getParameterInfo(detail.rating_system_parameter_id)?.weight || '1') !== 1" class="text-xs">
                                    ({{ getParameterInfo(detail.rating_system_parameter_id)?.weight }}x)
                                </span>
                            </span>
                            <span class="font-mono">
                                {{ formatRatingValue(detail.rating_value) }}
                                <span v-if="getParameterInfo(detail.rating_system_parameter_id)?.parameter_rating_max" class="text-muted-foreground">/ {{ getParameterInfo(detail.rating_system_parameter_id)?.parameter_rating_max }}</span>
                            </span>
                        </div>
                    </div>
                </div>

                <!-- Full Review -->
                <div v-if="rating.comments" class="space-y-2">
                    <h4 class="text-sm font-semibold flex items-center gap-1">
                        <MessageSquare class="w-4 h-4" />
                        Review
                    </h4>
                    <p class="text-sm text-muted-foreground whitespace-pre-wrap">{{ rating.comments }}</p>
                </div>

                <!-- Meta -->
                <div class="flex items-center justify-between text-xs text-muted-foreground pt-2 border-t">
                    <span>by {{ userName }}</span>
                    <span>{{ formatDate(rating.rating_date) }}</span>
                </div>

                <!-- Edit Button (owner only) -->
                <Button
                    v-if="isOwner && group"
                    variant="outline"
                    class="w-full"
                    @click="onEdit"
                >
                    <Pencil class="w-4 h-4 mr-1" />
                    Edit Rating
                </Button>
            </div>
        </DialogScrollContent>
    </Dialog>
</template>
