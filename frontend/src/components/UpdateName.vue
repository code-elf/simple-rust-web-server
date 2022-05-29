<script setup lang="ts">
import { ref } from 'vue';
import * as http from '../http';

const name = ref("")
const newName = ref("")
async function loadName() {
  const response = await http.get<{ name: string }>("http://localhost:8080/name");
  name.value = response.name;
  newName.value = name.value;
}
async function updateName() {
  await http.put(`http://localhost:8080/name`, {name: newName.value});
  name.value = newName.value;
}
loadName();
</script>

<template>
  <h1>Hello {{name}}</h1>
  Enter new name:
  <input v-model="newName"/>
  <button @click="updateName">Update</button>
</template>
