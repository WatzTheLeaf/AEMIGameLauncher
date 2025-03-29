<script setup>
import { ref, computed, onMounted, onUnmounted } from "vue";
import Game from "./Game.vue";

const props = defineProps({
    games: Array,
    activeGameId: Number
});

const emit = defineEmits(["update:activeGameId"]);

const activeGame = computed(() => props.games.find(game => game.id === props.activeGameId));

const canNavigateUp = computed(() => props.games.findIndex(game => game.id === props.activeGameId) > 0);
const canNavigateDown = computed(() => props.games.findIndex(game => game.id === props.activeGameId) < props.games.length - 1);

const navigateGames = (direction) => {
    const currentIndex = props.games.findIndex(game => game.id === props.activeGameId);
    if (direction === "up" && canNavigateUp.value) {
        emit("update:activeGameId", props.games[currentIndex - 1].id);
    } else if (direction === "down" && canNavigateDown.value) {
        emit("update:activeGameId", props.games[currentIndex + 1].id);
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