
import React from 'react';
import { Species } from '../system-architecture/useSpeciesData';

interface SpeciesDetailsDisplayProps {
  data: Species;
}

export const SpeciesDetailsDisplay: React.FC<SpeciesDetailsDisplayProps> = ({ data }) => {
  const params = data.parameters || {};
  
  return (
    <div className="space-y-4">
      <div className="bg-gray-800 p-4 rounded-md">
        <h3 className="text-lg font-bold text-orange-400 mb-2">{data.name}</h3>
        <p className="text-sm text-gray-300 italic mb-4">{data.scientific_name}</p>
        
        {data.description && (
          <div className="mb-4">
            <h4 className="text-sm font-semibold text-orange-300 mb-1">Description</h4>
            <p className="text-sm text-gray-300">{data.description}</p>
          </div>
        )}
        
        <h4 className="text-sm font-semibold text-orange-300 mb-2">Parameters</h4>
        <div className="grid grid-cols-2 gap-2">
          {Object.entries(params).map(([key, value]) => (
            <div key={key} className="bg-gray-900 p-2 rounded">
              <span className="text-xs text-gray-400">{key}: </span>
              <span className="text-sm text-white">{value}</span>
            </div>
          ))}
        </div>
        
        {data.habitat && (
          <div className="mt-4">
            <h4 className="text-sm font-semibold text-orange-300 mb-1">Habitat</h4>
            <p className="text-sm text-gray-300">{data.habitat}</p>
          </div>
        )}
        
        {data.behavior && (
          <div className="mt-4">
            <h4 className="text-sm font-semibold text-orange-300 mb-1">Behavior</h4>
            <p className="text-sm text-gray-300">{data.behavior}</p>
          </div>
        )}
      </div>
    </div>
  );
};
