<script setup lang="ts">
    import type { Rating } from "@/apiClient";
    import { Star, Disc, Music, MessageSquare, AlertTriangle, Users } from "lucide-vue-next";
    import { Card } from "@/components/ui/card";
    import { formatDate, formatRatingValue } from "@/helpers/ratingFormatters";
    import { RouterLink } from "vue-router";

    defineProps<{
        rating: Rating;
        coverArtUrl?: string;
        userName: string;
        groupName?: string;
        outdated?: boolean;
        groupAverage?: string;
        groupRatingCount?: number;
        ratingMax?: string;
    }>();

    defineEmits<{
        click: [];
        groupAverageClick: [];
    }>();
</script>

<template>
    <Card
        class="cursor-pointer transition-colors hover:bg-muted/50 !py-0 focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none rounded-md"
        tabindex="0"
        role="button"
        @click="$emit('click')"
        @keydown.enter="$emit('click')"
        @keydown.space.prevent="$emit('click')"
    >
        <div class="flex p-3 gap-3">
            <!-- Album Art + User Score -->
            <div class="flex flex-col items-center justify-between gap-1 flex-shrink-0">
                <div class="w-20 h-20 bg-muted rounded-md flex items-center justify-center overflow-hidden">
                    <img
                        v-if="coverArtUrl"
                        :src="coverArtUrl"
                        class="w-full h-full object-cover"
                        :alt="'Album cover for ' + rating.media_title"
                    />
                    <Disc v-else-if="rating.media_type === 'album'" class="w-8 h-8 text-muted-foreground" />
                    <Music v-else class="w-8 h-8 text-muted-foreground" />
                </div>
                <span v-if="groupAverage" class="inline-flex items-center gap-0.5 px-1.5 py-0.5 bg-muted rounded text-xs font-medium text-foreground">
                    <Star class="w-2.5 h-2.5" />
                    {{ formatRatingValue(rating.rating_value) }}<span v-if="ratingMax" class="text-muted-foreground">/{{ formatRatingValue(ratingMax) }}</span>
                </span>
            </div>

            <!-- Rating Content -->
            <div class="flex-1 min-w-0 flex flex-col">
                <div class="flex justify-between items-start gap-2">
                    <div class="min-w-0">
                        <h3 class="font-semibold truncate">{{ rating.media_title }}</h3>
                        <p class="text-sm text-muted-foreground truncate">{{ rating.artist_name }}</p>
                    </div>
                    <div class="flex items-center gap-1.5 flex-shrink-0">
                        <RouterLink v-if="groupName" :to="`/group/${rating.pulsarr_group_id}`" class="px-1.5 py-0.5 bg-secondary rounded text-xs text-muted-foreground max-w-[160px] truncate hover:bg-secondary/80 transition-colors" :title="groupName" @click.stop>{{ groupName }}</RouterLink>
                        <span v-if="outdated" class="flex items-center gap-1 px-1.5 py-0.5 bg-amber-500/15 text-amber-600 dark:text-amber-400 rounded text-xs font-medium">
                            <AlertTriangle class="w-3 h-3" />
                            Outdated
                        </span>
                        <!-- Group Average Badge (clickable) -->
                        <button
                            v-if="groupAverage"
                            class="flex items-center gap-1 px-2 py-1 bg-primary text-primary-foreground rounded font-bold text-sm hover:bg-primary/90 transition-colors"
                            @click.stop="$emit('groupAverageClick')"
                            :title="`Group average: ${formatRatingValue(groupAverage)}/${ratingMax ? formatRatingValue(ratingMax) : ''} (${groupRatingCount} ratings)`"
                        >
                            <Users class="w-3 h-3" />
                            {{ formatRatingValue(groupAverage) }}<span v-if="ratingMax" class="font-normal text-primary-foreground/70">/{{ formatRatingValue(ratingMax) }}</span>
                            <span class="text-xs font-normal text-primary-foreground/70">({{ groupRatingCount }})</span>
                        </button>
                        <!-- User score badge (when no group average, keep original behavior) -->
                        <div v-else class="flex items-center gap-1 px-2 py-1 bg-primary text-primary-foreground rounded font-bold text-sm">
                            <Star class="w-3 h-3" />
                            {{ formatRatingValue(rating.rating_value) }}<span v-if="ratingMax" class="font-normal text-primary-foreground/70">/{{ formatRatingValue(ratingMax) }}</span>
                        </div>
                    </div>
                </div>

                <p v-if="rating.comments" class="mt-2 text-sm text-muted-foreground line-clamp-2">
                    <MessageSquare class="inline w-3 h-3 mr-1" />
                    {{ rating.comments }}
                </p>

                <div class="mt-auto pt-2 flex items-center justify-between text-xs text-muted-foreground">
                    <span>by <RouterLink :to="`/user/${rating.pulsarr_user_id}`" class="hover:underline" @click.stop>{{ userName }}</RouterLink></span>
                    <span>{{ formatDate(rating.rating_date) }}</span>
                </div>
            </div>
        </div>
    </Card>
</template>
