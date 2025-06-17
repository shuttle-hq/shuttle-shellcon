
import React from 'react';
import { Species } from '../system-architecture/useSpeciesData';
import { Info } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";

interface SpeciesListDisplayProps {
  data: Species[];
}

export const SpeciesListDisplay: React.FC<SpeciesListDisplayProps> = ({ data }) => {
  // No longer need parameter parsing functions as we're simplifying the display

  return (
    <div className="overflow-x-auto">
      <table className="w-full text-sm text-left text-gray-300">
        <colgroup>
          <col style={{ width: "15%" }} /> {/* ID */}
          <col style={{ width: "40%" }} /> {/* Name */}
          <col style={{ width: "45%" }} /> {/* Scientific Name */}
        </colgroup>
        <thead className="text-xs text-orange-400 uppercase bg-gray-800">
          <tr>
            <th scope="col" className="px-3 py-2">ID</th>
            <th scope="col" className="px-3 py-2">Name</th>
            <th scope="col" className="px-3 py-2">Scientific Name</th>
          </tr>
        </thead>
        <tbody>
          {data.map((species) => (
            <tr key={species.id} className="border-b border-gray-700 hover:bg-gray-800">
              <td className="px-3 py-2 font-mono">{species.id}</td>
              <td className="px-3 py-2 font-medium">{species.name}</td>
              <td className="px-3 py-2 italic">{species.scientific_name}</td>
            </tr>
          ))}
        </tbody>
      </table>
      
      <div className="mt-4 p-2 bg-gray-850 rounded text-xs text-gray-400">
        <p>Total species: {data.length}</p>
      </div>
    </div>
  );
};
