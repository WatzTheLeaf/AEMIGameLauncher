<script setup>

defineProps({
    game: Object,
});

</script>

<template>
    <div class="relative mt-10 mb-10 overflow-hidden rounded-4xl cl-custom-shadow">
        <h1 class="absolute z-30 bottom-0 left-0 bg-slate-950/50 w-full text-slate-50 text-2xl text-start p-5 font-mono rounded-b-4xl">
            {{ game.name }}
        </h1>
        <img :src="game.image" :alt="game.name" class="w-full object-cover rounded-4xl" />
    </div>
</template>

<style>
    .cl-custom-shadow{
        -webkit-box-shadow: 0px 0px 10px 0 var(--color-slate-50);
        -moz-box-shadow: 0px 0px 10px 0 var(--color-slate-50);
        box-shadow: 0px 0px 10px 0 var(--color-slate-50);
    }
</style>