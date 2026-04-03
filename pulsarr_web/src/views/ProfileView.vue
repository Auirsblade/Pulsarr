<script setup lang="ts">
    import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
    import { Input } from "@/components/ui/input";
    import { Label } from "@/components/ui/label";
    import { Button } from "@/components/ui/button";
    import {
        AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent,
        AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger
    } from "@/components/ui/alert-dialog";
    import { ref, computed } from "vue";
    import { useRouter } from "vue-router";
    import { useContextStore } from "@/stores/context";
    import { storeToRefs } from "pinia";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler";
    import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
    import { User, Shield, Trash2, Eye } from "lucide-vue-next";
    import { toast } from "vue-sonner";
    import { formatDate } from "@/helpers/ratingFormatters";

    const router = useRouter();
    const contextStore = useContextStore();
    const { user } = storeToRefs(contextStore);

    // Edit profile state
    const editName = ref(user.value?.name ?? '');
    const editEmail = ref(user.value?.email ?? '');
    const savingProfile = ref(false);

    // Change password state
    const currentPassword = ref('');
    const newPassword = ref('');
    const confirmPassword = ref('');
    const savingPassword = ref(false);

    // Delete account state
    const deleteConfirmName = ref('');
    const deletingAccount = ref(false);
    const canDelete = computed(() => deleteConfirmName.value === user.value?.name);

    // Privacy settings state
    const profileVisibility = ref(user.value?.profile_visibility ?? 'public');
    const crateVisibility = ref(user.value?.crate_visibility ?? 'private');
    const savingPrivacy = ref(false);

    const saveProfile = () => {
        if (!user.value) return;
        savingProfile.value = true;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const updated = data as typeof user.value;
            if (updated && user.value) {
                user.value.name = updated.name;
                user.value.email = updated.email;
            }
            toast.success('Profile updated successfully');
            savingProfile.value = false;
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to update profile:', err);
            toast.error('Failed to update profile');
            savingProfile.value = false;
        };
        drh.post('/user/update', {
            pulsarr_user_id: user.value.pulsarr_user_id,
            name: editName.value,
            email: editEmail.value,
            profile_visibility: profileVisibility.value,
            crate_visibility: crateVisibility.value,
        });
    };

    const changePassword = () => {
        if (newPassword.value !== confirmPassword.value) {
            toast.error('Passwords do not match');
            return;
        }
        if (newPassword.value.length < 1) {
            toast.error('New password is required');
            return;
        }
        savingPassword.value = true;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            toast.success('Password changed successfully');
            currentPassword.value = '';
            newPassword.value = '';
            confirmPassword.value = '';
            savingPassword.value = false;
        };
        drh.onErrorCallback = (err: any) => {
            const msg = err?.msg || err?.err || 'Failed to change password';
            toast.error(msg);
            savingPassword.value = false;
        };
        drh.post('/user/change-password', {
            current_password: currentPassword.value,
            new_password: newPassword.value,
        });
    };

    const deleteAccount = () => {
        if (!user.value || !canDelete.value) return;
        deletingAccount.value = true;

        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            contextStore.clearSession();
            router.push('/');
            toast.success('Account deleted');
        };
        drh.onErrorCallback = (err) => {
            console.error('Failed to delete account:', err);
            toast.error('Failed to delete account');
            deletingAccount.value = false;
        };
        drh.delete(`/user/delete/${user.value.pulsarr_user_id}`);
    };
</script>

