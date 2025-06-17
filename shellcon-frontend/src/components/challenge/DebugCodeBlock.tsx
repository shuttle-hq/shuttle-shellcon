import React, { useEffect } from 'react';
import { CustomCodeBlock } from './shared/CustomCodeBlock';

// This component will help debug code block issues
interface DebugCodeBlockProps {
  content: string;
  language?: string;
  accentColor?: string;
}

const DebugCodeBlock: React.FC<DebugCodeBlockProps> = ({ 
  content, 
  language = 'text',
  accentColor = 'gray'
}) => {
  useEffect(() => {
    // Debug the content for troubleshooting
    console.log('Debug Code Block Content:', { 
      content,
      contentLength: content.length,
      language,
      // Check if there are problematic quote characters
      hasSingleQuotes: content.includes("'''"),
      hasDoubleQuotes: content.includes('"""'),
      // Check for mixed quote styles
      hasMixedQuotes: content.includes("'''") && content.includes('"""'),
      // Log the first few characters and last few characters
      start: content.substring(0, 20),
      end: content.substring(content.length - 20)
    });
  }, [content, language]);

  return (
    <CustomCodeBlock 
      className={`language-${language}`}
      accentColor={accentColor}
    >
      {content}
    </CustomCodeBlock>
  );
};

export default DebugCodeBlock;
