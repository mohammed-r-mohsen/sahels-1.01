const tailwindcss = require('tailwindcss');
const autoprefixer = require('autoprefixer');

module.exports = {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        brand: {
          50: '#eefaf7',
          100: '#d9f5ec',
          200: '#b5eadb',
          300: '#7dd5b0',
          400: '#45bb8a',
          500: '#1d9a6e',
          600: '#137d5b',
          700: '#11644b',
          800: '#114f3e',
          900: '#103f34',
        },
      },
    },
  },
  plugins: [tailwindcss, autoprefixer],
};
