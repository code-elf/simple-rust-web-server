<script setup lang="ts">
import {ref} from 'vue';
import * as http from "./http";

const authToken = ref("");
function setAuthToken(value: string) {
  authToken.value = value;
  localStorage.setItem("authToken", value);
  http.globals.headers = { Authorization: value }
}
const loadedToken = localStorage.getItem("authToken");
if(loadedToken) setAuthToken(loadedToken);
const username = ref("");
const password = ref("");
async function login() {
  const response = await http.post<{ token: string }>("http://localhost:8080/login", { username: username.value, password: password.value });
  setAuthToken(response.token);
}
async function register() {
  const response = await http.post<{ token: string }>("http://localhost:8080/users", { username: username.value, password: password.value });
  setAuthToken(response.token);
}
async function logout() {
  setAuthToken("");
}

</script>

<template>
  <template v-if="authToken">
    <h1>Hello App!</h1>
    <a href="#" @click.prevent="logout">Logout</a>
    <p>
      <router-link to="/">Go to Home</router-link>
      <router-link to="/about">Go to About</router-link>
    </p>
    <router-view></router-view>
  </template>
  <template v-else>
    Name: <input v-model="username"/><br>
    Password: <input type="password" v-model="password"/><br>
    <button @click="login">Login</button>
    <button @click="register">Register</button>
  </template>
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
