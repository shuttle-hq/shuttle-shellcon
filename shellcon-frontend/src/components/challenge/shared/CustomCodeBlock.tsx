import React from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { cn } from '@/lib/utils';

// Enhanced version of vscDarkPlus theme with better readability
const enhancedCodeTheme = {
  ...vscDarkPlus,
  'pre[class*="language-"]': {
    ...vscDarkPlus['pre[class*="language-"]'],
    background: '#111827', // bg-gray-900
    margin: 0,
    padding: '1rem',
    borderRadius: '0.5rem',
    overflowX: 'auto',
  },
  'code[class*="language-"]': {
    ...vscDarkPlus['code[class*="language-"]'],
    fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
    fontSize: '0.875rem',
    lineHeight: '1.5',
    color: '#e5e7eb', // text-gray-200
    textShadow: 'none',
  },
};

interface CustomCodeBlockProps {
  className?: string;
  children: React.ReactNode;
  accentColor?: string;
  [key: string]: any;
}

export const CustomCodeBlock: React.FC<CustomCodeBlockProps> = ({ 
  className = '', 
  children, 
  accentColor = 'orange',
  ...props 
}) => {
  // Extract language from className (e.g., 'language-js' -> 'js')
  const match = /language-(\w+)/.exec(className || '');
  
  // Get language from className or default to text
  let language = match ? match[1].toLowerCase() : 'text';
  
  // Auto-detect language based on code content if not explicitly specified
  if (language === 'text' && typeof children === 'string') {
    const codeStr = children.trim();
    
    // Rust detection patterns
    if (codeStr.includes('fn ') || 
        codeStr.includes('let ') || 
        codeStr.includes('use ') || 
        codeStr.includes('async ') || 
        codeStr.includes('impl ') || 
        codeStr.includes('tokio::') || 
        codeStr.includes('std::') || 
        codeStr.includes('Result<') || 
        codeStr.startsWith('//')) {
      language = 'rust';
    }
    // SQL detection patterns
    else if (codeStr.includes('SELECT ') || 
             codeStr.includes('INSERT INTO ') || 
             codeStr.includes('CREATE TABLE ') || 
             codeStr.includes('CREATE INDEX ') || 
             codeStr.startsWith('--')) {
      language = 'sql';
    }
  }
  
  // Determine if this is an inline code block or a code fence
  const isInline = !className?.includes('language-');
  
  // Handle both string and ReactNode children
  const codeContent = React.Children.map(children, child => {
    if (typeof child === 'string') {
      // For string content, preserve whitespace and formatting
      return child;
    } else {
      // For React nodes, convert to string but preserve formatting
      return String(child);
    }
  })?.join('') || '';

  // If there's no content, don't render anything
  if (!codeContent.trim()) return null;
  
  // For inline code, render with the appropriate accent color
  if (isInline) {
    const colorMap = {
      orange: 'text-orange-300',
      blue: 'text-blue-300',
      green: 'text-green-300',
      gray: 'text-gray-300'
    };
    
    const textColor = colorMap[accentColor as keyof typeof colorMap] || 'text-gray-300';
    
    return (
      <code 
        className={cn(
          'bg-gray-800/80 px-1.5 py-0.5 rounded text-sm font-mono',
          'border border-gray-700',
          textColor
        )}
        {...props}
      >
        {children}
      </code>
    );
  }
  
  // For code fences, render with syntax highlighting
  return (
    <div className="relative not-prose my-4 rounded-lg overflow-hidden border border-gray-700">
      {language && (
        <div className="flex justify-between items-center bg-gray-800/80 text-gray-400 text-xs px-4 py-1.5 border-b border-gray-700">
          <span className="font-mono">{language}</span>
        </div>
      )}
      <SyntaxHighlighter
        style={enhancedCodeTheme}
        language={language}
        PreTag="div"
        wrapLongLines={false}
        showLineNumbers={codeContent.split('\n').length > 5}
        codeTagProps={{
          style: { fontFamily: 'monospace' },
          className: 'font-mono'
        }}
        customStyle={{
          margin: 0,
          padding: '0.75rem 1rem',
          borderRadius: language ? '0 0 0.5rem 0.5rem' : '0.5rem',
          background: '#111827', // bg-gray-900
          fontSize: '0.875rem',
          lineHeight: '1.5',
        }}
        {...props}
      >
        {codeContent.replace(/\\n$/, '')}
      </SyntaxHighlighter>
    </div>
  );
};

export default CustomCodeBlock;
