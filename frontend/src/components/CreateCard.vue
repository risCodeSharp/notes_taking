<script lang="ts" setup>
import {
  UserIcon,
  EnvelopeIcon,
  LockClosedIcon,
  ArrowRightIcon,
} from "@heroicons/vue/24/outline";
import { type Ref, ref } from "vue";
import api from "@/services/api";
import type { AxiosError, AxiosResponse } from "axios";
import { useRouter } from "vue-router";

interface CreateUserRequest {
  name: string;
  email: string;
  password: string;
}

interface CreateUserResponse {
  data: {
    token: string;
    user: {
      email: string;
      id: number;
      name: string;
    };
  };
  message: string;
  success: boolean;
}

interface ErrorResponse {
  data: {
    message: string;
  };
  success: boolean;
}

const router = useRouter();

const name: Ref<string> = ref("");
const email: Ref<string> = ref("");
const password: Ref<string> = ref("");

const errorMessage: Ref<string> = ref("");

async function createUser() {
  try {
    const payload: CreateUserRequest = {
      name: name.value.trim(),
      email: email.value,
      password: password.value,
    };
    const response: AxiosResponse<CreateUserResponse> = await api.post(
      "/api/users/register",
      payload,
    );
    if (response.data.success) {
      //redirect
      router.push("/");
      
    } else {
      errorMessage.value = response.data.message;
    }
  } catch (error) {
    const err = error as AxiosError<ErrorResponse>;
    errorMessage.value =
      err.response?.data?.data?.message ?? "Something went wrong";
  }
}
</script>
<template>
  <div
    class="relative flex flex-col rounded-xl bg-white p-4 py-8 md:py-12 shadow-sm w-full"
  >
    <form class="flex flex-col items-center text-sm text-gray-700" @submit.prevent="createUser">
      <div class="w-full px-4 md:px-12 lg:px-20 xl:px-28">
        <label class="block font-semibold text-gray-700 text-sm mb-1">
          Full Name
          <div
            class="flex items-center mt-2 mb-6 h-10 pl-3 border border-slate-300 rounded-full bg-gray-50 focus-within:ring-1 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden"
          >
            <UserIcon class="w-4 h-4 text-gray-400 shrink-0" />
            <input
              type="text"
              class="h-full px-2 w-full outline-none bg-transparent text-gray-700 text-sm font-normal placeholder:text-gray-400"
              placeholder="Enter your full name"
              v-model="name"
              required
            />
          </div>
        </label>

        <label class="block font-semibold text-gray-700 text-sm mb-1">
          Email Address
          <div
            class="flex items-center mt-2 mb-6 h-10 pl-3 border border-slate-300 rounded-full bg-gray-50 focus-within:ring-1 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden"
          >
            <EnvelopeIcon class="h-4 w-4 text-gray-400 shrink-0" />
            <input
              type="email"
              class="h-full px-2 w-full outline-none bg-transparent text-gray-700 text-sm font-normal placeholder:text-gray-400"
              placeholder="Enter your email address"
              v-model="email"
              requiredl
            />
          </div>
        </label>

        <label class="block font-semibold text-gray-700 text-sm mb-1">
          Password
          <div
            class="flex items-center mt-2 mb-6 h-10 pl-3 border border-slate-300 rounded-full bg-gray-50 focus-within:ring-1 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden"
          >
            <LockClosedIcon class="h-4 w-4 text-gray-400 shrink-0" />
            <input
              type="password"
              class="h-full px-2 w-full outline-none bg-transparent text-gray-700 text-sm font-normal placeholder:text-gray-400"
              placeholder="Enter your password"
              v-model="password"
              required
            />
          </div>
        </label>
        <p v-if="errorMessage" class="text-red-500 text-sm mb-2">
          {{ errorMessage }}
        </p>
        <label
          for="remember"
          class="flex items-center gap-2 mb-6 cursor-pointer"
        >
          <input
            id="remember"
            type="checkbox"
            class="w-4 h-4 border border-gray-300 rounded accent-blue-600 focus:ring-1 focus:ring-blue-500"
            required
          />
          <p class="text-sm font-medium text-gray-600 select-none">
            I agree with the
            <a
              href="#"
              class="text-blue-600 font-semibold hover:text-blue-700 hover:underline transition-colors"
            >
              terms and conditions </a
            >.
          </p>
        </label>

        <button
          type="submit"
          class="flex items-center justify-center gap-1 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white text-sm font-semibold py-2.5 w-full rounded-full transition-colors"
        >
          Create Account
          <ArrowRightIcon class="w-5 h-5" />
        </button>
      </div>
    </form>
  </div>
</template>
