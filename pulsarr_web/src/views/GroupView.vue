<script setup lang="ts">
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
    import { Button } from "@/components/ui/button";
    import { onMounted, ref, computed } from "vue";
    import { useRoute, useRouter } from "vue-router";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { GroupDTO } from "@/apiClient";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import { Copy, Check, LogOut, Plus, Users, Star, Disc, Music, MessageSquare } from "lucide-vue-next";
    import AddRatingModal from "@/components/modals/AddRatingModal.vue";

    interface Rating {
        rating_id: number;
        pulsarr_user_id: number;
        pulsarr_group_id: number;
        rating_system_id: number;
        comments: string;
        rating_value: string;
        media_type: string;
        media_title: string;
        musicbrainz_id: string;
        artist_name: string;
        rating_date: string;
        release_date: string;
    }

    interface CoverArtInfo {
        front?: string;
        front_thumbnail_small?: string;
        front_thumbnail_large?: string;
    }

    const route = useRoute();
    const router = useRouter();
    const { user } = storeToRefs(useContextStore());

    const group = ref<GroupDTO | null>(null);
    const ratings = ref<Rating[]>([]);
    const coverArtCache = ref<Record<string, string>>({});
    const loading = ref(true);
    const error = ref<string | null>(null);
    const copied = ref(false);
    const showAddRatingModal = ref(false);

    const groupId = computed(() => Number(route.params.groupId));

    const shareLink = computed(() => {
        if (!group.value) return '';
        return `${window.location.origin}/join/${group.value.pulsarr_group_id}`;
    });

    const isMember = computed(() => {
        if (!group.value?.members || !user.value) return false;
        return group.value.members.some(m => m.user?.pulsarr_user_id === user.value?.pulsarr_user_id);
    });

    const isOwner = computed(() => {
        if (!group.value?.members || !user.value) return false;
        const membership = group.value.members.find(m => m.user?.pulsarr_user_id === user.value?.pulsarr_user_id);
        return membership?.group_role === 'OWNER';
    });

    const getUserName = (userId: number): string => {
        const member = group.value?.members?.find(m => m.user?.pulsarr_user_id === userId);
        return member?.user?.name || 'Unknown User';
    };

    onMounted(() => {
        fetchGroup();
    });

    const fetchGroup = () => {
        loading.value = true;
        error.value = null;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            group.value = data as GroupDTO;
            loading.value = false;
            fetchRatings();
        };
        drh.onErrorCallback = (err) => {
            error.value = 'Failed to load group';
            loading.value = false;
            console.error('Failed to fetch group:', err);
        };
        drh.get(`/group/${groupId.value}`);
    };

    const fetchRatings = () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            ratings.value = data as Rating[];
            // Fetch cover art for releases
            ratings.value.forEach(rating => {
                if (rating.media_type === 'release' && rating.musicbrainz_id) {
                    fetchCoverArt(rating.musicbrainz_id);
                }
            });
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to fetch ratings:', err);
        };
        drh.post('/rating/byGroups', [groupId.value]);
    };

    const fetchCoverArt = (releaseId: string) => {
        if (coverArtCache.value[releaseId]) return;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const coverArt = data as CoverArtInfo;
            if (coverArt.front_thumbnail_small || coverArt.front) {
                coverArtCache.value[releaseId] = coverArt.front_thumbnail_small || coverArt.front || '';
            }
        };
        drh.onErrorCallback = () => {
            // Cover art not available, ignore
        };
        drh.get(`/musicbrainz/release/${releaseId}/cover-art-info`);
    };

    const copyShareLink = async () => {
        try {
            await navigator.clipboard.writeText(shareLink.value);
            copied.value = true;
            setTimeout(() => {
                copied.value = false;
            }, 2000);
        } catch (err) {
            console.error('Failed to copy:', err);
        }
    };

    const leaveGroup = () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            router.push('/');
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to leave group:', err);
        };
        drh.post(`/group/leave/${groupId.value}`, {});
    };

    const onRatingCreated = () => {
        fetchRatings();
    };

    const formatDate = (dateStr: string): string => {
        const date = new Date(dateStr);
        return date.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
    };

    const formatRatingValue = (value: string): string => {
        const num = parseFloat(value);
        if (isNaN(num)) return value;
        // parseFloat + toString automatically trims trailing zeros
        return num.toString();
    };
</script>

