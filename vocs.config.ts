import { defineConfig } from 'vocs'
import sidebar from './sidebar'

export default defineConfig({
  title: 'Soroban-by-example',
  description: 'Learn soroban by studing simple snipperts',
  titleTemplate: '%s - Soroban-by-example',
  sidebar,
  rootDir: '.',
  iconUrl: {light: '/sbe-icon-light.svg', dark: '/sbe-icon-dark.svg'},
  logoUrl: {light: '/sbe-icon-light.svg', dark: '/sbe-icon-dark.svg'},
  socials: [
    {
      icon: 'github',
      link: 'https://github.com/kohasummons/soroban-by-example',
    },
    {
      icon: 'discord',
      link: 'https://discord.gg/kohasummons',
    },
    {
      icon: 'x',
      link: 'https://x.com/stellar',
    },
  ],
  theme: {
    accentColor: {
      light: '#ff9318',
      dark: '#ffc517',
    },
  },
  topNav: [
    {
      text: 'Playground',
      items: [
        {
          text: 'okashi',
          link: '/introduction',
        },
      ],
    },
    {
      text: 'Code Snippet',
      link: 'https://github.com/kohasummons',
    },
    {
      text: "Soroban",
      items: [
        {
          text: `Learn Soroban`,
          link: `/introduction`,
        },
        {
          text: 'Changelog',
          link: 'https://github.com/kohasummons/soroban-by-example',
        },
        {
          text: 'Contributing',
          link: 'https://github.com/kohasummons/soroban-by-example/blob/main/.github/CONTRIBUTING.md',
        },
      ],
    },
  ],
  search: {
    boostDocument(documentId) {
      if (documentId.startsWith('pages/docs')) return 2
      return 1
    },
  },
  editLink: {
    pattern: 'https://github.com/kohasummons/soroban-by-example/edit/main/pages/:path',
    text: 'Suggest changes to this page',
  },
  
})
