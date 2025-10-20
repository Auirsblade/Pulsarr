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
    import type { GroupDTO, UserDTO, RatingSystem } from "@/apiClient";
    import { Plus } from 'lucide-vue-next'
    import {
        Dialog,
        DialogContent,
        DialogDescription,
        DialogHeader,
        DialogTitle,
        DialogTrigger,
        DialogFooter,
    } from "@/components/ui/dialog"
    import {
        Select,
        SelectContent,
        SelectItem,
        SelectTrigger,
        SelectValue,
    } from "@/components/ui/select"
    import { Input } from "@/components/ui/input"
    import { Label } from "@/components/ui/label"
    import { Button } from "@/components/ui/button"
    import * as yup from 'yup'
// In your component where the form is defined
import { reactive } from 'vue';
    import { useForm } from "vee-validate";

    const showDialog = ref(false);

    const groups = ref<Array<GroupDTO>>();
    const ratingSystems = ref<Array<RatingSystem>>();
    const users = ref<Array<UserDTO>>();
    const privacyTypes = ref<Array<string>>();

    onMounted(async () => {
        getGroups();
        getUsers();
        await getRatingSystems();
        await getPrivacyTypes();
    });

    const schema = yup.object({
        name: yup.string().required().min(3).max(50),
        rating_system_id: yup.number().required().positive().integer(),
        privacy_type: yup.string().required(),
    });

    reactive({
        name: '',
        rating_system_id: 0,
        privacy_type: privacyTypes.value?.[0] ?? ''
    });

    const { handleSubmit, resetForm, defineField, errors } = useForm({
        validationSchema: schema,
        initialValues: {
            name: '',
            rating_system_id: 0,
            privacy_type: privacyTypes.value?.[0] ?? ''
        }
    })

// Add defineField for each form field
const [name, nameProps] = defineField('name')
const [ratingSystemId, ratingSystemProps] = defineField('rating_system_id')
const [privacyType, privacyTypeProps] = defineField('privacy_type')

    const onSubmit = handleSubmit(async (values) => {
        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            console.log('Group created:', data)
            showDialog.value = false
            resetForm()
            getGroups() // Refresh the groups list
        }
        drh.onErrorCallback = (error) => {
            console.error('Failed to create group:', error)
        }

        const groupPayload: GroupDTO = {
            pulsarr_group_id: 0, // Backend will assign real ID
            name: values.name,
            rating_system_id: values.rating_system_id!,
            privacy_type: values.privacy_type,
        }

        await drh.post('/group/create', groupPayload)
    })

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

    const getRatingSystems = async () => {
        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            console.log('Rating systems:', data)
            ratingSystems.value = data as RatingSystem[]
        }
        drh.onErrorCallback = (error) => {
            console.error('Failed to fetch rating systems:', error)
        }
        await drh.get("/rating-system/")
    }

    const getPrivacyTypes = async () => {
        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            console.log('Privacy types:', data)
            privacyTypes.value = data as string[]
        }
        drh.onErrorCallback = (error) => {
            console.error('Failed to fetch privacy types:', error)
        }
        await drh.get("/group/privacyTypes")
    }

    const openDialog = () => showDialog.value = true;
</script>

<template>
    <div class="p-2">
        <Card>
            <CardHeader>
                <div class="flex justify-between items-center">
                    <CardTitle>Groups</CardTitle>
                    <Dialog v-model:open="showDialog">
                        <DialogTrigger asChild>
                            <Button variant="ghost" size="icon" @click="openDialog">
                                <Plus class="h-4 w-4"/>
                            </Button>
                        </DialogTrigger>
                        <DialogContent>
                            <DialogHeader>
                                <DialogTitle>Create New Group</DialogTitle>
                                <DialogDescription>
                                    Create a new group with a rating system
                                </DialogDescription>
                            </DialogHeader>
                            <form @submit.prevent="onSubmit" class="space-y-4">
                                <div class="space-y-2">
                                    <Label for="name">Name</Label>
                                    <Input
                                        id="name"
                                        v-model="name"
                                        :class="{ 'border-red-500': errors.name }"
                                        v-bind="nameProps"
                                    />
                                    <span v-if="errors.name" class="text-red-500 text-sm">
                                        {{ errors.name }}
                                    </span>
                                </div>

                                <div class="space-y-2">
                                    <Label for="privacy_type">Privacy</Label>
                                    <Select v-model="privacyType">
                                        <SelectTrigger :class="{ 'border-red-500': errors.privacy_type }" v-bind="privacyTypeProps">
                                            <SelectValue placeholder="Select privacy type"/>
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem
                                                v-for="type in privacyTypes"
                                                :key="type"
                                                :value="type"
                                            >
                                                {{ type }}
                                            </SelectItem>
                                        </SelectContent>
                                    </Select>
                                    <span v-if="errors.privacy_type" class="text-red-500 text-sm">
            {{ errors.privacy_type }}
        </span>
                                </div>

                                <div class="space-y-2">
                                    <Label for="rating_system">Rating System</Label>
                                    <Select v-model="ratingSystemId">
                                        <SelectTrigger :class="{ 'border-red-500': errors.rating_system_id }" v-bind="ratingSystemProps">
                                            <SelectValue placeholder="Select a rating system"/>
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem
                                                v-for="system in ratingSystems"
                                                :key="system.rating_system_id"
                                                :value="system.rating_system_id"
                                            >
                                                {{ system.name }} ({{ system.master_rating_type }} - max: {{ system.rating_max }})
                                            </SelectItem>
                                        </SelectContent>
                                    </Select>
                                    <span v-if="errors.rating_system_id" class="text-red-500 text-sm">
            {{ errors.rating_system_id }}
        </span>
                                </div>

                                <DialogFooter>
                                    <Button type="submit">Create Group</Button>
                                </DialogFooter>
                            </form>
                        </DialogContent>
                    </Dialog>
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