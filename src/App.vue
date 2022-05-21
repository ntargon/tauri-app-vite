<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import HelloWorld from './components/HelloWorld.vue'
import Canvas from './components/Canvas.vue'
import Monitor from './components/Monitor.vue';
import DragAndDropUploader from './components/DragAndDropUploader.vue';
</script>

<template>
  <HelloWorld msg="Hello Vue 3 + TypeScript + Vite" />
  <DragAndDropUploader />
  <Monitor />
  <Canvas />
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
