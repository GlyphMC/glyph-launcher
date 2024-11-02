import type { Config } from "tailwindcss";

export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],

	theme: {
		extend: {},
		fontFamily: {
			display: ["Inter", "sans-serif"],
			mono: ["JetBrains Mono", "monospace"]
		},
		colors: {
			zinc: {
				50: "#f3f3f3",
				100: "#b6b6b6",
				200: "#7f7d82",
				300: "#363438",
				400: "#252427",
				500: "#1e1e20",
				600: "#1a1a1c",
				700: "#141415",
				800: "#121213",
				900: "#0c0c0d"
			},
			red: {
				50: "#fcefed",
				100: "#f8dbd6",
				200: "#f1bbb2",
				300: "#eba091",
				400: "#e2675d",
				500: "#d64237",
				600: "#a52a21",
				700: "#7d1e17",
				800: "#4d120e",
				900: "#360d09"
			},
			green: {
				50: "#f0fdf4",
				100: "#dcfce7",
				200: "#bbf7d0",
				300: "#86efac",
				400: "#4ade80",
				500: "#22c55e",
				600: "#16a34a",
				700: "#15803d",
				800: "#166534",
				900: "#14532d"
			}
		}
	},
	plugins: []
} as Config;
