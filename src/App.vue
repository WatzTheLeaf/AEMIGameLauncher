<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import GameBar from "./components/GameBar.vue";
import GameDetails from "./components/GameDetails.vue";

// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

</script>

<template>
  <main class="container">
    <section class="flex items-center justify-center bg-linear-to-b from-slate-800 to-slate-950 w-screen h-screen static">
      <div class="flex w-full h-full justify-evenly items-center">
        <GameBar class="z-20 backdrop-blur-md"></GameBar>
        <GameDetails class="z-20 backdrop-blur-md top-2/7"></GameDetails>
      </div>
      <img src="/src/assets/aemi-logo-landscape-black.svg" alt="aemi-logo" class="absolute invert right-1/10 w-[40vw] h-auto"> <!-- TODO à remplacer par un logo sans texte-->
    </section>
  </main>
</template>