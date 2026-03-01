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

    interface TemplateDTO {
        template_id: number;
        name: string;
        master_rating_type: string;
        rating_max: string;
        parameters?: { template_parameter_id: number; template_id: number; name: string; parameter_rating_max: string; weight: string }[];
    }
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import AddRatingSystemModal, { type CustomSystemData } from "@/components/modals/AddRatingSystemModal.vue";
    import { computed, nextTick, onMounted, ref } from "vue";
    import { onClickOutside } from "@vueuse/core";
    import { Search, ChevronDown, Check, X } from "lucide-vue-next";

    const { privacyTypes } = storeToRefs(useContextStore());
    const templates = ref<TemplateDTO[]>();
    const showAddRatingSystemModal = ref(false);
    const customSystem = ref<CustomSystemData | null>(null);

    // Rating system searchable dropdown state
    const rsSearchQuery = ref('');
    const rsDropdownOpen = ref(false);
    const rsDropdownRef = ref<HTMLElement | null>(null);
    const rsSearchInputRef = ref<HTMLInputElement | null>(null);

    const filteredSystems = computed(() => {
        const query = rsSearchQuery.value.toLowerCase().trim();
        if (query) {
            return templates.value?.filter(t =>
                t.name.toLowerCase().includes(query) ||
                t.master_rating_type.toLowerCase().includes(query)
            ) ?? [];
        }
        return templates.value ?? [];
    });

    const selectedSystem = computed(() =>
        templates.value?.find(t => t.template_id === ratingSystemId.value)
    );

    const hasRatingSystem = computed(() =>
        customSystem.value !== null || (ratingSystemId.value && ratingSystemId.value !== 0)
    );

    const selectSystem = (template: TemplateDTO) => {
        ratingSystemId.value = template.template_id;
        rsDropdownOpen.value = false;
        rsSearchQuery.value = '';
    };

    const onCustomSystemCreated = (data: CustomSystemData) => {
        customSystem.value = data;
        ratingSystemId.value = 0;
    };

    const clearCustomSystem = () => {
        customSystem.value = null;
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
        getTemplates()
    })

    const getTemplates = () => {
        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            templates.value = data as TemplateDTO[]
        }
        drh.onErrorCallback = (error) => {
            console.error('Failed to fetch templates:', error)
        }
        drh.get("/rating-system-template/")
    }

    const closeDialog = () => {
        emit('update:showDialog', false)
    }

    const schema = yup.object({
        name: yup.string().required().min(3).max(50),
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
    const [ratingSystemId] = defineField('rating_system_id');
    const [privacyType, privacyTypeProps] = defineField('privacy_type');

    const ratingSystemError = ref('');

    const onSubmit = handleSubmit((values) => {
        if (!hasRatingSystem.value) {
            ratingSystemError.value = 'Please select a rating system or create a custom one';
            return;
        }
        ratingSystemError.value = '';

        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            console.log('Group created:', data)
            toast.success('Group created')
            closeDialog()
            resetForm()
            customSystem.value = null;
        }
        drh.onErrorCallback = (error) => {
            console.error('Failed to create group:', error)
            toast.error('Failed to create group')
        }

        let groupPayload;

        if (customSystem.value) {
            // Inline creation — backend creates the rating system from the DTO
            groupPayload = {
                pulsarr_group_id: 0,
                name: values.name,
                rating_system_id: 0,
                privacy_type: values.privacy_type,
                rating_system: {
                    rating_system_id: 0,
                    name: customSystem.value.name,
                    master_rating_type: customSystem.value.master_rating_type,
                    rating_max: customSystem.value.rating_max,
                    parameters: customSystem.value.parameters.map(p => ({
                        rating_system_parameter_id: 0,
                        rating_system_id: 0,
                        name: p.name,
                        parameter_rating_max: p.parameter_rating_max,
                        weight: p.weight,
                    })),
                },
            }
        } else {
            // Clone from template
            groupPayload = {
                pulsarr_group_id: 0,
                name: values.name,
                rating_system_id: 0,
                privacy_type: values.privacy_type,
                rating_system: {
                    rating_system_id: 0,
                    name: '',
                    master_rating_type: '',
                    rating_max: '0',
                    template_id: ratingSystemId.value,
                },
            }
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

                    <!-- Custom system display -->
                    <div v-if="customSystem" class="flex items-center gap-3 p-3 bg-muted rounded-lg">
                        <div class="flex-1 min-w-0">
                            <div class="font-medium truncate">{{ customSystem.name }}</div>
                            <div class="text-sm text-muted-foreground">
                                {{ customSystem.master_rating_type }} · max {{ customSystem.rating_max }}
                            </div>
                            <div v-if="customSystem.parameters.length > 0" class="flex flex-wrap gap-1 mt-1">
                                <span
                                    v-for="(param, i) in customSystem.parameters"
                                    :key="i"
                                    class="px-1.5 py-0.5 bg-secondary rounded text-xs"
                                >
                                    {{ param.name }}
                                </span>
                            </div>
                        </div>
                        <Button type="button" variant="ghost" size="icon" aria-label="Remove custom system" @click="clearCustomSystem">
                            <X class="w-4 h-4" />
                        </Button>
                    </div>

                    <!-- Template picker -->
                    <div v-else class="flex gap-2">
                        <div class="relative flex-1" ref="rsDropdownRef">
                            <button
                                type="button"
                                @click="toggleRsDropdown"
                                :class="[
                                    'flex h-9 w-full items-center justify-between rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm ring-offset-background focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50 text-start',
                                    { 'border-red-500': ratingSystemError },
                                    { 'text-muted-foreground': !selectedSystem }
                                ]"
                                :disabled="!templates || templates.length === 0"
                            >
                                <span class="truncate">
                                    {{ selectedSystem
                                        ? `${selectedSystem.name} (${selectedSystem.master_rating_type} - max: ${selectedSystem.rating_max})`
                                        : (!templates || templates.length === 0
                                            ? 'No templates available'
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
                                            placeholder="Search templates..."
                                            class="flex h-8 w-full rounded-md border border-input bg-transparent pl-8 pr-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
                                        />
                                    </div>
                                </div>
                                <div class="max-h-48 overflow-y-auto p-1">
                                    <div
                                        v-for="template in filteredSystems"
                                        :key="template.template_id"
                                        @click="selectSystem(template)"
                                        class="flex w-full cursor-pointer items-center rounded-sm py-1.5 px-2 text-sm hover:bg-accent hover:text-accent-foreground"
                                    >
                                        <Check
                                            class="w-3.5 h-3.5 mr-2 shrink-0"
                                            :class="ratingSystemId === template.template_id ? 'opacity-100' : 'opacity-0'"
                                        />
                                        <span class="truncate">{{ template.name }} ({{ template.master_rating_type }} - max: {{ template.rating_max }})</span>
                                    </div>

                                    <div v-if="filteredSystems.length === 0" class="py-2 text-center text-sm text-muted-foreground">
                                        No templates found
                                    </div>
                                </div>
                            </div>
                        </div>

                        <AddRatingSystemModal :show-dialog="showAddRatingSystemModal" @update:showDialog="showAddRatingSystemModal = $event" @created="onCustomSystemCreated">
                            <template #openModal>
                                <Button type="button" variant="outline" class="whitespace-nowrap" @click="showAddRatingSystemModal = true">
                                    Create Custom
                                </Button>
                            </template>
                        </AddRatingSystemModal>
                    </div>
                    <span v-if="ratingSystemError" role="alert" class="text-red-500 text-sm">
                        {{ ratingSystemError }}
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
