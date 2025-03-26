<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
import Game from "./Game.vue";
import { invoke } from "@tauri-apps/api/core";

const games = ref([
    { id: 1, name: "Zelda", image: "/src/assets/zelda.jpg" },
    { id: 2, name: "Mario Kart", image: "/src/assets/mariokart.jpg" },
    { id: 3, name: "Metroid", image: "/src/assets/metroid.jpg" },
]);

const activeGameId = ref(games.value[0].id);

const activeGame = computed(() => games.value.find(game => game.id === activeGameId.value));

const canNavigateUp = computed(() => games.value.findIndex(game => game.id === activeGameId.value) > 0);
const canNavigateDown = computed(() => games.value.findIndex(game => game.id === activeGameId.value) < games.value.length - 1);

const navigateGames = (direction) => {
    const currentIndex = games.value.findIndex(game => game.id === activeGameId.value);
    if (direction === "up" && canNavigateUp.value) {
        activeGameId.value = games.value[currentIndex - 1].id;
    } else if (direction === "down" && canNavigateDown.value) {
        activeGameId.value = games.value[currentIndex + 1].id;
    }
};

const handleKeyDown = (event) => {
    if (event.key === "ArrowUp") navigateGames("up");
    if (event.key === "ArrowDown") navigateGames("down");
};

onMounted(() => window.addEventListener("keydown", handleKeyDown));
onUnmounted(() => window.removeEventListener("keydown", handleKeyDown));
</script>

<template>
    <div class="h-full p-4 w-2/5 overflow-y-auto text-center flex flex-col justify-center items-center">
        <div v-if="canNavigateUp" class="text-2xl cursor-pointer mb-2 text-slate-50" @click="navigateGames('up')">▲</div>
        <div v-else class="text-2xl cursor-pointer mb-2 text-slate-50">‎</div>
        
        <Game v-if="activeGame" :game="activeGame" />

        <div v-if="canNavigateDown" class="text-2xl cursor-pointer mt-2 text-slate-50" @click="navigateGames('down')">▼</div>
        <div v-else class="text-2xl cursor-pointer mt-2 text-slate-50">‎</div>
    </div>
</template>
