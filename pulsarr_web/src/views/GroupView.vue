<script setup lang="ts">
    import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
    import { Button } from "@/components/ui/button";
    import { Dialog, DialogScrollContent, DialogHeader, DialogTitle, DialogDescription } from "@/components/ui/dialog";
    import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/components/ui/alert-dialog";
    import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";
    import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
    import { Input } from "@/components/ui/input";
    import { Label } from "@/components/ui/label";
    import { onMounted, onUnmounted, ref, computed, watch } from "vue";
    import { useRoute, useRouter } from "vue-router";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { toast } from 'vue-sonner';
    import type { GroupDTO, Rating } from "@/apiClient";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import { Copy, Check, LogOut, Plus, Users, Star, Settings, MoreHorizontal, Shield, UserMinus, Crown, RefreshCw, UserPlus } from "lucide-vue-next";
    import AddRatingModal from "@/components/modals/AddRatingModal.vue";
    import ChangeRatingSystemModal from "@/components/modals/ChangeRatingSystemModal.vue";
    import RatingCard from "@/components/RatingCard.vue";
    import RatingDetailModal from "@/components/RatingDetailModal.vue";
    import EmptyState from "@/components/EmptyState.vue";
    import LoadingSpinner from "@/components/LoadingSpinner.vue";

    interface CoverArtInfo {
        front?: string;
        front_thumbnail_small?: string;
        front_thumbnail_large?: string;
    }

    const ROLE_LEVELS: Record<string, number> = {
        'Owner': 4,
        'Admin': 3,
        'Member': 2,
        'ViewOnly': 1,
    };

    const route = useRoute();
    const router = useRouter();
    const contextStore = useContextStore();
    const { user, privacyTypes } = storeToRefs(contextStore);

    const PAGE_SIZE = 20;

    const group = ref<GroupDTO | null>(null);
    const ratings = ref<Rating[]>([]);
    const coverArtCache = ref<Record<string, string>>({});
    const loading = ref(true);
    const error = ref<string | null>(null);
    const copied = ref(false);
    const showAddRatingModal = ref(false);
    const loadingMoreRatings = ref(false);
    const hasMoreRatings = ref(true);
    const ratingSentinel = ref<HTMLElement | null>(null);
    let ratingsObserver: IntersectionObserver | null = null;
    let fetchGeneration = 0;

    // Rating detail modal
    const selectedRating = ref<Rating | null>(null);
    const showRatingModal = ref(false);
    const ratingModalInitialTab = ref<string | undefined>(undefined);

    // Group averages cache
    const groupAverages = ref<Record<string, { average: string; count: number }>>({});

    // Edit rating state
    const editingRating = ref<Rating | null>(null);

    // Rate-this state (pre-select album from another user's review)
    const rateThisRating = ref<Rating | null>(null);

    // Settings state
    const showSettingsDialog = ref(false);
    const editName = ref('');
    const editPrivacyType = ref('');
    const settingsSaving = ref(false);

    // Delete confirmation state
    const showDeleteConfirm = ref(false);
    const deleteConfirmText = ref('');

    // Change rating system state
    const showChangeRatingSystemModal = ref(false);

    // Transfer ownership state
    const showTransferDialog = ref(false);
    const transferTargetId = ref<number | null>(null);
    const transferTargetName = ref('');

    const openRatingModal = (rating: Rating) => {
        ratingModalInitialTab.value = undefined;
        selectedRating.value = rating;
        showRatingModal.value = true;
    };

    const openRatingModalGroupTab = (rating: Rating) => {
        ratingModalInitialTab.value = 'group-ratings';
        selectedRating.value = rating;
        showRatingModal.value = true;
    };

    const ratingMax = computed(() => group.value?.rating_system?.rating_max?.toString());

    const fetchGroupAverages = (ratingsToFetch: Rating[]) => {
        const uniqueKeys = new Map<string, { group_id: number; musicbrainz_id: string }>();
        for (const r of ratingsToFetch) {
            const key = `${r.pulsarr_group_id}:${r.musicbrainz_id}`;
            if (!uniqueKeys.has(key) && !groupAverages.value[key]) {
                uniqueKeys.set(key, { group_id: r.pulsarr_group_id, musicbrainz_id: r.musicbrainz_id });
            }
        }
        const items = Array.from(uniqueKeys.values());
        if (items.length === 0) return;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const results = data as { pulsarr_group_id: number; musicbrainz_id: string; average_score: string; rating_count: number }[];
            for (const r of results) {
                const key = `${r.pulsarr_group_id}:${r.musicbrainz_id}`;
                const num = parseFloat(r.average_score);
                groupAverages.value[key] = {
                    average: isNaN(num) ? r.average_score : parseFloat(num.toFixed(2)).toString(),
                    count: r.rating_count,
                };
            }
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to fetch group averages:', err);
        };
        drh.post('/rating/group-averages', { items });
    };

    const groupId = computed(() => Number(route.params.groupId));

    const shareLink = computed(() => {
        if (!group.value) return '';
        return `${window.location.origin}/join/${group.value.pulsarr_group_id}`;
    });

    const currentUserMembership = computed(() => {
        if (!group.value?.members || !user.value) return null;
        return group.value.members.find(m => m.user?.pulsarr_user_id === user.value?.pulsarr_user_id) ?? null;
    });

    const currentUserRole = computed(() => currentUserMembership.value?.group_role ?? '');

    const isMember = computed(() => !!currentUserMembership.value);

    const isOwner = computed(() => currentUserRole.value === 'Owner');

    const isAdmin = computed(() => currentUserRole.value === 'Admin');

    const isAdminOrOwner = computed(() => isOwner.value || isAdmin.value);

    const isCrate = computed(() => group.value?.privacy_type === 'Personal');

    const canActOn = (memberRole: string): boolean => {
        return (ROLE_LEVELS[currentUserRole.value] ?? 0) > (ROLE_LEVELS[memberRole] ?? 0);
    };

    const availableRolesForMember = (memberRole: string): string[] => {
        if (!isAdminOrOwner.value) return [];
        const myLevel = ROLE_LEVELS[currentUserRole.value] ?? 0;
        return Object.entries(ROLE_LEVELS)
            .filter(([role, level]) => level < myLevel && role !== memberRole && role !== 'Owner')
            .map(([role]) => role);
    };

    const getUserName = (userId: number, userName?: string | null): string => {
        if (userName) return userName;
        const member = group.value?.members?.find(m => m.user?.pulsarr_user_id === userId);
        return member?.user?.name || 'Unknown User';
    };

    onMounted(() => {
        fetchGroup();
        contextStore.loadPrivacyTypes();
    });

    watch(groupId, () => {
        fetchGroup();
    });

    const fetchGroup = () => {
        const gen = ++fetchGeneration;
        loading.value = true;
        error.value = null;
        ratings.value = [];
        hasMoreRatings.value = true;
        loadingMoreRatings.value = false;
        groupAverages.value = {};
        if (ratingsObserver) ratingsObserver.disconnect();

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            if (gen !== fetchGeneration) return;
            group.value = data as GroupDTO;
            loading.value = false;
            fetchRatings(gen);
        };
        drh.onErrorCallback = (err) => {
            if (gen !== fetchGeneration) return;
            error.value = 'Failed to load group';
            loading.value = false;
            console.error('Failed to fetch group:', err);
        };
        drh.get(`/group/${groupId.value}`);
    };

    const fetchRatings = (gen: number) => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            if (gen !== fetchGeneration) return;
            ratings.value = data as Rating[];
            if (ratings.value.length < PAGE_SIZE) {
                hasMoreRatings.value = false;
            }
            ratings.value.forEach(rating => {
                if (rating.media_type === 'album' && rating.musicbrainz_id) {
                    fetchCoverArt(rating.musicbrainz_id);
                }
            });
            fetchGroupAverages(ratings.value);
        };
        drh.onErrorCallback = (err) => {
            if (gen !== fetchGeneration) return;
            console.error('Failed to fetch ratings:', err);
        };
        drh.post('/rating/byGroups', { group_ids: [groupId.value], take_size: PAGE_SIZE, offset: 0 });
    };

    const loadMoreRatings = () => {
        if (loadingMoreRatings.value || !hasMoreRatings.value) return;
        const gen = fetchGeneration;
        loadingMoreRatings.value = true;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            if (gen !== fetchGeneration) return;
            const newRatings = data as Rating[];
            ratings.value = [...ratings.value, ...newRatings];
            if (newRatings.length < PAGE_SIZE) {
                hasMoreRatings.value = false;
            }
            newRatings.forEach(rating => {
                if (rating.media_type === 'album' && rating.musicbrainz_id) {
                    fetchCoverArt(rating.musicbrainz_id);
                }
            });
            fetchGroupAverages(newRatings);
            loadingMoreRatings.value = false;
        };
        drh.onErrorCallback = (err) => {
            if (gen !== fetchGeneration) return;
            console.error('Failed to load more ratings:', err);
            loadingMoreRatings.value = false;
        };
        drh.post('/rating/byGroups', { group_ids: [groupId.value], take_size: PAGE_SIZE, offset: ratings.value.length });
    };

    const setupRatingsObserver = () => {
        if (ratingsObserver) ratingsObserver.disconnect();
        const el = ratingSentinel.value;
        if (!el) return;
        ratingsObserver = new IntersectionObserver(
            (entries) => {
                if (entries[0].isIntersecting) {
                    loadMoreRatings();
                }
            },
            { rootMargin: '200px' }
        );
        ratingsObserver.observe(el);
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
        drh.get(`/musicbrainz/release-group/${releaseId}/cover-art-info`);
    };

    const copyShareLink = async () => {
        try {
            await navigator.clipboard.writeText(shareLink.value);
            copied.value = true;
            toast.success('Link copied to clipboard');
            setTimeout(() => {
                copied.value = false;
            }, 2000);
        } catch (err) {
            console.error('Failed to copy:', err);
            toast.error('Failed to copy link');
        }
    };

    const joining = ref(false);

    const joinGroup = () => {
        joining.value = true;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            joining.value = false;
            toast.success('Joined group!');
            fetchGroup();
        };
        drh.onErrorCallback = (err) => {
            joining.value = false;
            console.error('Failed to join group:', err);
            toast.error('Failed to join group');
        };
        drh.post(`/group/join/${groupId.value}`, {});
    };

    const leaveGroup = () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            toast.success('Left group');
            router.push('/');
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to leave group:', err);
            toast.error('Failed to leave group');
        };
        drh.post(`/group/leave/${groupId.value}`, {});
    };

    const isRatingOutdated = (rating: Rating): boolean => {
        if (!group.value) return false;
        return rating.rating_system_id !== group.value.rating_system_id;
    };

    const onRatingSystemChanged = () => {
        fetchGroup();
    };

    const onRatingCreated = () => {
        rateThisRating.value = null;
        groupAverages.value = {};
        fetchRatings(fetchGeneration);
    };

    const onEditRating = (rating: Rating) => {
        editingRating.value = rating;
        showRatingModal.value = false;
        showAddRatingModal.value = true;
    };

    const onRateThisAlbum = (rating: Rating) => {
        editingRating.value = null;
        rateThisRating.value = rating;
        showRatingModal.value = false;
        showAddRatingModal.value = true;
    };

    const onRatingUpdated = () => {
        editingRating.value = null;
        rateThisRating.value = null;
        groupAverages.value = {};
        fetchRatings(fetchGeneration);
    };

    // --- Admin Actions ---

    const openSettings = () => {
        if (!group.value) return;
        editName.value = group.value.name;
        editPrivacyType.value = group.value.privacy_type;
        showSettingsDialog.value = true;
    };

    const saveGroupSettings = () => {
        if (!group.value) return;
        settingsSaving.value = true;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const updated = data as GroupDTO;
            group.value!.name = updated.name;
            group.value!.privacy_type = updated.privacy_type;
            showSettingsDialog.value = false;
            settingsSaving.value = false;
            toast.success('Settings saved');
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to update group:', err);
            settingsSaving.value = false;
            toast.error('Failed to save settings');
        };
        drh.post('/group/update', {
            ...group.value,
            name: editName.value,
            privacy_type: editPrivacyType.value,
            members: undefined,
            rating_system: undefined,
        });
    };

    const deleteGroup = () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            toast.success('Group deleted');
            router.push('/');
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to delete group:', err);
            toast.error('Failed to delete group');
        };
        drh.post(`/group/delete/${groupId.value}`, {});
    };

    const kickMember = (targetUserId: number) => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            toast.success('Member removed');
            fetchGroup();
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to kick member:', err);
            toast.error('Failed to remove member');
        };
        drh.post(`/group/kick/${groupId.value}/${targetUserId}`, {});
    };

    const changeRole = (targetUserId: number, newRole: string) => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            toast.success('Role updated');
            fetchGroup();
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to change role:', err);
            toast.error('Failed to change role');
        };
        drh.post(`/group/changeRole/${groupId.value}/${targetUserId}/${newRole}`, {});
    };

    const openTransferDialog = (targetUserId: number, targetName: string) => {
        transferTargetId.value = targetUserId;
        transferTargetName.value = targetName;
        showTransferDialog.value = true;
    };

    const transferOwnership = () => {
        if (!transferTargetId.value) return;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            toast.success('Ownership transferred');
            showTransferDialog.value = false;
            transferTargetId.value = null;
            fetchGroup();
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to transfer ownership:', err);
            toast.error('Failed to transfer ownership');
        };
        drh.post(`/group/transferOwnership/${groupId.value}/${transferTargetId.value}`, {});
    };

    const roleDisplayName = (role: string): string => {
        if (role === 'ViewOnly') return 'View Only';
        return role;
    };

    watch(ratingSentinel, () => {
        setupRatingsObserver();
    });

    onUnmounted(() => {
        if (ratingsObserver) ratingsObserver.disconnect();
    });
