<script setup lang="ts">
    import { Button } from "@/components/ui/button";
    import { Dialog, DialogHeader, DialogFooter, DialogContent, DialogTitle, DialogTrigger, DialogDescription } from "@/components/ui/dialog";
    import { Input } from "@/components/ui/input";
    import { Label } from "@/components/ui/label";
    import { ref, watch } from "vue";
    import type { SignInRequest, UserDTO } from "@/apiClient";
    import { useForm } from 'vee-validate';
    import * as yup from 'yup';

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
    const signin = ref<SignInRequest>({username: "", password: ""} as SignInRequest)
    const user = ref<UserDTO>({name: "", email: "", password: ""} as UserDTO);

    const submitSignup = () => {
        user.value = {pulsarr_user_id: 0, name: usernameInput.value, email: emailInput.value, password: passwordInput.value}
        console.log(user.value)
    }
    const submitSignin = () => {
        signin.value = {username: usernameInput.value, password: passwordInput.value}
        console.log(signin.value)
    }

</script>

<template>
    <div>
        <Dialog>
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
                    <div v-if="signup">
                        <div class="grid grid-cols-4 items-center gap-4">
                            <Label for="username" class="text-right">
                                Username
                            </Label>
                            <Input id="username" v-model="usernameInput" v-bind="usernameAttrs" class="col-span-3" />
                        </div>
                    </div>
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="email" class="text-right">
                            Email
                        </Label>
                        <Input id="email" default-value="" v-model="emailInput" class="col-span-3" />
                    </div>
                    <div class="grid grid-cols-4 items-center gap-4">
                        <Label for="password" class="text-right">
                            Password
                        </Label>
                        <Input id="password" v-model="passwordInput" class="col-span-3" />
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