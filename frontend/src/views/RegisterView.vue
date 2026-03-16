<script lang="ts" setup>
import CreateCard from "@/components/CreateCard.vue";
import NotesHeader from "@/components/NotesHeader.vue";
import NotesFooter from "@/components/NotesFooter.vue";

import { RouterLink } from "vue-router";

</script>
<template>
  <NotesHeader>
    <nav
      class="max-w-7xl mx-auto h-full flex justify-between items-center px-6"
    >
      <a href="/" class="font-bold text-lg text-gray-900">NoteInventory</a>
      <RouterLink
        to="/login"
        class="font-medium text-blue-600 hover:text-blue-700 transition-colors"
        >Sign in instead
      </RouterLink>
    </nav>
  </NotesHeader>

  <main class="h-full flex items-center justify-center px-4 py-12 bg-gray-300/20 ">
    <div
      class="flex flex-col gap-9 w-full max-w-sm md:max-w-lg lg:max-w-2xl xl:max-w-3xl"
    >
      <div class="text-center ">
        <h1 class="font-bold text-4xl mb-2 text-gray-900">
          Create your account
        </h1>
        <p class="font-normal text-gray-500">Join writers and thinkers</p>
      </div>

      <CreateCard class="shadow-lg" />
      <p class="text-center text-gray-500 ">
        Already have an account?
        <RouterLink to="/" class="font-semibold text-blue-700">Sign in instead</RouterLink>
      </p>
    </div>
  </main>
  <NotesFooter />
</template>
