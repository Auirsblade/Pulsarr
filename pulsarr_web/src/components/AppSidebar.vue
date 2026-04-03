<script setup lang="ts">
    import { onMounted, watch } from "vue";
    import { useRoute, useRouter } from "vue-router";
    import { useFeedStore } from "@/stores/feed";
    import { useContextStore } from "@/stores/context";
    import { storeToRefs } from "pinia";
    import { computed } from "vue";
    import { Activity, Home, Users, X, BarChart3, Archive } from "lucide-vue-next";
    import { Button } from "@/components/ui/button";

    defineProps<{
        open: boolean;
    }>();

    const emit = defineEmits<{
        'update:open': [value: boolean];
    }>();

    const route = useRoute();
    const router = useRouter();
    const feedStore = useFeedStore();
    const contextStore = useContextStore();
    const { myGroups } = storeToRefs(feedStore);
    const { isLoggedIn, crate } = storeToRefs(contextStore);

    const filteredGroups = computed(() =>
        myGroups.value.filter((g: any) => g.privacy_type !== 'Personal')
    );

    onMounted(() => {
        if (isLoggedIn.value) {
            feedStore.ensureGroupsLoaded();
        }
    });

    watch(isLoggedIn, (loggedIn) => {
        if (loggedIn) {
            feedStore.ensureGroupsLoaded();
        }
    });

    const navigateAuth = (path: string) => {
        emit('update:open', false);
        if (!isLoggedIn.value) {
            contextStore.showSignInDialog = true;
            return;
        }
        router.push(path);
    };
</script>

