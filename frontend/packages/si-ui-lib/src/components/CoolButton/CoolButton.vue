<template>
  <button class="si-button" :class="computedClasses" :disabled="disabled">
    <slot />
  </button>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import type { PropType } from 'vue';

export type ButtonSizes = 'xs' | 's' | 'm' | 'l' | 'xl';

const props = defineProps({
  disabled: { type: Boolean },
  size: { type: String as PropType<ButtonSizes>, default: 'm' },
});

const computedClasses = computed(() => {
  return {
    '--disabled': props.disabled,
    [`--size-${props.size}`]: true,
  }
})

</script>

<style lang="less">
.si-button {
  cursor: pointer;
  color: black;
  border-radius: 5px;
  padding: 5px 10px;
  border: 2px solid #333;

  @apply bg-action-500;

  &:hover {
    background: #DADADA;
  }

  &.--disabled {
    opacity: .5;
    pointer-events: none;
  }

  &.--size-xs {
    font-size: 11px;
  }
  &.--size-s {
    font-size: 14px;
  }
  &.--size-m { // default
    font-size: 16px;
  }
  &.--size-l {
    font-size: 20px;
  }
  &.--size-l {
    font-size: 24px;
  }
  &.--size-xl {
    font-size: 32px;
  }

}
</style>