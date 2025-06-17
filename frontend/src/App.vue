<script setup lang="ts">
import MainLayout from "./components/MainLayout.vue";
</script>

<template>
  <MainLayout />
</template>

<style>
/* Resetting default margin and padding */
body,
html {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

#app {
  padding: 0;
  margin: 0;
  height: 100vh;
  width: 100vw;
}
</style>
