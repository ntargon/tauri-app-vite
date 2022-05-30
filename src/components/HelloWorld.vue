<script setup lang="ts">
import { ref, onMounted } from 'vue'

defineProps<{ msg: string }>()

type Point = {
  x: number, y: number
};

const count = ref(0)
const points = ref<Array<Point>>([])

const countUp = () => {
  count.value++;
  points.value.push(
    {
      x: count.value * count.value, y: count.value
    }
  );
}

const saveCount = () => {
  localStorage.setItem("count", count.value.toString());
}

onMounted(() => {
  const countItem = localStorage.getItem("count");
  if (countItem) {
    count.value = JSON.parse(countItem);
  }
});

</script>

<template>
  <h1>{{ msg }}</h1>

  <button type="button" @click="countUp">count is: {{ count }}</button>
  <button type="button" @click="saveCount">save</button>
  <p v-for="(point, index) in points">{{ point }} {{ index }}</p>
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
