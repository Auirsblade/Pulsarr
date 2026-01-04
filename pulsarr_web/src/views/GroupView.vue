<script setup lang="ts">
    import { Card, CardContent, CardDescription, CardHeader, CardTitle, CardFooter } from "@/components/ui/card";
    import { Button } from "@/components/ui/button";
    import { onMounted, ref, computed } from "vue";
    import { useRoute, useRouter } from "vue-router";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { GroupDTO } from "@/apiClient";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import { Copy, Check, LogOut, Plus, Users, Star } from "lucide-vue-next";
    import AddRatingModal from "@/components/modals/AddRatingModal.vue";

    const route = useRoute();
    const router = useRouter();
    const { user } = storeToRefs(useContextStore());

    const group = ref<GroupDTO | null>(null);
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
        };
        drh.onErrorCallback = (err) => {
            error.value = 'Failed to load group';
            loading.value = false;
            console.error('Failed to fetch group:', err);
        };
        drh.get(`/group/${groupId.value}`);
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
            <Card>
                <CardHeader>
                    <div class="flex justify-between items-start">
                        <div>
                            <CardTitle class="text-2xl">{{ group.name }}</CardTitle>
                            <CardDescription class="flex items-center gap-2 mt-1">
                                <span class="px-2 py-0.5 bg-secondary rounded text-xs">{{ group.privacy_type }}</span>
                                <span class="text-muted-foreground">
                                    <Users class="inline w-4 h-4 mr-1" />
                                    {{ group.members?.length ?? 0 }} members
                                </span>
                            </CardDescription>
                        </div>
                        <div class="flex gap-2">
                            <AddRatingModal
                                v-if="isMember && group.rating_system"
                                :show-dialog="showAddRatingModal"
                                :group="group"
                                @update:showDialog="showAddRatingModal = $event"
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
                    <div class="flex items-center gap-2">
                        <span class="text-sm text-muted-foreground">Share link:</span>
                        <code class="flex-1 px-2 py-1 bg-muted rounded text-sm truncate">{{ shareLink }}</code>
                        <Button variant="outline" size="icon" @click="copyShareLink">
                            <Check v-if="copied" class="w-4 h-4 text-green-500" />
                            <Copy v-else class="w-4 h-4" />
                        </Button>
                    </div>
                </CardContent>
            </Card>

            <Card v-if="group.rating_system">
                <CardHeader>
                    <CardTitle class="flex items-center gap-2">
                        <Star class="w-5 h-5" />
                        Rating System: {{ group.rating_system.name }}
                    </CardTitle>
                    <CardDescription>
                        {{ group.rating_system.master_rating_type }} - Max: {{ group.rating_system.rating_max }}
                    </CardDescription>
                </CardHeader>
                <CardContent v-if="group.rating_system.parameters && group.rating_system.parameters.length > 0">
                    <div class="text-sm font-medium mb-2">Parameters:</div>
                    <div class="flex flex-wrap gap-2">
                        <span
                            v-for="param in group.rating_system.parameters"
                            :key="param.rating_system_parameter_id"
                            class="px-2 py-1 bg-secondary rounded text-sm"
                        >
                            {{ param.name }} (max: {{ param.parameter_rating_max }})
                        </span>
                    </div>
                </CardContent>
            </Card>

            <Card>
                <CardHeader>
                    <CardTitle class="flex items-center gap-2">
                        <Users class="w-5 h-5" />
                        Members
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <div v-if="group.members && group.members.length > 0" class="divide-y">
                        <div
                            v-for="member in group.members"
                            :key="member.user?.pulsarr_user_id"
                            class="flex items-center justify-between py-2"
                        >
                            <div>
                                <span class="font-medium">{{ member.user?.name }}</span>
                                <span class="ml-2 text-xs text-muted-foreground">{{ member.user?.email }}</span>
                            </div>
                            <span class="px-2 py-0.5 bg-secondary rounded text-xs uppercase">
                                {{ member.group_role }}
                            </span>
                        </div>
                    </div>
                    <div v-else class="text-muted-foreground text-sm">
                        No members yet
                    </div>
                </CardContent>
            </Card>
        </div>
    </div>
</template>
