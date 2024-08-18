import { defineConfig } from "vocs";
import sidebar from "./sidebar";

export default defineConfig({
  title: "Soroban by Example",
  description: "Learn soroban by exploring code samples",
  titleTemplate: "%s - soroban-by-example",
  /**@see https://vocs.dev/docs/api/config#element */
  // head: (
  //    <>
  //      <meta property="og:type" content="website" />
  //      <meta property="og:title" content="viem · TypeScript Interface for Ethereum" />
  //      <meta property="og:image" content="https://viem.sh/og-image.png" />
  //      <meta property="og:url" content="https://viem.sh" />
  //      <meta property="og:description" content="Build reliable Ethereum apps & libraries with lightweight, composable, & type-safe modules from viem." />
  //    </>
  //  ),
  sidebar,
  rootDir: ".",
  iconUrl: { light: "/sbe-icon-light.svg", dark: "/sbe-icon-dark.svg" },
  logoUrl: { light: "/sbe-icon-light.svg", dark: "/sbe-icon-dark.svg" },
  socials: [
    {
      icon: "github",
      link: "https://github.com/kohasummons/soroban-by-example",
    },
    {
      icon: "discord",
      link: "https://discord.gg/kohasummons",
    },
    {
      icon: "x",
      link: "https://x.com/stellar",
    },
  ],
  theme: {
    accentColor: {
      light: "#ff9318",
      dark: "#ffc517",
    },
  },
  topNav: [
    {
      text: "Playground",
      items: [
        {
          text: "okashi",
          link: "/introduction",
        },
      ],
    },
    {
      text: "Code Snippet",
      link: "https://github.com/kohasummons",
    },
    {
      text: "Soroban",
      items: [
        {
          text: `Learn Soroban`,
          link: `/introduction`,
        },
        {
          text: "Changelog",
          link: "https://github.com/kohasummons/soroban-by-example",
        },
        {
          text: "Contributing",
          link: "https://github.com/kohasummons/soroban-by-example/blob/main/.github/CONTRIBUTING.md",
        },
      ],
    },
  ],
  search: {
    boostDocument(documentId) {
      if (documentId.startsWith("pages")) return 2;
      return 1;
    },
  },
  editLink: {
    pattern:
      "https://github.com/kohasummons/soroban-by-example/edit/main/pages/:path",
    text: "Suggest changes to this page",
  },
});
