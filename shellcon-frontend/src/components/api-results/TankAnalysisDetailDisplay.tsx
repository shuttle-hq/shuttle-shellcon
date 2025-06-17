import React from 'react';
import { TankAnalysisDetail } from '../system-architecture/useAquaBrainData';
import { StatusBadge } from '@/components/ui/status-badge';

interface TankAnalysisDetailDisplayProps {
  data: TankAnalysisDetail;
}

export const TankAnalysisDetailDisplay: React.FC<TankAnalysisDetailDisplayProps> = ({ data }) => {
  // Check if the data is in the expected format
  if (!data || typeof data !== 'object') {
    return (
      <div className="p-4 bg-red-900/30 border border-red-500 rounded-md">
        <h3 className="text-lg font-bold text-red-400 mb-2">Invalid Data Format</h3>
        <p className="text-white">The tank analysis data is not in a valid format.</p>
      </div>
    );
  }

  // No need for a helper function as we're using the StatusBadge component

  return (
    <div className="space-y-4">
      <div className="bg-gray-800 p-4 rounded-md">
        <h3 className="text-lg font-bold text-orange-400 mb-4">
          Tank Analysis: {data.tank_id}
        </h3>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <div className="bg-gray-850 p-3 rounded-md border border-gray-700">
            <div className="text-sm text-gray-400 mb-1">Overall Health</div>
            <div className="flex items-center">
              <StatusBadge 
                status={data.overall_health} 
                type="overall_health" 
                className="text-sm px-3 py-1" 
              />
            </div>
          </div>
          
          <div className="bg-gray-850 p-3 rounded-md border border-gray-700">
            <div className="text-sm text-gray-400 mb-1">Last Updated</div>
            <div className="text-white">
              {new Date(data.timestamp).toLocaleString()}
            </div>
          </div>
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <div className="bg-gray-850 p-3 rounded-md border border-gray-700">
            <div className="text-sm text-gray-400 mb-1">Temperature</div>
            <div className="flex items-center">
              <StatusBadge 
                status={data.temperature_status} 
                type="temperature" 
                className="mr-2" 
              />
            </div>
          </div>
          
          <div className="bg-gray-850 p-3 rounded-md border border-gray-700">
            <div className="text-sm text-gray-400 mb-1">pH Level</div>
            <div className="flex items-center">
              <StatusBadge 
                status={data.ph_status} 
                type="ph" 
                className="mr-2" 
              />
            </div>
          </div>
          
          <div className="bg-gray-850 p-3 rounded-md border border-gray-700">
            <div className="text-sm text-gray-400 mb-1">Oxygen Level</div>
            <div className="flex items-center">
              <StatusBadge 
                status={data.oxygen_status} 
                type="oxygen" 
                className="mr-2" 
              />
            </div>
          </div>
          
          <div className="bg-gray-850 p-3 rounded-md border border-gray-700">
            <div className="text-sm text-gray-400 mb-1">Feeding Status</div>
            <div className="flex items-center">
              <StatusBadge 
                status={data.feeding_status} 
                type="feeding" 
                className="mr-2" 
              />
            </div>
          </div>
        </div>
        
        {data.recommendations && data.recommendations.length > 0 && (
          <div className="bg-gray-850 p-3 rounded-md border border-gray-700">
            <h4 className="text-sm font-semibold text-orange-300 mb-2">Recommendations</h4>
            <ul className="list-disc pl-5 space-y-1">
              {data.recommendations.map((recommendation, index) => (
                <li key={index} className="text-gray-300">{recommendation}</li>
              ))}
            </ul>
          </div>
        )}
      </div>
    </div>
  );
};
