import { useContextStore } from "@/stores/context.ts";

type DataResponseCallbackFunction = (data: unknown, response: Response) => void;
type ErrorResponseCallbackFunction = (data: unknown, response: Response|null, error: unknown) => void;
type ResponseCallbackFunction = (response: Response|null) => void;

export class DataRequestHandler {
    onSuccessCallback: DataResponseCallbackFunction = () => {};
    onErrorCallback: ErrorResponseCallbackFunction = () => {};
    onCompleteCallback: ResponseCallbackFunction = () => {};
    logResponse: boolean = false;
    logData: boolean = false;

    private getHeaders(contentType?: boolean): Headers {
        const headers = new Headers();
        const contextStore = useContextStore();
        
        if (contextStore.apiKey) {
            headers.append('pulsarr-api-key', contextStore.apiKey);
        }
        
        if (contentType) {
            headers.append('Content-Type', 'application/json');
        }
        
        return headers
    }

    async get(url: RequestInfo | URL) {
        return this.handleResponse(url, {
            method: 'GET',
            headers: this.getHeaders()
        });
    }

    async post(url: RequestInfo | URL, data?: object) {
        if (data) {
            return this.handleResponse(url, {
                method: 'POST',
                headers: this.getHeaders(true),
                body: JSON.stringify(data)
            });
        }
        return this.handleResponse(url, {
            method: 'POST',
            headers: this.getHeaders()
        });
    }

    async put(url: RequestInfo | URL, data: object) {
        return this.handleResponse(url, {
            method: 'PUT',
            headers: this.getHeaders(true),
            body: JSON.stringify(data)
        });
    }

    private async handleResponse(url: RequestInfo | URL, opts: RequestInit | undefined) {
        let response: Response | null = null;
        try {
            response = await fetch(import.meta.env.VITE_API_URL.concat(url.toString()), opts)
        } catch(error) {
            console.error(error);
            this.onErrorCallback(null, null, error);
            this.onCompleteCallback(null);
            return null;
        }

        if (this.logResponse) console.log(response);

        const data = await this.getData(response);
        if (this.logData) console.log(data);

        if (!response.ok) this.onErrorCallback(data, response, undefined);
        else this.onSuccessCallback(data, response);
        this.onCompleteCallback(response);

        return data;
    }

    private async getData(response: Response){
        const contentType = response.headers.get('content-type');
        if (contentType?.includes('application/json') || contentType?.includes('text/json'))
            return await response.json();
        if (contentType?.includes('text/plain'))
            return await response.text();
        return null;
    }
}