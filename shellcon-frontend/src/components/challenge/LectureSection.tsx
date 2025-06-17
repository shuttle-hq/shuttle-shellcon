
import React from 'react';
import { BookOpen } from "lucide-react";
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { CustomCodeBlock } from './shared/CustomCodeBlock';
import { cn } from '@/lib/utils'; // Make sure this import path is correct for your project

interface LectureSectionProps {
  lecture: string | undefined;
  isVisible: boolean;
}

const LectureSection: React.FC<LectureSectionProps> = ({ lecture, isVisible }) => {
  if (!isVisible || !lecture) return null;
  
  // Helper function to process the lecture content and ensure proper markdown table formatting
  const processMarkdown = (content: string): string => {
    // This ensures tables have proper spacing around them for better rendering
    return content
      .replace(/\|\s*\n\s*\|/g, '|\n|')  // Ensure newlines after table rows
      .replace(/\|\s*\n\s*\|/g, '|\n|')  // Repeat to catch any remaining
      .replace(/\|\s*\n\s*\|/g, '|\n|');  // One more time to be sure
  };

  return (
    <div className="mt-4 p-4 bg-green-900/20 rounded-md border-2 border-green-500/40 animate-fade-in">
      <h4 className="font-bold mb-4 text-green-300 flex items-center gap-2">
        <BookOpen className="h-4 w-4" />
        More Information:
      </h4>
      <div className="text-gray-300 prose prose-invert prose-sm max-w-none">
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
            a: ({ node, ...props }) => (
              <a className="text-orange-400 hover:text-orange-300 underline underline-offset-2" {...props} />
            ),
            // Blockquotes
            blockquote: ({ node, ...props }) => (
              <blockquote 
                className="border-l-4 border-orange-500 pl-4 py-1 my-4 text-gray-300 italic bg-gray-800/30" 
                {...props} 
              />
            ),
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
                  accentColor="green"
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
          {processMarkdown(lecture)}
        </ReactMarkdown>
      </div>
    </div>
  );
};

export default LectureSection;
