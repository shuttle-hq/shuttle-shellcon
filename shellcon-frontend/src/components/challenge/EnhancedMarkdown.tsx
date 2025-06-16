import React, { useMemo } from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { CustomCodeBlock } from './shared/CustomCodeBlock';
import { cn } from '@/lib/utils';

// This component enhances markdown rendering with special handling for code blocks
interface EnhancedMarkdownProps {
  content: string;
  accentColor?: 'blue' | 'green' | 'orange' | 'gray';
}

export const EnhancedMarkdown: React.FC<EnhancedMarkdownProps> = ({ 
  content, 
  accentColor = 'gray' 
}) => {
  // Pre-process the markdown content if needed
  const processedContent = useMemo(() => {
    if (!content) return '';
    
    // Simple preprocessing for code blocks
    let result = content;
    
    // Clean up malformed markdown - fix any issues with code blocks
    // This fixes issues where code blocks might be improperly formatted
    
    // First, normalize all code block delimiters to ensure consistent spacing
    // Ensure opening code fence has proper spacing
    result = result.replace(/```(\w*)\s*/g, '```$1\n');
    
    // Ensure closing code fence has proper spacing
    result = result.replace(/\s*```/g, '\n```');
    
    // Fix unclosed code blocks by checking if there are an odd number of code fences
    const codeBlockCount = (result.match(/```/g) || []).length;
    if (codeBlockCount % 2 !== 0) {
      // If odd number, add a closing fence at the end
      result += '\n```';
    }
    
    // Handle nested code blocks by finding any code fence that appears immediately after another
    result = result.replace(/```\s*```(\w*)/g, '```');
    
    // Fix any empty code blocks (ensure they at least have a newline inside)
    result = result.replace(/```(\w*)\n```/g, '```$1\n\n```');
    
    return result;
  }, [content]);
  
  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      components={{
        // Headings
        h1: ({ node, ...props }) => (
          <h1 className="text-2xl font-bold text-white mt-6 mb-4 pb-2 border-b border-gray-700" {...props} />
        ),
        h2: ({ node, ...props }) => (
          <h2 className="text-xl font-semibold text-white mt-5 mb-3 pt-2" {...props} />
        ),
        h3: ({ node, ...props }) => (
          <h3 className="text-lg font-medium text-gray-200 mt-4 mb-2" {...props} />
        ),
        h4: ({ node, ...props }) => (
          <h4 className="text-base font-medium text-gray-300 mt-3 mb-2" {...props} />
        ),
        // Paragraphs
        p: ({ node, ...props }) => (
          <p className="my-3 leading-relaxed text-gray-300" {...props} />
        ),
        // Lists
        ul: ({ node, ...props }) => (
          <ul className="list-disc pl-6 my-3 space-y-1" {...props} />
        ),
        ol: ({ node, ...props }) => (
          <ol className="list-decimal pl-6 my-3 space-y-1" {...props} />
        ),
        li: ({ node, ...props }) => (
          <li className="my-1" {...props} />
        ),
        // Links
        a: ({ node, ...props }) => {
          const linkColor = {
            blue: 'text-blue-400 hover:text-blue-300',
            green: 'text-green-400 hover:text-green-300',
            orange: 'text-orange-400 hover:text-orange-300',
            gray: 'text-gray-400 hover:text-gray-300'
          }[accentColor];
          
          return (
            <a className={`${linkColor} underline underline-offset-2`} {...props} />
          );
        },
        // Blockquotes
        blockquote: ({ node, ...props }) => {
          const borderColor = {
            blue: 'border-blue-500',
            green: 'border-green-500',
            orange: 'border-orange-500',
            gray: 'border-gray-500'
          }[accentColor];
          
          return (
            <blockquote 
              className={`border-l-4 ${borderColor} pl-4 py-1 my-4 text-gray-300 italic bg-gray-800/30`}
              {...props} 
            />
          );
        },
        // Tables
        table: ({ node, ...props }) => (
          <div className="my-4 overflow-x-auto rounded-lg border border-gray-700">
            <table 
              className="min-w-full divide-y divide-gray-700 text-sm"
              {...props} 
            />
          </div>
        ),
        thead: ({ node, ...props }) => (
          <thead className="bg-gray-800/70" {...props} />
        ),
        tbody: ({ node, ...props }) => (
          <tbody className="divide-y divide-gray-800" {...props} />
        ),
        tr: ({ node, ...props }) => (
          <tr className="hover:bg-gray-800/40 transition-colors" {...props} />
        ),
        th: ({ node, ...props }) => (
          <th 
            className="px-4 py-3 text-left text-xs font-semibold text-gray-200 uppercase tracking-wider bg-gray-800/80"
            {...props} 
          />
        ),
        td: ({ node, ...props }) => (
          <td 
            className="px-4 py-3 text-sm text-gray-300 border-t border-gray-800"
            {...props} 
          />
        ),
        // Code blocks
        code: ({ node, className, children, ...props }: any) => {
          return (
            <CustomCodeBlock 
              className={className}
              accentColor={accentColor}
              {...props}
            >
              {children}
            </CustomCodeBlock>
          );
        },
        pre: ({ node, ...props }: any) => (
          <div className="my-4 rounded-lg overflow-hidden border border-gray-700">
            <pre className="!bg-gray-900/80 !m-0" {...props} />
          </div>
        ),
        // Horizontal rule
        hr: ({ node, ...props }) => (
          <hr className="my-6 border-t border-gray-700" {...props} />
        ),
      }}
      className="space-y-4"
    >
      {processedContent}
    </ReactMarkdown>
  );
};

export default EnhancedMarkdown;
