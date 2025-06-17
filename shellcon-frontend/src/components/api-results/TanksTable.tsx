
import React from 'react';
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from "@/components/ui/table";

interface TanksTableProps {
  tanksArray: any[];
}

export const TanksTable: React.FC<TanksTableProps> = ({ tanksArray }) => {
  return (
    <div className="space-y-4">
      {tanksArray && tanksArray.length > 0 ? (
        <>
          <div className="flex flex-wrap justify-center gap-2">
            {tanksArray.map((tankId: string, index: number) => (
              <div 
                key={index} 
                className="bg-gray-800 hover:bg-gray-700 border border-orange-500/30 rounded-md px-3 py-1.5 
                         text-white font-mono text-sm transition-colors flex items-center"
              >
                <div className="h-2 w-2 bg-green-500 rounded-full mr-2"></div>
                {tankId}
              </div>
            ))}
          </div>
          <div className="flex items-center justify-between text-xs gap-x-4">
            <div className="text-gray-400 bg-gray-850 px-2 py-1 rounded flex-shrink-0 whitespace-nowrap">
              <span className="font-medium text-orange-400 mr-1">{tanksArray.length}</span>active tanks
            </div>
            <div className="text-gray-500 italic">Click 'Fetch Tank Readings' and select a tank to view readings</div>
          </div>
        </>
      ) : (
        <div className="text-center text-gray-400 py-6 bg-gray-850/50 rounded-md border border-gray-800">
          No tanks found
        </div>
      )}
    </div>
  );
};
