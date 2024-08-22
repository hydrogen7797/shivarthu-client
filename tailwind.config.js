/** @type {import('tailwindcss').Config} */
export default {
  content: {
    files: ["*.html", "./src/**/*.rs", "./node_modules/flowbite/**/*.js"],
  },
  // corePlugins: {
  //   preflight: false,
  // },
  theme: {
    extend: {},
  },
  plugins: [require("daisyui"), require("flowbite/plugin")],
};
