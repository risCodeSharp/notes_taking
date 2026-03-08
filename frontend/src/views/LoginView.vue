<script lang="ts" setup>
import LoginCard from "@/components/LoginCard.vue";
import NotesHeader from "@/components/NotesHeader.vue";
import NotesFooter from "@/components/NotesFooter.vue";

</script>
<template>
  <NotesHeader>
    <nav
      class="max-w-7xl mx-auto h-full flex justify-between items-center px-6"
    >
      <a href="/" class="font-bold text-lg text-gray-900">NoteInventory</a>
      <RouterLink
        to="/"
        class="font-medium text-blue-600 hover:text-blue-700 transition-colors"
        >Sign up</RouterLink
      >
    </nav>
  </NotesHeader>

  <main class="h-full flex items-center justify-center px-4 py-12">
    <div
      class="flex flex-col gap-8 w-full max-w-sm md:max-w-lg lg:max-w-2xl xl:max-w-3xl"
    >
      <div class="text-center">
        <h1 class="font-bold text-4xl mb-2 text-gray-900">Welcome back!</h1>
        <p class="font-normal text-gray-500">
          please enter you details to sign in
        </p>
      </div>

      <LoginCard />
      <p class="text-center text-gray-500">
        Don't have an account?
        <a href="#" class="font-semibold text-blue-700">Sign up instead</a>
      </p>
    </div>
  </main>
    <NotesFooter />

</template>
