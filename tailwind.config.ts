import { join } from 'path';
import type { Config } from 'tailwindcss';
// @ts-ignore
import { skeleton } from '@skeletonlabs/tw-plugin';

const config = {
	darkMode: 'class',
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')
	],
	theme: {
		extend: {
			colors: {
				primary: {
					50: '#e8f9f5',
					100: '#c2efe4',
					200: '#9be5d3',
					300: '#74dbc2',
					400: '#4dd1b1',
					500: '#1ABC9C', // Main turquoise
					600: '#16a689',
					700: '#129077',
					800: '#0e7a64',
					900: '#0a6451'
				},
				turquoise: {
					DEFAULT: '#1ABC9C',
					light: '#48C9B0',
					dark: '#16A085'
				}
			}
		}
	},
	plugins: [
		skeleton({
			themes: {
				preset: [
					{
						name: 'skeleton',
						enhancements: true
					}
				]
			}
		})
	]
} satisfies Config;

export default config;
