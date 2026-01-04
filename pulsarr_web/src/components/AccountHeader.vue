<script setup lang="ts">
    import { Button } from "@/components/ui/button";
    import { Dialog, DialogHeader, DialogFooter, DialogContent, DialogTitle, DialogTrigger, DialogDescription } from "@/components/ui/dialog";
    import { Input } from "@/components/ui/input";
    import { Label } from "@/components/ui/label";
    import { onMounted, ref, watch, computed } from "vue";
    import type { SignInRequest, SignInResponse, UserDTO } from "@/apiClient";
    import { useForm } from 'vee-validate';
    import * as yup from 'yup';
    import { DataRequestHandler } from "@/helpers/DataRequestHandler.ts";
    import { useContextStore } from "@/stores/context.ts";
    import { storeToRefs } from "pinia";
    import { CookieManager } from "@/helpers/CookieManager.ts";
    import { LogOut, User } from "lucide-vue-next";

    const contextStore = useContextStore();
    const { user, apiKey } = storeToRefs(contextStore);

    const isLoggedIn = computed(() => !!apiKey.value && !!user.value);

    onMounted(() => {
        contextStore.getSession();
    })

    const { values, errors, defineField } = useForm({
        validationSchema: yup.object({
            usernameInput: yup.string().required(),
            emailInput: yup.string().email().required(),
        }),
    });

    watch(errors, (errors) => {
        console.log(errors)
    })

    const signup = ref<boolean>(false);
    const [usernameInput, usernameAttrs] = defineField('usernameInput');
    const [emailInput] = defineField('emailInput');
    const [passwordInput] = defineField('passwordInput');

    const signInOpen = ref(false);

    const signinDrh = new DataRequestHandler();
    signinDrh.onSuccessCallback = (data) => {
        console.log('Signin successful:', data);
        contextStore.setSession(data as SignInResponse);
        signInOpen.value = false;
    };
    signinDrh.onErrorCallback = (error) => {
        console.error('Signin failed:', error);
    };

    const submitSignup = async () => {

        const userPayload: UserDTO = {
            pulsarr_user_id: 0,
            name: usernameInput.value,
            email: emailInput.value,
            password: passwordInput.value
        };

        console.log(userPayload);

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
        signupDrh.onErrorCallback = (error) => {
            console.error('Signup failed:', error);
        };

        await signupDrh.post('/user/add', userPayload);
    }
    const submitSignin = async () => {

        const signInPayload = {
            username: usernameInput.value ?? "",
            password: passwordInput.value,
            hw_key: CookieManager.getDeviceKey()
        } as SignInRequest;

        await signinDrh.post('/auth/signin', signInPayload);
    }

    const signOut = () => {
        CookieManager.removeApiKey();
        contextStore.$reset();
        window.location.reload();
    }

</script>

<template>
    <div>
        <!-- Logged in state -->
        <div v-if="isLoggedIn" class="flex items-center gap-2">
            <div class="flex items-center gap-2 px-3 py-1.5 rounded-md bg-secondary">
                <User class="w-4 h-4" />
                <span class="text-sm font-medium">{{ user?.name }}</span>
            </div>
            <Button variant="ghost" size="icon" @click="signOut" title="Sign Out">
                <LogOut class="w-4 h-4" />
            </Button>
        </div>

        <!-- Logged out state -->
        <Dialog v-else v-model:open="signInOpen">
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
                    </div>
                    <div v-if="signup" class="grid grid-cols-4 items-center gap-4">
                        <Label for="email" class="text-right">
                            Email
                        </Label>
                        <Input id="email" default-value="" v-model="emailInput" class="col-span-3" />
                    </div>
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="password" class="text-right">
                            Password
                        </Label>
                        <Input id="password" v-model="passwordInput" type="password" class="col-span-3" />
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