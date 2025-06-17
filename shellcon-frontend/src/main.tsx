
import { createRoot } from 'react-dom/client'
import App from './App.tsx'
import './index.css'

// Create a new root and render the application
const rootElement = document.getElementById("root");
if (!rootElement) throw new Error("Root element not found");

const root = createRoot(rootElement);
root.render(<App />);
