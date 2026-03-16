<script lang="ts" setup>
import {
  UserIcon,
  EnvelopeIcon,
  LockClosedIcon,
  ShieldCheckIcon,
  ArrowLeftIcon,
  CheckIcon,
  ExclamationTriangleIcon,
} from "@heroicons/vue/24/outline";
import { ref, computed, onMounted, watch } from "vue";
import { useRouter } from "vue-router";
import { useAuthStore } from "@/stores/authStore";
import api from "@/services/api";
import type { AxiosError } from "axios";
import NotesHeader from "@/components/NotesHeader.vue";
import Avatar from "primevue/avatar";

interface ErrorResponse { message: string }

const router    = useRouter();
const authStore = useAuthStore();

// ── Active section ────────────────────────────────────────────
type Section = "profile" | "security" | "danger";
const activeSection = ref<Section>("profile");

const sections = [
  { id: "profile"  as Section, label: "Profile",         icon: UserIcon          },
  { id: "security" as Section, label: "Security",        icon: ShieldCheckIcon   },
  { id: "danger"   as Section, label: "Danger Zone",     icon: ExclamationTriangleIcon },
];

// ── Fetch fresh user from server on mount ────────────────────
onMounted(async () => {
  await authStore.fetchMe();
});

// Keep form fields in sync if fetchMe() updates the store after mount
watch(
  () => authStore.user,
  (u) => {
    if (u) {
      profileEmail.value = u.email ?? "";
    }
  },
  { deep: true }
);

// ── Profile form ──────────────────────────────────────────────
const profileName    = ref("");
const profileEmail   = ref(authStore.user?.email ?? "");
const profileLoading = ref(false);
const profileSuccess = ref(false);
const profileError   = ref("");

async function saveProfile() {
  profileLoading.value = true;
  profileSuccess.value = false;
  profileError.value   = "";
  try {
    await api.put<any>("/api/users/me", {
      name:  profileName.value.trim(),
      email: profileEmail.value.trim(),
    });
    // Re-fetch from server so the store, navbar avatar, and form fields
    // all reflect the canonical backend value in one shot.
    await authStore.fetchMe();
    profileSuccess.value = true;
    setTimeout(() => (profileSuccess.value = false), 3000);
  } catch (e) {
    const err = e as AxiosError<ErrorResponse>;
    profileError.value = err.response?.data?.message ?? "Failed to update profile.";
  } finally {
    profileLoading.value = false;
  }
}

// ── Password form ─────────────────────────────────────────────
const currentPassword  = ref("");
const newPassword      = ref("");
const confirmPassword  = ref("");
const passwordLoading  = ref(false);
const passwordSuccess  = ref(false);
const passwordError    = ref("");

const passwordMismatch = computed(
  () => confirmPassword.value.length > 0 && newPassword.value !== confirmPassword.value
);

async function changePassword() {
  if (passwordMismatch.value) return;
  passwordLoading.value = true;
  passwordSuccess.value = false;
  passwordError.value   = "";
  try {
    await api.put("/api/users/me/password", {
      old_password: currentPassword.value,
      new_password:     newPassword.value,
    });
    passwordSuccess.value = true;
    currentPassword.value = "";
    newPassword.value     = "";
    confirmPassword.value = "";
    setTimeout(() => (passwordSuccess.value = false), 3000);
  } catch (e) {
    const err = e as AxiosError<ErrorResponse>;
    passwordError.value = err.response?.data?.message ?? "Failed to change password.";
  } finally {
    passwordLoading.value = false;
  }
}

// ── Danger zone ───────────────────────────────────────────────
const deleteConfirm  = ref("");
const deleteLoading  = ref(false);
const deleteError    = ref("");
const showDeleteModal = ref(false);

async function deleteAccount() {
  if (deleteConfirm.value !== "delete my account") return;
  deleteLoading.value = true;
  deleteError.value   = "";
  try {
    await api.delete("/api/users/me");
    authStore.logout();
    router.push("/register");
  } catch (e) {
    const err = e as AxiosError<ErrorResponse>;
    deleteError.value = err.response?.data?.message ?? "Failed to delete account.";
    deleteLoading.value = false;
  }
}

// ── Initials / display ────────────────────────────────────────
const initials = computed(() => {
  const src = authStore.user?.email ?? "?";
  return src.charAt(0).toUpperCase();
});
const displayName = computed(() => authStore.user?.email ?? "");
</script>

