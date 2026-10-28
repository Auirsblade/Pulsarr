import { ref } from "vue";
import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";

interface CoverArtInfo {
    front?: string;
    front_thumbnail_small?: string;
    front_thumbnail_large?: string;
}

// Module-level so every component shares one cache (and one set of in-flight requests).
const coverArtCache = ref<Record<string, string>>({});
const requested = new Set<string>();

const fetchOne = (releaseGroupId: string) => {
    if (requested.has(releaseGroupId)) return;
    requested.add(releaseGroupId);

    const drh = new DataRequestHandler();
    drh.onSuccessCallback = (data) => {
        const coverArt = data as CoverArtInfo;
        const url = coverArt.front_thumbnail_small || coverArt.front;
        if (url) coverArtCache.value[releaseGroupId] = url;
    };
    drh.onErrorCallback = (_data, response) => {
        // Only a definite 404 (no art) stays remembered; network failures and 5xx are retried next request.
        if (response?.status !== 404) requested.delete(releaseGroupId);
    };
    drh.get(`/musicbrainz/release-group/${releaseGroupId}/cover-art-info`);
};

export function useCoverArt() {
    const fetchCoverArt = (releaseGroupIds: string[]) => {
        for (const id of releaseGroupIds) {
            if (id) fetchOne(id);
        }
    };

    return { coverArtCache, fetchCoverArt };
}
