<script setup lang="ts">
    import { ref, watch } from "vue";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler";
    import { useContextStore } from "@/stores/context";
    import { storeToRefs } from "pinia";
    import { toast } from "vue-sonner";

    // Keep in sync with ALLOWED_REACTION_EMOJIS in pulsarr_fulcrum/src/api/rating.rs
    const REACTION_EMOJIS = ["👍", "❤️", "😂", "😮", "😢", "🔥"] as const;

    interface ReactionCount {
        emoji: string;
        count: number;
    }

    interface RatingReactionSummary {
        rating_id: number;
        counts: ReactionCount[];
        user_reactions: string[];
    }

    const props = defineProps<{
        ratingId: number;
    }>();

    const { isLoggedIn } = storeToRefs(useContextStore());

    const counts = ref<Record<string, number>>({});
    const userReactions = ref<Set<string>>(new Set());
    const pending = ref<Set<string>>(new Set());
    const loaded = ref(false);

    const applySummary = (summary: RatingReactionSummary) => {
        const next: Record<string, number> = {};
        for (const c of summary.counts) {
            next[c.emoji] = Number(c.count) || 0;
        }
        counts.value = next;
        userReactions.value = new Set(summary.user_reactions);
        loaded.value = true;
    };

    const fetchReactions = (ratingId: number) => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            applySummary(data as RatingReactionSummary);
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to fetch reactions:', err);
        };
        drh.get(`/rating/reaction/by_rating/${ratingId}`);
    };

    const toggleReaction = (emoji: string) => {
        if (!isLoggedIn.value || pending.value.has(emoji)) return;
        pending.value.add(emoji);
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            applySummary(data as RatingReactionSummary);
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to toggle reaction:', err);
            toast.error('Failed to update reaction');
        };
        drh.onCompleteCallback = () => {
            pending.value.delete(emoji);
        };
        drh.post('/rating/reaction/toggle', { rating_id: props.ratingId, emoji });
    };

    watch(
        () => props.ratingId,
        (id) => {
            if (id) {
                counts.value = {};
                userReactions.value = new Set();
                loaded.value = false;
                fetchReactions(id);
            }
        },
        { immediate: true }
    );
</script>

<template>
    <div class="flex flex-wrap items-center gap-1.5">
        <button
            v-for="emoji in REACTION_EMOJIS"
            :key="emoji"
            type="button"
            :disabled="!isLoggedIn || pending.has(emoji)"
            :aria-pressed="userReactions.has(emoji)"
            :class="[
                'inline-flex items-center gap-1 px-2 py-1 rounded-full border text-sm transition-colors',
                userReactions.has(emoji)
                    ? 'bg-primary/15 border-primary/40 text-primary-foreground dark:text-foreground'
                    : 'bg-muted/30 border-border hover:bg-muted/60',
                (!isLoggedIn || pending.has(emoji)) && 'opacity-60 cursor-not-allowed',
            ]"
            @click="toggleReaction(emoji)"
        >
            <span class="leading-none">{{ emoji }}</span>
            <span v-if="counts[emoji]" class="text-xs font-medium tabular-nums">{{ counts[emoji] }}</span>
        </button>
    </div>
</template>
