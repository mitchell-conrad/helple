module.exports = {
  corePlugins: {
    preflight: true,
  },
  content: [
    './js/**/*.js',
    './css/**/*.css',
    '../lib/*_web/**/*.*ex',
  ],
  theme: {
    extend: {},
  },
  plugins: [],
  darkMode: 'media' // or 'class'
}