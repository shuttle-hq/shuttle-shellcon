
import React from 'react';

interface ChallengeMetadataProps {
  service?: string;
  file?: string;
  function?: string;
}

const ChallengeMetadata: React.FC<ChallengeMetadataProps> = ({ 
  service, 
  file, 
  function: functionName 
}) => {
  if (!service && !file && !functionName) return null;

  return (
    <div className="mb-4 space-y-2">
      {service && (
        <div className="flex items-start gap-2 text-sm">
          <span className="font-semibold text-gray-400 min-w-[80px]">Service:</span>
          <span className="text-gray-300 bg-gray-700/40 px-2 py-1 rounded">{service}</span>
        </div>
      )}
      {file && (
        <div className="flex items-start gap-2 text-sm">
          <span className="font-semibold text-gray-400 min-w-[80px]">File:</span>
          <span className="text-gray-300 bg-gray-700/40 px-2 py-1 rounded font-mono text-xs">{file}</span>
        </div>
      )}
      {functionName && (
        <div className="flex items-start gap-2 text-sm">
          <span className="font-semibold text-gray-400 min-w-[80px]">Function:</span>
          <span className="text-gray-300 bg-gray-700/40 px-2 py-1 rounded font-mono text-xs">{functionName}</span>
        </div>
      )}
    </div>
  );
};

export default ChallengeMetadata;
