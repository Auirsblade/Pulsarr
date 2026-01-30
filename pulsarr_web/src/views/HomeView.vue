<script setup lang="ts">
    import { Card, CardContent, CardDescription, CardHeader, CardTitle, } from "@/components/ui/card";
    import { onMounted, ref, watch } from "vue";
    import { useRouter } from "vue-router";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { GroupDTO } from "@/apiClient";
    import { Plus, Users, ChevronRight } from 'lucide-vue-next'
    import { Button } from "@/components/ui/button"
    import AddGroupModal from "@/components/modals/AddGroupModal.vue";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";

    const router = useRouter();
    const { isLoggedIn } = storeToRefs(useContextStore());

    const showAddGroupModal = ref(false);
    const groups = ref<Array<GroupDTO>>();

    onMounted(async () => {
        getGroups();
    });

    watch(isLoggedIn, () => {
        getGroups();
    });

    const getGroups = () => {
        let groupDrh = new DataRequestHandler();
        groupDrh.onSuccessCallback = (data) => {
            groups.value = data as GroupDTO[];
        };
        if (isLoggedIn.value) {
            groupDrh.post("/group/", {});
        } else {
            groupDrh.get("/group/public");
        }
    };
</script>

<template>
    <div class="p-2">
        <Card>
            <CardHeader>
                <div class="flex justify-between items-center">
                    <CardTitle>Groups</CardTitle>
                    <AddGroupModal v-if="isLoggedIn" :show-dialog="showAddGroupModal"
                                   @update:show-dialog="(value) => { showAddGroupModal = value; getGroups() }">
                        <template #openModal>
                            <Button variant="ghost" size="icon" @click="showAddGroupModal = true">
                                <Plus class="h-4 w-4"/>
                            </Button>
                        </template>
                    </AddGroupModal>
                </div>
            </CardHeader>
            <CardContent>
                <div v-if="!groups || groups.length === 0" class="text-center py-8 text-muted-foreground">
                    {{ isLoggedIn ? 'No groups yet. Create one to get started!' : 'No public groups available. Sign in to see more.' }}
                </div>
                <div v-for="group in groups" :key="group.pulsarr_group_id">
                    <Card
                        class="mb-2 cursor-pointer hover:bg-accent/50 transition-colors"
                        @click="router.push(`/group/${group.pulsarr_group_id}`)"
                    >
                        <CardHeader>
                            <div class="flex items-center justify-between">
                                <div>
                                    <CardTitle>{{ group.name }}</CardTitle>
                                    <CardDescription class="flex items-center gap-2 mt-1">
                                        <span class="px-2 py-0.5 bg-secondary rounded text-xs">{{ group.privacy_type }}</span>
                                        <span v-if="group.members" class="text-muted-foreground text-xs">
                                            <Users class="inline w-3 h-3 mr-1" />
                                            {{ group.members.length }}
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
                </div>
            </CardContent>
        </Card>
    </div>
</template>