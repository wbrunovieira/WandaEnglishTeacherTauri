/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./src/**/*.{js,jsx,ts,tsx}"],
    theme: {
        extend: {
            colors: {
                customPurple1: "#350545",
                customPurple2: "#792990",
                customYellow: "#ffb947",
            },
        },
    },
    plugins: [],
};
