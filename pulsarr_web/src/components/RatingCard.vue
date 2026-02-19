<script setup lang="ts">
    import type { Rating } from "@/apiClient";
    import { Star, Disc, Music, MessageSquare } from "lucide-vue-next";
    import { Card } from "@/components/ui/card";
    import { formatDate, formatRatingValue } from "@/helpers/ratingFormatters";

    defineProps<{
        rating: Rating;
        coverArtUrl?: string;
        userName: string;
        groupName?: string;
    }>();

    defineEmits<{
        click: [];
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
            <!-- Album Art -->
            <div class="w-20 h-20 flex-shrink-0 bg-muted rounded-md flex items-center justify-center overflow-hidden">
                <img
                    v-if="coverArtUrl"
                    :src="coverArtUrl"
                    class="w-full h-full object-cover"
                    :alt="'Album cover for ' + rating.media_title"
                />
                <Disc v-else-if="rating.media_type === 'album'" class="w-8 h-8 text-muted-foreground" />
                <Music v-else class="w-8 h-8 text-muted-foreground" />
            </div>

            <!-- Rating Content -->
            <div class="flex-1 min-w-0">
                <div class="flex justify-between items-start gap-2">
                    <div class="min-w-0">
                        <h3 class="font-semibold truncate">{{ rating.media_title }}</h3>
                        <p class="text-sm text-muted-foreground truncate">{{ rating.artist_name }}</p>
                    </div>
                    <div class="flex items-center gap-1.5 flex-shrink-0">
                        <span v-if="groupName" class="px-1.5 py-0.5 bg-secondary rounded text-xs text-muted-foreground max-w-[100px] truncate">{{ groupName }}</span>
                        <div class="flex items-center gap-1 px-2 py-1 bg-primary text-primary-foreground rounded font-bold text-sm">
                            <Star class="w-3 h-3" />
                            {{ formatRatingValue(rating.rating_value) }}
                        </div>
                    </div>
                </div>

                <p v-if="rating.comments" class="mt-2 text-sm text-muted-foreground line-clamp-2">
                    <MessageSquare class="inline w-3 h-3 mr-1" />
                    {{ rating.comments }}
                </p>

                <div class="mt-2 flex items-center justify-between text-xs text-muted-foreground">
                    <span>by {{ userName }}</span>
                    <span>{{ formatDate(rating.rating_date) }}</span>
                </div>
            </div>
        </div>
    </Card>
</template>
