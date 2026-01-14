<script setup lang="ts">
    import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
    import { Button } from "@/components/ui/button";
    import { Label } from "@/components/ui/label";
    import { Input } from "@/components/ui/input";
    import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
    import { ref, watch } from "vue";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { Search, Music, Disc, User, Loader2 } from "lucide-vue-next";

    interface ArtistCredit {
        name?: string;
        artist?: {
            id: string;
            name: string;
        };
    }

    interface Artist {
        id: string;
        name: string;
        disambiguation?: string;
        country?: string;
    }

    interface Release {
        id: string;
        title: string;
        date?: string;
        'artist-credit'?: ArtistCredit[];
        country?: string;
    }

    interface Recording {
        id: string;
        title: string;
        length?: number;
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
    const searchType = ref<'release' | 'recording' | 'artist'>('release');
    const searching = ref(false);
    const results = ref<(Artist | Release | Recording)[]>([]);
    const coverArtCache = ref<Record<string, string>>({});

    let searchTimeout: ReturnType<typeof setTimeout> | null = null;

    const formatDuration = (ms?: number): string => {
        if (!ms) return '';
        const minutes = Math.floor(ms / 60000);
        const seconds = Math.floor((ms % 60000) / 1000);
        return `${minutes}:${seconds.toString().padStart(2, '0')}`;
    };

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
            if (searchType.value === 'artist') {
                results.value = data.artists || [];
            } else if (searchType.value === 'release') {
                results.value = data.releases || [];
                // Fetch cover art for releases
                (data.releases || []).forEach((release: Release) => {
                    fetchCoverArt(release.id);
                });
            } else if (searchType.value === 'recording') {
                results.value = data.recordings || [];
            }
            searching.value = false;
        };
        drh.onErrorCallback = (error) => {
            console.error('Search failed:', error);
            searching.value = false;
            results.value = [];
        };
        drh.get(`/musicbrainz/search/${searchType.value}?query=${encodeURIComponent(searchQuery.value)}&limit=10`);
    };

    const fetchCoverArt = (releaseId: string) => {
        if (coverArtCache.value[releaseId]) return;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data: CoverArtInfo) => {
            if (data.front_thumbnail_small || data.front) {
                coverArtCache.value[releaseId] = data.front_thumbnail_small || data.front || '';
            }
        };
        drh.onErrorCallback = () => {
            // Cover art not available, ignore
        };
        drh.get(`/musicbrainz/release/${releaseId}/cover-art-info`);
    };

    // Debounced search
    watch(searchQuery, () => {
        if (searchTimeout) {
            clearTimeout(searchTimeout);
        }
        searchTimeout = setTimeout(search, 300);
    });

    watch(searchType, () => {
        results.value = [];
        if (searchQuery.value) {
            search();
        }
    });

    const selectItem = (item: Artist | Release | Recording) => {
        let selected: SelectedMusic;

        if (searchType.value === 'artist') {
            const artist = item as Artist;
            selected = {
                musicbrainz_id: artist.id,
                media_type: 'artist',
                media_title: artist.name,
                artist_name: artist.name,
            };
        } else if (searchType.value === 'release') {
            const release = item as Release;
            selected = {
                musicbrainz_id: release.id,
                media_type: 'release',
                media_title: release.title,
                artist_name: getArtistName(release['artist-credit']),
                release_date: release.date,
                cover_art_url: coverArtCache.value[release.id],
            };
        } else {
            const recording = item as Recording;
            selected = {
                musicbrainz_id: recording.id,
                media_type: 'recording',
                media_title: recording.title,
                artist_name: getArtistName(recording['artist-credit']),
            };
        }

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
                <DialogTitle>Search Music</DialogTitle>
                <DialogDescription>
                    Search MusicBrainz for artists, albums, or tracks to review
                </DialogDescription>
            </DialogHeader>

            <div class="space-y-4">
                <div class="flex gap-2">
                    <Select v-model="searchType">
                        <SelectTrigger class="w-32">
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="release">
                                <div class="flex items-center gap-2">
                                    <Disc class="w-4 h-4" />
                                    Album
                                </div>
                            </SelectItem>
                            <SelectItem value="recording">
                                <div class="flex items-center gap-2">
                                    <Music class="w-4 h-4" />
                                    Track
                                </div>
                            </SelectItem>
                            <SelectItem value="artist">
                                <div class="flex items-center gap-2">
                                    <User class="w-4 h-4" />
                                    Artist
                                </div>
                            </SelectItem>
                        </SelectContent>
                    </Select>
                    <div class="relative flex-1">
                        <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
                        <Input
                            v-model="searchQuery"
                            placeholder="Search..."
                            class="pl-9"
                        />
                    </div>
                </div>

                <div class="overflow-y-auto max-h-[400px] space-y-2">
                    <div v-if="searching" class="flex justify-center py-8">
                        <Loader2 class="w-6 h-6 animate-spin text-muted-foreground" />
                    </div>

                    <div v-else-if="results.length === 0 && searchQuery.length >= 2" class="text-center py-8 text-muted-foreground">
                        No results found
                    </div>

                    <div v-else-if="results.length === 0" class="text-center py-8 text-muted-foreground">
                        Type to search...
                    </div>

                    <!-- Artist Results -->
                    <template v-else-if="searchType === 'artist'">
                        <div
                            v-for="artist in (results as Artist[])"
                            :key="artist.id"
                            class="flex items-center gap-3 p-3 rounded-lg border hover:bg-accent cursor-pointer transition-colors"
                            @click="selectItem(artist)"
                        >
                            <div class="w-12 h-12 bg-muted rounded flex items-center justify-center">
                                <User class="w-6 h-6 text-muted-foreground" />
                            </div>
                            <div class="flex-1 min-w-0">
                                <div class="font-medium truncate">{{ artist.name }}</div>
                                <div class="text-sm text-muted-foreground truncate">
                                    {{ artist.disambiguation || artist.country || 'Artist' }}
                                </div>
                            </div>
                        </div>
                    </template>

                    <!-- Release Results -->
                    <template v-else-if="searchType === 'release'">
                        <div
                            v-for="release in (results as Release[])"
                            :key="release.id"
                            class="flex items-center gap-3 p-3 rounded-lg border hover:bg-accent cursor-pointer transition-colors"
                            @click="selectItem(release)"
                        >
                            <div class="w-12 h-12 bg-muted rounded flex items-center justify-center overflow-hidden">
                                <img
                                    v-if="coverArtCache[release.id]"
                                    :src="coverArtCache[release.id]"
                                    class="w-full h-full object-cover"
                                    alt=""
                                />
                                <Disc v-else class="w-6 h-6 text-muted-foreground" />
                            </div>
                            <div class="flex-1 min-w-0">
                                <div class="font-medium truncate">{{ release.title }}</div>
                                <div class="text-sm text-muted-foreground truncate">
                                    {{ getArtistName(release['artist-credit']) }}
                                </div>
                                <div v-if="release.date" class="text-xs text-muted-foreground">
                                    {{ release.date }}
                                </div>
                            </div>
                        </div>
                    </template>

                    <!-- Recording Results -->
                    <template v-else-if="searchType === 'recording'">
                        <div
                            v-for="recording in (results as Recording[])"
                            :key="recording.id"
                            class="flex items-center gap-3 p-3 rounded-lg border hover:bg-accent cursor-pointer transition-colors"
                            @click="selectItem(recording)"
                        >
                            <div class="w-12 h-12 bg-muted rounded flex items-center justify-center">
                                <Music class="w-6 h-6 text-muted-foreground" />
                            </div>
                            <div class="flex-1 min-w-0">
                                <div class="font-medium truncate">{{ recording.title }}</div>
                                <div class="text-sm text-muted-foreground truncate">
                                    {{ getArtistName(recording['artist-credit']) }}
                                </div>
                            </div>
                            <div v-if="recording.length" class="text-sm text-muted-foreground">
                                {{ formatDuration(recording.length) }}
                            </div>
                        </div>
                    </template>
                </div>
            </div>
        </DialogContent>
    </Dialog>
</template>
