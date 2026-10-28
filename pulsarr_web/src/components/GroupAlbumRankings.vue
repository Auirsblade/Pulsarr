<script setup lang="ts">
    import { computed, onMounted, ref, watch } from "vue";
    import { storeToRefs } from "pinia";
    import type { GroupAlbumResponse, Rating } from "@/apiClient";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { formatRatingValue } from "@/helpers/ratingFormatters";
    import { useCoverArt } from "@/composables/useCoverArt";
    import { useContextStore } from "@/stores/context.ts";
    import { Card } from "@/components/ui/card";
    import { Button } from "@/components/ui/button";
    import { Input } from "@/components/ui/input";
    import { Label } from "@/components/ui/label";
    import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
    import EmptyState from "@/components/EmptyState.vue";
    import LoadingSpinner from "@/components/LoadingSpinner.vue";
    import { AlertCircle, CalendarX, Disc } from "lucide-vue-next";

    const PAGE_SIZE = 50;

    const props = defineProps<{
        groupId: number;
        ratingMax?: string;
    }>();

    const emit = defineEmits<{
        openAlbum: [rating: Rating];
    }>();

    const { user } = storeToRefs(useContextStore());
    const { coverArtCache, fetchCoverArt } = useCoverArt();

    const albums = ref<GroupAlbumResponse[]>([]);
    const loading = ref(false);
    const error = ref<string | null>(null);
    const fromDate = ref('');
    const toDate = ref('');
    const sortDir = ref('desc');
    const visibleCount = ref(PAGE_SIZE);
    const openingId = ref<string | null>(null);
    const openError = ref<string | null>(null);
    let fetchGeneration = 0;

    // release_date arrives as "YYYY-MM-DDTHH:MM:SS"; its first 10 chars compare correctly as strings.
    const releaseDay = (album: GroupAlbumResponse): string | null =>
        album.release_date ? album.release_date.slice(0, 10) : null;

    const subtitle = (album: GroupAlbumResponse): string =>
        [album.artist_name, releaseDay(album)?.slice(0, 4)].filter(Boolean).join(' · ');

    const hasDateFilter = computed(() => !!fromDate.value || !!toDate.value);

    const filteredAlbums = computed(() => {
        if (!hasDateFilter.value) return albums.value;
        return albums.value.filter(album => {
            const day = releaseDay(album);
            if (!day) return false;
            if (fromDate.value && day < fromDate.value) return false;
            if (toDate.value && day > toDate.value) return false;
            return true;
        });
    });

    const sortedAlbums = computed(() => {
        const direction = sortDir.value === 'asc' ? 1 : -1;
        return [...filteredAlbums.value].sort((a, b) => {
            const diff = parseFloat(a.average_score) - parseFloat(b.average_score);
            if (diff !== 0) return diff * direction;
            if (a.rating_count !== b.rating_count) return b.rating_count - a.rating_count;
            return (a.media_title ?? '').localeCompare(b.media_title ?? '');
        });
    });

    const visibleAlbums = computed(() => sortedAlbums.value.slice(0, visibleCount.value));

    watch([fromDate, toDate, sortDir], () => {
        visibleCount.value = PAGE_SIZE;
    });

    watch(visibleAlbums, (list) => {
        fetchCoverArt(list.map(album => album.musicbrainz_id));
    });

    const clearDates = () => {
        fromDate.value = '';
        toDate.value = '';
    };

    const refresh = () => {
        const gen = ++fetchGeneration;
        loading.value = true;
        error.value = null;
        openError.value = null;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            if (gen !== fetchGeneration) return;
            albums.value = data as GroupAlbumResponse[];
            loading.value = false;
        };
        drh.onErrorCallback = (err) => {
            if (gen !== fetchGeneration) return;
            console.error('Failed to fetch group albums:', err);
            error.value = 'Failed to load albums';
            loading.value = false;
        };
        drh.get(`/rating/group-albums/${props.groupId}`);
    };

    const openAlbum = (album: GroupAlbumResponse) => {
        if (openingId.value) return;
        openingId.value = album.musicbrainz_id;
        openError.value = null;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const ratings = data as Rating[];
            const mine = ratings.find(r => r.pulsarr_user_id === user.value?.pulsarr_user_id);
            const chosen = mine ?? ratings[0];
            if (chosen) emit('openAlbum', chosen);
            else openError.value = 'Could not load ratings for this album';
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to fetch album ratings:', err);
            openError.value = 'Could not load ratings for this album';
        };
        drh.onCompleteCallback = () => {
            openingId.value = null;
        };
        drh.get(`/rating/by-album/${props.groupId}/${encodeURIComponent(album.musicbrainz_id)}`);
    };

    onMounted(refresh);

    watch(() => props.groupId, refresh);

    defineExpose({ refresh });
