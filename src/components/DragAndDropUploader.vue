<script setup lang="ts">
import { ref } from "vue"
import { listen } from '@tauri-apps/api/event'
import { WebviewWindow } from "@tauri-apps/api/window"

const isEnter = ref(false);

const _dropFile = () => {
  console.log("drop");
  isEnter.value = false;
}

listen('tauri://file-drop', event => {
  if (isEnter.value) {
    _dropFile();
    console.log(event.payload);
    new WebviewWindow('label', {
      url: 'https://github.com/tauri-apps/tauri'
    })
  }
})

const dragEnter = () => {
  isEnter.value = true;
}
const dragLeave = () => {
  isEnter.value = false;
}

</script>

<template>
  <div>
    <div class="drop_area" @dragenter="dragEnter" @dragleave="dragLeave" :class="{enter :isEnter}">Drop Here</div>
  </div>
</template>

<style scoped>
  .drop_area {
    color: gray;
    font-weight: bold;
    font-size: 1.2em;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 500px;
    height: 300px;
    border: 5px solid gray;
    border-radius: 15px;
    margin: 0 auto;
  }
  .enter {
    border: 10px dotted powderblue;
  }
</style>

