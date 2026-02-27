<script setup lang="ts">
    import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";
    import { Button } from "@/components/ui/button";
    import { Label } from "@/components/ui/label";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { toast } from 'vue-sonner';
    import AddRatingSystemModal, { type CustomSystemData } from "@/components/modals/AddRatingSystemModal.vue";
    import { computed, nextTick, ref, watch } from "vue";
    import { onClickOutside } from "@vueuse/core";
    import { Search, ChevronDown, Check, AlertTriangle, X } from "lucide-vue-next";

    interface TemplateParameterDTO {
        template_parameter_id: number;
        template_id: number;
        name: string;
        parameter_rating_max: string;
        weight: string;
    }

    interface TemplateDTO {
        template_id: number;
        name: string;
        master_rating_type: string;
        rating_max: string;
        parameters?: TemplateParameterDTO[];
    }

    const props = defineProps<{
        open: boolean;
        groupId: number;
    }>();

    const emit = defineEmits<{
        'update:open': [value: boolean];
        'changed': [];
    }>();

    const templates = ref<TemplateDTO[]>([]);
    const selectedTemplateId = ref<number | null>(null);
    const showAddRatingSystemModal = ref(false);
    const submitting = ref(false);
    const customSystem = ref<CustomSystemData | null>(null);

    // Searchable dropdown state
    const searchQuery = ref('');
    const dropdownOpen = ref(false);
    const dropdownRef = ref<HTMLElement | null>(null);
    const searchInputRef = ref<HTMLInputElement | null>(null);

    const filteredTemplates = computed(() => {
        const query = searchQuery.value.toLowerCase().trim();
        if (!query) return templates.value;
        return templates.value.filter(t =>
            t.name.toLowerCase().includes(query) ||
            t.master_rating_type.toLowerCase().includes(query)
        );
    });

    const selectedTemplate = computed(() =>
        templates.value.find(t => t.template_id === selectedTemplateId.value)
    );

    const hasSelection = computed(() =>
        customSystem.value !== null || selectedTemplateId.value !== null
    );

    const selectTemplate = (template: TemplateDTO) => {
        selectedTemplateId.value = template.template_id;
        dropdownOpen.value = false;
        searchQuery.value = '';
    };

    const onCustomSystemCreated = (data: CustomSystemData) => {
        customSystem.value = data;
        selectedTemplateId.value = null;
    };

    const clearCustomSystem = () => {
        customSystem.value = null;
    };

    const toggleDropdown = () => {
        dropdownOpen.value = !dropdownOpen.value;
        if (dropdownOpen.value) {
            nextTick(() => searchInputRef.value?.focus());
        }
    };

    onClickOutside(dropdownRef, () => {
        dropdownOpen.value = false;
        searchQuery.value = '';
    });

    const fetchTemplates = () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            templates.value = data as TemplateDTO[];
        };
        drh.onErrorCallback = (error) => {
            console.error('Failed to fetch templates:', error);
        };
        drh.get('/rating-system-template/');
    };

    watch(() => props.open, (isOpen) => {
        if (isOpen) {
            fetchTemplates();
            selectedTemplateId.value = null;
            customSystem.value = null;
            searchQuery.value = '';
        }
    });

    const confirm = () => {
        if (!hasSelection.value) return;
        submitting.value = true;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            submitting.value = false;
            toast.success('Rating system changed');
            emit('changed');
            emit('update:open', false);
        };
        drh.onErrorCallback = (err) => {
            submitting.value = false;
            console.error('Failed to change rating system:', err);
            toast.error('Failed to change rating system');
        };

        if (customSystem.value) {
            // Inline creation from custom system data
            drh.post(`/group/changeRatingSystem/${props.groupId}`, {
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
            });
        } else {
            // Clone from template
            const template = selectedTemplate.value!;
            drh.post(`/group/changeRatingSystem/${props.groupId}`, {
                rating_system_id: 0,
                name: template.name,
                master_rating_type: template.master_rating_type,
                rating_max: template.rating_max,
                template_id: template.template_id,
            });
        }
    };
</script>

