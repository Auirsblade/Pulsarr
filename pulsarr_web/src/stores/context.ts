import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import type { SignInResponse, UserDTO } from "@/apiClient";
import { CookieManager } from "@/helpers/CookieManager.ts";
import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";

export const useContextStore = defineStore('context', () => {
    const user = ref<UserDTO>();
    const apiKey = ref();
    const privacyTypes = ref<Array<string>>([]);
    const ratingTypes = ref<Array<string>>([]);
    const showSignInDialog = ref(false);
    const isLoggedIn = computed(() => !!apiKey.value && !!user.value);

    // Crate state
    const crate = ref<any>(null);
    const crateLoaded = ref(false);
    const needsCrateSetup = computed(() => isLoggedIn.value && crateLoaded.value && !crate.value);

    const setSession = async (signin: SignInResponse) => {
        user.value = signin.user;
        apiKey.value = signin.pulsarr_api_key;
        setApiKey();
        await loadCrate();
    }

    const getSession = async () => {
        const pulsarrApiKey = CookieManager.getApiKey();
        if (pulsarrApiKey) {
            apiKey.value = pulsarrApiKey;
            // Fetch current user data
            const drh = new DataRequestHandler();
            drh.onSuccessCallback = (data) => {
                user.value = data as UserDTO;
            };
            drh.onErrorCallback = (error) => {
                console.error('Failed to fetch current user:', error);
                // API key might be invalid, clear it
                apiKey.value = undefined;
                CookieManager.removeApiKey();
            };
            await drh.get("/user/currentUser");
            await loadCrate();
        }
    }

    function setApiKey() {
        if (apiKey.value) {
            CookieManager.setApiKey(apiKey.value);
        } else {
            CookieManager.removeApiKey();
        }
    }

    const clearSession = () => {
        user.value = undefined;
        apiKey.value = undefined;
        crate.value = null;
        crateLoaded.value = false;
        showSignInDialog.value = false;
        CookieManager.removeApiKey();
    }

    const loadCrate = async () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            crate.value = data;
        };
        drh.onErrorCallback = () => {
            crate.value = null;
        };
        await drh.get("/group/myCrate");
        crateLoaded.value = true;
    }

    const loadPrivacyTypes = () => {
        if (privacyTypes.value.length > 0) return;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            privacyTypes.value = data as string[];
        };
        drh.onErrorCallback = (error) => {
            console.error('Failed to fetch privacy types:', error);
        };
        drh.get("/group/privacyTypes");
    }

    const loadRatingTypes = () => {
        if (ratingTypes.value.length > 0) return;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            ratingTypes.value = data as string[];
        };
        drh.onErrorCallback = (error) => {
            console.error('Failed to fetch rating types:', error);
        };
        drh.get("/rating-system/ratingTypes");
    }

    return {
        user,
        apiKey,
        privacyTypes,
        ratingTypes,
        showSignInDialog,
        isLoggedIn,
        crate,
        crateLoaded,
        needsCrateSetup,
        setSession,
        getSession,
        clearSession,
        loadPrivacyTypes,
        loadRatingTypes,
        loadCrate,
    }
})
