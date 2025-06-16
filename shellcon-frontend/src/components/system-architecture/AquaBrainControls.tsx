import React, { useState } from 'react';
import { Button } from "@/components/ui/button";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { ActivitySquare, BarChart } from "lucide-react";

interface AquaBrainControlsProps {
  onGetAllTankAnalysis: () => Promise<void>;
  onGetTankAnalysis: (tankId: string) => Promise<void>;
  tanksList: string[];
  summaryLoading: boolean;
  detailLoading: boolean;
}

const AquaBrainControls: React.FC<AquaBrainControlsProps> = ({
  onGetAllTankAnalysis,
  onGetTankAnalysis,
  tanksList,
  summaryLoading,
  detailLoading
}) => {
  const [showTankSelector, setShowTankSelector] = useState<boolean>(false);
  const [selectedTankId, setSelectedTankId] = useState<string>("");

  return (
    <div className="flex flex-col space-y-2">
      <Button 
        variant="outline" 
        size="sm" 
        className="bg-gray-800 border-orange-500/50 text-orange-400 hover:bg-gray-700 w-full justify-start"
        onClick={onGetAllTankAnalysis}
        disabled={summaryLoading}
      >
        <ActivitySquare className="mr-2 h-4 w-4" />
        {summaryLoading ? "Loading Analysis..." : "View All Tank Analysis"}
      </Button>
      
      <div>
        <Button 
          variant="outline" 
          size="sm" 
          className="bg-gray-800 border-orange-500/50 text-orange-400 hover:bg-gray-700 w-full justify-start"
          onClick={() => setShowTankSelector(!showTankSelector)}
        >
          <BarChart className="mr-2 h-4 w-4" />
          Tank Health Analysis
        </Button>
        
        {showTankSelector && (
          <div className="mt-2 p-2 bg-gray-850 rounded border border-gray-700">
            <Select value={selectedTankId} onValueChange={setSelectedTankId}>
              <SelectTrigger className="w-full bg-gray-800 border-orange-500/30 text-white">
                <SelectValue placeholder="Select tank..." />
              </SelectTrigger>
              <SelectContent className="bg-gray-800 border-orange-500/30 max-h-[200px]">
                {tanksList.map((tankId) => (
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
              className="w-full mt-2 bg-gray-700 border-orange-500/50 text-orange-400 hover:bg-gray-600 justify-center"
              onClick={() => {
                if (selectedTankId) {
                  onGetTankAnalysis(selectedTankId);
                  setShowTankSelector(false);
                }
              }}
              disabled={detailLoading || !selectedTankId}
            >
              View Analysis
            </Button>
          </div>
        )}
      </div>
    </div>
  );
};

export default AquaBrainControls;
