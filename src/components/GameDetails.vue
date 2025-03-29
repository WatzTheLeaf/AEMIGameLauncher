<script setup>
import { ref, computed } from "vue";
import Tag from "./Tag.vue";

const props = defineProps({
    game: Object
});

const parsedAuthors = computed(() => {
    if (!props.game || !props.game.authors) return "Auteurs inconnus";
    return "Par : " + props.game.authors.join(", ");
});

const tags = ref([
{ id: 1, name: "Horreur", color: "bg-red-800"},
{ id: 2, name: "Jeu de role", color: "bg-amber-600"},
{ id: 3, name: "WonderJam", color: "bg-emerald-900"},
{ id: 4, name: "Primé", color: "bg-yellow-500"},
{ id: 5, name: "Multijoueur", color: "bg-indigo-500"},
]);

</script>

<template>
    <div class="h-2/3 p-4 m-8 flex w-1/2 bg-slate-950/80 rounded-4xl text-slate-200 flex-col justify-center">
        <h1 class="text-5xl m-5">{{game.name}}</h1>
        <h2 class="text-3xl m-5">{{parsedAuthors}}</h2>
        <div class="flex m-5 max-h-55/100 relative items-center flex-wp justify-evenly">
            <p class="text-lg flex-1 text-justify overflow-hidden max-h-full text-overflow-wrap the-p">{{ game.description }}</p>
        </div>
        <div class="flex flex-wrap overflow-hidden">
            <!-- 5 TAGS MAXIMUM -->
            <Tag :tag="tags[0]"/>
            <Tag :tag="tags[1]"/>
            <Tag :tag="tags[2]"/>
            <Tag :tag="tags[3]"/>
            <Tag :tag="tags[4]"/>
        </div>
    </div>
</template>

<style>
.the-p{
    word-wrap: break-word;
    white-space: -moz-pre-wrap;
    white-space: pre-wrap;
}
</style>