/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        navy: { 800: '#1e293b', 900: '#0f172a' },
        ocean: { 500: '#0284c7', 600: '#0369a1' },
        safety: { 500: '#f97316' } // Safety orange action elements
      }
    },
  },
  plugins: [],
}
