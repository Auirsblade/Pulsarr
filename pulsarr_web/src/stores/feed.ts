import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { GroupDTO, Rating } from '@/apiClient'
import { DataRequestHandler } from '@/helpers/DataRequestHandler'

const PAGE_SIZE = 20;

export const useFeedStore = defineStore('feed', () => {
    const myGroups = ref<GroupDTO[]>([]);
    const activeGroupIds = ref<number[]>([]);
    const groupDetails = ref<Map<number, GroupDTO>>(new Map());
    const ratings = ref<Rating[]>([]);
    const loading = ref(false);
    const loadingMore = ref(false);
    const hasMore = ref(true);
    const error = ref<string | null>(null);

    const getUserName = (userId: number, userName?: string | null): string => {
        if (userName) return userName;
        for (const group of groupDetails.value.values()) {
            const member = group.members?.find(m => m.user?.pulsarr_user_id === userId);
            if (member?.user?.name) return member.user.name;
        }
        return 'Unknown User';
    };

    const getGroupName = (groupId: number): string => {
        return groupDetails.value.get(groupId)?.name
            ?? myGroups.value.find(g => g.pulsarr_group_id === groupId)?.name
            ?? 'Unknown Group';
    };

    const getGroupForRating = (groupId: number): GroupDTO | undefined => {
        return groupDetails.value.get(groupId);
    };

    const fetchRatingsForActiveGroups = (offset: number, onDone?: () => void) => {
        if (activeGroupIds.value.length === 0) {
            ratings.value = [];
            hasMore.value = false;
            onDone?.();
            return;
        }
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const fetched = data as Rating[];
            fetched.sort((a, b) => new Date(b.rating_date).getTime() - new Date(a.rating_date).getTime());
            if (offset === 0) {
                ratings.value = fetched;
            } else {
                ratings.value = [...ratings.value, ...fetched];
            }
            hasMore.value = fetched.length >= PAGE_SIZE;
            onDone?.();
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to fetch ratings:', err);
            onDone?.();
        };
        drh.post('/rating/byGroups', { group_ids: activeGroupIds.value, take_size: PAGE_SIZE, offset });
    };

    const fetchGroupDetails = (groups: GroupDTO[], detailEndpoint: (id: number) => string, onDone: () => void) => {
        if (groups.length === 0) {
            groupDetails.value = new Map();
            onDone();
            return;
        }

        const detailMap = new Map<number, GroupDTO>();
        let remaining = groups.length;

        const checkDone = () => {
            remaining--;
            if (remaining === 0) {
                groupDetails.value = detailMap;
                onDone();
            }
        };

        for (const g of groups) {
            const drh = new DataRequestHandler();
            drh.onSuccessCallback = (data) => {
                const detail = data as GroupDTO;
                detailMap.set(detail.pulsarr_group_id, detail);
                checkDone();
            };
            drh.onErrorCallback = () => checkDone();
            drh.get(detailEndpoint(g.pulsarr_group_id));
        }
    };

    const loadFeed = () => {
        loading.value = true;
        error.value = null;
        hasMore.value = true;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const groups = data as GroupDTO[];
            myGroups.value = groups;
            activeGroupIds.value = groups.map(g => g.pulsarr_group_id);

            fetchGroupDetails(groups, (id) => `/group/${id}`, () => {
                fetchRatingsForActiveGroups(0, () => {
                    loading.value = false;
                });
            });
        };
        drh.onErrorCallback = (err) => {
            error.value = 'Failed to load activity feed';
            console.error('Feed load error:', err);
            loading.value = false;
        };
        drh.get('/group/myGroups');
    };

    const loadAllFeed = () => {
        loading.value = true;
        error.value = null;
        hasMore.value = true;

        let userGroups: GroupDTO[] | null = null;
        let publicGroups: GroupDTO[] | null = null;

        const onBothLoaded = () => {
            if (!userGroups || !publicGroups) return;

            myGroups.value = userGroups;

            // Merge and deduplicate
            const memberIds = new Set(userGroups.map(g => g.pulsarr_group_id));
            const merged = [...userGroups];
            for (const g of publicGroups) {
                if (!memberIds.has(g.pulsarr_group_id)) {
                    merged.push(g);
                }
            }
            activeGroupIds.value = merged.map(g => g.pulsarr_group_id);

            // User's own groups get full details, public-only groups get preview
            const detailEndpoint = (id: number) =>
                memberIds.has(id) ? `/group/${id}` : `/group/preview/${id}`;

            fetchGroupDetails(merged, detailEndpoint, () => {
                fetchRatingsForActiveGroups(0, () => {
                    loading.value = false;
                });
            });
        };

        const onError = (err: unknown) => {
            error.value = 'Failed to load activity feed';
            console.error('Feed load error:', err);
            loading.value = false;
        };

        const drhUser = new DataRequestHandler();
        drhUser.onSuccessCallback = (data) => {
            userGroups = data as GroupDTO[];
            onBothLoaded();
        };
        drhUser.onErrorCallback = onError;
        drhUser.get('/group/myGroups');

        const drhPublic = new DataRequestHandler();
        drhPublic.onSuccessCallback = (data) => {
            publicGroups = data as GroupDTO[];
            onBothLoaded();
        };
        drhPublic.onErrorCallback = onError;
        drhPublic.get('/group/public');
    };

    const loadPublicFeed = () => {
        loading.value = true;
        error.value = null;
        hasMore.value = true;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const groups = data as GroupDTO[];
            myGroups.value = groups;
            activeGroupIds.value = groups.map(g => g.pulsarr_group_id);

            fetchGroupDetails(groups, (id) => `/group/preview/${id}`, () => {
                fetchRatingsForActiveGroups(0, () => {
                    loading.value = false;
                });
            });
        };
        drh.onErrorCallback = (err) => {
            error.value = 'Failed to load public feed';
            console.error('Public feed load error:', err);
            loading.value = false;
        };
        drh.get('/group/public');
    };

    const loadMoreRatings = () => {
        if (loadingMore.value || !hasMore.value || activeGroupIds.value.length === 0) return;
        loadingMore.value = true;

        fetchRatingsForActiveGroups(ratings.value.length, () => {
            loadingMore.value = false;
        });
    };

    const ensureGroupsLoaded = () => {
        if (myGroups.value.length > 0) return;
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            myGroups.value = data as GroupDTO[];
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to load groups:', err);
        };
        drh.get('/group/myGroups');
    };

    return {
        myGroups,
        groupDetails,
        ratings,
        loading,
        loadingMore,
        hasMore,
        error,
        getUserName,
        getGroupName,
        getGroupForRating,
        loadFeed,
        loadAllFeed,
        loadPublicFeed,
        loadMoreRatings,
        ensureGroupsLoaded,
    };
});
