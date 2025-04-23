module.exports = {
    mode: "all",
    content: [
        // include all rust, html and css files in the src directory
        "./rustcms-ui/src/**/*.{rs,html}",
        // include all html files in the output (dist) directory
        //"./publish/data/www/**/*.html",
    ],
    theme: {
        extend: {},
    },
    plugins: [],
}