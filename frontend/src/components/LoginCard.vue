<script lang="ts" setup>
import {
  UserIcon,
  EnvelopeIcon,
  LockClosedIcon,
  ArrowRightIcon,
} from "@heroicons/vue/24/outline";
</script>
<template>
    <div class="relative flex flex-col rounded-xl bg-white p-4 py-8 md:py-12 shadow-sm w-full">
          <form class="flex flex-col items-center text-sm text-gray-700">
            <div class="w-full px-4 md:px-12 lg:px-20 xl:px-28">


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
                    required
                  />
                </div>
              </label>

              <div class="flex justify-between">
              <label class="block font-semibold text-gray-700 text-sm mb-1" for="password">
                Password
              </label>
              <RouterLink to="#" class=" text-blue-600 hover:text-blue-700 transition-colors">Forget Password</RouterLink>
              </div>
                <div
                  class="flex items-center mt-2 mb-6 h-10 pl-3 border border-slate-300 rounded-full bg-gray-50 focus-within:ring-1 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden"
                >
                  <LockClosedIcon class="h-4 w-4 text-gray-400 shrink-0" />
                  <input
                    type="password"
                    class="h-full px-2 w-full outline-none bg-transparent text-gray-700 text-sm font-normal placeholder:text-gray-400"
                    placeholder="Enter your password"
                    id="password"
                    required
                  />
                </div>

              <button
                type="submit"
                class="mt-15 flex items-center justify-center gap-1 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white text-sm font-semibold py-2.5 w-full rounded-full transition-colors"
              >
                Login
                <ArrowRightIcon class="w-5 h-5"/>
              </button>

            </div>
          </form>
        </div>

</template>