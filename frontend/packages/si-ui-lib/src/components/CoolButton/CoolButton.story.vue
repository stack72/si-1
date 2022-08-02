<template>
  <Story title="Atoms/CoolButton" :layout="{ type: 'grid' }">
    <Variant title="Default">
      <CoolButton @click="hstEvent('Click', $event)">Click me</CoolButton>
    </Variant>

    <Variant title="Disabled" auto-props-disabled>
      <CoolButton disabled @click="hstEvent('Click', $event)"
        >Click me</CoolButton
      >
    </Variant>

    <Variant title="Sizes" auto-props-disabled>
      <CoolButton
        v-for="size in BUTTON_SIZES"
        :key="size"
        :size="size"
        @click="hstEvent('Click', $event)"
        >size = "{{ size }}"
      </CoolButton>
    </Variant>
  </Story>
</template>

<script lang="ts" setup>
import { hstEvent } from "histoire/client";
import CoolButton, { ButtonSizes } from "./CoolButton.vue";

const BUTTON_SIZES: ButtonSizes[] = ["xs", "s", "m", "l", "xl"];
</script>

<docs lang="md">
# Cool Button!

Checkout this [cool video](https://www.youtube.com/watch?v=dQw4w9WgXcQ)!
</docs>
