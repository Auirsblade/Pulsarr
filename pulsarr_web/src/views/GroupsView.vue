<script setup lang="ts">
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
    import { Input } from "@/components/ui/input";
    import { computed, onMounted, ref, watch } from "vue";
    import { useRouter } from "vue-router";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { GroupDTO } from "@/apiClient";
    import { Plus, Users, ChevronRight, Search } from 'lucide-vue-next'
    import { Button } from "@/components/ui/button"
    import AddGroupModal from "@/components/modals/AddGroupModal.vue";
    import EmptyState from "@/components/EmptyState.vue";
    import LoadingSpinner from "@/components/LoadingSpinner.vue";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import { useInfiniteScroll } from "@/composables/useInfiniteScroll";

    const router = useRouter();
    const { isLoggedIn } = storeToRefs(useContextStore());

    const showAddGroupModal = ref(false);
    const sentinel = ref<HTMLElement | null>(null);

    // Search state
    const searchQuery = ref('');
    const searchResults = ref<GroupDTO[]>([]);
    const isSearching = ref(false);
    let searchTimeout: ReturnType<typeof setTimeout> | null = null;

    const isSearchMode = computed(() => searchQuery.value.length >= 2);
    const displayedGroups = computed(() => isSearchMode.value ? searchResults.value : groups.value);

    const performSearch = () => {
        if (!isSearchMode.value) {
            searchResults.value = [];
            return;
        }
        isSearching.value = true;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            searchResults.value = data as GroupDTO[];
            isSearching.value = false;
        };
        drh.onErrorCallback = () => {
            searchResults.value = [];
            isSearching.value = false;
        };
        drh.get(`/group/search?name=${encodeURIComponent(searchQuery.value)}`);
    };

    watch(searchQuery, () => {
        if (searchTimeout) clearTimeout(searchTimeout);
        searchTimeout = setTimeout(performSearch, 300);
    });

    const fetchGroups = (offset: number, limit: number, onSuccess: (data: GroupDTO[]) => void, onError: (err: unknown) => void) => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => onSuccess(data as GroupDTO[]);
        drh.onErrorCallback = (err) => onError(err);
        if (isLoggedIn.value) {
            drh.post("/group/", { take_size: limit, offset });
        } else {
            drh.get("/group/public");
        }
    };

    const { items: groups, loading, loadingMore, hasMore, loadInitial, setupSentinel } = useInfiniteScroll(fetchGroups);

    onMounted(() => {
        loadInitial();
    });

    watch(isLoggedIn, () => {
        loadInitial();
    });

    watch(sentinel, (el) => {
        setupSentinel(el);
    });

    const onGroupCreated = () => {
        loadInitial();
    };
</script>

<template>
    <div class="p-4 max-w-4xl mx-auto">
        <!-- Page Header -->
        <div class="flex items-center justify-between mb-4">
            <h1 class="text-2xl font-bold flex items-center gap-2">
                <Users class="w-6 h-6" />
                Groups
            </h1>
            <AddGroupModal v-if="isLoggedIn" :show-dialog="showAddGroupModal"
                           @update:show-dialog="(value) => { showAddGroupModal = value; onGroupCreated() }">
                <template #openModal>
                    <Button @click="showAddGroupModal = true">
                        <Plus class="h-4 w-4 mr-1" />
                        New Group
                    </Button>
                </template>
            </AddGroupModal>
        </div>

        <!-- Search input (logged in only) -->
        <div v-if="isLoggedIn" class="mb-4">
            <div class="relative">
                <Search class="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
                <Input
                    v-model="searchQuery"
                    placeholder="Search groups..."
                    class="pl-9"
                />
            </div>
        </div>

        <LoadingSpinner v-if="loading || isSearching" text="Loading groups..." />

        <EmptyState
            v-else-if="displayedGroups.length === 0 && isSearchMode"
            :icon="Search"
            title="No groups found"
            description="Try a different search term"
        />

        <EmptyState
            v-else-if="displayedGroups.length === 0"
            :icon="Users"
            :title="isLoggedIn ? 'No groups yet' : 'No public groups available'"
            :description="isLoggedIn ? 'Create or join a group to start rating music with friends' : 'Sign in to discover and join groups'"
        />

        <div v-else class="space-y-3">
            <Card
                v-for="group in displayedGroups"
                :key="group.pulsarr_group_id"
                class="cursor-pointer hover:bg-accent/50 transition-colors focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none rounded-md"
                tabindex="0"
                role="button"
                @click="router.push(`/group/${group.pulsarr_group_id}`)"
                @keydown.enter="router.push(`/group/${group.pulsarr_group_id}`)"
                @keydown.space.prevent="router.push(`/group/${group.pulsarr_group_id}`)"
            >
                <CardHeader>
                    <div class="flex items-center justify-between">
                        <div>
                            <CardTitle>{{ group.name }}</CardTitle>
                            <CardDescription class="flex items-center gap-2 mt-1">
                                <span class="px-2 py-0.5 bg-secondary rounded text-xs">{{ group.privacy_type }}</span>
                                <span class="text-muted-foreground text-xs">
                                    <Users class="inline w-3 h-3 mr-1" />
                                    {{ group.member_count ?? group.members?.length ?? 0 }}
                                </span>
                            </CardDescription>
                        </div>
                        <ChevronRight class="w-5 h-5 text-muted-foreground" />
                    </div>
                </CardHeader>
                <CardContent v-if="group.rating_system">
                    <div class="text-sm">
                        <span class="text-muted-foreground">Rating System:</span>
                        <span class="ml-1 font-medium">{{ group.rating_system.name }}</span>
                        <span class="ml-2 text-muted-foreground">
                            ({{ group.rating_system.master_rating_type }}, max: {{ group.rating_system.rating_max }})
                        </span>
                    </div>
                </CardContent>
            </Card>

            <div ref="sentinel" v-if="hasMore && !isSearchMode" />
            <LoadingSpinner v-if="loadingMore && !isSearchMode" text="Loading more..." />
        </div>
    </div>
</template>
