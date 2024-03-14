<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const words = ref("");
const snackbar = ref(false); 

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { words: words.value });
  snackbar.value = true; // Update snackbar using its ref value
}
const onClick = () => {
  snackbar.value = false; // Update snackbar using its ref value
};
</script>

<template>
  <div class="flex items-center justify-center mx-20 w-full">
      <v-form class="w-3/4" @submit.prevent="greet">
        <v-responsive
          class="mx-20 w-full"
        >
          <v-text-field id="greet-input" v-model="words" label="说点什么" placeholder="输入文字..."></v-text-field>
        </v-responsive>
          <v-btn prepend-icon="$vuetify" type="submit">问好</v-btn>

            <v-snackbar v-model="snackbar" location="center">
              <div class="flex">
                <div class="w-3/4">
                  <p>{{ greetMsg }}</p>
                </div>
                <div class="w-1/4">
                  <v-btn @click="onClick">关闭</v-btn>
                </div>
              </div>
            </v-snackbar>
      </v-form>
  </div>
</template>