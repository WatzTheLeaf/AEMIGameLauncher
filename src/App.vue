<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import GameBar from "./components/GameBar.vue";
import GameDetails from "./components/GameDetails.vue";

// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }


const games = ref([
{ id: 1, name: "Zelda", image: "/src/assets/zelda.jpg", description: "0845633483454116882138652220338621669235003026522087797107923466708735091807580366573520416241326132251782939518712990937279102623785712299223188310489472029003784803286446047862428427954095909714921931600049452118862929006077041014486600422050097985485948925833669488214926981815865267800844061152432449447616189125654897049543275417742017715258304822644884656257588043594146309255733377669459347732227143783046187904955954152595384319135966283594918734045544257732767108937475078633688298446112413399270378404975101690275310543330926870026717589044912395110114152469691094524410852532222938534420111286337162961083176908915590711760535672212151774636796501546467428854340053654209418614470186842982", authors:["Jean-Luc", "Jean-Marie", "Manu"]},
{ id: 2, name: "Mario Kart", image: "/src/assets/mariokart.jpg", description: "Wonderkart c'est cool", authors: ["LLF", "WTE"]},
{ id: 3, name: "Metroid", image: "/src/assets/metroid.jpg", description: "AAAAAAAAAAAAH", authors: ["Jean Clanche"]},
{ id: 4, name: "Rayman", image: "/src/assets/rayman.jpg", description: "===============================================", authors: ["John Doe"]},
]);

const activeGameId = ref(games.value[0].id);

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

onMounted(() => window.addEventListener("keydown", handleKeyDown));
onUnmounted(() => window.removeEventListener("keydown", handleKeyDown));

</script>

<template>
  <main class="container">
    <section class="flex items-center justify-center bg-linear-to-b from-slate-800 to-slate-950 w-screen h-screen static" id="csbp">
      <div class="flex w-full h-full items-center" :class="{'justify-evenly': isNavigating, 'justify-center': !isNavigating}">
        <div v-if="!isNavigating" class="text-5xl cursor-pointer mb-2 text-slate-50">◂</div>
        <GameBar v-if="isNavigating" class="z-20" v-model:activeGameId="activeGameId" :games="games" />
        <GameDetails class="z-20 backdrop-blur-xl top-2/7" :game="activeGame" />
        <div v-if="isNavigating" class="text-5xl cursor-pointer mb-2 text-slate-50">▸</div>
      </div>
      <img src="/src/assets/aemi-logo-landscape-black.svg" alt="aemi-logo" class="absolute invert right-1/10 w-[40vw] h-auto">
      <img v-if="activeGame" :src="activeGame.image" alt="csb" class="absolute h-screen w-screen z-5 blur-[2px] transition-opacity duration-1000" :class="{'opacity-0': isNavigating, 'opacity-100': !isNavigating}">
    </section>
  </main>
</template>

.transition-opacity {
  transition: opacity 2s;
}