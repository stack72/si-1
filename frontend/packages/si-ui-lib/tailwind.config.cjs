const colors = require("tailwindcss/colors");
const siStyleDict = require('@si/style-dict');

// from fletchers "dark" theme file
const theme = {
  colors: {
    ...siStyleDict.colors,
    
    // TODO(fnichol): Pre-existing colors from prior iterations. Should we
    // consider removing these to limit our color palette choices, at least as
    // far as outdated customized colors goes? Note that as of this note, we have
    // an older interface iteration co-existing with the current iteration
    // interface so this is here in an attempt to preserve backwards
    // compatibility. Also note that I had to replace the `success` and `warning`
    // keys as they collided with new color names.
    //
    primary: "#151B1E",
    secondary: "#ECEFF1",
    accent: "#607D8B",
    error: "#FF5252",
    info: "#2196F3",
    // success: "#4CAF50",
    // warning: "#FB8C00",
    black: "#000000",
    selectordark: "#1B1B1B",
    selector1: "#343B3F",
  },
  
  height: {
    sm: "16px",
    md: "16px",
    lg: "24px",
    xl: "48px",
  },
  
  screens: {
    sm: "640px",
    md: "768px",
    lg: "1024px",
    xl: "1280px",
  },
  
  spacing: {
    13: "3.25rem",
    14: "3.5rem",
    52: "13rem",
    54: "13.5rem",
    56: "14rem",
    60: "15rem",
    64: "16rem",
    72: "18rem",
    80: "20rem",
    96: "24rem",
  },
  
  margin: {
    "-05": "-0.05rem",
  },
  
  maxHeight: {
    0: "0",
    "1/4": "25%",
    "1/2": "50%",
    "3/4": "75%",
  },
  
  width: {
    "1/7": "14.285714285714286%",
    "2/7": "28.571428571428571%",
  },
  
  zIndex: {
    60: 60,
    70: 70,
    80: 80,
    90: 90,
    100: 100,
  },
}

module.exports = {
  darkMode: "class",
  content: [
    // these will be read as relative to the project root, so will not 
    "./src/**/*.html",
    "./src/**/*.vue"
  ],
  theme: {
    cursor: {
      resize: "ew-resize",
      pointer: "pointer",
      move: "move",
    },
    fontFamily: {
      sans: ["Inter", "Sans-serif"],
    },
    colors: {
      transparent: "transparent",
      current: "currentColor",
      black: colors.black,
      white: colors.white,
      gray: colors.neutral,
      blue: colors.blue,
      blueGray: colors.slate,
      indigo: colors.indigo,
      red: colors.rose,
      yellow: colors.amber,
      green: colors.green,
    },
    extend: {
      spacing: theme.spacing,
      colors: theme.colors,
      margin: theme.margin,
      maxHeight: theme.maxHeight,
      zIndex: theme.zIndex,
      width: theme.width,
    },
  },
  variants: {
    borderColor: ["group-hover"],
    textColor: ["group-hover"],
    backgroundColor: ["odd", "even"],
    extend: {
      opacity: ["disabled"],
    },
  },
  plugins: [
    // provides some css resets for form elements to make them easier to style with tailwind
    // see https://github.com/tailwindlabs/tailwindcss-forms
    require("@tailwindcss/forms")
  ],
};
