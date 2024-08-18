/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./*.{js,ts,jsx,tsx,md,mdx}", "./pages/**/*.{js,ts,jsx,tsx,md,mdx}", "./components/**/*.{js,ts,jsx,tsx,md,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['var(--font-geist-sans)'],
        mono: ['var(--font-geist-mono)'],
      },
    },
    
  },
  darkMode: 'class',
  important: true,
  plugins: [],
}

