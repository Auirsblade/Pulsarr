<script setup lang="ts">
    import {Button} from '@/components/ui/button';
    import {Icon} from '@iconify/vue'
    import { RouterLink, RouterView } from 'vue-router'
    import {useColorMode} from "@vueuse/core";

    const mode = useColorMode();
</script>

<template>
    <div class="min-h-dvh">
        <header class="inline-flex items-center w-full p-2">
            <h1 class="text-4xl font-bold">Pulsarr Music</h1>
            <RouterLink class="mt-auto ml-2" to="/">Home</RouterLink>
            <RouterLink class="mt-auto ml-2" to="/about">About</RouterLink>
            <Button @click="mode = mode == 'light' ? 'dark' : 'light'" class="ml-auto shadow-none" variant="secondary">
                <Icon icon="radix-icons:moon" class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"/>
                <Icon icon="radix-icons:sun" class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"/>
            </Button>
        </header>
        <hr>

        <RouterView/>
    </div>
</template>
