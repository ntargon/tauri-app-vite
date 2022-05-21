<script setup lang="ts">
import init, { greet, start } from 'wasm'
import { onMounted } from 'vue';


const hoge =  async () => {
    greet();
    start();
};

onMounted(async () => {
    await init();
})


</script>

<template>
<h1>Canvas</h1>
<button type="button" @click="hoge">press</button>
<canvas id="canvas" width="400" height="400"></canvas>
</template>

<style scoped>
</style>

