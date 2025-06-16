
import React from 'react';

interface ValidationMessageProps {
  message: string | null;
  isSolved: boolean;
}

const ValidationMessage: React.FC<ValidationMessageProps> = ({ message, isSolved }) => {
  if (!message) return null;
  
  return (
    <div className={`mb-4 p-3 rounded border ${isSolved ? 'border-green-500/40 bg-green-900/20 text-green-400' : 'border-orange-500/40 bg-orange-900/20 text-orange-400'}`}>
      {message}
    </div>
  );
};

export default ValidationMessage;
