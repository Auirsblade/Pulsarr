<script setup lang="ts">
    import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
    import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
    import { Button } from "@/components/ui/button";
    import { Label } from "@/components/ui/label";
    import { Input } from "@/components/ui/input";
    import { useForm } from "vee-validate";
    import * as yup from "yup";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { GroupDTO, RatingSystemDTO } from "@/apiClient";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import AddRatingSystemModal from "@/components/modals/AddRatingSystemModal.vue";
    import { onMounted, type Ref, ref } from "vue";

    const { privacyTypes } = storeToRefs(useContextStore());
    const ratingSystems = ref<RatingSystemDTO[]>();
    const showAddRatingSystemModal = ref(false);

    defineProps({
        showDialog: {
            type: Boolean,
            required: true,
        }
    });

    const emit = defineEmits<{
        'update:showDialog': [value: boolean]
    }>()

    onMounted(() => {
        getRatingSystems()
    })

    const getRatingSystems = () => {
        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            console.log('Rating systems:', data)
            ratingSystems.value = data as RatingSystemDTO[]
        }
        drh.onErrorCallback = (error) => {
            console.error('Failed to fetch rating systems:', error)
        }
        drh.get("/rating-system/")
    }

    const closeDialog = () => {
        emit('update:showDialog', false)
    }

    const schema = yup.object({
        name: yup.string().required().min(3).max(50),
        rating_system_id: yup.number().required().integer().notOneOf([0], 'Please select a rating system'),
        privacy_type: yup.string().required(),
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
    const [name, nameProps] = defineField('name');
    const [ratingSystemId, ratingSystemProps] = defineField('rating_system_id');
    const [privacyType, privacyTypeProps] = defineField('privacy_type');

    const onSubmit = handleSubmit(async (values) => {
        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            console.log('Group created:', data)
            closeDialog()
            resetForm()
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

</script>

<template>
    <Dialog :open="showDialog" @update:open="(value) => emit('update:showDialog', value)">
        <DialogTrigger asChild>
            <slot name="openModal"></slot>
        </DialogTrigger>
        <DialogContent class="min-w-fit">
            <DialogHeader>
                <DialogTitle>Create New Group</DialogTitle>
                <DialogDescription>
                    Create a new group with a rating system
                </DialogDescription>
            </DialogHeader>
            <form @submit.prevent="onSubmit" class="space-y-4">
                <div class="space-y-2">
                    <Label for="name">Name</Label>
                    <Input id="name" v-model="name" :class="{ 'border-red-500': errors.name }" v-bind="nameProps"/>
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
                            <SelectItem v-for="type in privacyTypes" :key="type" :value="type">
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
                    <div class="flex gap-2 ">
                        <Select v-model="ratingSystemId" :disabled="!ratingSystems || ratingSystems.length === 0" class="flex-1">
                            <SelectTrigger :class="{ 'border-red-500': errors.rating_system_id }" v-bind="ratingSystemProps">
                                <SelectValue
                                    :placeholder="!ratingSystems || ratingSystems.length === 0 ? 'No rating systems available. Please create one first.' : 'Select a rating system'"/>
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem v-for="system in ratingSystems" :key="system.rating_system_id" :value="system.rating_system_id">
                                    {{ system.name }} ({{ system.master_rating_type }} - max: {{ system.rating_max }})
                                </SelectItem>
                            </SelectContent>
                        </Select>
                        <AddRatingSystemModal :show-dialog="showAddRatingSystemModal" @update:showDialog="(value) => { showAddRatingSystemModal = value; getRatingSystems(); }">
                            <template #openModal>
                                <Button variant="outline" class="whitespace-nowrap" @click="showAddRatingSystemModal = true">
                                    Add Rating System
                                </Button>
                            </template>
                        </AddRatingSystemModal>
                    </div>
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
</template>

<style scoped>

</style>