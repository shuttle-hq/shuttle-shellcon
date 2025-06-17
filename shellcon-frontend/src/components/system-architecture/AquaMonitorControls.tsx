
import React, { useState, useEffect } from 'react';
import { Button } from "@/components/ui/button";
import { Eye, Search, ThermometerSnowflake } from "lucide-react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { toast } from "@/components/ui/sonner";

interface AquaMonitorControlsProps {
  onViewAllTanks: () => void;
  onFetchTankReadings: (tankId: string) => void;
  onCheckSensorStatus: () => void;
  tanksList: any[];
  tanksLoading: boolean;
  readingsLoading: boolean;
  sensorLoading: boolean;
}

const AquaMonitorControls: React.FC<AquaMonitorControlsProps> = ({
  onViewAllTanks,
  onFetchTankReadings,
  onCheckSensorStatus,
  tanksList,
  tanksLoading,
  readingsLoading,
  sensorLoading
}) => {
  const [selectedTankId, setSelectedTankId] = useState<string>("");
  const [showTankSelector, setShowTankSelector] = useState<boolean>(false);
  return (
    <div className="flex flex-col space-y-2">
      <Button 
        variant="outline" 
        size="sm" 
        className="bg-gray-800 border-orange-500/50 text-orange-400 hover:bg-gray-700 w-full justify-start"
        onClick={onViewAllTanks}
        disabled={tanksLoading}
      >
        <Eye className="mr-2 h-4 w-4" />
        View All Tanks
      </Button>
      
      <div className="space-y-2">
        <Button 
          variant="outline" 
          size="sm" 
          className="bg-gray-800 border-orange-500/50 text-orange-400 hover:bg-gray-700 w-full justify-start"
          onClick={() => setShowTankSelector(!showTankSelector)}
          disabled={readingsLoading}
        >
          <Search className="mr-2 h-4 w-4" />
          Fetch Tank Readings
        </Button>

        {/* Tank Selector Dropdown */}
        {showTankSelector && (
          <div className="pt-2 space-y-2 bg-gray-800 p-3 rounded-md border border-orange-500/30">
            <Select value={selectedTankId} onValueChange={setSelectedTankId}>
              <SelectTrigger className="w-full bg-gray-800 border-orange-500/30 text-white">
                <SelectValue placeholder="Select tank..." />
              </SelectTrigger>
              <SelectContent className="bg-gray-800 border-orange-500/30 max-h-[200px]">
                {tanksList.map(tankId => (
                  <SelectItem
                    key={tankId}
                    value={tankId}
                    className="text-white hover:bg-gray-700"
                  >
                    {tankId}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            
            <Button
              variant="outline"
              size="sm"
              className="w-full bg-gray-700 border-orange-500/50 text-orange-400 hover:bg-gray-600 justify-center"
              onClick={() => {
                if (selectedTankId) {
                  onFetchTankReadings(selectedTankId);
                  setShowTankSelector(false);
                }
              }}
              disabled={readingsLoading || !selectedTankId}
            >
              View Readings
            </Button>
          </div>
        )}
      </div>
      
      <Button 
        variant="outline" 
        size="sm" 
        className="bg-gray-800 border-orange-500/50 text-orange-400 hover:bg-gray-700 w-full justify-start"
        onClick={onCheckSensorStatus}
        disabled={sensorLoading}
      >
        <ThermometerSnowflake className="mr-2 h-4 w-4" />
        Check Sensor Status
      </Button>
    </div>
  );
};

export default AquaMonitorControls;
