<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api'
import { emit, listen } from '@tauri-apps/api/event'

defineProps<{ msg: string }>()

const count = ref(0)
const hoge = ref(1)

const double_hoge = () => hoge.value *= 2;
const start_server = () => {
  invoke('start_server')
    .then((res)=> {
      console.log(res);
    })
    .catch((err)=> {
      console.log(err);
    });
};

// listen to the `click` event and get a function to remove the event listener
// there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
onMounted(() => {
  listen('back-to-front', event => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log(event.payload);
    count.value++;
  })
});
</script>

<template>
  <h1>{{ msg }}</h1>

  <button type="button" @click="count++">count is: {{ count }}</button>
  <button type="button" @click="double_hoge">hoge is: {{ hoge }}</button>
  <button type="button" @click="start_server">start server</button>
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
