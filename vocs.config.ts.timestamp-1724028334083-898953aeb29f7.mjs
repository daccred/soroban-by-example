// vocs.config.ts
import { defineConfig } from "file:///Users/globaltechgirl/Desktop/web3/soroban-by-example/node_modules/.pnpm/vocs@1.0.0-alpha.55_@types+node@22.4.0_@types+react@18.3.3_react-dom@18.3.1_react@18.3.1__rea_lim6gf3mg2cxbuo2w2izsi3ql4/node_modules/vocs/_lib/index.js";

// sidebar.ts
var sidebar = [
  {
    text: "Introduction",
    items: [
      { text: "Soroban", link: "/introduction" },
      { text: "Installation", link: "/installation" },
      { text: "Getting Started", link: "/getting-started" },
      { text: "Hello World", link: "/hello-world" },
      { text: "Counter App", link: "/counter" }
    ]
  },
  {
    text: "Basic",
    items: [
      { text: "Integers", link: "/integers" },
      { text: "Boolean", link: "/boolean" },
      { text: "Strings", link: "/strings" },
      { text: "Symbol", link: "/symbol" },
      { text: "Bytes", link: "/bytes" },
      { text: "Vec", link: "/vec" },
      { text: "Map", link: "/map" },
      { text: "Address", link: "/address" },
      { text: "Structs", link: "/structs" }
    ]
  },
  {
    text: "Interface",
    items: [
      { text: "Token", link: "/token-interface" },
      { text: "Admin", link: "/admin-interface" }
    ]
  }
];
var sidebar_default = sidebar;

