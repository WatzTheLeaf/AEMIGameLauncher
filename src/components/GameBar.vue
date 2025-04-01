<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
import Game from "./Game.vue";

const props = defineProps({
    games: Array,
    activeGameId: Number
});

const nullGame = { id: 0, name: "", image: "/src/assets/blank.jpg", description: "", authors: [""]};

const emit = defineEmits(["update:activeGameId"]);

const activeGameIndex = computed(() => props.games.findIndex(game => game.id === props.activeGameId));
const activeGame = computed(() => props.games[activeGameIndex.value] || null);

const canNavigateUp = computed(() => activeGameIndex.value > 0);
const canNavigateDown = computed(() => activeGameIndex.value < props.games.length - 1);

const previousGame = computed(() => canNavigateUp.value ? props.games[activeGameIndex.value - 1] : null);
const nextGame = computed(() => canNavigateDown.value ? props.games[activeGameIndex.value + 1] : null);

const navigateGames = (direction) => {
    if (direction === "up" && previousGame.value) {
        emit("update:activeGameId", previousGame.value.id);
    } else if (direction === "down" && nextGame.value) {
        emit("update:activeGameId", nextGame.value.id);
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
    <div class="h-full p-4 w-2/5 text-center overflow-y-hidden flex flex-col justify-center items-center">

        <Game v-if="previousGame" :game="previousGame" class="w-1/3 opacity-50"/>
        <Game v-else="previousGame" :game="nullGame" class="w-1/3 opacity-0"/>

        <div v-if="canNavigateUp" class="text-2xl cursor-pointer mb-2 text-slate-50" @click="navigateGames('up')">▲</div>
        <div v-else class="text-2xl cursor-pointer mb-2 text-slate-50">‎</div>

        <Game v-if="activeGame" :game="activeGame" />

        <div v-if="canNavigateDown" class="text-2xl cursor-pointer mt-2 text-slate-50" @click="navigateGames('down')">▼</div>
        <div v-else class="text-2xl cursor-pointer mt-2 text-slate-50">‎</div>

        <Game v-if="nextGame" :game="nextGame" class="w-1/3 opacity-50"/>
        <Game v-else="nextGame" :game="nullGame" class="w-1/3 opacity-0"/>

    </div>
</template>