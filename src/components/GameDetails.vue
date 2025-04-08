<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps({
  game: Object
});

const parsedAuthors = computed(() => {
  if (!props.game || !props.game.authors) return "Auteurs inconnus";
  return "Par : " + props.game.authors.join(", ");
});

const tags = ref([]);
async function retrieveTags() {
  try {
    const tagsData = await invoke("retrieve_tags");
    const parsedTags = JSON.parse(tagsData);
    tags.value = parsedTags.slice(0, 5).map(tag => ({
      name: tag.name,
      color: tag.color || "bg-slate-700"
    }));
  } catch (error) {
    console.error("Error while retrieving tags :", error);
    tags.value = [];
  }
}

onMounted(() => {
  retrieveTags();
});

</script>

<template>
  <div class="h-2/3 p-4 m-8 flex w-1/2 bg-slate-950/80 rounded-4xl text-slate-200 flex-col justify-center">
    <h1 class="text-5xl m-5">{{ game.name }}</h1>
    <h2 class="text-3xl m-5">{{ parsedAuthors }}</h2>
    <div class="flex m-5 max-h-55/100 relative items-center flex-wp justify-evenly">
      <p class="text-lg flex-1 text-justify overflow-hidden max-h-full text-overflow-wrap the-p">{{ game.description }}</p>
    </div>
    <div class="flex flex-wrap overflow-hidden gap-2 px-4">
        <!-- ADD TAGS HERE (5 max)-->
    </div>
  </div>
</template>

<style>
.the-p {
  word-wrap: break-word;
  white-space: -moz-pre-wrap;
  white-space: pre-wrap;
}
</style>
