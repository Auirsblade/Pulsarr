import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { SignInResponse, UserDTO } from "@/apiClient";
import { CookieManager } from "@/helpers/CookieManager.ts";

export const useContextStore = defineStore('context', () => {
    const user = ref<UserDTO>();
    const apiKey = ref();

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

    return { user, apiKey, setSession, getSession }
})
