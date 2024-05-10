import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import fs from 'fs';
import { AppSettings } from './src/appsettings';
// import devtools from 'solid-devtools/vite';

// Read and parse the JSON into a appsettings object
const appSettings: AppSettings = JSON.parse(fs.readFileSync('./appsettings.json', 'utf8'));

export default defineConfig({
  plugins: [
    /* 
    Uncomment the following line to enable solid-devtools.
    For more info see https://github.com/thetarnav/solid-devtools/tree/main/packages/extension#readme
    */
    // devtools(),
    solidPlugin(),
  ],
  server: {
    port: 3000,
  },
  build: {
    target: 'esnext',
    outDir: 'dist'
  },
  define: {
    'process.env.Appsettings': appSettings,
  },
});
