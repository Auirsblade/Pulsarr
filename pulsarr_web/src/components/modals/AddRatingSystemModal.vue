<script setup lang="ts">
    import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
    import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
    import { Button } from "@/components/ui/button";
    import { Label } from "@/components/ui/label";
    import { Input } from "@/components/ui/input";
    import { useForm } from "vee-validate";
    import * as yup from "yup";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { RatingSystemDTO } from "@/apiClient";
    import { storeToRefs } from "pinia";
    import { useContextStore } from "@/stores/context.ts";

    const { ratingTypes } = storeToRefs(useContextStore());

    const props = defineProps({
        showDialog: {
            type: Boolean,
            required: true,
        },
    });

    const emit = defineEmits<{
        'update:showDialog': [value: boolean]
    }>();

    const closeDialog = () => {
        emit('update:showDialog', false)
    }

    const schema = yup.object({
        name: yup.string().required().min(3).max(50),
        master_rating_type: yup.string().required(),
        rating_max: yup.string().required(),
    });

    const { handleSubmit, resetForm, defineField, errors } = useForm({
        validationSchema: schema,
        initialValues: {
            name: '',
            master_rating_type: ratingTypes.value?.[0] ?? '',
            rating_max: ''
        }
    })

    // Add defineField for each form field
    const [name, nameProps] = defineField('name');
    const [masterRatingType, masterRatingTypeProps] = defineField('master_rating_type');
    const [ratingMax, ratingMaxProps] = defineField('rating_max');

    const onSubmit = handleSubmit(async (values) => {
        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            console.log('Rating System created:', data)
            closeDialog()
            resetForm()
        }
        drh.onErrorCallback = (error) => {
            console.error('Failed to create rating system:', error)
        }

        const ratingSystemPayload: RatingSystemDTO = {
            rating_system_id: 0, // Backend will assign real ID
            name: values.name,
            master_rating_type: values.master_rating_type,
            rating_max: values.rating_max,
            parameters: [] // Empty parameters array for initial creation
        }

        await drh.post('/rating-system/add', ratingSystemPayload)
    })

</script>

<template>
    <Dialog :open="showDialog" @update:open="(value) => emit('update:showDialog', value)">
        <DialogTrigger asChild>
            <slot name="openModal"></slot>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Create New Rating System</DialogTitle>
                <DialogDescription>
                    Create a new rating system with a name, type, and maximum rating
                </DialogDescription>
            </DialogHeader>
            <form @submit.prevent="onSubmit" class="space-y-4">
                <div class="space-y-2">
                    <Label for="name">Rating System Name</Label>
                    <Input id="name" v-model="name" :class="{ 'border-red-500': errors.name }" v-bind="nameProps"/>
                    <span v-if="errors.name" class="text-red-500 text-sm">
                        {{ errors.name }}
                    </span>
                </div>

                <div class="space-y-2">
                    <Label for="master_rating_type">Rating Type</Label>
                    <Select v-model="masterRatingType">
                        <SelectTrigger :class="{ 'border-red-500': errors.master_rating_type }" v-bind="masterRatingTypeProps">
                            <SelectValue placeholder="Select rating type"/>
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem v-for="type in ratingTypes" :key="type" :value="type">
                                {{ type }}
                            </SelectItem>
                        </SelectContent>
                    </Select>
                    <span v-if="errors.master_rating_type" class="text-red-500 text-sm">
                        {{ errors.master_rating_type }}
                    </span>
                </div>

                <div class="space-y-2">
                    <Label for="rating_max">Maximum Rating</Label>
                    <Input id="rating_max" type="number" v-model="ratingMax" :class="{ 'border-red-500': errors.rating_max }" v-bind="ratingMaxProps"/>
                    <span v-if="errors.rating_max" class="text-red-500 text-sm">
                        {{ errors.rating_max }}
                    </span>
                </div>

                <DialogFooter>
                    <Button type="submit">Create Rating System</Button>
                </DialogFooter>
            </form>
        </DialogContent>
    </Dialog>
</template>

<style scoped>

</style>
