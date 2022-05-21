<script setup lang="ts">
import { ref } from 'vue'

const img_src = ref("")

const connection = new WebSocket("ws://127.0.0.1:54321")
connection.onmessage = (e) => {
  img_src.value = e.data;
}

</script>

<template>
  <h1>video</h1>
  <img v-bind:src="'data:image/jpeg;base64,'+img_src" width="800" height="800"/>
</template>

<style scoped>
</style>
