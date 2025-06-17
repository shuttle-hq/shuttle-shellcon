import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react-swc";
import path from "path";
import { componentTagger } from "lovable-tagger";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  // Load env file based on mode
  const env = loadEnv(mode, process.cwd(), '');
  
  // Get API URLs from environment variables or use defaults
  const AQUA_MONITOR_URL = env.VITE_AQUA_MONITOR_URL || 'http://localhost:8000';
  const SPECIES_HUB_URL = env.VITE_SPECIES_HUB_URL || 'http://localhost:8001';
  const AQUA_BRAIN_URL = env.VITE_AQUA_BRAIN_URL || 'http://localhost:8002';
  
  console.log(`Mode: ${mode}`);
  console.log('Using backend services:');
  console.log(`- AQUA_MONITOR_URL: ${AQUA_MONITOR_URL}`);
  console.log(`- SPECIES_HUB_URL: ${SPECIES_HUB_URL}`);
  console.log(`- AQUA_BRAIN_URL: ${AQUA_BRAIN_URL}`);
  
  const isUsingCloudBackends = mode === 'prod';
  console.log(`Using cloud backends: ${isUsingCloudBackends}`);
  
  return {
    server: {
      host: "::",
      port: 8080,
      proxy: {
        // Standard API paths - we'll always use the /api prefix format
        // Do NOT strip /api for ANY backend - all backends expect it
        
        // Aqua Monitor paths
        '/api/tanks': {
          target: AQUA_MONITOR_URL,
          changeOrigin: true,
          secure: false,
          // Don't strip /api prefix - backend servers expect it
          rewrite: undefined,
          configure: (proxy) => {
            proxy.on('proxyReq', (proxyReq, req) => {
              console.log(`Proxying: ${req.url} → ${AQUA_MONITOR_URL}`);
            });
          }
        },
        '/api/sensors': {
          target: AQUA_MONITOR_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined
        },
        '/api/challenges/1/validate': {
          target: AQUA_MONITOR_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined
        },
        
        // Species Hub paths
        // Challenge 2 validation endpoint
        '/api/challenges/2/validate': {
          target: SPECIES_HUB_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined,
          configure: (proxy) => {
            proxy.on('proxyReq', (proxyReq, req) => {
              console.log(`Proxying challenge 2 validation: ${req.url} → ${SPECIES_HUB_URL}`);
            });
          }
        },
        '/api/species': {
          target: SPECIES_HUB_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined
        },
        '/api/species/validate-solution': {
          target: SPECIES_HUB_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined,
          configure: (proxy) => {
            proxy.on('proxyReq', (proxyReq, req) => {
              console.log(`Proxying species validation: ${req.url} → ${SPECIES_HUB_URL}`);
            });
          }
        },
        '/api/feeding': {
          target: SPECIES_HUB_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined
        },
        // Challenge 3 validation endpoint - Memory Miser (Memory Optimization)
        '/api/challenges/3/validate': {
          target: AQUA_BRAIN_URL,  // Using Aqua Brain service as specified in challenge API response
          changeOrigin: true,
          secure: false,
          rewrite: undefined,
          configure: (proxy) => {
            proxy.on('proxyReq', (proxyReq, req) => {
              console.log(`Proxying challenge 3 validation: ${req.url} → ${AQUA_BRAIN_URL}`);
            });
          }
        },
        
        // Challenge 4 validation endpoint - The Leaky Connection (Resource Optimization)
        '/api/challenges/4/validate': {
          target: AQUA_MONITOR_URL,  // Using Aqua Monitor service as verified by direct curl test
          changeOrigin: true,
          secure: false,
          rewrite: undefined,
          configure: (proxy) => {
            proxy.on('proxyReq', (proxyReq, req) => {
              console.log(`Proxying challenge 4 validation: ${req.url} → ${AQUA_MONITOR_URL}`);
            });
          }
        },
        
        // Aqua Brain paths
        '/api/system': {
          target: AQUA_BRAIN_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined
        },
        '/api/analysis': {
          target: AQUA_BRAIN_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined
        },
        '/api/challenges/current': {
          target: AQUA_BRAIN_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined,
          configure: (proxy) => {
            proxy.on('proxyReq', (proxyReq, req) => {
              console.log(`Proxying challenges/current: ${req.url} → ${AQUA_BRAIN_URL}`);
            });
          }
        },
        '/api/challenges': {
          target: AQUA_BRAIN_URL,
          changeOrigin: true,
          secure: false,
          rewrite: undefined,
          configure: (proxy) => {
            proxy.on('proxyReq', (proxyReq, req) => {
              console.log(`Proxying challenges: ${req.url} → ${AQUA_BRAIN_URL}`);
            });
          }
        }
      }
    },
    plugins: [
      react(),
      mode === 'development' &&
      componentTagger(),
    ].filter(Boolean),
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "./src"),
      },
    },
  };
});
