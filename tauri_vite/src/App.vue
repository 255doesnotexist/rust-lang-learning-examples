<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
</script>

<template>
  <div class="flex">
    <v-sheet
      class="absolute inset-0 flex items-center justify-center flex-wrap text-center mx-auto"
      elevation="4"
      rounded
    >
      <v-card class="mx-4 my-6">
      <v-card-text :style="{ margin: '20px' }"> <!-- Apply margin here -->
          <div :style="{ margin: '20px' }">

          <p :style="{ fontSize: '24px' }">这是 Tauri。</p>
          

          <div class="row">
            <a href="https://vitejs.dev" target="_blank">
              <img src="/vite.svg" class="logo vite" alt="Vite logo" />
            </a>
            <a href="https://tauri.app" target="_blank">
              <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
            </a>
            <a href="https://vuejs.org/" target="_blank">
              <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
            </a>
          </div>

          <p>点击 Tauri、Vite、Vue 图标可查看详情。</p>

          <p>
            推荐 IDE 配置:
            <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
            +
            <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
            +
            <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
              >Tauri</a
            >
            +
            <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
              >rust-analyzer</a
            >
          </p>

          </div>

          <Greet />
      </v-card-text>
      </v-card>
    </v-sheet>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
