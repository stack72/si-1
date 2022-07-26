import { defineConfig, defaultColors } from 'histoire'


export default defineConfig({
  setupFile: '/src/histoire.setup.ts',
  theme: {
    title: 'SystemInit Dessign System',
    // favicon: '/my-favicon.svg',
    // logo: {
    //   square: '/src/img/logo-square.svg',
    //   light: '/src/img/logo-light.svg',
    //   dark: '/src/img/logo-dark.svg',
    // },
    colors: {
      primary: defaultColors.cyan,
    },
    logoHref: 'https://systeminit.com',
  },
  plugins: [],
  responsivePresets: [
    {
      label: 'Mobile (Small)',
      width: 320,
      height: 560,
    },
    {
      label: 'Mobile (Medium)',
      width: 360,
      height: 640,
    },
    {
      label: 'Mobile (Large)',
      width: 414,
      height: 896,
    },
    {
      label: 'Tablet',
      width: 768,
      height: 1024,
    },
    {
      label: 'Laptop (Small)',
      width: 1024,
      height: null,
    },
    {
      label: 'Laptop (Large)',
      width: 1366,
      height: null,
    },
    {
      label: 'Desktop',
      width: 1920,
      height: null,
    },
    {
      label: '4K',
      width: 3840,
      height: null,
    },
  ],
  tree: {
    groups: [
      { id: 'core', title: "Design system core", include: (file) => file.path.includes('/docs/') },
      { id: 'components', title: "Components", include: (file) => file.path.includes('/components/') },
    ]
  }
})