<script setup lang="ts"></script>

<template>
  <router-view />
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
