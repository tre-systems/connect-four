export default {
  plugins: process.env.VITEST ? [] : ['@tailwindcss/postcss'],
};
