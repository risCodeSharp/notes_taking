<script lang="ts" setup>
import {
  EnvelopeIcon,
  LockClosedIcon,
  ArrowRightIcon,
} from "@heroicons/vue/24/outline";
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "@/stores/authStore";
import type { AxiosError } from "axios";

interface ErrorResponse { message: string }

const router    = useRouter();
const authStore = useAuthStore();

const email        = ref("");
const password     = ref("");
const errorMessage = ref("");
const isLoading    = ref(false);

async function loginUser() {
  isLoading.value    = true;
  errorMessage.value = "";
  try {
    await authStore.login({ email: email.value, password: password.value });
    router.push("/");
  } catch (error) {
    const err = error as AxiosError<ErrorResponse>;
    errorMessage.value = err.response?.data?.message ?? "Invalid email or password.";
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <div class="relative flex flex-col rounded-xl bg-white p-4 py-8 md:py-12 shadow-sm w-full">
    <form class="flex flex-col items-center text-sm text-gray-700" @submit.prevent="loginUser">
      <div class="w-full px-4 md:px-12 lg:px-20 xl:px-28">

        <label class="block font-semibold text-gray-700 text-sm mb-1">
          Email Address
          <div class="flex items-center mt-2 mb-6 h-10 pl-3 border border-slate-300 rounded-full bg-gray-50 focus-within:ring-1 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden">
            <EnvelopeIcon class="h-4 w-4 text-gray-400 shrink-0" />
            <input
              type="email"
              class="h-full px-2 w-full outline-none bg-transparent text-gray-700 text-sm font-normal placeholder:text-gray-400"
              placeholder="Enter your email address"
              v-model="email"
              required
            />
          </div>
        </label>

        <div class="flex justify-between">
          <label class="block font-semibold text-gray-700 text-sm mb-1" for="password">Password</label>
          <RouterLink to="#" class="text-blue-600 hover:text-blue-700 transition-colors text-sm">Forgot Password</RouterLink>
        </div>

        <div class="flex items-center mt-2 mb-6 h-10 pl-3 border border-slate-300 rounded-full bg-gray-50 focus-within:ring-1 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden">
          <LockClosedIcon class="h-4 w-4 text-gray-400 shrink-0" />
          <input
            type="password"
            class="h-full px-2 w-full outline-none bg-transparent text-gray-700 text-sm font-normal placeholder:text-gray-400"
            placeholder="Enter your password"
            id="password"
            v-model="password"
            required
          />
        </div>

        <p v-if="errorMessage" class="text-red-500 text-sm mb-3 px-1">{{ errorMessage }}</p>

        <button
          type="submit"
          :disabled="isLoading"
          class="mt-2 flex items-center justify-center gap-1 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 disabled:opacity-60 disabled:cursor-not-allowed text-white text-sm font-semibold py-2.5 w-full rounded-full transition-colors"
        >
          <span>{{ isLoading ? 'Signing in…' : 'Login' }}</span>
          <i v-if="isLoading" class="pi pi-spin pi-spinner w-5 h-5"></i>
          <ArrowRightIcon v-else class="w-5 h-5" />
        </button>

      </div>
    </form>
  </div>
</template>