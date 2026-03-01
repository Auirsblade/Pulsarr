<script setup lang="ts">
    import { Button } from "@/components/ui/button";
    import { Dialog, DialogHeader, DialogFooter, DialogContent, DialogTitle, DialogTrigger, DialogDescription } from "@/components/ui/dialog";
    import { Input } from "@/components/ui/input";
    import { Label } from "@/components/ui/label";
    import { ref, computed, watch } from "vue";
    import type { SignInRequest, SignInResponse } from "@/apiClient";
    import { useForm } from 'vee-validate';
    import * as yup from 'yup';
    import { toast } from 'vue-sonner';
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import { CookieManager } from "@/helpers/CookieManager.ts";
    import { LogOut, User, Settings } from "lucide-vue-next";
    import {
        DropdownMenu, DropdownMenuContent, DropdownMenuItem,
        DropdownMenuSeparator, DropdownMenuTrigger
    } from "@/components/ui/dropdown-menu";
    import { useRouter } from "vue-router";

    const router = useRouter();
    const contextStore = useContextStore();
    const { user, isLoggedIn, showSignInDialog } = storeToRefs(contextStore);

    const signup = ref<boolean>(false);

    const validationSchema = computed(() => {
        const base: Record<string, yup.StringSchema> = {
            usernameInput: yup.string().required('Username is required').min(3, 'Username must be at least 3 characters').max(30, 'Username must be at most 30 characters'),
            passwordInput: yup.string().required('Password is required').min(8, 'Password must be at least 8 characters'),
        };
        if (signup.value) {
            base.emailInput = yup.string().email('Please enter a valid email address').required('Email is required');
            base.confirmPasswordInput = yup.string().required('Please confirm your password').oneOf([yup.ref('passwordInput')], 'Passwords do not match');
        }
        return yup.object(base);
    });

    const { errors, defineField, handleSubmit, resetForm } = useForm({
        validationSchema,
    });

    watch(signup, () => { resetForm(); });

    const [usernameInput, usernameAttrs] = defineField('usernameInput');
    const [emailInput] = defineField('emailInput');
    const [passwordInput] = defineField('passwordInput');
    const [confirmPasswordInput] = defineField('confirmPasswordInput');

    const signinDrh = new DataRequestHandler();
    signinDrh.onSuccessCallback = (data) => {
        console.log('Signin successful:', data);
        contextStore.setSession(data as SignInResponse);
        showSignInDialog.value = false;
    };
    signinDrh.onErrorCallback = (data) => {
        const body = data as Record<string, string> | null;
        const msg = body?.msg || body?.err || 'Something went wrong';
        toast.error(msg);
    };

    const submitSignup = handleSubmit(async () => {
        const userPayload = {
            name: usernameInput.value,
            email: emailInput.value,
            password: passwordInput.value,
        };

        const signupDrh = new DataRequestHandler();
        signupDrh.onSuccessCallback = async (data) => {
            console.log('Signup successful:', data);

            const signInPayload = {
                username: userPayload.name,
                password: userPayload.password,
                hw_key: CookieManager.getDeviceKey()
            } as SignInRequest;

            await signinDrh.post('/auth/signin', signInPayload);
        };
        signupDrh.onErrorCallback = (data) => {
            const body = data as Record<string, string> | null;
            const msg = body?.msg || body?.err || 'Something went wrong';
            toast.error(msg);
        };

        await signupDrh.post('/user/add', userPayload);
    });

    const submitSignin = handleSubmit(async () => {
        const signInPayload = {
            username: usernameInput.value ?? "",
            password: passwordInput.value,
            hw_key: CookieManager.getDeviceKey()
        } as SignInRequest;

        await signinDrh.post('/auth/signin', signInPayload);
    });

    const signOut = () => {
        contextStore.clearSession();
    }

</script>

<template>
    <div>
        <!-- Logged in state -->
        <div v-if="isLoggedIn">
            <DropdownMenu>
                <DropdownMenuTrigger as-child>
                    <button class="flex items-center gap-2 px-3 py-1.5 rounded-md bg-secondary hover:bg-secondary/80 transition-colors cursor-pointer max-w-[160px]">
                        <User class="w-4 h-4 flex-shrink-0" />
                        <span class="text-sm font-medium truncate">{{ user?.name }}</span>
                    </button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                    <DropdownMenuItem @click="router.push('/profile')">
                        <Settings class="w-4 h-4 mr-2" />
                        Profile
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem @click="signOut">
                        <LogOut class="w-4 h-4 mr-2" />
                        Sign Out
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>

        <!-- Logged out state -->
        <Dialog v-else v-model:open="showSignInDialog">
            <DialogTrigger as-child>
                <Button variant="outline">
                    Sign In
                </Button>
            </DialogTrigger>
            <DialogContent class="sm:max-w-[425px]">
                <DialogHeader>
                    <DialogTitle>Sign In</DialogTitle>
                    <DialogDescription>
                        <div v-if="signup" class="inline-flex w-full">
                            <div class="my-auto">Already have an account?</div>
                            <Button class="ml-auto" variant="secondary" @click="signup = false">
                                Sign In
                            </Button>
                        </div>
                        <div v-else class="inline-flex w-full">
                            <div class="my-auto">Don't have an account?</div>
                            <Button class="ml-auto" variant="secondary" @click="signup = true">
                                Sign Up
                            </Button>
                        </div>
                    </DialogDescription>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                    <div>
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="username" class="text-right">
                                Username
                            </Label>
                            <Input id="username" v-model="usernameInput" v-bind="usernameAttrs" class="col-span-3" />
                        </div>
                        <p v-if="errors.usernameInput" class="text-sm text-destructive mt-1 text-right">{{ errors.usernameInput }}</p>
                    </div>
                    <div v-if="signup">
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="email" class="text-right">
                                Email
                            </Label>
                            <Input id="email" default-value="" v-model="emailInput" class="col-span-3" />
                        </div>
                        <p v-if="errors.emailInput" class="text-sm text-destructive mt-1 text-right">{{ errors.emailInput }}</p>
                    </div>
                    <div>
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="password" class="text-right">
                                Password
                            </Label>
                            <Input id="password" v-model="passwordInput" type="password" class="col-span-3" />
                        </div>
                        <p v-if="errors.passwordInput" class="text-sm text-destructive mt-1 text-right">{{ errors.passwordInput }}</p>
                    </div>
                    <div v-if="signup">
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="confirmPassword" class="text-right">
                                Confirm Password
                            </Label>
                            <Input id="confirmPassword" v-model="confirmPasswordInput" type="password" class="col-span-3" />
                        </div>
                        <p v-if="errors.confirmPasswordInput" class="text-sm text-destructive mt-1 text-right">{{ errors.confirmPasswordInput }}</p>
                    </div>
                </div>
                <DialogFooter>
                    <div v-if="signup">
                        <Button class="mx-auto" type="submit" @click="submitSignup">
                            Sign Up
                        </Button>
                    </div>
                    <div v-else>
                        <Button class="mx-auto" type="submit" @click="submitSignin">
                            Sign In
                        </Button>
                    </div>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    </div>
</template>