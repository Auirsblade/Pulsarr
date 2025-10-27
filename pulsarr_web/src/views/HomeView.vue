<script setup lang="ts">
    import { Card, CardContent, CardDescription, CardHeader, CardTitle, } from "@/components/ui/card";
    import { onMounted, reactive, ref, watch } from "vue";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { GroupDTO, RatingSystemDTO, UserDTO } from "@/apiClient";
    import { Plus } from 'lucide-vue-next'
    import { Button } from "@/components/ui/button"
    import AddGroupModal from "@/components/modals/AddGroupModal.vue";

    const showAddGroupModal = ref(false);

    const groups = ref<Array<GroupDTO>>();
    const ratingSystems = ref<Array<RatingSystemDTO>>();
    const users = ref<Array<UserDTO>>();

    onMounted(async () => {
        getGroups();
        getUsers();
    });

    const getGroups = () => {
        let groupDrh = new DataRequestHandler();
        groupDrh.onSuccessCallback = (data) => {
            console.log(data);
            groups.value = data as GroupDTO[];
        };
        groupDrh.post("/group/", {});
    };

    const getUsers = () => {
        let userDrh = new DataRequestHandler();
        userDrh.onSuccessCallback = (data) => {
            console.log(data);
            users.value = data as UserDTO[];
        };
        userDrh.get("/user/");
    }
</script>

<template>
    <div class="p-2">
        <Card>
            <CardHeader>
                <div class="flex justify-between items-center">
                    <CardTitle>Groups</CardTitle>
                    <AddGroupModal :rating-systems="ratingSystems" :show-dialog="showAddGroupModal"
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
                <div v-for="group in groups">
                    <Card class="mb-2">
                        <CardHeader>
                            <CardTitle>
                                {{ group.name }}
                            </CardTitle>
                        </CardHeader>
                        <CardContent>
                            <Card>
                                <CardHeader>
                                    <CardTitle>
                                        {{ ratingSystems?.find(x => x.rating_system_id == group.rating_system_id)?.name }}
                                    </CardTitle>
                                    <CardDescription>
                                        Rating System
                                    </CardDescription>
                                </CardHeader>
                                <CardContent>
                                    <div>
                                        <span class="font-semibold">Rating type: </span>{{
                                            ratingSystems?.find(x => x.rating_system_id == group.rating_system_id)?.master_rating_type
                                        }}
                                    </div>
                                    <div>
                                        <span class="font-semibold">Max Rating: </span>{{
                                            ratingSystems?.find(x => x.rating_system_id == group.rating_system_id)?.rating_max
                                        }}
                                    </div>
                                </CardContent>
                            </Card>
                        </CardContent>
                    </Card>
                </div>
            </CardContent>
        </Card>
    </div>
</template>