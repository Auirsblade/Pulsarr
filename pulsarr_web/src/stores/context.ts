import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { SignInResponse, UserDTO } from "@/apiClient";
import { CookieManager } from "@/helpers/CookieManager.ts";
import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";

export const useContextStore = defineStore('context', () => {
    const user = ref<UserDTO>();
    const apiKey = ref();
    const privacyTypes = ref<Array<string>>([]);
    const ratingTypes = ref<Array<string>>([]);

    const setSession = (signin: SignInResponse) => {
        user.value = signin.user;
        apiKey.value = signin.pulsarr_api_key;
        setApiKey();
    }

    const getSession = () => {
        const pulsarrApiKey = CookieManager.getApiKey();
        if (pulsarrApiKey) {
            apiKey.value = pulsarrApiKey;
        }
    }

    function setApiKey() {
        if (apiKey.value) {
            CookieManager.setApiKey(apiKey.value);
        } else {
            CookieManager.removeApiKey();
        }
    }

    const loadPrivacyTypes = async () => {
        if (privacyTypes.value.length > 0) return; // Already loaded
        
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            privacyTypes.value = data as string[];
        };
        drh.onErrorCallback = (error) => {
            console.error('Failed to fetch privacy types:', error);
        };
        await drh.get("/group/privacyTypes");
    }

    const loadRatingTypes = async () => {
        if (ratingTypes.value.length > 0) return; // Already loaded
        
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            ratingTypes.value = data as string[];
        };
        drh.onErrorCallback = (error) => {
            console.error('Failed to fetch rating types:', error);
        };
        await drh.get("/rating-system/ratingTypes");
    }

    return { 
        user, 
        apiKey, 
        privacyTypes, 
        ratingTypes,
        setSession, 
        getSession,
        loadPrivacyTypes,
        loadRatingTypes
    }
})
