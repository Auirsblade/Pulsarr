<script setup lang="ts">
    import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
    import { Music, Users } from "lucide-vue-next";
</script>

<template>
    <div class="p-4 max-w-4xl mx-auto space-y-4">
        <h1 class="text-2xl font-bold flex items-center gap-2">
            <Music class="w-6 h-6" />
            About Pulsarr
        </h1>

        <Card>
            <CardHeader>
                <CardTitle>What is Pulsarr?</CardTitle>
            </CardHeader>
            <CardContent class="space-y-3 text-sm text-muted-foreground">
                <p>
                    Pulsarr is a music rating and sharing platform designed for groups of friends with different
                    streaming subscriptions. It brings everyone together in one place to discuss, rate, and curate music.
                </p>
                <p>
                    Create or join groups, set up custom rating systems, and share your thoughts on albums and artists.
                    Whether your group prefers simple 1-10 scores or detailed multi-parameter breakdowns, Pulsarr
                    adapts to your style.
                </p>
            </CardContent>
        </Card>

        <Card>
            <CardHeader>
                <CardTitle class="flex items-center gap-2">
                    <Users class="w-5 h-5" />
                    Credits
                </CardTitle>
            </CardHeader>
            <CardContent class="space-y-3 text-sm text-muted-foreground">
                <div>
                    <h3 class="font-medium text-foreground mb-1">Contributors</h3>
                    <ul class="list-disc list-inside space-y-1">
                        <li>Asher Carlow</li>
                        <li>Ethan Hart</li>
                        <li>Joseph Sedivy</li>
                        <li>Jude Hoogendyk</li>
                    </ul>
                </div>
                <div>
                    <h3 class="font-medium text-foreground mb-1">Acknowledgements</h3>
                    <p>
                        Music metadata provided by
                        <a href="https://musicbrainz.org" target="_blank" rel="noopener noreferrer" class="underline text-foreground hover:text-primary">MusicBrainz</a>.
                    </p>
                </div>
            </CardContent>
        </Card>
    </div>
</template>