// vocs.config.ts
var vocs_config_default = defineConfig({
  title: "Soroban-by-example",
  description: "Learn soroban by studing simple snipperts",
  titleTemplate: "%s - Soroban-by-example",
  sidebar: sidebar_default,
  rootDir: ".",
  iconUrl: { light: "/sbe-icon-light.svg", dark: "/sbe-icon-dark.svg" },
  logoUrl: { light: "/sbe-icon-light.svg", dark: "/sbe-icon-dark.svg" },
  socials: [
    {
      icon: "github",
      link: "https://github.com/kohasummons/soroban-by-example"
    },
    {
      icon: "discord",
      link: "https://discord.gg/kohasummons"
    },
    {
      icon: "x",
      link: "https://x.com/stellar"
    }
  ],
  theme: {
    accentColor: {
      light: "#ff9318",
      dark: "#ffc517"
    }
  },
  topNav: [
    {
      text: "Playground",
      items: [
        {
          text: "okashi",
          link: "/introduction"
        }
      ]
    },
    {
      text: "Code Snippet",
      link: "https://github.com/kohasummons"
    },
    {
      text: "Soroban",
      items: [
        {
          text: `Learn Soroban`,
          link: `/introduction`
        },
        {
          text: "Changelog",
          link: "https://github.com/kohasummons/soroban-by-example"
        },
        {
          text: "Contributing",
          link: "https://github.com/kohasummons/soroban-by-example/blob/main/.github/CONTRIBUTING.md"
        }
      ]
    }
  ],
  search: {
    boostDocument(documentId) {
      if (documentId.startsWith("pages/docs")) return 2;
      return 1;
    }
  },
  editLink: {
    pattern: "https://github.com/kohasummons/soroban-by-example/edit/main/pages/:path",
    text: "Suggest changes to this page"
  }
});
export {
  vocs_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidm9jcy5jb25maWcudHMiLCAic2lkZWJhci50cyJdLAogICJzb3VyY2VzQ29udGVudCI6IFsiY29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2Rpcm5hbWUgPSBcIi9Vc2Vycy9nbG9iYWx0ZWNoZ2lybC9EZXNrdG9wL3dlYjMvc29yb2Jhbi1ieS1leGFtcGxlXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCIvVXNlcnMvZ2xvYmFsdGVjaGdpcmwvRGVza3RvcC93ZWIzL3Nvcm9iYW4tYnktZXhhbXBsZS92b2NzLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vVXNlcnMvZ2xvYmFsdGVjaGdpcmwvRGVza3RvcC93ZWIzL3Nvcm9iYW4tYnktZXhhbXBsZS92b2NzLmNvbmZpZy50c1wiO2ltcG9ydCB7IGRlZmluZUNvbmZpZyB9IGZyb20gJ3ZvY3MnXG5pbXBvcnQgc2lkZWJhciBmcm9tICcuL3NpZGViYXInXG5cbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyh7XG4gIHRpdGxlOiAnU29yb2Jhbi1ieS1leGFtcGxlJyxcbiAgZGVzY3JpcHRpb246ICdMZWFybiBzb3JvYmFuIGJ5IHN0dWRpbmcgc2ltcGxlIHNuaXBwZXJ0cycsXG4gIHRpdGxlVGVtcGxhdGU6ICclcyAtIFNvcm9iYW4tYnktZXhhbXBsZScsXG4gIHNpZGViYXIsXG4gIHJvb3REaXI6ICcuJyxcbiAgaWNvblVybDoge2xpZ2h0OiAnL3NiZS1pY29uLWxpZ2h0LnN2ZycsIGRhcms6ICcvc2JlLWljb24tZGFyay5zdmcnfSxcbiAgbG9nb1VybDoge2xpZ2h0OiAnL3NiZS1pY29uLWxpZ2h0LnN2ZycsIGRhcms6ICcvc2JlLWljb24tZGFyay5zdmcnfSxcbiAgc29jaWFsczogW1xuICAgIHtcbiAgICAgIGljb246ICdnaXRodWInLFxuICAgICAgbGluazogJ2h0dHBzOi8vZ2l0aHViLmNvbS9rb2hhc3VtbW9ucy9zb3JvYmFuLWJ5LWV4YW1wbGUnLFxuICAgIH0sXG4gICAge1xuICAgICAgaWNvbjogJ2Rpc2NvcmQnLFxuICAgICAgbGluazogJ2h0dHBzOi8vZGlzY29yZC5nZy9rb2hhc3VtbW9ucycsXG4gICAgfSxcbiAgICB7XG4gICAgICBpY29uOiAneCcsXG4gICAgICBsaW5rOiAnaHR0cHM6Ly94LmNvbS9zdGVsbGFyJyxcbiAgICB9LFxuICBdLFxuICB0aGVtZToge1xuICAgIGFjY2VudENvbG9yOiB7XG4gICAgICBsaWdodDogJyNmZjkzMTgnLFxuICAgICAgZGFyazogJyNmZmM1MTcnLFxuICAgIH0sXG4gIH0sXG4gIHRvcE5hdjogW1xuICAgIHtcbiAgICAgIHRleHQ6ICdQbGF5Z3JvdW5kJyxcbiAgICAgIGl0ZW1zOiBbXG4gICAgICAgIHtcbiAgICAgICAgICB0ZXh0OiAnb2thc2hpJyxcbiAgICAgICAgICBsaW5rOiAnL2ludHJvZHVjdGlvbicsXG4gICAgICAgIH0sXG4gICAgICBdLFxuICAgIH0sXG4gICAge1xuICAgICAgdGV4dDogJ0NvZGUgU25pcHBldCcsXG4gICAgICBsaW5rOiAnaHR0cHM6Ly9naXRodWIuY29tL2tvaGFzdW1tb25zJyxcbiAgICB9LFxuICAgIHtcbiAgICAgIHRleHQ6IFwiU29yb2JhblwiLFxuICAgICAgaXRlbXM6IFtcbiAgICAgICAge1xuICAgICAgICAgIHRleHQ6IGBMZWFybiBTb3JvYmFuYCxcbiAgICAgICAgICBsaW5rOiBgL2ludHJvZHVjdGlvbmAsXG4gICAgICAgIH0sXG4gICAgICAgIHtcbiAgICAgICAgICB0ZXh0OiAnQ2hhbmdlbG9nJyxcbiAgICAgICAgICBsaW5rOiAnaHR0cHM6Ly9naXRodWIuY29tL2tvaGFzdW1tb25zL3Nvcm9iYW4tYnktZXhhbXBsZScsXG4gICAgICAgIH0sXG4gICAgICAgIHtcbiAgICAgICAgICB0ZXh0OiAnQ29udHJpYnV0aW5nJyxcbiAgICAgICAgICBsaW5rOiAnaHR0cHM6Ly9naXRodWIuY29tL2tvaGFzdW1tb25zL3Nvcm9iYW4tYnktZXhhbXBsZS9ibG9iL21haW4vLmdpdGh1Yi9DT05UUklCVVRJTkcubWQnLFxuICAgICAgICB9LFxuICAgICAgXSxcbiAgICB9LFxuICBdLFxuICBzZWFyY2g6IHtcbiAgICBib29zdERvY3VtZW50KGRvY3VtZW50SWQpIHtcbiAgICAgIGlmIChkb2N1bWVudElkLnN0YXJ0c1dpdGgoJ3BhZ2VzL2RvY3MnKSkgcmV0dXJuIDJcbiAgICAgIHJldHVybiAxXG4gICAgfSxcbiAgfSxcbiAgZWRpdExpbms6IHtcbiAgICBwYXR0ZXJuOiAnaHR0cHM6Ly9naXRodWIuY29tL2tvaGFzdW1tb25zL3Nvcm9iYW4tYnktZXhhbXBsZS9lZGl0L21haW4vcGFnZXMvOnBhdGgnLFxuICAgIHRleHQ6ICdTdWdnZXN0IGNoYW5nZXMgdG8gdGhpcyBwYWdlJyxcbiAgfSxcbiAgXG59KVxuIiwgImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvVXNlcnMvZ2xvYmFsdGVjaGdpcmwvRGVza3RvcC93ZWIzL3Nvcm9iYW4tYnktZXhhbXBsZVwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL1VzZXJzL2dsb2JhbHRlY2hnaXJsL0Rlc2t0b3Avd2ViMy9zb3JvYmFuLWJ5LWV4YW1wbGUvc2lkZWJhci50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vVXNlcnMvZ2xvYmFsdGVjaGdpcmwvRGVza3RvcC93ZWIzL3Nvcm9iYW4tYnktZXhhbXBsZS9zaWRlYmFyLnRzXCI7Y29uc3Qgc2lkZWJhciA9IFtcbiAgICB7XG4gICAgICB0ZXh0OiAnSW50cm9kdWN0aW9uJywgXG4gICAgICBpdGVtczogWyBcbiAgICAgICAgICAgIHsgdGV4dDogJ1Nvcm9iYW4nLCBsaW5rOiAnL2ludHJvZHVjdGlvbicgfSxcbiAgICAgICAgICAgIHsgdGV4dDogJ0luc3RhbGxhdGlvbicsIGxpbms6ICcvaW5zdGFsbGF0aW9uJyB9LFxuICAgICAgICAgICAgeyB0ZXh0OiAnR2V0dGluZyBTdGFydGVkJywgbGluazogJy9nZXR0aW5nLXN0YXJ0ZWQnIH0sXG4gICAgICAgICAgICB7IHRleHQ6ICdIZWxsbyBXb3JsZCcsIGxpbms6ICcvaGVsbG8td29ybGQnIH0sXG4gICAgICAgICAgICB7IHRleHQ6ICdDb3VudGVyIEFwcCcsIGxpbms6ICcvY291bnRlcicgfSxcbiAgICAgICAgXSxcbiAgICB9LFxuICAgIHtcbiAgICAgIHRleHQ6ICdCYXNpYycsXG4gICAgICBpdGVtczogWyBcbiAgICAgICAgICAgIHsgdGV4dDogJ0ludGVnZXJzJywgbGluazogJy9pbnRlZ2VycycgfSxcbiAgICAgICAgICAgIHsgdGV4dDogJ0Jvb2xlYW4nLCBsaW5rOiAnL2Jvb2xlYW4nIH0sXG4gICAgICAgICAgICB7IHRleHQ6ICdTdHJpbmdzJywgbGluazogJy9zdHJpbmdzJyB9LFxuICAgICAgICAgICAgeyB0ZXh0OiAnU3ltYm9sJywgbGluazogJy9zeW1ib2wnIH0sXG4gICAgICAgICAgICB7IHRleHQ6ICdCeXRlcycsIGxpbms6ICcvYnl0ZXMnIH0sXG4gICAgICAgICAgICB7IHRleHQ6ICdWZWMnLCBsaW5rOiAnL3ZlYycgfSxcbiAgICAgICAgICAgIHsgdGV4dDogJ01hcCcsIGxpbms6ICcvbWFwJyB9LFxuICAgICAgICAgICAgeyB0ZXh0OiAnQWRkcmVzcycsIGxpbms6ICcvYWRkcmVzcycgfSxcbiAgICAgICAgICAgIHsgdGV4dDogJ1N0cnVjdHMnLCBsaW5rOiAnL3N0cnVjdHMnIH0sXG4gICAgICAgIF0sXG4gICAgfSxcbiAgICB7XG4gICAgICB0ZXh0OiAnSW50ZXJmYWNlJyxcbiAgICAgIGl0ZW1zOiBbIFxuICAgICAgICAgICAgeyB0ZXh0OiAnVG9rZW4nLCBsaW5rOiAnL3Rva2VuLWludGVyZmFjZScgfSxcbiAgICAgICAgICAgIHsgdGV4dDogJ0FkbWluJywgbGluazogJy9hZG1pbi1pbnRlcmZhY2UnIH0sXG4gICAgICAgIF0sXG4gICAgfSxcbl1cblxuZXhwb3J0IGRlZmF1bHQgc2lkZWJhciJdLAogICJtYXBwaW5ncyI6ICI7QUFBaVYsU0FBUyxvQkFBb0I7OztBQ0FyQyxJQUFNLFVBQVU7QUFBQSxFQUNyVjtBQUFBLElBQ0UsTUFBTTtBQUFBLElBQ04sT0FBTztBQUFBLE1BQ0QsRUFBRSxNQUFNLFdBQVcsTUFBTSxnQkFBZ0I7QUFBQSxNQUN6QyxFQUFFLE1BQU0sZ0JBQWdCLE1BQU0sZ0JBQWdCO0FBQUEsTUFDOUMsRUFBRSxNQUFNLG1CQUFtQixNQUFNLG1CQUFtQjtBQUFBLE1BQ3BELEVBQUUsTUFBTSxlQUFlLE1BQU0sZUFBZTtBQUFBLE1BQzVDLEVBQUUsTUFBTSxlQUFlLE1BQU0sV0FBVztBQUFBLElBQzVDO0FBQUEsRUFDSjtBQUFBLEVBQ0E7QUFBQSxJQUNFLE1BQU07QUFBQSxJQUNOLE9BQU87QUFBQSxNQUNELEVBQUUsTUFBTSxZQUFZLE1BQU0sWUFBWTtBQUFBLE1BQ3RDLEVBQUUsTUFBTSxXQUFXLE1BQU0sV0FBVztBQUFBLE1BQ3BDLEVBQUUsTUFBTSxXQUFXLE1BQU0sV0FBVztBQUFBLE1BQ3BDLEVBQUUsTUFBTSxVQUFVLE1BQU0sVUFBVTtBQUFBLE1BQ2xDLEVBQUUsTUFBTSxTQUFTLE1BQU0sU0FBUztBQUFBLE1BQ2hDLEVBQUUsTUFBTSxPQUFPLE1BQU0sT0FBTztBQUFBLE1BQzVCLEVBQUUsTUFBTSxPQUFPLE1BQU0sT0FBTztBQUFBLE1BQzVCLEVBQUUsTUFBTSxXQUFXLE1BQU0sV0FBVztBQUFBLE1BQ3BDLEVBQUUsTUFBTSxXQUFXLE1BQU0sV0FBVztBQUFBLElBQ3hDO0FBQUEsRUFDSjtBQUFBLEVBQ0E7QUFBQSxJQUNFLE1BQU07QUFBQSxJQUNOLE9BQU87QUFBQSxNQUNELEVBQUUsTUFBTSxTQUFTLE1BQU0sbUJBQW1CO0FBQUEsTUFDMUMsRUFBRSxNQUFNLFNBQVMsTUFBTSxtQkFBbUI7QUFBQSxJQUM5QztBQUFBLEVBQ0o7QUFDSjtBQUVBLElBQU8sa0JBQVE7OztBRC9CZixJQUFPLHNCQUFRLGFBQWE7QUFBQSxFQUMxQixPQUFPO0FBQUEsRUFDUCxhQUFhO0FBQUEsRUFDYixlQUFlO0FBQUEsRUFDZjtBQUFBLEVBQ0EsU0FBUztBQUFBLEVBQ1QsU0FBUyxFQUFDLE9BQU8sdUJBQXVCLE1BQU0scUJBQW9CO0FBQUEsRUFDbEUsU0FBUyxFQUFDLE9BQU8sdUJBQXVCLE1BQU0scUJBQW9CO0FBQUEsRUFDbEUsU0FBUztBQUFBLElBQ1A7QUFBQSxNQUNFLE1BQU07QUFBQSxNQUNOLE1BQU07QUFBQSxJQUNSO0FBQUEsSUFDQTtBQUFBLE1BQ0UsTUFBTTtBQUFBLE1BQ04sTUFBTTtBQUFBLElBQ1I7QUFBQSxJQUNBO0FBQUEsTUFDRSxNQUFNO0FBQUEsTUFDTixNQUFNO0FBQUEsSUFDUjtBQUFBLEVBQ0Y7QUFBQSxFQUNBLE9BQU87QUFBQSxJQUNMLGFBQWE7QUFBQSxNQUNYLE9BQU87QUFBQSxNQUNQLE1BQU07QUFBQSxJQUNSO0FBQUEsRUFDRjtBQUFBLEVBQ0EsUUFBUTtBQUFBLElBQ047QUFBQSxNQUNFLE1BQU07QUFBQSxNQUNOLE9BQU87QUFBQSxRQUNMO0FBQUEsVUFDRSxNQUFNO0FBQUEsVUFDTixNQUFNO0FBQUEsUUFDUjtBQUFBLE1BQ0Y7QUFBQSxJQUNGO0FBQUEsSUFDQTtBQUFBLE1BQ0UsTUFBTTtBQUFBLE1BQ04sTUFBTTtBQUFBLElBQ1I7QUFBQSxJQUNBO0FBQUEsTUFDRSxNQUFNO0FBQUEsTUFDTixPQUFPO0FBQUEsUUFDTDtBQUFBLFVBQ0UsTUFBTTtBQUFBLFVBQ04sTUFBTTtBQUFBLFFBQ1I7QUFBQSxRQUNBO0FBQUEsVUFDRSxNQUFNO0FBQUEsVUFDTixNQUFNO0FBQUEsUUFDUjtBQUFBLFFBQ0E7QUFBQSxVQUNFLE1BQU07QUFBQSxVQUNOLE1BQU07QUFBQSxRQUNSO0FBQUEsTUFDRjtBQUFBLElBQ0Y7QUFBQSxFQUNGO0FBQUEsRUFDQSxRQUFRO0FBQUEsSUFDTixjQUFjLFlBQVk7QUFDeEIsVUFBSSxXQUFXLFdBQVcsWUFBWSxFQUFHLFFBQU87QUFDaEQsYUFBTztBQUFBLElBQ1Q7QUFBQSxFQUNGO0FBQUEsRUFDQSxVQUFVO0FBQUEsSUFDUixTQUFTO0FBQUEsSUFDVCxNQUFNO0FBQUEsRUFDUjtBQUVGLENBQUM7IiwKICAibmFtZXMiOiBbXQp9Cg==
