<template>
<Story
  title="2 - Color Palette"
  :layout="{ type: 'single' }"
  icon="carbon:color-palette"
  responsive-disabled
>

  <div class="color-group">
    <h2>shades</h2>
    <div class="color-group__swatches">
      <template v-for="color in baseColors">
        <div class="color" :style="{backgroundColor: color.value }">{{color.name}}</div>
      </template>
    </div>
  </div>

  <template v-for="colorGroup, colorGroupName in colorGroups">
    <div class="color-group">
      <h2>{{ colorGroupName }}</h2>
      <div class="color-group__swatches">
        <template v-for="color in colorGroup">
          <div class="color" :style="{backgroundColor: color.value }">{{ colorGroupName }} - {{color.name}}</div>
        </template>
      </div>
    </div>
  </template>
</Story>
</template>

<script lang="ts" setup>
// @ts-ignore
import siStyleDict from '@si/style-dict/verbose.json';
import * as _ from 'lodash-es';

const colorTokens = siStyleDict.colors;
const baseColors = _.pick(colorTokens, 'black', 'white');
const colorGroups = _.omit(colorTokens, 'black', 'white');

</script>

<style scoped>
.color-group {
  
}

.color-group__swatches {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.color {
  width: 100px;
  height: 100px;
  background: red;
}
</style>

<docs lang="md">
# Colors

we like colors
</docs>

