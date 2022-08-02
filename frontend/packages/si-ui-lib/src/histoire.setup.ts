import { defineSetupVue3 } from "histoire/client";

// import our core styles, which includes tailwind classes
import "./style/index.css";

// hooks into vue app setup
// eslint-disable-next-line @typescript-eslint/no-unused-vars
export const setupVue3 = defineSetupVue3(({ app, story, variant }) => {
  console.log("setup vue 3 app for histoire");

  // can do things like set up stores, mock router, etc...
  // const pinia = createPinia()
  // app.use(pinia) // Add Pinia store
});
