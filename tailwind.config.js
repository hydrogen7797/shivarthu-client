/** @type {import('tailwindcss').Config} */
export default {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [require("daisyui"), require('flowbite/plugin')],
}