<template>
    <!-- Desktop sidebar -->
    <aside class="hidden lg:flex flex-col w-64 flex-shrink-0 border-r border-sidebar-border bg-sidebar text-sidebar-foreground h-full overflow-y-auto">
        <nav class="flex flex-col gap-1 p-3">
            <RouterLink
                to="/"
                class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                :class="route.path === '/' ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'hover:bg-sidebar-accent/50'"
            >
                <Activity class="w-4 h-4" />
                Feed
            </RouterLink>
            <a
                href="#"
                class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                :class="route.path === '/groups' ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'hover:bg-sidebar-accent/50'"
                @click.prevent="navigateAuth('/groups')"
            >
                <Home class="w-4 h-4" />
                Groups
            </a>
            <a
                href="#"
                class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                :class="route.path === '/history' ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'hover:bg-sidebar-accent/50'"
                @click.prevent="navigateAuth('/history')"
            >
                <BarChart3 class="w-4 h-4" />
                My Ratings
            </a>
        </nav>

        <hr class="border-sidebar-border mx-3" />

        <div v-if="crate" class="p-3 pb-0">
            <RouterLink
                :to="`/group/${crate.pulsarr_group_id}`"
                class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                :class="route.path === `/group/${crate.pulsarr_group_id}` ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'hover:bg-sidebar-accent/50'"
            >
                <Archive class="w-4 h-4" />
                <div class="flex flex-col">
                    <span>{{ crate.name }}</span>
                    <span class="text-xs font-normal text-muted-foreground">Personal collection</span>
                </div>
            </RouterLink>
        </div>

        <div class="p-3">
            <h3 class="flex items-center gap-2 px-3 py-1 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
                <Users class="w-3 h-3" />
                My Groups
            </h3>
            <div v-if="!isLoggedIn" class="px-3 py-2 text-sm text-muted-foreground">
                Sign in to see your groups
            </div>
            <div v-else-if="filteredGroups.length === 0" class="flex items-center gap-2 px-3 py-2 text-sm text-muted-foreground">
                <Users class="w-4 h-4" />
                No groups yet — join or create one
            </div>
            <nav class="flex flex-col gap-0.5 mt-1">
                <RouterLink
                    v-for="group in filteredGroups"
                    :key="group.pulsarr_group_id"
                    :to="`/group/${group.pulsarr_group_id}`"
                    class="px-3 py-1.5 rounded-md text-sm transition-colors"
                    :class="route.path === `/group/${group.pulsarr_group_id}` ? 'bg-sidebar-accent text-sidebar-accent-foreground font-medium' : 'hover:bg-sidebar-accent/50'"
                >
                    {{ group.name }}
                </RouterLink>
            </nav>
        </div>
    </aside>

    <!-- Mobile overlay sidebar -->
    <Teleport to="body">
        <Transition name="sidebar">
            <div v-if="open" class="lg:hidden fixed inset-0 z-50 flex">
                <!-- Backdrop -->
                <div class="absolute inset-0 bg-black/50" aria-hidden="true" @click="$emit('update:open', false)" />

                <!-- Sidebar panel -->
                <aside class="relative w-64 bg-sidebar text-sidebar-foreground h-full overflow-y-auto flex flex-col shadow-xl" role="dialog" aria-label="Navigation menu">
                    <div class="flex items-center justify-between p-3 border-b border-sidebar-border">
                        <span class="font-semibold text-sm">Navigation</span>
                        <Button variant="ghost" size="icon" class="h-8 w-8" aria-label="Close navigation menu" @click="$emit('update:open', false)">
                            <X class="w-4 h-4" />
                        </Button>
                    </div>

                    <nav class="flex flex-col gap-1 p-3">
                        <RouterLink
                            to="/"
                            class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                            :class="route.path === '/' ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'hover:bg-sidebar-accent/50'"
                            @click="$emit('update:open', false)"
                        >
                            <Activity class="w-4 h-4" />
                            Feed
                        </RouterLink>
                        <a
                            href="#"
                            class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                            :class="route.path === '/groups' ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'hover:bg-sidebar-accent/50'"
                            @click.prevent="navigateAuth('/groups')"
                        >
                            <Home class="w-4 h-4" />
                            Groups
                        </a>
                        <a
                            href="#"
                            class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                            :class="route.path === '/history' ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'hover:bg-sidebar-accent/50'"
                            @click.prevent="navigateAuth('/history')"
                        >
                            <BarChart3 class="w-4 h-4" />
                            My Ratings
                        </a>
                    </nav>

                    <hr class="border-sidebar-border mx-3" />

                    <div v-if="crate" class="p-3 pb-0">
                        <RouterLink
                            :to="`/group/${crate.pulsarr_group_id}`"
                            class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium transition-colors"
                            :class="route.path === `/group/${crate.pulsarr_group_id}` ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'hover:bg-sidebar-accent/50'"
                            @click="$emit('update:open', false)"
                        >
                            <Archive class="w-4 h-4" />
                            <div class="flex flex-col">
                                <span>{{ crate.name }}</span>
                                <span class="text-xs font-normal text-muted-foreground">Personal collection</span>
                            </div>
                        </RouterLink>
                    </div>

                    <div class="p-3">
                        <h3 class="flex items-center gap-2 px-3 py-1 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
                            <Users class="w-3 h-3" />
                            My Groups
                        </h3>
                        <div v-if="!isLoggedIn" class="px-3 py-2 text-sm text-muted-foreground">
                            Sign in to see your groups
                        </div>
                        <div v-else-if="filteredGroups.length === 0" class="flex items-center gap-2 px-3 py-2 text-sm text-muted-foreground">
                            <Users class="w-4 h-4" />
                            No groups yet — join or create one
                        </div>
                        <nav class="flex flex-col gap-0.5 mt-1">
                            <RouterLink
                                v-for="group in filteredGroups"
                                :key="group.pulsarr_group_id"
                                :to="`/group/${group.pulsarr_group_id}`"
                                class="px-3 py-1.5 rounded-md text-sm transition-colors"
                                :class="route.path === `/group/${group.pulsarr_group_id}` ? 'bg-sidebar-accent text-sidebar-accent-foreground font-medium' : 'hover:bg-sidebar-accent/50'"
                                @click="$emit('update:open', false)"
                            >
                                {{ group.name }}
                            </RouterLink>
                        </nav>
                    </div>
                </aside>
            </div>
        </Transition>
    </Teleport>
</template>

<style scoped>
.sidebar-enter-active,
.sidebar-leave-active {
    transition: opacity 0.2s ease;
}
.sidebar-enter-active aside,
.sidebar-leave-active aside {
    transition: transform 0.2s ease;
}
.sidebar-enter-from,
.sidebar-leave-to {
    opacity: 0;
}
.sidebar-enter-from aside,
.sidebar-leave-to aside {
    transform: translateX(-100%);
}
</style>
