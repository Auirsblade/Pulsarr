<script setup lang="ts">
    import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
    import { Button } from "@/components/ui/button";
    import { Label } from "@/components/ui/label";
    import { Input } from "@/components/ui/input";
    import { ref, computed } from "vue";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { toast } from 'vue-sonner';
    import type { GroupDTO } from "@/apiClient";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import { Music, X, Disc, User } from "lucide-vue-next";
    import MusicSearchModal, { type SelectedMusic } from "@/components/modals/MusicSearchModal.vue";

    interface ParameterRating {
        rating_system_parameter_id: number;
        name: string;
        max: string;
        weight: string;
        value: string;
    }

    const props = defineProps<{
        showDialog: boolean;
        group: GroupDTO;
    }>();

    const emit = defineEmits<{
        'update:showDialog': [value: boolean];
        'created': [];
    }>();

    const { user } = storeToRefs(useContextStore());

    const selectedMusic = ref<SelectedMusic | null>(null);
    const showMusicSearch = ref(false);
    const ratingValue = ref('');
    const comments = ref('');
    const parameterRatings = ref<ParameterRating[]>([]);
    const submitting = ref(false);
    const error = ref<string | null>(null);

    const ratingMax = computed(() => {
        return props.group.rating_system?.rating_max || '10';
    });

    // Rating type and calculation logic
    const ratingType = computed(() => {
        return props.group.rating_system?.master_rating_type || 'Absolute';
    });

    const isManualOverall = computed(() => {
        return ratingType.value === 'Absolute';
    });

    const hasParameters = computed(() => {
        return parameterRatings.value.length > 0;
    });

    const allParametersFilled = computed(() => {
        return parameterRatings.value.every(p => p.value !== '');
    });

    const calculatedRating = computed(() => {
        if (isManualOverall.value || !hasParameters.value) return null;
        if (!allParametersFilled.value) return null;

        const ratings = parameterRatings.value;

        if (ratingType.value === 'Average') {
            // Weighted average: (sum of value * weight) / (sum of weights)
            let weightedSum = 0;
            let totalWeight = 0;
            ratings.forEach(r => {
                const value = parseFloat(r.value) || 0;
                const weight = parseFloat(r.weight) || 1;
                weightedSum += value * weight;
                totalWeight += weight;
            });
            return totalWeight > 0 ? (weightedSum / totalWeight).toFixed(2) : '0';
        }

        if (ratingType.value === 'Cumulative') {
            // Weighted sum: sum of (value * weight)
            let sum = 0;
            ratings.forEach(r => {
                const value = parseFloat(r.value) || 0;
                const weight = parseFloat(r.weight) || 1;
                sum += value * weight;
            });
            return sum.toFixed(2);
        }

        return null;
    });

    const calculatedMax = computed(() => {
        if (ratingType.value !== 'Cumulative' || !hasParameters.value) return null;

        // Sum of (param_max * weight)
        let maxSum = 0;
        parameterRatings.value.forEach(p => {
            const max = parseFloat(p.max) || 0;
            const weight = parseFloat(p.weight) || 1;
            maxSum += max * weight;
        });
        return maxSum.toFixed(2);
    });

    const displayMax = computed(() => {
        if (ratingType.value === 'Cumulative' && calculatedMax.value) {
            return calculatedMax.value;
        }
        return ratingMax.value;
    });

    const initializeParameterRatings = () => {
        if (props.group.rating_system?.parameters) {
            parameterRatings.value = props.group.rating_system.parameters.map(p => ({
                rating_system_parameter_id: p.rating_system_parameter_id,
                name: p.name,
                max: p.parameter_rating_max,
                weight: p.weight || '1',
                value: ''
            }));
        }
    };

    const onMusicSelected = (music: SelectedMusic) => {
        selectedMusic.value = music;
        initializeParameterRatings();
    };

    const clearSelection = () => {
        selectedMusic.value = null;
        ratingValue.value = '';
        comments.value = '';
        parameterRatings.value = [];
    };

    const closeDialog = () => {
        emit('update:showDialog', false);
        clearSelection();
        error.value = null;
    };

    const getMusicIcon = () => {
        if (!selectedMusic.value) return Music;
        switch (selectedMusic.value.media_type) {
            case 'release': return Disc;
            case 'artist': return User;
            default: return Music;
        }
    };

    const submit = () => {
        if (!selectedMusic.value || !user.value) return;

        // For non-Absolute types with parameters, validate all parameters are filled
        if (!isManualOverall.value && hasParameters.value) {
            if (!allParametersFilled.value) {
                error.value = 'All parameter ratings are required';
                return;
            }
        }

        // Determine the final rating value
        let finalRatingValue: string;
        if (isManualOverall.value) {
            if (!ratingValue.value) {
                error.value = 'Please enter a rating';
                return;
            }
            finalRatingValue = ratingValue.value;
        } else {
            if (!calculatedRating.value) {
                error.value = 'Please rate all parameters';
                return;
            }
            finalRatingValue = calculatedRating.value;
        }

        const rating = parseFloat(finalRatingValue);
        const max = parseFloat(displayMax.value);
        if (isNaN(rating) || rating < 0 || rating > max) {
            error.value = `Rating must be between 0 and ${max}`;
            return;
        }

        submitting.value = true;
        error.value = null;

        // Format date for NaiveDateTime (no timezone, format: YYYY-MM-DDTHH:MM:SS)
        const formatNaiveDateTime = (date: Date): string => {
            return date.toISOString().slice(0, 19);
        };

        const releaseDate = selectedMusic.value.release_date
            ? new Date(selectedMusic.value.release_date)
            : new Date();

        const ratingPayload = {
            rating_id: 0,
            pulsarr_user_id: user.value.pulsarr_user_id,
            pulsarr_group_id: props.group.pulsarr_group_id,
            rating_system_id: props.group.rating_system_id,
            comments: comments.value,
            rating_value: finalRatingValue,
            media_type: selectedMusic.value.media_type,
            media_title: selectedMusic.value.media_title,
            musicbrainz_id: selectedMusic.value.musicbrainz_id,
            artist_name: selectedMusic.value.artist_name,
            release_date: formatNaiveDateTime(releaseDate)
        };

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data: any) => {
            const createdRatingId = data.rating_id;

            // Create rating details for each parameter
            const details = parameterRatings.value.filter(p => p.value);
            if (details.length > 0) {
                let remaining = details.length;
                const onDetailDone = () => {
                    remaining--;
                    if (remaining === 0) {
                        submitting.value = false;
                        toast.success('Rating submitted');
                        emit('created');
                        closeDialog();
                    }
                };
                for (const p of details) {
                    const detailDrh = new DataRequestHandler();
                    detailDrh.onSuccessCallback = () => onDetailDone();
                    detailDrh.onErrorCallback = () => {
                        console.error('Failed to create rating detail');
                        onDetailDone();
                    };
                    detailDrh.post('/rating/rating_detail/add', {
                        rating_detail_id: 0,
                        rating_id: createdRatingId,
                        rating_system_parameter_id: p.rating_system_parameter_id,
                        rating_value: p.value
                    });
                }
            } else {
                submitting.value = false;
                toast.success('Rating submitted');
                emit('created');
                closeDialog();
            }
        };
        drh.onErrorCallback = (err) => {
            error.value = 'Failed to create rating';
            submitting.value = false;
            toast.error('Failed to submit rating');
            console.error('Failed to create rating:', err);
        };

        drh.post('/rating/add', ratingPayload);
    };
