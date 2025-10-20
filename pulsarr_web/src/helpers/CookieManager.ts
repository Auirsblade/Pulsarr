// src/helpers/CookieManager.ts
export class CookieManager {
    private static readonly API_KEY_COOKIE = 'pulsarr_api_key'
    private static readonly DEVICE_KEY_COOKIE = 'pulsarr_device_key'
    
    static setApiKey(apiKey: string): void {
        const isSecure = window.location.protocol === 'https:'
        const cookieOptions = [
            `${this.API_KEY_COOKIE}=${apiKey}`,
            'path=/',
            'samesite=strict',
            isSecure ? 'secure' : '',
            `max-age=${30 * 24 * 60 * 60}` // 30 days
        ].filter(Boolean).join('; ')
        
        document.cookie = cookieOptions
    }

    static getApiKey(): string | null {
        return this.getCookie(this.API_KEY_COOKIE)
    }

    static removeApiKey(): void {
        document.cookie = `${this.API_KEY_COOKIE}=; path=/; expires=Thu, 01 Jan 1970 00:00:01 GMT`
    }

    static getDeviceKey(): string {
        let deviceKey = this.getCookie(this.DEVICE_KEY_COOKIE)
        if (!deviceKey) {
            deviceKey = crypto.randomUUID()
            this.setDeviceKey(deviceKey)
        }
        return deviceKey
    }

    private static setDeviceKey(deviceKey: string): void {
        const isSecure = window.location.protocol === 'https:'
        const cookieOptions = [
            `${this.DEVICE_KEY_COOKIE}=${deviceKey}`,
            'path=/',
            'samesite=strict',
            isSecure ? 'secure' : '',
            `max-age=${365 * 24 * 60 * 60}` // 1 year
        ].filter(Boolean).join('; ')
        
        document.cookie = cookieOptions
    }

    private static getCookie(name: string): string | null {
        const match = document.cookie.match(new RegExp('(^| )' + name + '=([^;]+)'))
        return match ? match[2] : null
    }
}