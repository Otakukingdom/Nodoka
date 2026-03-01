/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Match existing iced color scheme
        primary: {
          DEFAULT: '#E11D48',
          light: '#F43F5E',
          dark: '#BE123C',
        },
        secondary: {
          DEFAULT: '#2563EB',
          light: '#3B82F6',
          dark: '#1D4ED8',
        },
      },
      fontFamily: {
        sans: ['Atkinson Hyperlegible', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [
    require('daisyui'),
  ],
  daisyui: {
    themes: [
      {
        light: {
          "primary": "#E11D48",
          "secondary": "#2563EB",
          "accent": "#F59E0B",
          "neutral": "#292524",
          "base-100": "#F5F5F4",
          "info": "#3B82F6",
          "success": "#10B981",
          "warning": "#F59E0B",
          "error": "#EF4444",
        },
        dark: {
          "primary": "#FB7185",
          "secondary": "#60A5FA",
          "accent": "#FBBF24",
          "neutral": "#292524",
          "base-100": "#1C1917",
          "info": "#3B82F6",
          "success": "#10B981",
          "warning": "#F59E0B",
          "error": "#EF4444",
        },
      },
    ],
  },
};
