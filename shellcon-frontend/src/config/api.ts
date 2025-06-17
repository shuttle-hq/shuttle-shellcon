// Simple unified API URL and path management

// Get current environment mode
const isDevelopment = import.meta.env.DEV === true;

// Get API base URLs from environment variables
const API_BASE_URLS = {
  AQUA_MONITOR: import.meta.env.VITE_AQUA_MONITOR_URL || 'http://localhost:8000',
  SPECIES_HUB: import.meta.env.VITE_SPECIES_HUB_URL || 'http://localhost:8001',
  AQUA_BRAIN: import.meta.env.VITE_AQUA_BRAIN_URL || 'http://localhost:8002',
};

// The API prefix used by all backend services
const API_PREFIX = '/api';

// In development mode, always use the Vite proxy with relative paths
// This ensures CORS issues are handled properly

// Export the API URLs for use in the application
export const API_URLS = {
  // In development mode, always use relative paths to go through the Vite proxy
  // In production, use the actual backend URLs directly
  AQUA_MONITOR: isDevelopment ? API_PREFIX : API_BASE_URLS.AQUA_MONITOR,
  SPECIES_HUB: isDevelopment ? API_PREFIX : API_BASE_URLS.SPECIES_HUB,
  AQUA_BRAIN: isDevelopment ? API_PREFIX : API_BASE_URLS.AQUA_BRAIN,
};

// Export the base API prefix for general use
export const API_BASE_URL = API_PREFIX;

// Log configuration for debugging
console.log('API Configuration:');
console.log('- Base URLs:');
console.log(`  - AQUA_MONITOR: ${API_BASE_URLS.AQUA_MONITOR}`);
console.log(`  - SPECIES_HUB: ${API_BASE_URLS.SPECIES_HUB}`);
console.log(`  - AQUA_BRAIN: ${API_BASE_URLS.AQUA_BRAIN}`);
console.log('- API URLs:');
console.log(`  - AQUA_MONITOR: ${API_URLS.AQUA_MONITOR}`);
console.log(`  - SPECIES_HUB: ${API_URLS.SPECIES_HUB}`);
console.log(`  - AQUA_BRAIN: ${API_URLS.AQUA_BRAIN}`);
