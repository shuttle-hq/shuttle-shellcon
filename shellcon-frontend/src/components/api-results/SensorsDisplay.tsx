
import React from 'react';
import { StatusBadge } from "@/components/ui/status-badge";
import { Clock, Activity, Server } from "lucide-react";

interface SensorsDisplayProps {
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

export const SensorsDisplay: React.FC<SensorsDisplayProps> = ({ data }) => {
  // Check if we have valid data
  if (!data) {
    return (
      <div className="text-center p-4 text-gray-400">
        No sensor data available
      </div>
    );
  }
  
  return (
    <div className="space-y-6">
      {/* Status Card */}
      <div className="bg-gray-950/50 rounded-md p-4 border border-gray-800">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-medium text-orange-400 flex items-center">
            <Server className="mr-2 h-5 w-5" />
            System Sensor Status
          </h3>
          <StatusBadge 
            status={data.status || 'unknown'} 
            className="px-3 py-1 text-sm"
          />
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {/* Active Sensors */}
          <div className="bg-gray-850 rounded-md p-3 border border-gray-700">
            <div className="flex items-center justify-between">
              <span className="text-gray-400 text-sm">Active Sensors</span>
              <span className="text-2xl font-bold text-orange-300">{data.active_sensors}</span>
            </div>
            <div className="mt-2 h-2 w-full bg-gray-700 rounded-full overflow-hidden">
              <div 
                className="h-2 bg-orange-500" 
                style={{width: data.active_sensors === 24 ? '100%' : `${(data.active_sensors / 24) * 100}%`}}
              />
            </div>
          </div>
          
          {/* Last Updated */}
          <div className="bg-gray-850 rounded-md p-3 border border-gray-700">
            <div className="text-gray-400 text-sm mb-1 flex items-center">
              <Clock className="h-3 w-3 mr-1" /> 
              Last Updated
            </div>
            <div className="text-white">
              {data.last_updated ? formatDate(data.last_updated) : "Unknown"}
            </div>
          </div>
        </div>
      </div>
      
      {/* System Activity */}
      <div className="bg-gray-950/50 rounded-md p-4 border border-gray-800">
        <h3 className="text-md font-medium text-orange-400 flex items-center mb-3">
          <Activity className="mr-2 h-4 w-4" />
          System Activity
        </h3>
        <div className="p-3 bg-gray-850 rounded-md border border-gray-700">
          <p className="text-sm">
            {data.status === 'online' && (
              <span className="text-green-400">All sensor systems are operating normally. {data.active_sensors} sensors are actively reporting data.</span>
            )}
            {data.status === 'degraded' && (
              <span className="text-amber-400">Sensor system is operating with reduced capacity. Only {data.active_sensors} sensors are reporting data.</span>
            )}
            {data.status === 'offline' && (
              <span className="text-red-400">Sensor system is currently offline. No data is being reported.</span>
            )}
            {!data.status && (
              <span className="text-gray-400">Sensor status information unavailable.</span>
            )}
          </p>
        </div>
      </div>
    </div>
  );
};
