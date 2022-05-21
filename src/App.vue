<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
import DragAndDropUploader from './components/DragAndDropUploader.vue';
</script>

<template>
  <HelloWorld msg="Hello Vue 3 + TypeScript + Vite" />
  <DragAndDropUploader />
  <router-view />
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
