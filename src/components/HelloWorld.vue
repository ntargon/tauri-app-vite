<script setup lang="ts">
import { ref } from 'vue'

defineProps<{ msg: string }>()

const count = ref(0)
const message = ref(0)
const img_src = ref("")

const connection = new WebSocket("ws://127.0.0.1:54321")
connection.onmessage = (e) => {
  console.log(e);
  const tmp = e.data as string;
  message.value = tmp.length;
  img_src.value = tmp;
}

</script>

<template>
  <h1>{{ msg }}</h1>

  <button type="button" @click="count++">count is: {{ count }}</button>
  <h1>video</h1>
  <img v-bind:src="'data:image/jpeg;base64,'+img_src" width="800" height="800"/>
</template>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
