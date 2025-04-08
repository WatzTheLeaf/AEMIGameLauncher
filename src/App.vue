<script setup>
import { ref, computed, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import GameBar from "./components/GameBar.vue";
import GameDetails from "./components/GameDetails.vue";

const games = ref([]);
const activeGameId = ref(null);

async function retrieveGames() {
  try {
    const configMsg = await invoke("retrieve_games");
    const rawGames = JSON.parse(configMsg);
    games.value = rawGames.map((game, index) => ({
      id: index + 1,
      name: game.name || `Jeu inconnu`,
      image: game.preview_image || '/src/assets/blank.jpg',
      description: game.description || 'Aucune description.',
      authors: game.authors && game.authors.length ? game.authors : ['Inconnu']
    }));
    if (games.value.length > 0) {
      activeGameId.value = games.value[0].id;
    }
  } catch (error) {
    console.error("Error while retrieving games :", error);
  }
}

const activeGame = computed(() => games.value.find(game => game.id === activeGameId.value));

const isNavigating = ref(true);

const navigate = () => {
  isNavigating.value = true;
};

const readDetails = () => {
  isNavigating.value = false;
};

const handleKeyDown = (event) => {
  if (event.key === "ArrowLeft") navigate();
  if (event.key === "ArrowRight") readDetails();
};

onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
  retrieveGames();
});

onUnmounted(() => window.removeEventListener("keydown", handleKeyDown));
</script>

<template>
  <main class="container">
    <section class="flex items-center justify-center bg-linear-to-b from-slate-800 to-slate-950 w-screen h-screen static" id="csbp">
      <div class="flex w-full h-full items-center" :class="{'justify-evenly': isNavigating, 'justify-center': !isNavigating}">
        <div v-if="!isNavigating" class="text-5xl cursor-pointer mb-2 text-slate-50 z-20">◂</div>
        <GameBar v-if="isNavigating" class="z-20" v-model:activeGameId="activeGameId" :games="games" />
        <GameDetails v-if="activeGame" class="z-20 backdrop-blur-xl top-2/7" :game="activeGame"/>
        <div v-if="isNavigating" class="text-5xl cursor-pointer mb-2 text-slate-50 z-20">▸</div>
      </div>
      <img src="/src/assets/aemi-logo-landscape-black.svg" alt="aemi-logo" class="absolute invert right-1/10 w-[40vw] h-auto">
      <img v-if="activeGame" :src="activeGame.image" alt="csb" class="absolute h-screen w-screen z-5 blur-[2px] transition-opacity duration-1000" :class="{'opacity-0': isNavigating, 'opacity-100': !isNavigating}">
    </section>
  </main>
</template>

.transition-opacity {
  transition: opacity 2s;
}