<template>
    <div class="p-4 max-w-4xl mx-auto">
        <div v-if="loading" class="flex justify-center py-8">
            <span class="text-muted-foreground">Loading group...</span>
        </div>

        <div v-else-if="error" class="flex justify-center py-8">
            <span class="text-destructive">{{ error }}</span>
        </div>

        <div v-else-if="group" class="space-y-4">
            <!-- Main Group Card -->
            <Card>
                <CardHeader>
                    <div class="flex justify-between items-start">
                        <div>
                            <CardTitle class="text-2xl">{{ group.name }}</CardTitle>
                            <CardDescription class="flex items-center gap-2 mt-1">
                                <span class="px-2 py-0.5 bg-secondary rounded text-xs">{{ group.privacy_type }}</span>
                            </CardDescription>
                        </div>
                        <div class="flex gap-2">
                            <AddRatingModal
                                v-if="isMember && group.rating_system"
                                :show-dialog="showAddRatingModal"
                                :group="group"
                                @update:showDialog="showAddRatingModal = $event"
                                @created="onRatingCreated"
                            >
                                <template #openModal>
                                    <Button @click="showAddRatingModal = true">
                                        <Plus class="w-4 h-4 mr-1" />
                                        Add Rating
                                    </Button>
                                </template>
                            </AddRatingModal>
                            <Button v-if="isMember && !isOwner" variant="outline" @click="leaveGroup">
                                <LogOut class="w-4 h-4 mr-1" />
                                Leave
                            </Button>
                        </div>
                    </div>
                </CardHeader>
                <CardContent class="space-y-4">
                    <!-- Share Link -->
                    <div class="flex items-center gap-2">
                        <span class="text-sm text-muted-foreground">Share link:</span>
                        <code class="flex-1 px-2 py-1 bg-muted rounded text-sm truncate">{{ shareLink }}</code>
                        <Button variant="outline" size="icon" @click="copyShareLink">
                            <Check v-if="copied" class="w-4 h-4 text-green-500" />
                            <Copy v-else class="w-4 h-4" />
                        </Button>
                    </div>

                    <!-- Rating System and Members - Side by side on large screens -->
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                        <!-- Rating System Info -->
                        <div v-if="group.rating_system" class="p-3 bg-muted/50 rounded-lg">
                            <div class="flex items-center gap-2 text-sm font-semibold mb-2">
                                <Star class="w-4 h-4" />
                                Rating System
                            </div>
                            <div class="text-sm">
                                <span class="font-medium">{{ group.rating_system.name }}</span>
                                <span class="text-muted-foreground ml-2">
                                    {{ group.rating_system.master_rating_type }} · max {{ group.rating_system.rating_max }}
                                </span>
                            </div>
                            <div v-if="group.rating_system.parameters && group.rating_system.parameters.length > 0" class="mt-2 flex flex-wrap gap-1">
                                <span
                                    v-for="param in group.rating_system.parameters"
                                    :key="param.rating_system_parameter_id"
                                    class="px-2 py-0.5 bg-secondary rounded text-xs"
                                >
                                    {{ param.name }}
                                </span>
                            </div>
                        </div>

                        <!-- Members -->
                        <div class="p-3 bg-muted/50 rounded-lg">
                            <div class="flex items-center gap-2 text-sm font-semibold mb-2">
                                <Users class="w-4 h-4" />
                                Members ({{ group.members?.length ?? 0 }})
                            </div>
                            <ul v-if="group.members && group.members.length > 0" class="space-y-1">
                                <li
                                    v-for="member in group.members"
                                    :key="member.user?.pulsarr_user_id"
                                    class="text-sm flex items-center gap-2"
                                >
                                    <span class="w-2 h-2 rounded-full bg-primary"></span>
                                    {{ member.user?.name }}
                                    <span v-if="member.group_role === 'OWNER'" class="text-xs text-muted-foreground">(owner)</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                </CardContent>
            </Card>

            <!-- Ratings Section -->
            <div class="space-y-3">
                <h2 class="text-lg font-semibold flex items-center gap-2">
                    <Star class="w-5 h-5" />
                    Ratings ({{ ratings.length }})
                </h2>

                <div v-if="ratings.length === 0" class="text-center py-8 text-muted-foreground">
                    No ratings yet. Be the first to add one!
                </div>

                <Card v-for="rating in ratings" :key="rating.rating_id">
                    <div class="flex px-3 gap-3">
                        <!-- Album Art -->
                        <div class="w-20 h-20 flex-shrink-0 bg-muted rounded-md flex items-center justify-center overflow-hidden">
                            <img
                                v-if="coverArtCache[rating.musicbrainz_id]"
                                :src="coverArtCache[rating.musicbrainz_id]"
                                class="w-full h-full object-cover"
                                alt=""
                            />
                            <Disc v-else-if="rating.media_type === 'release'" class="w-8 h-8 text-muted-foreground" />
                            <Music v-else class="w-8 h-8 text-muted-foreground" />
                        </div>

                        <!-- Rating Content -->
                        <div class="flex-1 min-w-0">
                            <div class="flex justify-between items-start gap-2">
                                <div class="min-w-0">
                                    <h3 class="font-semibold truncate">{{ rating.media_title }}</h3>
                                    <p class="text-sm text-muted-foreground truncate">{{ rating.artist_name }}</p>
                                </div>
                                <div class="flex items-center gap-1 px-2 py-1 bg-primary text-primary-foreground rounded font-bold text-sm flex-shrink-0">
                                    <Star class="w-3 h-3" />
                                    {{ formatRatingValue(rating.rating_value) }}
                                </div>
                            </div>

                            <p v-if="rating.comments" class="mt-2 text-sm text-muted-foreground line-clamp-2">
                                <MessageSquare class="inline w-3 h-3 mr-1" />
                                {{ rating.comments }}
                            </p>

                            <div class="mt-2 flex items-center justify-between text-xs text-muted-foreground">
                                <span>by {{ getUserName(rating.pulsarr_user_id) }}</span>
                                <span>{{ formatDate(rating.rating_date) }}</span>
                            </div>
                        </div>
                    </div>
                </Card>
            </div>
        </div>
    </div>
</template>