<template>
  <div class="flex flex-col min-h-screen bg-gray-50">

    <!-- Header -->
    <NotesHeader>
      <nav class="max-w-7xl mx-auto h-full flex justify-between items-center px-6">
        <a href="/" class="font-bold text-lg text-gray-900">NoteInventory</a>
        <button
          @click="router.push('/')"
          class="flex items-center gap-1.5 text-sm font-medium text-gray-500 hover:text-blue-600 transition-colors"
        >
          <ArrowLeftIcon class="w-4 h-4" />
          Back to Editor
        </button>
      </nav>
    </NotesHeader>

    <main class="flex-1 max-w-5xl mx-auto w-full px-4 py-10 md:py-14">

      <!-- Page title + avatar hero -->
      <div class="flex items-center gap-5 mb-10">
        <div class="relative">
          <div class="p-1 rounded-full bg-linear-to-tr from-sky-400 to-blue-600 shadow-lg shadow-sky-200/50">
            <div class="bg-white p-0.5 rounded-full">
              <Avatar
                :label="initials"
                shape="circle"
                class="bg-linear-to-br from-sky-500 to-indigo-600 text-white font-black shadow-lg"
                style="width: 64px; height: 64px; font-size: 1.6rem;"
              />
            </div>
          </div>
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900 tracking-tight">Account Settings</h1>
          <p class="text-sm text-gray-500 mt-0.5">{{ displayName }}</p>
        </div>
      </div>

      <!-- Body: sidebar + content -->
      <div class="flex gap-6 items-start">

        <!-- Sidebar nav -->
        <aside class="w-48 shrink-0 hidden md:block">
          <nav class="flex flex-col gap-1 sticky top-6">
            <button
              v-for="s in sections"
              :key="s.id"
              @click="activeSection = s.id"
              :class="[
                'flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all duration-150 text-left w-full',
                activeSection === s.id
                  ? 'bg-blue-50 text-blue-700 shadow-sm ring-1 ring-blue-100'
                  : 'text-gray-500 hover:text-gray-800 hover:bg-gray-100',
                s.id === 'danger' && activeSection !== 'danger' ? 'hover:text-red-600 hover:bg-red-50' : ''
              ]"
            >
              <component
                :is="s.icon"
                class="w-4 h-4 shrink-0"
                :class="s.id === 'danger' ? (activeSection === 'danger' ? 'text-red-500' : 'text-gray-400') : ''"
              />
              <span :class="s.id === 'danger' && activeSection === 'danger' ? 'text-red-600' : ''">
                {{ s.label }}
              </span>
            </button>
          </nav>
        </aside>

        <!-- Mobile tabs -->
        <div class="flex gap-1 mb-4 md:hidden w-full">
          <button
            v-for="s in sections"
            :key="s.id"
            @click="activeSection = s.id"
            :class="[
              'flex-1 py-2 text-xs font-semibold rounded-lg transition-all',
              activeSection === s.id ? 'bg-blue-600 text-white shadow' : 'bg-white text-gray-500 border border-gray-200',
            ]"
          >{{ s.label }}</button>
        </div>

        <!-- Content panel -->
        <div class="flex-1 min-w-0">

          <!-- ── Profile ──────────────────────────────────────── -->
          <section v-if="activeSection === 'profile'" class="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 md:p-8">
            <div class="mb-6 pb-5 border-b border-gray-100">
              <h2 class="text-base font-bold text-gray-900 flex items-center gap-2">
                <UserIcon class="w-4 h-4 text-blue-500" /> Profile Information
              </h2>
              <p class="text-sm text-gray-500 mt-1">Update your name and email address.</p>
            </div>

            <form @submit.prevent="saveProfile" class="flex flex-col gap-5">

              <!-- Name -->
              <label class="flex flex-col gap-1.5">
                <span class="text-sm font-semibold text-gray-700">Full Name</span>
                <div class="flex items-center h-11 pl-3.5 border border-slate-200 rounded-full bg-gray-50 focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden">
                  <UserIcon class="w-4 h-4 text-gray-400 shrink-0" />
                  <input
                    v-model="profileName"
                    type="text"
                    placeholder="Your full name"
                    class="h-full px-2.5 w-full outline-none bg-transparent text-gray-700 text-sm placeholder:text-gray-400"
                  />
                </div>
              </label>

              <!-- Email -->
              <label class="flex flex-col gap-1.5">
                <span class="text-sm font-semibold text-gray-700">Email Address</span>
                <div class="flex items-center h-11 pl-3.5 border border-slate-200 rounded-full bg-gray-50 focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden">
                  <EnvelopeIcon class="w-4 h-4 text-gray-400 shrink-0" />
                  <input
                    v-model="profileEmail"
                    type="email"
                    placeholder="you@example.com"
                    class="h-full px-2.5 w-full outline-none bg-transparent text-gray-700 text-sm placeholder:text-gray-400"
                  />
                </div>
              </label>

              <!-- Feedback -->
              <p v-if="profileError" class="text-sm text-red-500 -mt-1 px-1">{{ profileError }}</p>
              <div v-if="profileSuccess" class="flex items-center gap-2 text-sm text-emerald-600 bg-emerald-50 border border-emerald-100 rounded-full px-4 py-2">
                <CheckIcon class="w-4 h-4 shrink-0" />
                Profile updated successfully.
              </div>

              <div class="flex justify-end pt-1">
                <button
                  type="submit"
                  :disabled="profileLoading"
                  class="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 disabled:opacity-60 disabled:cursor-not-allowed text-white text-sm font-semibold px-6 py-2.5 rounded-full transition-colors shadow-sm"
                >
                  <i v-if="profileLoading" class="pi pi-spin pi-spinner text-xs"></i>
                  {{ profileLoading ? "Saving…" : "Save Changes" }}
                </button>
              </div>
            </form>
          </section>

          <!-- ── Security ─────────────────────────────────────── -->
          <section v-if="activeSection === 'security'" class="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 md:p-8">
            <div class="mb-6 pb-5 border-b border-gray-100">
              <h2 class="text-base font-bold text-gray-900 flex items-center gap-2">
                <ShieldCheckIcon class="w-4 h-4 text-blue-500" /> Change Password
              </h2>
              <p class="text-sm text-gray-500 mt-1">Use a strong password you don't use elsewhere.</p>
            </div>

            <form @submit.prevent="changePassword" class="flex flex-col gap-5">

              <label class="flex flex-col gap-1.5">
                <span class="text-sm font-semibold text-gray-700">Current Password</span>
                <div class="flex items-center h-11 pl-3.5 border border-slate-200 rounded-full bg-gray-50 focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden">
                  <LockClosedIcon class="w-4 h-4 text-gray-400 shrink-0" />
                  <input
                    v-model="currentPassword"
                    type="password"
                    placeholder="Enter current password"
                    required
                    class="h-full px-2.5 w-full outline-none bg-transparent text-gray-700 text-sm placeholder:text-gray-400"
                  />
                </div>
              </label>

              <label class="flex flex-col gap-1.5">
                <span class="text-sm font-semibold text-gray-700">New Password</span>
                <div class="flex items-center h-11 pl-3.5 border border-slate-200 rounded-full bg-gray-50 focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-blue-500 transition-all overflow-hidden">
                  <LockClosedIcon class="w-4 h-4 text-gray-400 shrink-0" />
                  <input
                    v-model="newPassword"
                    type="password"
                    placeholder="Enter new password"
                    required
                    minlength="8"
                    class="h-full px-2.5 w-full outline-none bg-transparent text-gray-700 text-sm placeholder:text-gray-400"
                  />
                </div>
              </label>

              <label class="flex flex-col gap-1.5">
                <span class="text-sm font-semibold text-gray-700">Confirm New Password</span>
                <div
                  class="flex items-center h-11 pl-3.5 border rounded-full bg-gray-50 focus-within:ring-2 transition-all overflow-hidden"
                  :class="passwordMismatch
                    ? 'border-red-300 focus-within:ring-red-400'
                    : 'border-slate-200 focus-within:ring-blue-500 focus-within:border-blue-500'"
                >
                  <LockClosedIcon class="w-4 h-4 shrink-0" :class="passwordMismatch ? 'text-red-400' : 'text-gray-400'" />
                  <input
                    v-model="confirmPassword"
                    type="password"
                    placeholder="Confirm new password"
                    required
                    class="h-full px-2.5 w-full outline-none bg-transparent text-gray-700 text-sm placeholder:text-gray-400"
                  />
                </div>
                <p v-if="passwordMismatch" class="text-xs text-red-500 px-3">Passwords don't match.</p>
              </label>

              <!-- Strength hint -->
              <div v-if="newPassword.length > 0" class="flex items-center gap-2 px-1">
                <div class="flex gap-1 flex-1">
                  <div
                    v-for="i in 4" :key="i"
                    class="h-1 flex-1 rounded-full transition-all duration-300"
                    :class="newPassword.length >= i * 2
                      ? (newPassword.length >= 12 ? 'bg-emerald-400' : newPassword.length >= 8 ? 'bg-yellow-400' : 'bg-red-400')
                      : 'bg-gray-200'"
                  />
                </div>
                <span class="text-xs text-gray-400">
                  {{ newPassword.length < 6 ? 'Too short' : newPassword.length < 10 ? 'Fair' : newPassword.length < 14 ? 'Good' : 'Strong' }}
                </span>
              </div>

              <p v-if="passwordError" class="text-sm text-red-500 -mt-1 px-1">{{ passwordError }}</p>
              <div v-if="passwordSuccess" class="flex items-center gap-2 text-sm text-emerald-600 bg-emerald-50 border border-emerald-100 rounded-full px-4 py-2">
                <CheckIcon class="w-4 h-4 shrink-0" />
                Password changed successfully.
              </div>

              <div class="flex justify-end pt-1">
                <button
                  type="submit"
                  :disabled="passwordLoading || passwordMismatch || !currentPassword || !newPassword"
                  class="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 disabled:opacity-60 disabled:cursor-not-allowed text-white text-sm font-semibold px-6 py-2.5 rounded-full transition-colors shadow-sm"
                >
                  <i v-if="passwordLoading" class="pi pi-spin pi-spinner text-xs"></i>
                  {{ passwordLoading ? "Updating…" : "Update Password" }}
                </button>
              </div>
            </form>
          </section>

          <!-- ── Danger Zone ──────────────────────────────────── -->
          <section v-if="activeSection === 'danger'" class="bg-white rounded-2xl shadow-sm border border-red-100 p-6 md:p-8">
            <div class="mb-6 pb-5 border-b border-red-100">
              <h2 class="text-base font-bold text-red-700 flex items-center gap-2">
                <ExclamationTriangleIcon class="w-4 h-4" /> Danger Zone
              </h2>
              <p class="text-sm text-gray-500 mt-1">These actions are permanent and cannot be undone.</p>
            </div>

            <div class="rounded-xl border border-red-200 bg-red-50/50 p-5">
              <div class="flex items-start justify-between gap-4 flex-wrap">
                <div>
                  <p class="text-sm font-semibold text-gray-800">Delete Account</p>
                  <p class="text-sm text-gray-500 mt-0.5 max-w-sm">
                    Permanently deletes your account and all associated notebooks and notes. There is no going back.
                  </p>
                </div>
                <button
                  @click="showDeleteModal = true"
                  class="shrink-0 flex items-center gap-2 bg-red-600 hover:bg-red-700 active:bg-red-800 text-white text-sm font-semibold px-5 py-2.5 rounded-full transition-colors shadow-sm"
                >
                  Delete Account
                </button>
              </div>
            </div>
          </section>

        </div>
      </div>
    </main>

    <!-- Delete confirmation modal -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="opacity-0"
        leave-active-class="transition duration-150 ease-in"
        leave-to-class="opacity-0"
      >
        <div v-if="showDeleteModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm px-4">
          <Transition
            enter-active-class="transition duration-200 ease-out"
            enter-from-class="opacity-0 scale-95"
            leave-active-class="transition duration-150 ease-in"
            leave-to-class="opacity-0 scale-95"
          >
            <div v-if="showDeleteModal" class="bg-white rounded-2xl shadow-2xl border border-gray-100 w-full max-w-md p-7">
              <div class="flex items-center gap-3 mb-4">
                <div class="p-2 rounded-xl bg-red-50">
                  <ExclamationTriangleIcon class="w-5 h-5 text-red-500" />
                </div>
                <h3 class="text-base font-bold text-gray-900">Delete your account?</h3>
              </div>
              <p class="text-sm text-gray-500 mb-5 leading-relaxed">
                This will permanently delete your account, all notebooks, and all notes.
                Type <strong class="text-gray-700 font-semibold">delete my account</strong> to confirm.
              </p>
              <input
                v-model="deleteConfirm"
                type="text"
                placeholder="delete my account"
                class="w-full h-11 px-4 border border-gray-200 rounded-full bg-gray-50 text-sm outline-none focus:ring-2 focus:ring-red-400 focus:border-red-400 transition-all mb-2"
              />
              <p v-if="deleteError" class="text-sm text-red-500 mb-3 px-1">{{ deleteError }}</p>
              <div class="flex gap-3 mt-4">
                <button
                  @click="showDeleteModal = false; deleteConfirm = ''"
                  class="flex-1 py-2.5 text-sm font-semibold text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-full transition-colors"
                >
                  Cancel
                </button>
                <button
                  @click="deleteAccount"
                  :disabled="deleteConfirm !== 'delete my account' || deleteLoading"
                  class="flex-1 flex items-center justify-center gap-2 py-2.5 text-sm font-semibold text-white bg-red-600 hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed rounded-full transition-colors"
                >
                  <i v-if="deleteLoading" class="pi pi-spin pi-spinner text-xs"></i>
                  {{ deleteLoading ? "Deleting…" : "Delete Forever" }}
                </button>
              </div>
            </div>
          </Transition>
        </div>
      </Transition>
    </Teleport>

  </div>
</template>