import { createRouter, createWebHistory } from "vue-router";
import MainLayout from "../components/MainLayout.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: MainLayout,
      children: [
        {
          path: "",
          name: "home",
          component: () => import("../views/HomeView.vue"),
        },
        {
          path: "/cxxm",
          name: "cxxm",
          component: () => import("../views/CxxmView.vue"),
        },
        {
          path: "/fbfx",
          name: "fbfx",
          component: () => import("../views/FbfxView.vue"),
        },
        {
          path: "/project-info",
          name: "project-info",
          component: () => import("../views/ProjectInfoView.vue"),
        },
      ],
    },
  ],
});

export default router;
