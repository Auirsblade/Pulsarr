<script setup lang="ts">
    import {
        Card,
        CardContent,
        CardDescription,
        CardFooter,
        CardHeader,
        CardTitle,
    } from "@/components/ui/card";
    import { onMounted, ref } from "vue";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { PulsarrGroupModel, PulsarrUserModel, RatingSystemModel } from "@/apiClient";

    const groups = ref();
    const ratingSystems = ref();
    const users = ref();

    const getGroups = () => {
        let groupDrh = new DataRequestHandler();
        groupDrh.onSuccessCallback = (data) => {
            console.log(data);
            groups.value = data as PulsarrGroupModel[];
        };
        groupDrh.get("/group/");
    };

    const getRatingSystems = () => {
        let ratingSystemDrh = new DataRequestHandler();
        ratingSystemDrh.onSuccessCallback = (data) => {
            console.log(data);
            ratingSystems.value = data as RatingSystemModel[];
        };
        ratingSystemDrh.get("/rating-system/");
    };

    const getUsers = () => {
        let userDrh = new DataRequestHandler();
        userDrh.onSuccessCallback = (data) => {
            console.log(data);
            users.value = data as PulsarrUserModel[];
        };
        userDrh.get("/user/");
    }

    onMounted(() => {
        getGroups()
        getRatingSystems()
        getUsers()
    })

</script>

<template>
    <div class="p-2">
        <Card>
            <CardHeader>
                <CardTitle>
                    Groups
                </CardTitle>
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
                                        {{ ratingSystems?.find(x => x.rating_system_id == group.rating_system_id).name }}
                                    </CardTitle>
                                    <CardDescription>
                                        Rating System
                                    </CardDescription>
                                </CardHeader>
                                <CardContent>
                                    <div>
                                        <span class="font-semibold">Rating type: </span>{{ ratingSystems?.find(x => x.rating_system_id == group.rating_system_id).master_rating_type }}
                                    </div>
                                    <div>
                                        <span class="font-semibold">Max Rating: </span>{{ ratingSystems?.find(x => x.rating_system_id == group.rating_system_id).rating_max }}
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
