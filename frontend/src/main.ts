import { createApp } from 'vue'
import App from './App.vue'
import UpdateName from "./components/UpdateName.vue";
import * as VueRouter from "vue-router";

const router = VueRouter.createRouter({
	history: VueRouter.createWebHistory(),
	routes: [
		{ path: '/', component: UpdateName },
	]
})

const app = createApp(App);
app.use(router);
app.mount('#app')
