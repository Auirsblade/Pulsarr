<script setup lang="ts">
    import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
    import { Button } from "@/components/ui/button";
    import { Input } from "@/components/ui/input";
    import { Label } from "@/components/ui/label";
    import { onMounted, ref } from "vue";
    import { useRoute, useRouter } from "vue-router";
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { useForm } from 'vee-validate';
    import * as yup from 'yup';
    import { toast } from 'vue-sonner';
    import LoadingSpinner from "@/components/LoadingSpinner.vue";

    const route = useRoute();
    const router = useRouter();

    type PageState = 'loading' | 'invalid' | 'form' | 'success';
    const pageState = ref<PageState>('loading');
    const token = ref<string>('');

    const validationSchema = yup.object({
        newPassword: yup.string().required('Password is required').min(8, 'Password must be at least 8 characters'),
        confirmPassword: yup.string().required('Please confirm your password').oneOf([yup.ref('newPassword')], 'Passwords do not match'),
    });

    const { errors, defineField, handleSubmit } = useForm({ validationSchema });

    const [newPassword] = defineField('newPassword');
    const [confirmPassword] = defineField('confirmPassword');

    onMounted(() => {
        const rawToken = route.query.token;
        if (!rawToken || typeof rawToken !== 'string') {
            pageState.value = 'invalid';
            return;
        }
        token.value = rawToken;
        validateToken();
    });

    const validateToken = () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = (data) => {
            const result = data as { valid: boolean };
            pageState.value = result.valid ? 'form' : 'invalid';
        };
        drh.onErrorCallback = () => {
            pageState.value = 'invalid';
        };
        drh.post('/auth/validate-reset-token', { token: token.value });
    };

    const submitReset = handleSubmit(async () => {
        const drh = new DataRequestHandler();
        drh.onSuccessCallback = () => {
            toast.success('Password reset successfully. You can now sign in.');
            pageState.value = 'success';
        };
        drh.onErrorCallback = (data) => {
            const body = data as Record<string, string> | null;
            const msg = body?.msg || body?.err || 'Something went wrong';
            toast.error(msg);
        };
        await drh.post('/auth/reset-password', { token: token.value, new_password: newPassword.value });
    });
</script>

<template>
    <div class="p-4 max-w-md mx-auto mt-8">
        <!-- Loading state -->
        <div v-if="pageState === 'loading'">
            <LoadingSpinner text="Validating reset link..." />
        </div>

        <!-- Invalid / expired token -->
        <div v-else-if="pageState === 'invalid'" class="flex flex-col items-center py-8 gap-4">
            <span class="text-destructive text-center">This password reset link is invalid or has expired.</span>
            <Button variant="outline" @click="router.push('/')">Go Home</Button>
        </div>

        <!-- Success state -->
        <div v-else-if="pageState === 'success'" class="flex flex-col items-center py-8 gap-4">
            <span class="text-lg font-medium text-center">Your password has been reset successfully!</span>
            <span class="text-muted-foreground text-center">You can now sign in with your new password.</span>
            <Button @click="router.push('/')">Go Home</Button>
        </div>

        <!-- Reset form -->
        <Card v-else-if="pageState === 'form'">
            <CardHeader class="text-center">
                <CardTitle class="text-xl">Reset Password</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4">
                <div>
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="new-password" class="text-right col-span-1">
                            New Password
                        </Label>
                        <Input id="new-password" v-model="newPassword" type="password" class="col-span-3" />
                    </div>
                    <p v-if="errors.newPassword" class="text-sm text-destructive mt-1 text-right">{{ errors.newPassword }}</p>
                </div>
                <div>
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="confirm-password" class="text-right col-span-1">
                            Confirm
                        </Label>
                        <Input id="confirm-password" v-model="confirmPassword" type="password" class="col-span-3" />
                    </div>
                    <p v-if="errors.confirmPassword" class="text-sm text-destructive mt-1 text-right">{{ errors.confirmPassword }}</p>
                </div>
                <div class="flex justify-end pt-2">
                    <Button type="submit" @click="submitReset">
                        Reset Password
                    </Button>
                </div>
            </CardContent>
        </Card>
    </div>
</template>