<template>
    <Dialog :open="open" @update:open="emit('update:open', $event)">
        <DialogContent class="max-w-md">
            <DialogHeader>
                <DialogTitle>Change Rating System</DialogTitle>
                <DialogDescription>
                    Select a new rating system for this group.
                </DialogDescription>
            </DialogHeader>

            <div class="space-y-4">
                <!-- Warning -->
                <div class="flex items-start gap-2 p-3 bg-amber-500/10 border border-amber-500/20 rounded-md text-sm">
                    <AlertTriangle class="w-4 h-4 text-amber-500 flex-shrink-0 mt-0.5" />
                    <span class="text-amber-700 dark:text-amber-400">
                        Existing ratings will be marked as outdated. They won't be deleted, but will be visually distinguished from ratings made under the new system.
                    </span>
                </div>

                <!-- Rating System Selection -->
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
                        <div class="relative flex-1" ref="dropdownRef">
                            <button
                                type="button"
                                @click="toggleDropdown"
                                :class="[
                                    'flex h-9 w-full items-center justify-between rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm ring-offset-background focus:outline-none focus:ring-1 focus:ring-ring text-start',
                                    { 'text-muted-foreground': !selectedTemplate }
                                ]"
                                :disabled="templates.length === 0"
                            >
                                <span class="truncate">
                                    {{ selectedTemplate
                                        ? `${selectedTemplate.name} (${selectedTemplate.master_rating_type} - max: ${selectedTemplate.rating_max})`
                                        : (templates.length === 0
                                            ? 'No templates available'
                                            : 'Select a template')
                                    }}
                                </span>
                                <ChevronDown class="w-4 h-4 opacity-50 shrink-0 ml-2" />
                            </button>

                            <div
                                v-if="dropdownOpen"
                                class="absolute z-50 mt-1 w-full rounded-md border bg-popover text-popover-foreground shadow-md"
                            >
                                <div class="p-2">
                                    <div class="relative">
                                        <Search class="absolute left-2 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground" />
                                        <input
                                            ref="searchInputRef"
                                            v-model="searchQuery"
                                            placeholder="Search templates..."
                                            class="flex h-8 w-full rounded-md border border-input bg-transparent pl-8 pr-3 py-1 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring"
                                        />
                                    </div>
                                </div>
                                <div class="max-h-48 overflow-y-auto p-1">
                                    <div
                                        v-for="template in filteredTemplates"
                                        :key="template.template_id"
                                        @click="selectTemplate(template)"
                                        class="flex w-full cursor-pointer items-center rounded-sm py-1.5 px-2 text-sm hover:bg-accent hover:text-accent-foreground"
                                    >
                                        <Check
                                            class="w-3.5 h-3.5 mr-2 shrink-0"
                                            :class="selectedTemplateId === template.template_id ? 'opacity-100' : 'opacity-0'"
                                        />
                                        <div class="min-w-0">
                                            <span class="truncate block">{{ template.name }}</span>
                                            <span class="text-xs text-muted-foreground">{{ template.master_rating_type }} - max: {{ template.rating_max }}</span>
                                        </div>
                                    </div>
                                    <div v-if="filteredTemplates.length === 0" class="py-2 text-center text-sm text-muted-foreground">
                                        No templates found
                                    </div>
                                </div>
                            </div>
                        </div>

                        <AddRatingSystemModal :show-dialog="showAddRatingSystemModal" @update:showDialog="showAddRatingSystemModal = $event" @created="onCustomSystemCreated">
                            <template #openModal>
                                <Button variant="outline" class="whitespace-nowrap" @click="showAddRatingSystemModal = true">
                                    Create Custom
                                </Button>
                            </template>
                        </AddRatingSystemModal>
                    </div>
                </div>

                <!-- Selected template preview (only when using template, not custom) -->
                <div v-if="selectedTemplate && !customSystem" class="p-3 bg-muted/50 rounded-md space-y-1">
                    <div class="text-sm font-medium">{{ selectedTemplate.name }}</div>
                    <div class="text-xs text-muted-foreground">
                        {{ selectedTemplate.master_rating_type }} · max {{ selectedTemplate.rating_max }}
                    </div>
                    <div v-if="selectedTemplate.parameters && selectedTemplate.parameters.length > 0" class="flex flex-wrap gap-1 mt-1">
                        <span
                            v-for="param in selectedTemplate.parameters"
                            :key="param.template_parameter_id"
                            class="px-1.5 py-0.5 bg-secondary rounded text-xs"
                        >
                            {{ param.name }}
                        </span>
                    </div>
                </div>
            </div>

            <DialogFooter>
                <Button variant="outline" @click="emit('update:open', false)">Cancel</Button>
                <Button
                    :disabled="!hasSelection || submitting"
                    @click="confirm"
                >
                    {{ submitting ? 'Changing...' : 'Change Rating System' }}
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
