import { ref, onUnmounted, type Ref } from 'vue'

export type FetchCallback<T> = (
    offset: number,
    limit: number,
    onSuccess: (data: T[]) => void,
    onError: (err: unknown) => void,
) => void

export function useInfiniteScroll<T>(
    fetchFn: FetchCallback<T>,
    pageSize = 20
) {
    const items = ref<T[]>([]) as Ref<T[]>
    const loading = ref(false)
    const loadingMore = ref(false)
    const hasMore = ref(true)
    let observer: IntersectionObserver | null = null

    const loadInitial = () => {
        loading.value = true
        hasMore.value = true
        items.value = []
        fetchFn(0, pageSize, (data) => {
            items.value = data
            hasMore.value = data.length >= pageSize
            loading.value = false
        }, () => {
            loading.value = false
        })
    }

    const loadMore = () => {
        if (loadingMore.value || !hasMore.value) return
        loadingMore.value = true
        fetchFn(items.value.length, pageSize, (data) => {
            items.value = [...items.value, ...data]
            hasMore.value = data.length >= pageSize
            loadingMore.value = false
        }, () => {
            loadingMore.value = false
        })
    }

    const setupSentinel = (el: Element | null) => {
        if (observer) {
            observer.disconnect()
        }
        if (!el) return
        observer = new IntersectionObserver(
            (entries) => {
                if (entries[0].isIntersecting) {
                    loadMore()
                }
            },
            { rootMargin: '200px' }
        )
        observer.observe(el)
    }

    onUnmounted(() => {
        if (observer) {
            observer.disconnect()
        }
    })

    return {
        items,
        loading,
        loadingMore,
        hasMore,
        loadInitial,
        loadMore,
        setupSentinel,
    }
}
