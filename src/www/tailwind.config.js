/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            '--tw-prose-body': 'rgb(75 85 99)', // gray-600
            '--tw-prose-headings': 'rgb(88 28 135 / 0.5)', // violet-900/50
            '--tw-prose-lead': 'rgb(75 85 99)', // gray-600
            '--tw-prose-links': 'rgb(124 58 237)', // violet-600
            '--tw-prose-bold': 'rgb(17 24 39)', // gray-900
            '--tw-prose-counters': 'rgb(107 114 128)', // gray-500
            '--tw-prose-bullets': 'rgb(209 213 219)', // gray-300
            '--tw-prose-hr': 'rgb(229 231 235)', // gray-200
            '--tw-prose-quotes': 'rgb(17 24 39)', // gray-900
            '--tw-prose-quote-borders': 'rgb(229 231 235)', // gray-200
            '--tw-prose-captions': 'rgb(107 114 128)', // gray-500
            '--tw-prose-code': 'rgb(124 58 237)', // violet-600
            '--tw-prose-pre-code': 'rgb(229 231 235)', // gray-200
            '--tw-prose-pre-bg': 'rgb(17 24 39)', // gray-900
            '--tw-prose-th-borders': 'rgb(209 213 219)', // gray-300
            '--tw-prose-td-borders': 'rgb(229 231 235)', // gray-200
            
            // Dark mode colors
            '--tw-prose-invert-body': 'rgb(156 163 175)', // gray-400
            '--tw-prose-invert-headings': 'rgb(196 181 253)', // violet-300
            '--tw-prose-invert-lead': 'rgb(156 163 175)', // gray-400
            '--tw-prose-invert-links': 'rgb(196 181 253)', // violet-300
            '--tw-prose-invert-bold': 'rgb(255 255 255)', // white
            '--tw-prose-invert-counters': 'rgb(107 114 128)', // gray-500
            '--tw-prose-invert-bullets': 'rgb(75 85 99)', // gray-600
            '--tw-prose-invert-hr': 'rgb(55 65 81)', // gray-700
            '--tw-prose-invert-quotes': 'rgb(243 244 246)', // gray-100
            '--tw-prose-invert-quote-borders': 'rgb(55 65 81)', // gray-700
            '--tw-prose-invert-captions': 'rgb(107 114 128)', // gray-500
            '--tw-prose-invert-code': 'rgb(196 181 253)', // violet-300
            '--tw-prose-invert-pre-code': 'rgb(229 231 235)', // gray-200
            '--tw-prose-invert-pre-bg': 'rgb(0 0 0 / 0.5)', // black/50
            '--tw-prose-invert-th-borders': 'rgb(75 85 99)', // gray-600
            '--tw-prose-invert-td-borders': 'rgb(55 65 81)', // gray-700
          },
        },
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}