<script setup lang="ts">
    import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
    import { Button } from "@/components/ui/button";
    import { Label } from "@/components/ui/label";
    import { Input } from "@/components/ui/input";
    import { ref, computed } from "vue";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import type { GroupDTO } from "@/apiClient";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import { Music, X, Disc, User } from "lucide-vue-next";
    import MusicSearchModal, { type SelectedMusic } from "@/components/modals/MusicSearchModal.vue";

    interface ParameterRating {
        rating_system_parameter_id: number;
        name: string;
        max: string;
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

    const initializeParameterRatings = () => {
        if (props.group.rating_system?.parameters) {
            parameterRatings.value = props.group.rating_system.parameters.map(p => ({
                rating_system_parameter_id: p.rating_system_parameter_id,
                name: p.name,
                max: p.parameter_rating_max,
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

    const submit = async () => {
        if (!selectedMusic.value || !user.value) return;

        if (!ratingValue.value) {
            error.value = 'Please enter a rating';
            return;
        }

        const rating = parseFloat(ratingValue.value);
        const max = parseFloat(ratingMax.value);
        if (isNaN(rating) || rating < 0 || rating > max) {
            error.value = `Rating must be between 0 and ${max}`;
            return;
        }

        submitting.value = true;
        error.value = null;

        const ratingPayload = {
            rating_id: 0,
            pulsarr_user_id: user.value.pulsarr_user_id,
            pulsarr_group_id: props.group.pulsarr_group_id,
            rating_system_id: props.group.rating_system_id,
            comments: comments.value,
            rating_value: ratingValue.value,
            media_type: selectedMusic.value.media_type,
            media_title: selectedMusic.value.media_title,
            musicbrainz_id: selectedMusic.value.musicbrainz_id,
            artist_name: selectedMusic.value.artist_name,
            rating_date: new Date().toISOString(),
            release_date: selectedMusic.value.release_date
                ? new Date(selectedMusic.value.release_date).toISOString()
                : new Date().toISOString()
        };

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = async (data: any) => {
            const createdRatingId = data.rating_id;

            // Create rating details for each parameter
            if (parameterRatings.value.length > 0) {
                const detailPromises = parameterRatings.value
                    .filter(p => p.value)
                    .map(p => {
                        return new Promise<void>((resolve, reject) => {
                            const detailDrh = new DataRequestHandler();
                            detailDrh.onSuccessCallback = () => resolve();
                            detailDrh.onErrorCallback = () => reject();
                            detailDrh.post('/rating/rating_detail/add', {
                                rating_detail_id: 0,
                                rating_id: createdRatingId,
                                rating_system_parameter_id: p.rating_system_parameter_id,
                                rating_value: p.value
                            });
                        });
                    });

                try {
                    await Promise.all(detailPromises);
                } catch (e) {
                    console.error('Failed to create some rating details:', e);
                }
            }

            submitting.value = false;
            emit('created');
            closeDialog();
        };
        drh.onErrorCallback = (err) => {
            error.value = 'Failed to create rating';
            submitting.value = false;
            console.error('Failed to create rating:', err);
        };

        await drh.post('/rating/add', ratingPayload);
    };
</script>

<template>
    <Dialog :open="showDialog" @update:open="closeDialog">
        <DialogTrigger asChild>
            <slot name="openModal"></slot>
        </DialogTrigger>
        <DialogContent class="max-w-lg">
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
                                alt=""
                            />
                            <component :is="getMusicIcon()" v-else class="w-6 h-6 text-muted-foreground" />
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="font-medium truncate">{{ selectedMusic.media_title }}</div>
                            <div class="text-sm text-muted-foreground truncate">{{ selectedMusic.artist_name }}</div>
                            <div class="text-xs text-muted-foreground capitalize">{{ selectedMusic.media_type }}</div>
                        </div>
                        <Button variant="ghost" size="icon" @click="clearSelection">
                            <X class="w-4 h-4" />
                        </Button>
                    </div>
                </div>

                <!-- Overall Rating -->
                <div v-if="selectedMusic" class="space-y-2">
                    <Label for="rating">Overall Rating (0 - {{ ratingMax }})</Label>
                    <Input
                        id="rating"
                        type="number"
                        v-model="ratingValue"
                        :min="0"
                        :max="ratingMax"
                        step="0.1"
                        placeholder="Enter rating..."
                    />
                </div>

                <!-- Parameter Ratings -->
                <div v-if="selectedMusic && parameterRatings.length > 0" class="space-y-3">
                    <Label>Detailed Ratings</Label>
                    <div class="space-y-2">
                        <div
                            v-for="param in parameterRatings"
                            :key="param.rating_system_parameter_id"
                            class="flex items-center gap-2"
                        >
                            <Label class="flex-1 text-sm">{{ param.name }} (0 - {{ param.max }})</Label>
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
                <div v-if="error" class="text-sm text-destructive">
                    {{ error }}
                </div>
            </div>

            <DialogFooter>
                <Button variant="outline" @click="closeDialog">Cancel</Button>
                <Button
                    :disabled="!selectedMusic || !ratingValue || submitting"
                    @click="submit"
                >
                    {{ submitting ? 'Submitting...' : 'Submit Rating' }}
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