</script>

<template>
    <Dialog :open="showDialog" @update:open="closeDialog">
        <DialogTrigger asChild>
            <slot name="openModal"></slot>
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Add Rating</DialogTitle>
                <DialogDescription>
                    Rate music in {{ group.name }}
                </DialogDescription>
            </DialogHeader>

            <div class="space-y-4">
                <!-- Music Selection -->
                <div class="space-y-2">
                    <Label>Music</Label>
                    <div v-if="!selectedMusic" class="border-2 border-dashed rounded-lg p-6 text-center">
                        <Music class="w-8 h-8 mx-auto mb-2 text-muted-foreground" />
                        <p class="text-sm text-muted-foreground mb-3">Select music to rate</p>
                        <MusicSearchModal
                            :show-dialog="showMusicSearch"
                            @update:showDialog="showMusicSearch = $event"
                            @select="onMusicSelected"
                        >
                            <template #openModal>
                                <Button variant="outline" @click="showMusicSearch = true">
                                    Search Music
                                </Button>
                            </template>
                        </MusicSearchModal>
                    </div>
                    <div v-else class="flex items-center gap-3 p-3 bg-muted rounded-lg">
                        <div class="w-12 h-12 bg-background rounded flex items-center justify-center overflow-hidden">
                            <img
                                v-if="selectedMusic.cover_art_url"
                                :src="selectedMusic.cover_art_url"
                                class="w-full h-full object-cover"
                                :alt="'Album cover for ' + selectedMusic.media_title"
                            />
                            <component :is="getMusicIcon()" v-else class="w-6 h-6 text-muted-foreground" />
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="font-medium truncate">{{ selectedMusic.media_title }}</div>
                            <div class="text-sm text-muted-foreground truncate">{{ selectedMusic.artist_name }}</div>
                            <div class="text-xs text-muted-foreground capitalize">{{ selectedMusic.media_type }}</div>
                        </div>
                        <Button variant="ghost" size="icon" aria-label="Clear music selection" @click="clearSelection">
                            <X class="w-4 h-4" />
                        </Button>
                    </div>
                </div>

                <!-- Parameter Ratings (shown first for non-Absolute types) -->
                <div v-if="selectedMusic && hasParameters" class="space-y-3">
                    <div class="flex items-center justify-between">
                        <Label>{{ isManualOverall ? 'Detailed Ratings (Optional)' : 'Parameter Ratings (Required)' }}</Label>
                    </div>
                    <p v-if="!isManualOverall" class="text-xs text-muted-foreground">
                        {{ ratingType === 'Average' ? 'Overall = weighted average of these ratings' : 'Overall = sum of (rating × weight)' }}
                    </p>
                    <div class="space-y-2">
                        <div
                            v-for="param in parameterRatings"
                            :key="param.rating_system_parameter_id"
                            class="flex items-center gap-2"
                        >
                            <div class="flex-1">
                                <Label class="text-sm">{{ param.name }}</Label>
                                <span class="text-xs text-muted-foreground ml-1">
                                    (0-{{ param.max }}{{ parseFloat(param.weight) !== 1 ? `, ${param.weight}x` : '' }})
                                </span>
                            </div>
                            <Input
                                type="number"
                                v-model="param.value"
                                :min="0"
                                :max="param.max"
                                step="0.1"
                                class="w-24"
                                placeholder="--"
                            />
                        </div>
                    </div>
                </div>

                <!-- Overall Rating -->
                <div v-if="selectedMusic" class="space-y-2">
                    <Label for="rating">Overall Rating (0 - {{ displayMax }})</Label>

                    <!-- Manual input for Absolute type -->
                    <Input
                        v-if="isManualOverall"
                        id="rating"
                        type="number"
                        v-model="ratingValue"
                        :min="0"
                        :max="ratingMax"
                        step="0.1"
                        placeholder="Enter rating..."
                    />

                    <!-- Auto-calculated display for Average/Cumulative -->
                    <div v-else class="flex items-center gap-3">
                        <div class="px-4 py-2 bg-muted rounded-md font-mono text-xl font-semibold min-w-[80px] text-center">
                            {{ calculatedRating ?? '--' }}
                        </div>
                        <span class="text-sm text-muted-foreground">
                            (auto-calculated from parameters)
                        </span>
                    </div>
                </div>

                <!-- Comments -->
                <div v-if="selectedMusic" class="space-y-2">
                    <Label for="comments">Comments (optional)</Label>
                    <textarea
                        id="comments"
                        v-model="comments"
                        class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                        placeholder="Share your thoughts..."
                    />
                </div>

                <!-- Error -->
                <div v-if="error" class="text-sm text-destructive" role="alert">
                    {{ error }}
                </div>
            </div>

            <DialogFooter>
                <Button variant="outline" @click="closeDialog">Cancel</Button>
                <Button
                    :disabled="!selectedMusic || submitting || (isManualOverall ? !ratingValue : !calculatedRating)"
                    @click="submit"
                >
                    {{ submitting ? 'Submitting...' : 'Submit Rating' }}
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
