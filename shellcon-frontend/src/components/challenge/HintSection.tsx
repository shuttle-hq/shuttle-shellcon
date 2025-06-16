
import React from 'react';
import { HelpCircle } from "lucide-react";
import EnhancedMarkdown from './EnhancedMarkdown';

interface HintSectionProps {
  hint: string | undefined;
  isVisible: boolean;
}

const HintSection: React.FC<HintSectionProps> = ({ hint, isVisible }) => {
  // Type guard to ensure isVisible is strictly a boolean
  const shouldShow = Boolean(isVisible);
  
  if (!shouldShow || !hint) return null;
  
  // Helper function to process the markdown content and ensure proper formatting
  const processMarkdown = (content: string): string => {
    // This ensures tables have proper spacing around them for better rendering
    return content
      .replace(/\|\s*\n\s*\|/g, '|\n|')  // Ensure newlines after table rows
      .replace(/\|\s*\n\s*\|/g, '|\n|')  // Repeat to catch any remaining
      .replace(/\|\s*\n\s*\|/g, '|\n|');  // One more time to be sure
  };
  
  return (
    <div className="mt-4 p-4 bg-blue-900/20 rounded-md border-2 border-blue-500/40 mb-4">
      <h4 className="font-bold mb-2 text-blue-300 flex items-center gap-2">
        <HelpCircle className="h-4 w-4" />
        Hint:
      </h4>
      <div className="text-gray-300 prose prose-invert prose-sm max-w-none">
        <EnhancedMarkdown 
          content={hint} 
          accentColor="blue"
        />
      </div>
    </div>
  );
};

export default HintSection;
