import React from 'react';
import { TankAnalysisSummary } from '../system-architecture/useAquaBrainData';
import { StatusBadge } from '@/components/ui/status-badge';

interface TankAnalysisSummaryDisplayProps {
  data: TankAnalysisSummary[];
}

export const TankAnalysisSummaryDisplay: React.FC<TankAnalysisSummaryDisplayProps> = ({ data }) => {
  // Check if the data is in the expected format
  if (!data || !Array.isArray(data) || data.length === 0) {
    return (
      <div className="p-4 bg-gray-800 border border-orange-500/30 rounded-md">
        <h3 className="text-lg font-bold text-orange-400 mb-2">Tank Analysis</h3>
        <p className="text-gray-300">No tank analysis data available.</p>
      </div>
    );
  }

  // No need for a helper function as we're using the StatusBadge component

  return (
    <div className="space-y-4">
      <div className="bg-gray-800 p-4 rounded-md">
        <h3 className="text-lg font-bold text-orange-400 mb-4">Tank Health Summary</h3>
        
        <div className="overflow-x-auto">
          <table className="w-full text-sm text-left text-gray-300">
            <thead className="text-xs text-orange-400 uppercase bg-gray-900">
              <tr>
                <th scope="col" className="px-4 py-2">Tank ID</th>
                <th scope="col" className="px-4 py-2">Species</th>
                <th scope="col" className="px-4 py-2">Health Status</th>
                <th scope="col" className="px-4 py-2">Last Updated</th>
              </tr>
            </thead>
            <tbody>
              {data.map((tank, index) => (
                <tr key={index} className="border-b border-gray-700 hover:bg-gray-700">
                  <td className="px-4 py-2 font-medium">{tank.tank_id}</td>
                  <td className="px-4 py-2">{tank.species_name || `Species #${tank.species_id}`}</td>
                  <td className="px-4 py-2">
                    <StatusBadge status={tank.overall_health} type="overall_health" />
                  </td>
                  <td className="px-4 py-2 text-gray-400">
                    {new Date(tank.timestamp).toLocaleString()}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        
        <div className="mt-4 p-3 bg-gray-850 rounded">
          <h4 className="text-sm font-semibold text-orange-300 mb-2">Health Status Legend</h4>
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-2">
            <div className="flex items-center">
              <StatusBadge status="good" className="mr-2" />
              <span className="text-sm text-gray-300">Optimal conditions</span>
            </div>
            <div className="flex items-center">
              <StatusBadge status="caution" className="mr-2" />
              <span className="text-sm text-gray-300">Needs attention</span>
            </div>
            <div className="flex items-center">
              <StatusBadge status="at_risk" className="mr-2" />
              <span className="text-sm text-gray-300">Immediate action required</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
