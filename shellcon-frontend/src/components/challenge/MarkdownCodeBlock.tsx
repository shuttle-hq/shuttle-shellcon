import React from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

// Customize the vscDarkPlus theme to match our design system
const customVscDarkPlus = {
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
  },
};

// Define the proper type for code props from ReactMarkdown
interface CodeProps {
  node?: any;
  className?: string;
  children: React.ReactNode;
  [key: string]: any; // For any additional props
}

export const MarkdownCodeBlock: React.FC<CodeProps> = ({ 
  className = '', 
  children, 
  ...props 
}) => {
  // Extract language from className (e.g., 'language-js' -> 'js')
  const match = /language-(\w+)/.exec(className || '');
  const language = match ? match[1] : 'text';
  
  // Handle both string and ReactNode children
  const codeContent = React.Children.map(children, child => {
    if (typeof child === 'string') {
      // Preserve indentation and formatting
      return child;
    } else {
      // Handle non-string children by converting them to strings
      return String(child);
    }
  })?.join('') || '';

  return (
    <div className="relative not-prose">
      {language && (
        <div className="flex justify-between items-center bg-gray-800/80 text-gray-400 text-xs px-4 py-1.5 rounded-t-lg border-b border-gray-700">
          <span className="font-mono">{language}</span>
        </div>
      )}
      <SyntaxHighlighter
        style={customVscDarkPlus}
        language={language}
        PreTag="div"
        wrapLongLines={false}
        showLineNumbers={codeContent.split('\n').length > 5}
        customStyle={{
          margin: 0,
          padding: language ? '0.75rem 1rem' : '1rem',
          borderRadius: language ? '0 0 0.5rem 0.5rem' : '0.5rem',
          background: '#111827', // bg-gray-900
          fontSize: '0.875rem',
          lineHeight: '1.5',
        }}
        codeTagProps={{
          className: 'font-mono',
        }}
        {...props}
      >
        {codeContent.replace(/\n$/, '')}
      </SyntaxHighlighter>
    </div>
  );
};

export default MarkdownCodeBlock;
