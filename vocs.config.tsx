import * as React from "react"; 
import { defineConfig } from "vocs";
import sidebar from "./sidebar";

export default defineConfig({
  title: "Soroban by Example",
  description: "Learn soroban by exploring code samples",
  titleTemplate: "%s - soroban-by-example",
  /**@see https://vocs.dev/docs/api/config#element */
  head: ( 
    <>
      <meta property="og:type" content="website" />
      <meta property="og:title" content="Soroban by Examples · Learn soroban and build on Stellar by exploring code samples" />
      <meta property="og:image" content="https://sorobanexamples.xyz/soroban-og.png" />
      <meta property="og:url" content="https://sorobanexamples.xyz" />
      <meta property="og:description" content="A carefully curated, lightweight, composable, and tweakable Soroban starter pack to get you writing for the Stellar blockchain faster than you can say Jack." />
    </> 
  ),
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
        {
          text: "Soroboy ✨",
          link: "https://chatgpt.com/g/g-oW0Pjt0tu-soroboy",
        }
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
