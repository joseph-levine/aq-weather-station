import { defineConfig } from 'vite'

export default defineConfig({
    build: {
        rollupOptions: {
            output: {
                dir: 'dist',
                entryFileNames: 'app.js',
                assetFileNames: 'app.css',
                chunkFileNames: "chunk.js",
                manualChunks: undefined,
            }
        }
    }
})
