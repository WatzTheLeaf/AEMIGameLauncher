<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

defineProps({
    tag: Object,
});

</script>

<template>
    <div class="inline-block p-5 rounded-lg max-w-max m-2" :class="tag.color">
        <h1 class="text-2xl text-start font-mono">
            {{ tag.name }}
        </h1>
    </div>
</template>

<style>
    .cl-custom-shadow{
        -webkit-box-shadow: 0px 0px 10px 0 var(--color-slate-50);
        -moz-box-shadow: 0px 0px 10px 0 var(--color-slate-50);
        box-shadow: 0px 0px 10px 0 var(--color-slate-50);
    }
</style>