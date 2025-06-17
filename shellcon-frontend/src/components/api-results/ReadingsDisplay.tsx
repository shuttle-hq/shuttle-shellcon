
import React, { useState } from 'react';
import { Badge } from "@/components/ui/badge";
import { ChevronDown, ChevronUp, Clock } from "lucide-react";
import { Button } from "@/components/ui/button";

interface ReadingsDisplayProps {
  data: any;
}

// Helper function to format the date nicely
const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: true
  });
};

export const ReadingsDisplay: React.FC<ReadingsDisplayProps> = ({ data }) => {
  const [showPreviousReadings, setShowPreviousReadings] = useState(false);
  
  // Extract the most recent reading if data is an array
  const latestReading = Array.isArray(data) && data.length > 0 ? data[0] : null;
  const previousReadings = Array.isArray(data) && data.length > 1 ? data.slice(1) : [];
  
  return (
    <div className="space-y-4">
      {latestReading && (
        <div className="bg-gray-950/50 rounded-md p-3">
          <p className="text-orange-400 font-medium">Tank ID: <span className="text-white font-mono">{latestReading.tank_id}</span></p>
          <p className="text-orange-400 text-sm mt-1 flex items-center">
            <Clock className="h-3 w-3 mr-1" /> 
            Last Updated: <span className="text-white ml-1">{formatDate(latestReading.timestamp)}</span>
          </p>
        </div>
      )}
      
      {latestReading ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div className="border border-gray-700 rounded-md p-3 bg-gray-850">
            <div className="text-sm text-gray-400 mb-1 capitalize">Temperature</div>
            <div className="flex justify-between items-center">
              <span className="font-semibold text-lg">
                {latestReading.temperature} 
                <span className="text-xs text-gray-400 ml-1">°C</span>
              </span>
            </div>
          </div>
          
          <div className="border border-gray-700 rounded-md p-3 bg-gray-850">
            <div className="text-sm text-gray-400 mb-1 capitalize">pH</div>
            <div className="flex justify-between items-center">
              <span className="font-semibold text-lg">
                {latestReading.ph} 
              </span>
            </div>
          </div>
          
          <div className="border border-gray-700 rounded-md p-3 bg-gray-850">
            <div className="text-sm text-gray-400 mb-1 capitalize">Oxygen Level</div>
            <div className="flex justify-between items-center">
              <span className="font-semibold text-lg">
                {latestReading.oxygen_level} 
                <span className="text-xs text-gray-400 ml-1">mg/L</span>
              </span>
            </div>
          </div>
          
          <div className="border border-gray-700 rounded-md p-3 bg-gray-850">
            <div className="text-sm text-gray-400 mb-1 capitalize">Salinity</div>
            <div className="flex justify-between items-center">
              <span className="font-semibold text-lg">
                {latestReading.salinity} 
                <span className="text-xs text-gray-400 ml-1">ppt</span>
              </span>
            </div>
          </div>
        </div>
      ) : (
        <div className="border border-gray-700 rounded-md p-4 text-center text-gray-400">
          No readings available
        </div>
      )}
      
      {Array.isArray(data) && data.length > 1 && (
        <div className="mt-6">
          <Button 
            variant="outline" 
            size="sm"
            onClick={() => setShowPreviousReadings(!showPreviousReadings)}
            className="w-full flex items-center justify-center py-2 border-orange-500/30 bg-gray-850 text-orange-400 hover:bg-gray-800"
          >
            {showPreviousReadings ? (
              <>
                <ChevronUp className="mr-2 h-4 w-4" />
                Hide Previous Readings
              </>
            ) : (
              <>
                <ChevronDown className="mr-2 h-4 w-4" />
                View Previous Readings ({previousReadings.length})
              </>
            )}
          </Button>
          
          {showPreviousReadings && (
            <div className="mt-4 space-y-4 max-h-[300px] overflow-y-auto p-2 bg-gray-900 rounded">
              {previousReadings.map((reading, index) => (
                <div key={index} className="border-t border-gray-700 pt-3 first:border-t-0 first:pt-0">
                  <div className="text-xs text-orange-400 mb-2 flex items-center">
                    <Clock className="h-3 w-3 mr-1" />
                    {formatDate(reading.timestamp)}
                  </div>
                  <div className="grid grid-cols-2 gap-3 text-sm">
                    <div>
                      <span className="text-gray-400">Temperature:</span>{" "}
                      <span className="font-medium">{reading.temperature}°C</span>
                    </div>
                    <div>
                      <span className="text-gray-400">pH:</span>{" "}
                      <span className="font-medium">{reading.ph}</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Oxygen:</span>{" "}
                      <span className="font-medium">{reading.oxygen_level} mg/L</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Salinity:</span>{" "}
                      <span className="font-medium">{reading.salinity} ppt</span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
};
