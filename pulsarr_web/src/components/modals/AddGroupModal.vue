<script setup lang="ts">
    import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
    import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
    import { Button } from "@/components/ui/button";
    import { Label } from "@/components/ui/label";
    import { Input } from "@/components/ui/input";
    import { useForm } from "vee-validate";
    import * as yup from "yup";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { toast } from 'vue-sonner';
    import type { GroupDTO, RatingSystemDTO } from "@/apiClient";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import AddRatingSystemModal from "@/components/modals/AddRatingSystemModal.vue";
    import { computed, nextTick, onMounted, type Ref, ref } from "vue";
    import { onClickOutside } from "@vueuse/core";
    import { Search, ChevronDown, Check } from "lucide-vue-next";

    const { privacyTypes } = storeToRefs(useContextStore());
    const ratingSystems = ref<RatingSystemDTO[]>();
    const showAddRatingSystemModal = ref(false);

    // Rating system searchable dropdown state
    const rsSearchQuery = ref('');
    const showAllSystems = ref(false);
    const rsDropdownOpen = ref(false);
    const rsDropdownRef = ref<HTMLElement | null>(null);
    const rsSearchInputRef = ref<HTMLInputElement | null>(null);

    const defaultSystems = computed(() =>
        ratingSystems.value?.filter(s => s.rating_system_id < 0) ?? []
    );

    const userSystems = computed(() =>
        ratingSystems.value?.filter(s => s.rating_system_id > 0) ?? []
    );

    const filteredSystems = computed(() => {
        const query = rsSearchQuery.value.toLowerCase().trim();

        if (query) {
            return ratingSystems.value?.filter(s =>
                s.name.toLowerCase().includes(query) ||
                s.master_rating_type.toLowerCase().includes(query)
            ) ?? [];
        }

        if (showAllSystems.value) {
            return ratingSystems.value ?? [];
        }

        return defaultSystems.value;
    });

    const selectedSystem = computed(() =>
        ratingSystems.value?.find(s => s.rating_system_id === ratingSystemId.value)
    );

    const selectSystem = (system: RatingSystemDTO) => {
        ratingSystemId.value = system.rating_system_id;
        rsDropdownOpen.value = false;
        rsSearchQuery.value = '';
    };

    const toggleRsDropdown = () => {
        rsDropdownOpen.value = !rsDropdownOpen.value;
        if (rsDropdownOpen.value) {
            nextTick(() => rsSearchInputRef.value?.focus());
        }
    };

    onClickOutside(rsDropdownRef, () => {
        rsDropdownOpen.value = false;
        rsSearchQuery.value = '';
    });

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

    const onSubmit = handleSubmit((values) => {
        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            console.log('Group created:', data)
            toast.success('Group created')
            closeDialog()
            resetForm()
        }
        drh.onErrorCallback = (error) => {
            console.error('Failed to create group:', error)
            toast.error('Failed to create group')
        }

        const groupPayload: GroupDTO = {
            pulsarr_group_id: 0, // Backend will assign real ID
            name: values.name,
            rating_system_id: values.rating_system_id!,
            privacy_type: values.privacy_type,
        }

        drh.post('/group/create', groupPayload)
    })

</script>

