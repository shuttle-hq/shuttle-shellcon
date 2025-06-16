
import React, { useState, useEffect } from 'react';
import { Button } from "@/components/ui/button";
import { Species } from './useSpeciesData';
import { List, Info, Clock } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";

interface SpeciesHubControlsProps {
  onFetchAllSpecies: () => Promise<void>;
  onGetSpeciesDetails: (speciesId: string) => Promise<void>;
  onGetFeedingSchedule: (speciesId: string, tankType?: string, customDiet?: string) => Promise<void>;
  setShowFeedingScheduleDialog: (show: boolean) => void;
  loadSpeciesForDropdown: () => Promise<Species[]>;
  speciesList: Species[];
  speciesLoading: boolean;
  detailsLoading: boolean;
  scheduleLoading: boolean;
}

const SpeciesHubControls: React.FC<SpeciesHubControlsProps> = ({ 
  onFetchAllSpecies,
  onGetSpeciesDetails,
  onGetFeedingSchedule,
  setShowFeedingScheduleDialog,
  loadSpeciesForDropdown,
  speciesList,
  speciesLoading,
  detailsLoading,
  scheduleLoading
}) => {
  const [selectedSpeciesId, setSelectedSpeciesId] = useState<string>("");
  const [showSpeciesSelector, setShowSpeciesSelector] = useState<boolean>(false);
  const [showFeedingSelector, setShowFeedingSelector] = useState<boolean>(false);
  const [selectedFeedingSpeciesId, setSelectedFeedingSpeciesId] = useState<string>("");
  
  useEffect(() => {
    if ((showSpeciesSelector || showFeedingSelector) && speciesList.length === 0) {
      loadSpeciesForDropdown();
    }
  }, [showSpeciesSelector, showFeedingSelector, speciesList.length, loadSpeciesForDropdown]);
  return (
    <div className="space-y-2 mt-4">
      {/* List All Species */}
      <Button 
        variant="outline" 
        size="sm"
        className="w-full bg-gray-800 border-orange-500/50 text-orange-400 hover:bg-gray-700 justify-start"
        onClick={onFetchAllSpecies}
        disabled={speciesLoading}
      >
        <List className="mr-2 h-4 w-4" />
        {speciesLoading ? "Loading..." : "List All Species"}
      </Button>

      {/* Get Species Details */}
      <div className="space-y-2">
        <Button 
          variant="outline" 
          size="sm"
          className="w-full bg-gray-800 border-orange-500/50 text-orange-400 hover:bg-gray-700 justify-start"
          onClick={() => setShowSpeciesSelector(!showSpeciesSelector)}
          disabled={detailsLoading}
        >
          <Info className="mr-2 h-4 w-4" />
          {detailsLoading ? "Loading Details..." : "Get Species Details"}
        </Button>

        {/* Species Selector Dropdown */}
        {showSpeciesSelector && (
          <div className="pt-2 space-y-2 bg-gray-800 p-3 rounded-md border border-orange-500/30">
            <Select value={selectedSpeciesId} onValueChange={setSelectedSpeciesId}>
              <SelectTrigger className="w-full bg-gray-800 border-orange-500/30 text-white">
                <SelectValue placeholder="Select species..." />
              </SelectTrigger>
              <SelectContent className="bg-gray-800 border-orange-500/30 max-h-[200px]">
                {speciesList.map(species => (
                  <SelectItem
                    key={species.id}
                    value={species.id}
                    className="text-white hover:bg-gray-700"
                  >
                    {species.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            
            <Button
              variant="outline"
              size="sm"
              className="w-full bg-gray-700 border-orange-500/50 text-orange-400 hover:bg-gray-600 justify-center"
              onClick={() => {
                if (selectedSpeciesId) {
                  onGetSpeciesDetails(selectedSpeciesId);
                }
              }}
              disabled={detailsLoading || !selectedSpeciesId}
            >
              View Details
            </Button>
          </div>
        )}
      </div>

      {/* Get Feeding Schedule */}
      <div className="space-y-2">
        <Button 
          variant="outline" 
          size="sm"
          className="w-full bg-gray-800 border-orange-500/50 text-orange-400 hover:bg-gray-700 justify-start"
          onClick={() => setShowFeedingSelector(!showFeedingSelector)}
          disabled={scheduleLoading}
        >
          <Clock className="mr-2 h-4 w-4" />
          {scheduleLoading ? "Loading Schedule..." : "Get Feeding Schedule"}
        </Button>
        {/* Feeding Schedule Selector Dropdown */}
        {showFeedingSelector && (
          <div className="pt-2 space-y-2 bg-gray-800 p-3 rounded-md border border-orange-500/30">
            <Select value={selectedFeedingSpeciesId} onValueChange={setSelectedFeedingSpeciesId}>
              <SelectTrigger className="w-full bg-gray-800 border-orange-500/30 text-white">
                <SelectValue placeholder="Select species..." />
              </SelectTrigger>
              <SelectContent className="bg-gray-800 border-orange-500/30 max-h-[200px]">
                {speciesList.map(species => (
                  <SelectItem
                    key={species.id}
                    value={species.id}
                    className="text-white hover:bg-gray-700"
                  >
                    {species.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Button
              variant="outline"
              size="sm"
              className="w-full bg-gray-700 border-orange-500/50 text-orange-400 hover:bg-gray-600 justify-center"
              onClick={async () => {
                if (selectedFeedingSpeciesId) {
                  await onGetFeedingSchedule(selectedFeedingSpeciesId);
                  setShowFeedingScheduleDialog(true);
                }
              }}
              disabled={scheduleLoading || !selectedFeedingSpeciesId}
            >
              View Feeding Schedule
            </Button>
          </div>
        )}
      </div>
    </div>
  );
};

export default SpeciesHubControls;
