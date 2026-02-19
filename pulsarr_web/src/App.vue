<script setup lang="ts">
    import { Button } from '@/components/ui/button';
    import { Icon } from '@iconify/vue'
    import { RouterView } from 'vue-router'
    import { useColorMode } from "@vueuse/core";
    import AccountHeader from "@/components/AccountHeader.vue";
    import AppSidebar from "@/components/AppSidebar.vue";
    import { Toaster } from 'vue-sonner';
    import 'vue-sonner/style.css';
    import { onMounted, ref } from "vue";
    import { useContextStore } from "@/stores/context.ts";
    import { Menu } from "lucide-vue-next";

    const appContext = useContextStore();

    const sidebarOpen = ref(false);

    onMounted(() => {
        appContext.loadPrivacyTypes();
        appContext.loadRatingTypes();
    })

    const mode = useColorMode();
</script>

<template>
    <div class="min-h-dvh flex flex-col">
        <header class="inline-flex items-center w-full p-2 flex-shrink-0">
            <Button variant="ghost" size="icon" class="lg:hidden mr-1" aria-label="Open navigation menu" @click="sidebarOpen = true">
                <Menu class="w-5 h-5" />
            </Button>
            <h1 class="text-2xl sm:text-4xl font-bold truncate">Pulsarr Music</h1>
            <AccountHeader class="ml-auto"/>
            <Button @click="mode = mode == 'light' ? 'dark' : 'light'" class="ml-2 shadow-none" variant="secondary" :aria-label="mode === 'light' ? 'Switch to dark mode' : 'Switch to light mode'">
                <Icon icon="radix-icons:moon" aria-hidden="true" class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"/>
                <Icon icon="radix-icons:sun" aria-hidden="true" class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"/>
            </Button>
        </header>
        <hr>

        <div class="flex flex-1 overflow-hidden">
            <AppSidebar :open="sidebarOpen" @update:open="sidebarOpen = $event" />
            <main class="flex-1 overflow-y-auto">
                <RouterView/>
            </main>
        </div>
        <Toaster :theme="mode === 'dark' ? 'dark' : 'light'" position="bottom-right" rich-colors />
    </div>
</template>
