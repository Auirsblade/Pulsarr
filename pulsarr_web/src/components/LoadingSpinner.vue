<script setup lang="ts">
    import { Loader2 } from "lucide-vue-next";

    defineProps<{
        text?: string;
    }>();
</script>

<template>
    <div class="flex items-center justify-center gap-2 py-4 text-muted-foreground">
        <Loader2 class="w-5 h-5 animate-spin" />
        <span v-if="text" class="text-sm">{{ text }}</span>
    </div>
</template>
