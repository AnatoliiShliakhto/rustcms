module.exports = {
    mode: "all",
    content: [
        // include all rust, html and css files in the src directory
        "./rustcms-ui/src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        //"./src/**/*.html",
    ],
    theme: {
        extend: {},
    },
    plugins: [],
}