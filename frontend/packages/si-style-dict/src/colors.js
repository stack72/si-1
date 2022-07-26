
const colors = {
  black: { value: '#000000' },
  white: { value: '#FFFFFF' },

  // Neutral (gray)
  // Probably the most frequently used colors in the palette. These colors are used for backgrounds, text colors, strokes, separators, dialogs, menus, modals etc
  neutral: {
    50:  { value: "#FAFAFA" },
    100: { value: "#F5F5F5" },
    200: { value: "#E5E5E5" },
    300: { value: "#D4D4D4" },
    400: { value: "#A3A3A3" },
    500: { value: "#737373" },
    600: { value: "#525252" },
    700: { value: "#404040" },
    800: { value: "#333333" },
    900: { value: "#262626" },
  },

  // Action (blue)
  // The action is used across all the interactive elements in the product such as buttons, links, inputs, active states, highlights etc.
  action: {
    50:  { value: "#EFF6FE" },
    100: { value: "#E2F3FE" },
    200: { value: "#B2E0FF" },
    300: { value: "#B2E0FF" },
    400: { value: "#0E9BFF" },
    500: { value: "#2F80ED" },
    600: { value: "#1975DC" },
    700: { value: "#3B65A8" },
    800: { value: "#395080" },
    900: { value: "#424F6B" },
  },

  // Success (green)
  // These colors tend to convey positive emotions
  // Generally used across success and completed states
  success: {
    50:  { value: "#F0FDF4" },
    100: { value: "#DCFCE7" },
    200: { value: "#BBF7D0" },
    300: { value: "#86EFAC" },
    400: { value: "#4ADE80" },
    500: { value: "#22C55E" },
    600: { value: "#16A34A" },
    700: { value: "#15803D" },
    800: { value: "#166534" },
    900: { value: "#14532D" },
  },
  
  // Warning (yellow)
  // Colors that conventionally intended to convey the feeling of caution. 
  // Generally used across warning states
  warning: {
    50:  { value: "#FFFBEB" },
    100: { value: "#FEF3C7" },
    200: { value: "#FDE68A" },
    300: { value: "#FCD34D" },
    400: { value: "#FBBF24" },
    500: { value: "#F59E0B" },
    600: { value: "#D97706" },
    700: { value: "#B45309" },
    800: { value: "#92400E" },
    900: { value: "#78350F" },
  },

  // Destructive (red)
  // Colors that conventionally intended to convey feelings of urgency or even negativity.
  // Generally used across error states and for destructive actions
  destructive: {
    50:  { value: "#FEF2F2" },
    100: { value: "#FEE2E2" },
    200: { value: "#FECACA" },
    300: { value: "#FCA5A5" },
    400: { value: "#F87171" },
    500: { value: "#EF4444" },
    600: { value: "#DC2626" },
    700: { value: "#B91C1C" },
    800: { value: "#991B1B" },
    900: { value: "#7F1D1D" },
  }
}

// add some named aliases to hopefully discourage people from using too many shades most of the time
for (groupName in colors) {
  // skip black + white
  console.log(typeof colors[groupName])
  if (colors[groupName].value) continue;

  colors[groupName].lightest = { value: `{colors.${groupName}.50.value}` };
  colors[groupName].light = { value: `{colors.${groupName}.300.value}` };
  colors[groupName].base = { value: `{colors.${groupName}.500.value}` };
  colors[groupName].dark = { value: `{colors.${groupName}.700.value}` };
  colors[groupName].darkest = { value: `{colors.${groupName}.900.value}` };
}

module.exports = { colors };