</script>

<template>
    <div class="p-4 max-w-4xl mx-auto">
        <LoadingSpinner v-if="loading" text="Loading group..." />

        <Card v-else-if="error">
            <CardContent class="py-12">
                <EmptyState
                    :icon="Users"
                    title="Group not available"
                    description="This group may be private or no longer exists."
                />
                <div class="flex justify-center mt-4">
                    <Button variant="outline" @click="$router.push('/groups')">Browse Groups</Button>
                </div>
            </CardContent>
        </Card>

        <div v-else-if="group" class="space-y-6">
            <!-- Page Header -->
            <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                <div>
                    <h1 class="text-2xl font-bold flex items-center gap-2">
                        <Users class="w-6 h-6" />
                        {{ group.name }}
                    </h1>
                    <div class="flex items-center gap-2 mt-1">
                        <span v-if="isCrate" class="px-2 py-0.5 bg-primary/10 text-primary rounded text-xs font-medium">My Crate</span>
                        <span v-else class="px-2 py-0.5 bg-secondary rounded text-xs">{{ group.privacy_type }}</span>
                        <span v-if="!isCrate" class="text-sm text-muted-foreground">{{ group.members?.length ?? 0 }} members</span>
                    </div>
                </div>
                <div class="flex gap-2">
                    <Button v-if="isAdminOrOwner" variant="outline" size="icon" aria-label="Group settings" @click="openSettings">
                        <Settings class="w-4 h-4" />
                    </Button>
                    <Button v-if="isMember && !isOwner && !isCrate" variant="outline" @click="leaveGroup">
                        <LogOut class="w-4 h-4 mr-1" />
                        Leave
                    </Button>
                    <Button v-if="user && !isMember && !isCrate" :disabled="joining" @click="joinGroup">
                        <UserPlus class="w-4 h-4 mr-1" />
                        {{ joining ? 'Joining...' : 'Join Group' }}
                    </Button>
                </div>
            </div>

            <!-- Share Link -->
            <div v-if="!isCrate" class="flex items-center gap-2">
                <span class="text-sm text-muted-foreground">Share link:</span>
                <code class="flex-1 px-2 py-1 bg-muted rounded text-sm truncate">{{ shareLink }}</code>
                <Button variant="outline" size="icon" :aria-label="copied ? 'Link copied' : 'Copy share link'" @click="copyShareLink">
                    <Check v-if="copied" class="w-4 h-4 text-green-500" />
                    <Copy v-else class="w-4 h-4" />
                </Button>
            </div>

            <!-- Info Cards -->
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                <!-- Rating System Card -->
                <Card v-if="group.rating_system">
                    <CardHeader class="pb-2">
                        <CardTitle class="text-sm font-medium text-muted-foreground flex items-center gap-2">
                            <Star class="w-4 h-4" />
                            Rating System
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div class="text-lg font-bold">{{ group.rating_system.name }}</div>
                        <div class="text-sm text-muted-foreground mt-1">
                            {{ group.rating_system.master_rating_type }} · max {{ group.rating_system.rating_max }}
                        </div>
                        <div v-if="group.rating_system.parameters && group.rating_system.parameters.length > 0" class="mt-3 flex flex-wrap gap-1.5">
                            <span
                                v-for="param in group.rating_system.parameters"
                                :key="param.rating_system_parameter_id"
                                class="px-2 py-0.5 bg-secondary rounded text-xs"
                            >
                                {{ param.name }}
                            </span>
                        </div>
                    </CardContent>
                </Card>

                <!-- Members Card -->
                <Card v-if="!isCrate">
                    <CardHeader class="pb-2">
                        <CardTitle class="text-sm font-medium text-muted-foreground flex items-center gap-2">
                            <Users class="w-4 h-4" />
                            Members ({{ group.members?.length ?? 0 }})
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <ul v-if="group.members && group.members.length > 0" class="space-y-2">
                            <li
                                v-for="member in group.members"
                                :key="member.user?.pulsarr_user_id"
                                class="text-sm flex items-center justify-between gap-2"
                            >
                                <div class="flex items-center gap-2 min-w-0">
                                    <span class="w-2 h-2 rounded-full bg-primary flex-shrink-0"></span>
                                    <RouterLink :to="`/user/${member.user?.pulsarr_user_id}`" class="truncate font-medium hover:underline">{{ member.user?.name }}</RouterLink>
                                    <span v-if="member.group_role === 'Owner'" class="inline-flex items-center gap-1 text-xs text-muted-foreground flex-shrink-0">
                                        <Crown class="w-3 h-3" /> Owner
                                    </span>
                                    <span v-else-if="member.group_role === 'Admin'" class="inline-flex items-center gap-1 text-xs text-muted-foreground flex-shrink-0">
                                        <Shield class="w-3 h-3" /> Admin
                                    </span>
                                    <span v-else-if="member.group_role === 'ViewOnly'" class="text-xs text-muted-foreground flex-shrink-0">
                                        View Only
                                    </span>
                                </div>

                                <!-- Member action dropdown -->
                                <DropdownMenu v-if="isAdminOrOwner && canActOn(member.group_role ?? '') && member.user?.pulsarr_user_id !== user?.pulsarr_user_id">
                                    <DropdownMenuTrigger as-child>
                                        <Button variant="ghost" size="icon" class="h-8 w-8" :aria-label="'Actions for ' + member.user?.name">
                                            <MoreHorizontal class="w-3.5 h-3.5" />
                                        </Button>
                                    </DropdownMenuTrigger>
                                    <DropdownMenuContent align="end">
                                        <DropdownMenuItem
                                            v-for="role in availableRolesForMember(member.group_role ?? '')"
                                            :key="role"
                                            @click="changeRole(member.user!.pulsarr_user_id!, role)"
                                        >
                                            <Shield class="w-4 h-4 mr-2" />
                                            Set to {{ roleDisplayName(role) }}
                                        </DropdownMenuItem>
                                        <DropdownMenuItem
                                            v-if="isOwner"
                                            @click="openTransferDialog(member.user!.pulsarr_user_id!, member.user?.name ?? '')"
                                        >
                                            <Crown class="w-4 h-4 mr-2" />
                                            Transfer Ownership
                                        </DropdownMenuItem>
                                        <DropdownMenuSeparator />
                                        <DropdownMenuItem
                                            class="text-destructive"
                                            @click="kickMember(member.user!.pulsarr_user_id!)"
                                        >
                                            <UserMinus class="w-4 h-4 mr-2" />
                                            Kick
                                        </DropdownMenuItem>
                                    </DropdownMenuContent>
                                </DropdownMenu>
                            </li>
                        </ul>
                    </CardContent>
                </Card>
            </div>

            <!-- Settings Dialog -->
            <Dialog :open="showSettingsDialog" @update:open="showSettingsDialog = $event">
                <DialogScrollContent class="max-w-md">
                    <DialogHeader>
                        <DialogTitle>{{ isCrate ? 'Crate Settings' : 'Group Settings' }}</DialogTitle>
                        <DialogDescription>{{ isCrate ? 'Update your Crate name and rating system.' : 'Update your group\'s name and privacy settings.' }}</DialogDescription>
                    </DialogHeader>
                    <div class="space-y-4 py-2">
                        <div class="space-y-2">
                            <Label for="group-name">{{ isCrate ? 'Crate Name' : 'Group Name' }}</Label>
                            <Input id="group-name" v-model="editName" />
                        </div>
                        <div v-if="!isCrate" class="space-y-2">
                            <Label for="privacy-type">Privacy Type</Label>
                            <Select v-model="editPrivacyType">
                                <SelectTrigger>
                                    <SelectValue placeholder="Select privacy type" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem v-for="pt in privacyTypes" :key="pt" :value="pt">
                                        {{ pt }}
                                    </SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                        <!-- Rating System Section -->
                        <div class="space-y-2">
                            <Label>Rating System</Label>
                            <div v-if="group?.rating_system" class="p-3 bg-muted/50 rounded-md">
                                <div class="text-sm font-medium">{{ group.rating_system.name }}</div>
                                <div class="text-xs text-muted-foreground mt-0.5">
                                    {{ group.rating_system.master_rating_type }} · max {{ group.rating_system.rating_max }}
                                </div>
                            </div>
                            <Button
                                variant="outline"
                                size="sm"
                                class="w-full"
                                @click="showSettingsDialog = false; showChangeRatingSystemModal = true;"
                            >
                                <RefreshCw class="w-4 h-4 mr-1" />
                                Change Rating System
                            </Button>
                        </div>

                        <div class="flex justify-between items-center pt-2">
                            <Button
                                v-if="isOwner && !isCrate"
                                variant="destructive"
                                size="sm"
                                @click="showSettingsDialog = false; deleteConfirmText = ''; showDeleteConfirm = true;"
                            >
                                Delete Group
                            </Button>
                            <div class="flex gap-2 ml-auto">
                                <Button variant="outline" @click="showSettingsDialog = false">Cancel</Button>
                                <Button @click="saveGroupSettings" :disabled="settingsSaving || !editName.trim()">
                                    {{ settingsSaving ? 'Saving...' : 'Save' }}
                                </Button>
                            </div>
                        </div>
                    </div>
                </DialogScrollContent>
            </Dialog>

            <!-- Delete Group Confirmation -->
            <AlertDialog :open="showDeleteConfirm" @update:open="showDeleteConfirm = $event">
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>Delete Group</AlertDialogTitle>
                        <AlertDialogDescription>
                            This action cannot be undone. This will permanently delete the group
                            <strong>{{ group.name }}</strong> and all associated data.
                            <br /><br />
                            Type the group name to confirm:
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <Input v-model="deleteConfirmText" :placeholder="group.name" />
                    <AlertDialogFooter>
                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                        <AlertDialogAction
                            class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                            :disabled="deleteConfirmText !== group.name"
                            @click="deleteGroup"
                        >
                            Delete
                        </AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>

            <!-- Transfer Ownership Confirmation -->
            <AlertDialog :open="showTransferDialog" @update:open="showTransferDialog = $event">
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>Transfer Ownership</AlertDialogTitle>
                        <AlertDialogDescription>
                            Are you sure you want to transfer ownership of <strong>{{ group.name }}</strong>
                            to <strong>{{ transferTargetName }}</strong>?
                            <br /><br />
                            You will be demoted to Admin.
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                        <AlertDialogAction @click="transferOwnership">Transfer</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>

            <!-- Ratings Section -->
            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <h2 class="text-lg font-semibold flex items-center gap-2">
                        <Star class="w-5 h-5" />
                        Ratings ({{ ratings.length }})
                    </h2>
                    <AddRatingModal
                        v-if="isMember && group.rating_system"
                        :show-dialog="showAddRatingModal"
                        :group="group"
                        :edit-rating="editingRating"
                        :edit-cover-art-url="editingRating ? coverArtCache[editingRating.musicbrainz_id] : undefined"
                        :rate-this-rating="rateThisRating"
                        :rate-this-cover-art-url="rateThisRating ? coverArtCache[rateThisRating.musicbrainz_id] : undefined"
                        @update:showDialog="(v) => { showAddRatingModal = v; if (!v) { editingRating = null; rateThisRating = null; } }"
                        @created="onRatingCreated"
                        @updated="onRatingUpdated"
                    >
                        <template #openModal>
                            <Button @click="editingRating = null; rateThisRating = null; showAddRatingModal = true;">
                                <Plus class="w-4 h-4 mr-1" />
                                Add Rating
                            </Button>
                        </template>
                    </AddRatingModal>
                </div>

                <EmptyState
                    v-if="ratings.length === 0"
                    :icon="Star"
                    title="No ratings yet"
                    description="Be the first to rate an album in this group"
                />

                <RatingCard
                    v-for="rating in ratings"
                    :key="rating.rating_id"
                    :rating="rating"
                    :cover-art-url="coverArtCache[rating.musicbrainz_id]"
                    :user-name="getUserName(rating.pulsarr_user_id, rating.user_name)"
                    :outdated="isRatingOutdated(rating)"
                    :group-average="groupAverages[`${rating.pulsarr_group_id}:${rating.musicbrainz_id}`]?.average"
                    :group-rating-count="groupAverages[`${rating.pulsarr_group_id}:${rating.musicbrainz_id}`]?.count"
                    :rating-max="ratingMax"
                    @click="openRatingModal(rating)"
                    @group-average-click="openRatingModalGroupTab(rating)"
                />

                <div ref="ratingSentinel" v-if="hasMoreRatings && ratings.length > 0" />
                <LoadingSpinner v-if="loadingMoreRatings" text="Loading more ratings..." />

                <RatingDetailModal
                    :rating="selectedRating"
                    :open="showRatingModal"
                    :cover-art-url="selectedRating ? coverArtCache[selectedRating.musicbrainz_id] : undefined"
                    :user-name="selectedRating ? getUserName(selectedRating.pulsarr_user_id, selectedRating.user_name) : ''"
                    :rating-system="group?.rating_system"
                    :outdated="selectedRating ? isRatingOutdated(selectedRating) : false"
                    :group="group"
                    :group-average="selectedRating ? groupAverages[`${selectedRating.pulsarr_group_id}:${selectedRating.musicbrainz_id}`]?.average : undefined"
                    :group-rating-count="selectedRating ? groupAverages[`${selectedRating.pulsarr_group_id}:${selectedRating.musicbrainz_id}`]?.count : undefined"
                    :rating-max="ratingMax"
                    :initial-tab="ratingModalInitialTab"
                    @update:open="showRatingModal = $event"
                    @edit="onEditRating"
                    @rateThis="onRateThisAlbum"
                />

                <!-- Change Rating System Modal -->
                <ChangeRatingSystemModal
                    v-if="group"
                    :open="showChangeRatingSystemModal"
                    :group-id="group.pulsarr_group_id"
                    @update:open="showChangeRatingSystemModal = $event"
                    @changed="onRatingSystemChanged"
                />
            </div>
        </div>
    </div>
</template>