</script>

<template>
    <div class="space-y-3">
        <div class="flex flex-wrap items-end gap-3">
            <div class="space-y-1">
                <Label for="albums-from" class="text-xs text-muted-foreground">Released from</Label>
                <Input id="albums-from" v-model="fromDate" type="date" class="w-40" :max="toDate || undefined" />
            </div>
            <div class="space-y-1">
                <Label for="albums-to" class="text-xs text-muted-foreground">Released to</Label>
                <Input id="albums-to" v-model="toDate" type="date" class="w-40" :min="fromDate || undefined" />
            </div>
            <Button v-if="hasDateFilter" variant="ghost" size="sm" @click="clearDates">Clear</Button>
            <div class="space-y-1 sm:ml-auto">
                <Label for="albums-sort" class="text-xs text-muted-foreground">Sort</Label>
                <Select v-model="sortDir">
                    <SelectTrigger id="albums-sort" class="w-40">
                        <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="desc">Highest score</SelectItem>
                        <SelectItem value="asc">Lowest score</SelectItem>
                    </SelectContent>
                </Select>
            </div>
        </div>

        <LoadingSpinner v-if="loading && albums.length === 0" text="Loading albums..." />

        <EmptyState
            v-else-if="error"
            :icon="AlertCircle"
            title="Couldn't load albums"
            description="Something went wrong fetching this group's albums."
        >
            <Button variant="outline" @click="refresh">Retry</Button>
        </EmptyState>

        <EmptyState
            v-else-if="albums.length === 0"
            :icon="Disc"
            title="No albums rated yet"
            description="Albums rated under the current rating system will show up here."
        />

        <EmptyState
            v-else-if="sortedAlbums.length === 0"
            :icon="CalendarX"
            title="No albums released in this date range"
        >
            <Button variant="outline" @click="clearDates">Clear dates</Button>
        </EmptyState>

        <template v-else>
            <div class="flex items-center justify-between text-sm">
                <span class="text-muted-foreground">
                    {{ sortedAlbums.length }} {{ sortedAlbums.length === 1 ? 'album' : 'albums' }}
                </span>
                <span v-if="openError" role="alert" class="text-destructive">{{ openError }}</span>
            </div>

            <Card class="!py-0 gap-0 overflow-hidden">
                <button
                    v-for="(album, index) in visibleAlbums"
                    :key="album.musicbrainz_id"
                    type="button"
                    class="flex w-full items-center gap-3 p-3 text-left border-b last:border-b-0 transition-colors hover:bg-muted/50 focus-visible:bg-muted/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-inset"
                    :aria-busy="openingId === album.musicbrainz_id"
                    @click="openAlbum(album)"
                >
                    <span class="w-8 flex-shrink-0 text-center text-sm font-semibold text-muted-foreground tabular-nums">
                        {{ index + 1 }}
                    </span>
                    <span class="w-12 h-12 flex-shrink-0 bg-muted rounded-md flex items-center justify-center overflow-hidden">
                        <img
                            v-if="coverArtCache[album.musicbrainz_id]"
                            :src="coverArtCache[album.musicbrainz_id]"
                            class="w-full h-full object-cover"
                            alt=""
                        />
                        <Disc v-else class="w-5 h-5 text-muted-foreground" />
                    </span>
                    <span class="block flex-1 min-w-0">
                        <span class="block font-semibold truncate" :title="album.media_title ?? undefined">{{ album.media_title }}</span>
                        <span class="block text-sm text-muted-foreground truncate" :title="subtitle(album)">{{ subtitle(album) }}</span>
                    </span>
                    <span class="block flex-shrink-0 text-right">
                        <span class="block font-bold tabular-nums">
                            {{ formatRatingValue(album.average_score) }}<span v-if="ratingMax" class="text-sm font-normal text-muted-foreground">/{{ formatRatingValue(ratingMax) }}</span>
                        </span>
                        <span class="block text-xs text-muted-foreground">
                            {{ album.rating_count }} {{ album.rating_count === 1 ? 'rating' : 'ratings' }}
                        </span>
                    </span>
                </button>
            </Card>

            <div v-if="visibleCount < sortedAlbums.length" class="flex justify-center">
                <Button variant="outline" @click="visibleCount += PAGE_SIZE">Show more</Button>
            </div>
        </template>
    </div>
</template>
