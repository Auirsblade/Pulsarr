<script setup lang="ts">
    import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
    import { Input } from "@/components/ui/input";
    import { ref, watch } from "vue";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { Search, Disc, Loader2 } from "lucide-vue-next";
    import EmptyState from "@/components/EmptyState.vue";

    interface ArtistCredit {
        name?: string;
        artist?: {
            id: string;
            name: string;
        };
    }

    interface ReleaseGroup {
        id: string;
        title?: string;
        'primary-type'?: string;
        'first-release-date'?: string;
        'artist-credit'?: ArtistCredit[];
    }

    interface CoverArtInfo {
        front?: string;
        front_thumbnail_small?: string;
        front_thumbnail_large?: string;
    }

    export interface SelectedMusic {
        musicbrainz_id: string;
        media_type: string;
        media_title: string;
        artist_name: string;
        release_date?: string;
        cover_art_url?: string;
    }

    const props = defineProps({
        showDialog: {
            type: Boolean,
            required: true,
        }
    });

    const emit = defineEmits<{
        'update:showDialog': [value: boolean];
        'select': [music: SelectedMusic];
    }>();

    const searchQuery = ref('');
    const searching = ref(false);
    const results = ref<ReleaseGroup[]>([]);
    const coverArtCache = ref<Record<string, string>>({});

    let searchTimeout: ReturnType<typeof setTimeout> | null = null;

    const getArtistName = (credits?: ArtistCredit[]): string => {
        if (!credits || credits.length === 0) return 'Unknown Artist';
        return credits.map(c => c.name || c.artist?.name || '').filter(n => n).join(', ');
    };

    const search = () => {
        if (!searchQuery.value || searchQuery.value.length < 2) {
            results.value = [];
            return;
        }

        searching.value = true;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data: any) => {
            results.value = data['release-groups'] || [];
            // Fetch cover art for release groups
            (data['release-groups'] || []).forEach((releaseGroup: ReleaseGroup) => {
                fetchCoverArt(releaseGroup.id);
            });
            searching.value = false;
        };
        drh.onErrorCallback = (error) => {
            console.error('Search failed:', error);
            searching.value = false;
            results.value = [];
        };
        drh.get(`/musicbrainz/search/release-group?query=${encodeURIComponent(searchQuery.value)}&limit=10`);
    };

    const fetchCoverArt = (releaseGroupId: string) => {
        if (coverArtCache.value[releaseGroupId]) return;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            let coverArtInfo = data as CoverArtInfo
            if (coverArtInfo.front_thumbnail_small || coverArtInfo.front) {
                coverArtCache.value[releaseGroupId] = coverArtInfo.front_thumbnail_small || coverArtInfo.front || '';
            }
        };
        drh.onErrorCallback = () => {
            // Cover art not available, ignore
        };
        drh.get(`/musicbrainz/release-group/${releaseGroupId}/cover-art-info`);
    };

    // Debounced search
    watch(searchQuery, () => {
        if (searchTimeout) {
            clearTimeout(searchTimeout);
        }
        searchTimeout = setTimeout(search, 300);
    });

    const selectItem = (releaseGroup: ReleaseGroup) => {
        const selected: SelectedMusic = {
            musicbrainz_id: releaseGroup.id,
            media_type: 'album',
            media_title: releaseGroup.title || 'Unknown Album',
            artist_name: getArtistName(releaseGroup['artist-credit']),
            release_date: releaseGroup['first-release-date'],
            cover_art_url: coverArtCache.value[releaseGroup.id],
        };

        emit('select', selected);
        emit('update:showDialog', false);
        searchQuery.value = '';
        results.value = [];
    };

    const closeDialog = () => {
        emit('update:showDialog', false);
        searchQuery.value = '';
        results.value = [];
    };
</script>

<template>
    <Dialog :open="showDialog" @update:open="closeDialog">
        <DialogTrigger asChild>
            <slot name="openModal"></slot>
        </DialogTrigger>
        <DialogContent class="max-w-2xl max-h-[80vh] flex flex-col">
            <DialogHeader>
                <DialogTitle>Search Albums</DialogTitle>
                <DialogDescription>
                    Search for albums to review
                </DialogDescription>
            </DialogHeader>

            <div class="space-y-4">
                <div class="relative">
                    <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
                    <Input
                        v-model="searchQuery"
                        placeholder="Search for an album..."
                        class="pl-9"
                    />
                </div>

                <div class="overflow-y-auto flex-1 min-h-0 space-y-2">
                    <div v-if="searching" class="flex justify-center py-8">
                        <Loader2 class="w-6 h-6 animate-spin text-muted-foreground" />
                    </div>

                    <EmptyState
                        v-else-if="results.length === 0 && searchQuery.length >= 2"
                        :icon="Search"
                        title="No results found"
                        description="Try a different search term or check the spelling"
                    />

                    <EmptyState
                        v-else-if="results.length === 0"
                        :icon="Search"
                        title="Search for music"
                        description="Type an artist or album name to get started"
                    />

                    <!-- Release Group Results -->
                    <template v-else>
                        <div
                            v-for="releaseGroup in results"
                            :key="releaseGroup.id"
                            class="flex items-center gap-3 p-3 rounded-lg border hover:bg-accent cursor-pointer transition-colors focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
                            tabindex="0"
                            role="button"
                            @click="selectItem(releaseGroup)"
                            @keydown.enter="selectItem(releaseGroup)"
                            @keydown.space.prevent="selectItem(releaseGroup)"
                        >
                            <div class="w-12 h-12 bg-muted rounded flex items-center justify-center overflow-hidden">
                                <img
                                    v-if="coverArtCache[releaseGroup.id]"
                                    :src="coverArtCache[releaseGroup.id]"
                                    class="w-full h-full object-cover"
                                    :alt="'Cover art for ' + releaseGroup.title"
                                />
                                <Disc v-else class="w-6 h-6 text-muted-foreground" />
                            </div>
                            <div class="flex-1 min-w-0">
                                <div class="font-medium truncate">{{ releaseGroup.title }}</div>
                                <div class="text-sm text-muted-foreground truncate">
                                    {{ getArtistName(releaseGroup['artist-credit']) }}
                                </div>
                                <div class="text-xs text-muted-foreground flex gap-2">
                                    <span v-if="releaseGroup['primary-type']">{{ releaseGroup['primary-type'] }}</span>
                                    <span v-if="releaseGroup['first-release-date']">{{ releaseGroup['first-release-date'] }}</span>
                                </div>
                            </div>
                        </div>
                    </template>
                </div>
            </div>
        </DialogContent>
    </Dialog>
</template>
