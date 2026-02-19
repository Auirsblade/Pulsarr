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
    import type { RatingSystemDTO, RatingSystemParameterDTO } from "@/apiClient";
    import { storeToRefs } from "pinia";
    import { useContextStore } from "@/stores/context.ts";
    import { ref, computed, watch, type Ref } from "vue";
    import { Plus, Trash2 } from "lucide-vue-next";

    const { ratingTypes } = storeToRefs(useContextStore());

    const WEIGHT_OPTIONS = [
        { value: '0.5', label: '0.5x' },
        { value: '1', label: '1x' },
        { value: '1.5', label: '1.5x' },
        { value: '2', label: '2x' },
        { value: '2.5', label: '2.5x' },
        { value: '3', label: '3x' },
    ];

    interface ParameterInput {
        name: string;
        parameter_rating_max: string;
        weight: string;
    }

    const parameters = ref<ParameterInput[]>([]);
    const parameterErrors = ref<Record<number, string>>({});

    const addParameter = () => {
        parameters.value.push({ name: '', parameter_rating_max: '', weight: '1' });
    };

    const removeParameter = (index: number) => {
        parameters.value.splice(index, 1);
    };

    // Computed properties for type-aware UI
    const parametersRequired = computed(() => {
        return masterRatingType.value === 'Average' || masterRatingType.value === 'Cumulative';
    });

    const parametersLabel = computed(() => {
        if (masterRatingType.value === 'Absolute') {
            return 'Rating Parameters (Optional)';
        }
        return 'Rating Parameters (Required)';
    });

    const isManualRatingMax = computed(() => masterRatingType.value === 'Absolute');

    const computedRatingMax = computed(() => {
        if (masterRatingType.value === 'Average') {
            let totalWeight = 0;
            let weightedSum = 0;
            parameters.value.forEach(p => {
                const max = parseFloat(p.parameter_rating_max) || 0;
                const weight = parseFloat(p.weight) || 1;
                weightedSum += max * weight;
                totalWeight += weight;
            });
            if (totalWeight === 0) return '';
            const avg = weightedSum / totalWeight;
            return avg > 0 ? avg.toString() : '';
        }
        if (masterRatingType.value === 'Cumulative') {
            const sum = parameters.value.reduce((acc, p) => {
                const max = parseFloat(p.parameter_rating_max) || 0;
                const weight = parseFloat(p.weight) || 1;
                return acc + (max * weight);
            }, 0);
            return sum > 0 ? sum.toString() : '';
        }
        return '';
    });

    const parametersDescription = computed(() => {
        switch (masterRatingType.value) {
            case 'Absolute':
                return 'Add optional sub-ratings for detailed breakdown. Overall rating is manually entered.';
            case 'Average':
                return 'Overall rating = weighted average of these parameters. All parameters required.';
            case 'Cumulative':
                return 'Overall rating = sum of (parameter × weight). Max = sum of all (parameter max × weight).';
            default:
                return 'Add sub-ratings for more detailed reviews (e.g., Lyrics, Production, Vocals)';
        }
    });

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

    watch([computedRatingMax, () => masterRatingType.value], () => {
        if (!isManualRatingMax.value) {
            ratingMax.value = computedRatingMax.value;
        }
    });

    const validateParameters = (): boolean => {
        parameterErrors.value = {};
        let valid = true;
        parameters.value.forEach((p, i) => {
            if (!p.name.trim()) {
                parameterErrors.value[i] = 'name';
                valid = false;
            }
            if (!p.parameter_rating_max) {
                parameterErrors.value[i] = parameterErrors.value[i] ? 'both' : 'max';
                valid = false;
            }
        });
        return valid;
    };

    const onSubmit = handleSubmit((values) => {
        if (parameters.value.length > 0 && !validateParameters()) {
            toast.error('Please fill in all parameter fields')
            return
        }

        const drh = new DataRequestHandler()
        drh.onSuccessCallback = (data) => {
            console.log('Rating System created:', data)
            toast.success('Rating system created')
            closeDialog()
            resetForm()
            parameters.value = []
            parameterErrors.value = {}
        }
        drh.onErrorCallback = (error) => {
            console.error('Failed to create rating system:', error)
            toast.error('Failed to create rating system')
        }

        const ratingSystemPayload: RatingSystemDTO = {
            rating_system_id: 0, // Backend will assign real ID
            name: values.name,
            master_rating_type: values.master_rating_type,
            rating_max: values.rating_max,
            parameters: parameters.value.map((p) => ({
                rating_system_parameter_id: 0,
                rating_system_id: 0,
                name: p.name,
                parameter_rating_max: p.parameter_rating_max,
                weight: p.weight
            }))
        }

        drh.post('/rating-system/add', ratingSystemPayload)
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
                    <Input id="name" v-model="name" :class="{ 'border-red-500': errors.name }" v-bind="nameProps" :aria-invalid="!!errors.name" :aria-describedby="errors.name ? 'rs-name-error' : undefined"/>
                    <span v-if="errors.name" id="rs-name-error" role="alert" class="text-red-500 text-sm">
                        {{ errors.name }}
                    </span>
                </div>

                <div class="space-y-2">
                    <Label for="master_rating_type">Rating Type</Label>
                    <Select v-model="masterRatingType">
                        <SelectTrigger :class="{ 'border-red-500': errors.master_rating_type }" v-bind="masterRatingTypeProps" :aria-invalid="!!errors.master_rating_type" :aria-describedby="errors.master_rating_type ? 'rating-type-error' : undefined">
                            <SelectValue placeholder="Select rating type"/>
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem v-for="type in ratingTypes" :key="type" :value="type">
                                {{ type }}
                            </SelectItem>
                        </SelectContent>
                    </Select>
                    <span v-if="errors.master_rating_type" id="rating-type-error" role="alert" class="text-red-500 text-sm">
                        {{ errors.master_rating_type }}
                    </span>
                </div>

                <div class="space-y-2">
                    <Label for="rating_max">Maximum Rating</Label>
                    <Input
                        id="rating_max"
                        type="number"
                        v-model="ratingMax"
                        :disabled="!isManualRatingMax"
                        :class="{ 'border-red-500': errors.rating_max }"
                        v-bind="ratingMaxProps"
                        :aria-invalid="!!errors.rating_max"
                        :aria-describedby="errors.rating_max ? 'rating-max-error' : undefined"
                    />
                    <p v-if="!isManualRatingMax" class="text-xs text-muted-foreground">
                        Automatically calculated from parameters.
                    </p>
                    <span v-if="errors.rating_max" id="rating-max-error" role="alert" class="text-red-500 text-sm">
                        {{ errors.rating_max }}
                    </span>
                </div>

                <div class="space-y-2">
                    <div class="flex items-center justify-between">
                        <Label>{{ parametersLabel }}</Label>
                        <Button type="button" variant="outline" size="sm" @click="addParameter">
                            <Plus class="w-4 h-4 mr-1" />
                            Add Parameter
                        </Button>
                    </div>
                    <p class="text-sm text-muted-foreground">
                        {{ parametersDescription }}
                    </p>
                    <div v-if="parameters.length > 0" class="space-y-3 mt-2">
                        <div v-for="(param, index) in parameters" :key="index" class="flex gap-2 items-end">
                            <div class="flex-1 space-y-1">
                                <Label :for="`param-name-${index}`" class="text-xs">Name</Label>
                                <Input
                                    :id="`param-name-${index}`"
                                    v-model="param.name"
                                    placeholder="e.g., Lyrics"
                                    :class="{ 'border-red-500': parameterErrors[index] === 'name' || parameterErrors[index] === 'both' }"
                                />
                            </div>
                            <div class="w-20 space-y-1">
                                <Label :for="`param-max-${index}`" class="text-xs">Max</Label>
                                <Input
                                    :id="`param-max-${index}`"
                                    type="number"
                                    v-model="param.parameter_rating_max"
                                    placeholder="10"
                                    :class="{ 'border-red-500': parameterErrors[index] === 'max' || parameterErrors[index] === 'both' }"
                                />
                            </div>
                            <div class="w-20 space-y-1">
                                <Label :for="`param-weight-${index}`" class="text-xs">Weight</Label>
                                <Select v-model="param.weight">
                                    <SelectTrigger :id="`param-weight-${index}`">
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem
                                            v-for="opt in WEIGHT_OPTIONS"
                                            :key="opt.value"
                                            :value="opt.value"
                                        >
                                            {{ opt.label }}
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                            </div>
                            <Button
                                type="button"
                                variant="ghost"
                                size="icon"
                                class="text-destructive hover:text-destructive"
                                :aria-label="'Delete parameter ' + (param.name || index + 1)"
                                @click="removeParameter(index)"
                            >
                                <Trash2 class="w-4 h-4" />
                            </Button>
                        </div>
                    </div>
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
