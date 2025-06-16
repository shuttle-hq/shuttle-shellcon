
import React from 'react';

interface LoadingStateProps {
  message?: string;
}

export const LoadingState: React.FC<LoadingStateProps> = ({ message = "Loading..." }) => (
  <div className="flex justify-center items-center py-8">
    <div className="animate-pulse flex flex-col items-center">
      <div className="h-10 w-10 bg-orange-500/20 rounded-full mb-2"></div>
      <div className="text-orange-400">{message}</div>
    </div>
  </div>
);

interface ErrorStateProps {
  error: string;
}

export const ErrorState: React.FC<ErrorStateProps> = ({ error }) => (
  <div className="p-4 border-2 border-red-500/30 rounded-md bg-red-900/20">
    <h3 className="text-red-400 font-medium mb-1">Error</h3>
    <p className="text-sm text-gray-300">{error}</p>
  </div>
);

interface NoDataStateProps {
  message?: string;
}

export const NoDataState: React.FC<NoDataStateProps> = ({ message = "No data available" }) => (
  <div className="p-4 border-2 border-gray-500/30 rounded-md bg-gray-800">
    <p className="text-gray-400 text-center">{message}</p>
  </div>
);

interface JsonViewProps {
  data: any;
}

export const JsonView: React.FC<JsonViewProps> = ({ data }) => (
  <pre className="bg-gray-950 p-4 rounded-md overflow-x-auto text-sm text-gray-300">
    {JSON.stringify(data, null, 2)}
  </pre>
);
