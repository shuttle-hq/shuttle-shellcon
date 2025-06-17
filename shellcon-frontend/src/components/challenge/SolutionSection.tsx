
import React from 'react';
import { Code } from "lucide-react";
import EnhancedMarkdown from './EnhancedMarkdown';

interface SolutionProps {
  solution: string | {
    code?: string;
    explanation?: string;
    lecture?: string;
  };
  isVisible: boolean;
}

// Helper function to process the markdown content and ensure proper formatting
const processMarkdown = (content: string): string => {
  // This ensures tables have proper spacing around them for better rendering
  return content
    .replace(/\|\s*\n\s*\|/g, '|\n|')  // Ensure newlines after table rows
    .replace(/\|\s*\n\s*\|/g, '|\n|')  // Repeat to catch any remaining
    .replace(/\|\s*\n\s*\|/g, '|\n|');  // One more time to be sure
};

// Helper function to detect code language from content
const detectLanguage = (code: string): string => {
  if (!code) return 'text';
  if (code.includes('fn ') || code.includes('pub fn') || code.includes('impl')) return 'rust';
  if (code.includes('function') || code.includes('=>') || code.includes('const ')) return 'javascript';
  if (code.includes('def ') || code.includes('class ') || code.includes('import ')) return 'python';
  if (code.includes('SELECT') || code.includes('INSERT') || code.includes('UPDATE')) return 'sql';
  return 'text';
};

const SolutionSection: React.FC<SolutionProps> = ({ solution, isVisible }) => {
  // Type guard to ensure isVisible is strictly a boolean
  const shouldShow = Boolean(isVisible);
  
  if (!shouldShow || !solution) return null;

  // Handle string solution (backward compatibility)
  if (typeof solution === 'string') {
    return (
      <div className="mt-4 p-4 bg-gray-900 rounded-md border-2 border-orange-500/40 animate-fade-in">
        <h4 className="font-bold mb-2 text-orange-300 flex items-center gap-2">
          <Code className="h-4 w-4" />
          Solution:
        </h4>
        <div className="text-gray-300 prose prose-invert prose-sm max-w-none">
          <EnhancedMarkdown 
            content={solution}
            accentColor="orange"
          />
        </div>
      </div>
    );
  }

  // Handle object solution
  if (!solution.code && !solution.explanation) return null;

  return (
    <div className="mt-4 p-4 bg-gray-900 rounded-md border-2 border-orange-500/40 mb-4 animate-fade-in">
      <h4 className="font-bold mb-2 text-orange-300 flex items-center gap-2">
        <Code className="h-4 w-4" />
        Solution:
      </h4>
      
      {/* Code Block with Syntax Highlighting */}
      {solution.code && (
        <div className="mb-4">
          <h5 className="text-sm font-semibold text-orange-200 mb-1">Code Changes:</h5>
          <div className="my-4">
            <EnhancedMarkdown 
              content={`\`\`\`${detectLanguage(solution.code)}
${solution.code}
\`\`\``}
              accentColor="orange"
            />
          </div>
        </div>
      )}

      {/* Explanation with Markdown Support */}
      {solution.explanation && (
        <div>
          <h5 className="text-sm font-semibold text-orange-200 mb-1">Explanation:</h5>
          <div className="text-gray-300 prose prose-invert prose-sm max-w-none">
            <EnhancedMarkdown 
              content={solution.explanation}
              accentColor="orange"
            />
          </div>
        </div>
      )}
    </div>
  );
};

export default SolutionSection;