<template>
    <div class="p-4 max-w-2xl mx-auto space-y-6">
        <h1 class="text-2xl font-bold flex items-center gap-2">
            <User class="w-6 h-6" />
            Profile & Settings
        </h1>

        <!-- Account Info -->
        <Card>
            <CardHeader>
                <CardTitle>Account Info</CardTitle>
                <CardDescription>Your account details</CardDescription>
            </CardHeader>
            <CardContent v-if="user" class="space-y-3">
                <div class="grid grid-cols-[100px_1fr] gap-2 text-sm">
                    <span class="text-muted-foreground">Name</span>
                    <span class="font-medium">{{ user.name }}</span>
                    <span class="text-muted-foreground">Email</span>
                    <span class="font-medium">{{ user.email }}</span>
                    <span class="text-muted-foreground">Joined</span>
                    <span class="font-medium">{{ user.join_date ? formatDate(user.join_date) : 'N/A' }}</span>
                </div>
            </CardContent>
        </Card>

        <!-- Edit Profile -->
        <Card>
            <CardHeader>
                <CardTitle>Edit Profile</CardTitle>
                <CardDescription>Update your name and email</CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="space-y-2">
                    <Label for="edit-name">Name</Label>
                    <Input id="edit-name" v-model="editName" />
                </div>
                <div class="space-y-2">
                    <Label for="edit-email">Email</Label>
                    <Input id="edit-email" v-model="editEmail" type="email" />
                </div>
                <Button @click="saveProfile" :disabled="savingProfile">
                    {{ savingProfile ? 'Saving...' : 'Save Changes' }}
                </Button>
            </CardContent>
        </Card>

        <!-- Privacy Settings -->
        <Card>
            <CardHeader>
                <CardTitle class="flex items-center gap-2">
                    <Eye class="w-5 h-5" />
                    Privacy
                </CardTitle>
                <CardDescription>Control who can see your profile and Crate ratings</CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="space-y-2">
                    <Label>Profile Visibility</Label>
                    <Select v-model="profileVisibility">
                        <SelectTrigger>
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="public">Public — others can see your ratings</SelectItem>
                            <SelectItem value="private">Private — only your username is visible</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
                <div class="space-y-2">
                    <Label>Crate Visibility</Label>
                    <Select v-model="crateVisibility">
                        <SelectTrigger>
                            <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="public">Public — Crate ratings appear on your profile</SelectItem>
                            <SelectItem value="private">Private — only you can see your Crate ratings</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
                <Button @click="saveProfile" :disabled="savingProfile">
                    {{ savingProfile ? 'Saving...' : 'Save Privacy Settings' }}
                </Button>
            </CardContent>
        </Card>

        <!-- Change Password -->
        <Card>
            <CardHeader>
                <CardTitle class="flex items-center gap-2">
                    <Shield class="w-5 h-5" />
                    Change Password
                </CardTitle>
                <CardDescription>Update your password</CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="space-y-2">
                    <Label for="current-password">Current Password</Label>
                    <Input id="current-password" v-model="currentPassword" type="password" />
                </div>
                <div class="space-y-2">
                    <Label for="new-password">New Password</Label>
                    <Input id="new-password" v-model="newPassword" type="password" />
                </div>
                <div class="space-y-2">
                    <Label for="confirm-password">Confirm New Password</Label>
                    <Input id="confirm-password" v-model="confirmPassword" type="password" />
                </div>
                <Button @click="changePassword" :disabled="savingPassword">
                    {{ savingPassword ? 'Changing...' : 'Change Password' }}
                </Button>
            </CardContent>
        </Card>

        <!-- Danger Zone -->
        <Card class="border-destructive">
            <CardHeader>
                <CardTitle class="text-destructive flex items-center gap-2">
                    <Trash2 class="w-5 h-5" />
                    Danger Zone
                </CardTitle>
                <CardDescription>Permanently delete your account and all associated data.</CardDescription>
            </CardHeader>
            <CardContent>
                <AlertDialog>
                    <AlertDialogTrigger as-child>
                        <Button variant="destructive">Delete Account</Button>
                    </AlertDialogTrigger>
                    <AlertDialogContent>
                        <AlertDialogHeader>
                            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                            <AlertDialogDescription>
                                This action cannot be undone. This will permanently delete your account
                                and remove your data from our servers.
                                <div class="mt-4 space-y-2">
                                    <Label for="delete-confirm">Type your username <strong>{{ user?.name }}</strong> to confirm</Label>
                                    <Input id="delete-confirm" v-model="deleteConfirmName" placeholder="Enter your username" />
                                </div>
                            </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                            <AlertDialogAction
                                :disabled="!canDelete || deletingAccount"
                                @click="deleteAccount"
                                class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                            >
                                {{ deletingAccount ? 'Deleting...' : 'Delete Account' }}
                            </AlertDialogAction>
                        </AlertDialogFooter>
                    </AlertDialogContent>
                </AlertDialog>
            </CardContent>
        </Card>
    </div>
</template>