<template>
    <Dialog :open="showDialog" @update:open="(value) => emit('update:showDialog', value)">
        <DialogTrigger asChild>
            <slot name="openModal"></slot>
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
                    <Input id="name" v-model="name" :class="{ 'border-red-500': errors.name }" v-bind="nameProps" :aria-invalid="!!errors.name" :aria-describedby="errors.name ? 'group-name-error' : undefined"/>
                    <span v-if="errors.name" id="group-name-error" role="alert" class="text-red-500 text-sm">
                        {{ errors.name }}
                    </span>
                </div>

                <div class="space-y-2">
                    <Label for="privacy_type">Privacy</Label>
                    <Select v-model="privacyType">
                        <SelectTrigger :class="{ 'border-red-500': errors.privacy_type }" v-bind="privacyTypeProps" :aria-invalid="!!errors.privacy_type" :aria-describedby="errors.privacy_type ? 'privacy-type-error' : undefined">
                            <SelectValue placeholder="Select privacy type"/>
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem v-for="type in privacyTypes" :key="type" :value="type">
                                {{ type }}
                            </SelectItem>
                        </SelectContent>
                    </Select>
                    <span v-if="errors.privacy_type" id="privacy-type-error" role="alert" class="text-red-500 text-sm">
                        {{ errors.privacy_type }}
                    </span>
                </div>

                <div class="space-y-2">
                    <Label>Rating System</Label>
                    <div class="flex gap-2">
                        <div class="relative flex-1" ref="rsDropdownRef">
                            <button
                                type="button"
                                @click="toggleRsDropdown"
                                :class="[
                                    'flex h-9 w-full items-center justify-between rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm ring-offset-background focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50 text-start',
                                    { 'border-red-500': errors.rating_system_id },
                                    { 'text-muted-foreground': !selectedSystem }
                                ]"
                                :disabled="!ratingSystems || ratingSystems.length === 0"
                                :aria-invalid="!!errors.rating_system_id"
                                :aria-describedby="errors.rating_system_id ? 'rating-system-error' : undefined"
                            >
                                <span class="truncate">
                                    {{ selectedSystem
                                        ? `${selectedSystem.name} (${selectedSystem.master_rating_type} - max: ${selectedSystem.rating_max})`
                                        : (!ratingSystems || ratingSystems.length === 0
                                            ? 'No rating systems available'
                                            : 'Select a rating system')
                                    }}
                                </span>
                                <ChevronDown class="w-4 h-4 opacity-50 shrink-0 ml-2" />
                            </button>

                            <div
                                v-if="rsDropdownOpen"
                                class="absolute z-50 mt-1 w-full rounded-md border bg-popover text-popover-foreground shadow-md"
                            >
                                <div class="p-2">
                                    <div class="relative">
                                        <Search class="absolute left-2 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground" />
                                        <input
                                            ref="rsSearchInputRef"
                                            v-model="rsSearchQuery"
                                            placeholder="Search rating systems..."
                                            class="flex h-8 w-full rounded-md border border-input bg-transparent pl-8 pr-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
                                        />
                                    </div>
                                </div>
                                <div class="max-h-48 overflow-y-auto p-1">
                                    <div
                                        v-for="system in filteredSystems"
                                        :key="system.rating_system_id"
                                        @click="selectSystem(system)"
                                        class="flex w-full cursor-pointer items-center rounded-sm py-1.5 px-2 text-sm hover:bg-accent hover:text-accent-foreground"
                                    >
                                        <Check
                                            class="w-3.5 h-3.5 mr-2 shrink-0"
                                            :class="ratingSystemId === system.rating_system_id ? 'opacity-100' : 'opacity-0'"
                                        />
                                        <span class="truncate">{{ system.name }} ({{ system.master_rating_type }} - max: {{ system.rating_max }})</span>
                                    </div>

                                    <div v-if="filteredSystems.length === 0" class="py-2 text-center text-sm text-muted-foreground">
                                        No rating systems found
                                    </div>

                                    <template v-if="!rsSearchQuery && !showAllSystems && userSystems.length > 0">
                                        <div class="border-t my-1"></div>
                                        <button
                                            type="button"
                                            @click.stop="showAllSystems = true"
                                            class="flex w-full py-1.5 px-2 text-sm text-muted-foreground hover:text-foreground hover:bg-accent rounded-sm text-left"
                                        >
                                            Show all ({{ userSystems.length }} more)
                                        </button>
                                    </template>
                                </div>
                            </div>
                        </div>

                        <AddRatingSystemModal :show-dialog="showAddRatingSystemModal" @update:showDialog="(value) => { showAddRatingSystemModal = value; getRatingSystems(); }">
                            <template #openModal>
                                <Button variant="outline" class="whitespace-nowrap" @click="showAddRatingSystemModal = true">
                                    Add Rating System
                                </Button>
                            </template>
                        </AddRatingSystemModal>
                    </div>
                    <span v-if="errors.rating_system_id" id="rating-system-error" role="alert" class="text-red-500 text-sm">
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