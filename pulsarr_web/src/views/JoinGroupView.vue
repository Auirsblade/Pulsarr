<script setup lang="ts">
    import { Card, CardContent, CardDescription, CardHeader, CardTitle, CardFooter } from "@/components/ui/card";
    import { Button } from "@/components/ui/button";
    import { onMounted, ref, computed, watch } from "vue";
    import { useRoute, useRouter } from "vue-router";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { GroupDTO } from "@/apiClient";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import { Users, Star, CheckCircle } from "lucide-vue-next";

    const route = useRoute();
    const router = useRouter();
    const contextStore = useContextStore();
    const { user, isLoggedIn, showSignInDialog } = storeToRefs(contextStore);

    const group = ref<GroupDTO | null>(null);
    const loading = ref(true);
    const error = ref<string | null>(null);
    const joining = ref(false);
    const joined = ref(false);

    const groupId = computed(() => Number(route.params.groupId));

    const isAlreadyMember = ref(false);

    onMounted(() => {
        fetchGroup();
    });

    watch(isLoggedIn, () => {
        fetchGroup();
    });

    const fetchGroup = () => {
        loading.value = true;
        error.value = null;
        isAlreadyMember.value = false;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            group.value = data as GroupDTO;
            loading.value = false;
            if (isLoggedIn.value) {
                checkMembership();
            }
        };
        drh.onErrorCallback = (err) => {
            error.value = 'Failed to load group. It may not exist.';
            loading.value = false;
            console.error('Failed to fetch group:', err);
        };
        drh.get(`/group/preview/${groupId.value}`);
    };

    const checkMembership = () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const fullGroup = data as GroupDTO;
            if (fullGroup.members && user.value) {
                isAlreadyMember.value = fullGroup.members.some(
                    m => m.user?.pulsarr_user_id === user.value?.pulsarr_user_id
                );
            }
        };
        drh.onErrorCallback = () => {
            // Swallow errors (e.g. 403 for private groups) — user can still join
        };
        drh.get(`/group/${groupId.value}`);
    };

    const joinGroup = () => {
        joining.value = true;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            joined.value = true;
            joining.value = false;
            setTimeout(() => {
                router.push(`/group/${groupId.value}`);
            }, 1500);
        };
        drh.onErrorCallback = (err) => {
            error.value = 'Failed to join group';
            joining.value = false;
            console.error('Failed to join group:', err);
        };
        drh.post(`/group/join/${groupId.value}`, {});
    };

    const goToGroup = () => {
        router.push(`/group/${groupId.value}`);
    };
</script>

<template>
    <div class="p-4 max-w-md mx-auto mt-8">
        <div v-if="loading" class="flex justify-center py-8">
            <span class="text-muted-foreground">Loading group...</span>
        </div>

        <div v-else-if="error" class="flex flex-col items-center py-8 gap-4">
            <span class="text-destructive">{{ error }}</span>
            <Button variant="outline" @click="router.push('/')">Go Home</Button>
        </div>

        <div v-else-if="joined" class="flex flex-col items-center py-8 gap-4">
            <CheckCircle class="w-16 h-16 text-green-500" />
            <span class="text-lg font-medium">Successfully joined {{ group?.name }}!</span>
            <span class="text-muted-foreground">Redirecting...</span>
        </div>

        <Card v-else-if="group">
            <CardHeader class="text-center">
                <CardTitle class="text-xl">Join Group</CardTitle>
                <CardDescription>You've been invited to join a group</CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="text-center">
                    <h2 class="text-2xl font-bold">{{ group.name }}</h2>
                    <div class="flex justify-center items-center gap-2 mt-2">
                        <span class="px-2 py-0.5 bg-secondary rounded text-xs">{{ group.privacy_type }}</span>
                        <span class="text-muted-foreground text-sm">
                            <Users class="inline w-4 h-4 mr-1" />
                            {{ group.member_count ?? group.members?.length ?? 0 }} members
                        </span>
                    </div>
                </div>

                <div v-if="group.rating_system" class="p-3 bg-muted rounded-lg">
                    <div class="flex items-center gap-2 text-sm font-medium">
                        <Star class="w-4 h-4" />
                        {{ group.rating_system.name }}
                    </div>
                    <div class="text-xs text-muted-foreground mt-1">
                        {{ group.rating_system.master_rating_type }} - Max: {{ group.rating_system.rating_max }}
                    </div>
                </div>
            </CardContent>
            <CardFooter class="flex flex-col gap-2">
                <div v-if="!isLoggedIn" class="w-full text-center">
                    <p class="text-sm text-muted-foreground mb-3">
                        Create an account to join {{ group?.name }}
                    </p>
                    <Button class="w-full" @click="showSignInDialog = true">
                        Sign Up / Sign In
                    </Button>
                </div>
                <div v-else-if="isAlreadyMember" class="w-full">
                    <Button class="w-full" @click="goToGroup">
                        You're already a member - View Group
                    </Button>
                </div>
                <div v-else class="w-full">
                    <Button class="w-full" :disabled="joining" @click="joinGroup">
                        {{ joining ? 'Joining...' : 'Join Group' }}
                    </Button>
                </div>
            </CardFooter>
        </Card>
    </div>
</template>
