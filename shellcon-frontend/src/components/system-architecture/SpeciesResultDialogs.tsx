
import React, { useState } from 'react';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { ApiResultDisplay } from "../ApiResultDisplay";
import { CustomDialogClose } from "./CustomDialogClose";
import { SpeciesListDisplay } from "../api-results/SpeciesListDisplay";
import { Species, FeedingSchedule } from './useSpeciesData';
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { 
  Select, 
  SelectContent, 
  SelectItem, 
  SelectTrigger, 
  SelectValue 
} from "@/components/ui/select";
import { Search, Info, Clock } from "lucide-react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

interface SpeciesResultDialogsProps {
  // Species List
  showSpeciesListDialog: boolean;
  setShowSpeciesListDialog: (show: boolean) => void;
  speciesList: Species[];
  speciesLoading: boolean;
  speciesError: string | null;
  
  // Search Results
  showSearchResultsDialog: boolean;
  setShowSearchResultsDialog: (show: boolean) => void;
  searchResults: Species[];
  searchLoading: boolean;
  searchError: string | null;
  
  // Species Details
  showSpeciesDetailsDialog: boolean;
  setShowSpeciesDetailsDialog: (show: boolean) => void;
  speciesDetails: Species | null;
  detailsLoading: boolean;
  detailsError: string | null;
  
  // Feeding Schedule
  showFeedingScheduleDialog: boolean;
  setShowFeedingScheduleDialog: (show: boolean) => void;
  feedingSchedule: FeedingSchedule | null;
  scheduleLoading: boolean;
  scheduleError: string | null;
  
  // Handlers
  onSearchSpecies: (searchTerm: string, searchType: 'name' | 'scientific_name') => Promise<void>;
  onGetSpeciesDetails: (speciesId: string) => Promise<void>;
  onGetFeedingSchedule: (speciesId: string, tankType?: string, customDiet?: string) => Promise<void>;
}

const SpeciesResultDialogs: React.FC<SpeciesResultDialogsProps> = ({
  // Species List
  showSpeciesListDialog,
  setShowSpeciesListDialog,
  speciesList,
  speciesLoading,
  speciesError,
  
  // Search Results
  showSearchResultsDialog,
  setShowSearchResultsDialog,
  searchResults,
  searchLoading,
  searchError,
  
  // Species Details
  showSpeciesDetailsDialog,
  setShowSpeciesDetailsDialog,
  speciesDetails,
  detailsLoading,
  detailsError,
  
  // Feeding Schedule
  showFeedingScheduleDialog,
  setShowFeedingScheduleDialog,
  feedingSchedule,
  scheduleLoading,
  scheduleError,
  
  // Handlers
  onSearchSpecies,
  onGetSpeciesDetails,
  onGetFeedingSchedule
}) => {
  // Search controls
  const [searchTerm, setSearchTerm] = useState<string>('');
  const [searchType, setSearchType] = useState<'name' | 'scientific_name'>('name');
  
  // Species details controls
  const [selectedSpeciesId, setSelectedSpeciesId] = useState<string>('');
  
  // Feeding schedule controls
  const [feedingSpeciesId, setFeedingSpeciesId] = useState<string>('');
  const [tankType, setTankType] = useState<string>('default');
  const [customDiet, setCustomDiet] = useState<string>('');

  // Handle search submission
  const handleSearch = () => {
    if (searchTerm.trim()) {
      onSearchSpecies(searchTerm, searchType);
    }
  };

  // Handle species details selection
  const handleSelectSpecies = () => {
    if (selectedSpeciesId) {
      onGetSpeciesDetails(selectedSpeciesId);
    }
  };

  // Handle feeding schedule selection
  const handleGetFeedingSchedule = () => {
    if (feedingSpeciesId) {
      onGetFeedingSchedule(
        feedingSpeciesId, 
        tankType !== 'default' ? tankType : undefined,
        customDiet.trim() ? customDiet : undefined
      );
    }
  };
  
  return (
    <>
      {/* Species List Dialog (Simplified) */}
      <Dialog open={showSpeciesListDialog} onOpenChange={setShowSpeciesListDialog}>
        <DialogContent className="bg-gray-900 border-2 border-orange-500/40 text-white max-w-3xl overflow-auto" onInteractOutside={e => e.preventDefault()} onEscapeKeyDown={e => e.preventDefault()} hideCloseButton>
          <DialogHeader>
            <DialogTitle className="text-orange-400">Species Database</DialogTitle>
            <CustomDialogClose />
          </DialogHeader>
          
          <div className="mt-4">
            <SpeciesListDisplay 
              data={speciesList}
            />
          </div>
        </DialogContent>
      </Dialog>

      {/* Species Details Dialog (Detailed View) */}
      <Dialog open={showSpeciesDetailsDialog} onOpenChange={setShowSpeciesDetailsDialog}>
        <DialogContent className="bg-gray-900 border-2 border-orange-500/40 text-white max-w-4xl w-[80vw]" onInteractOutside={e => e.preventDefault()} onEscapeKeyDown={e => e.preventDefault()} hideCloseButton>
          <DialogHeader>
            <DialogTitle className="text-orange-400 flex items-center">
              {speciesDetails ? (
                <>
                  <Info className="mr-2 h-5 w-5" />
                  {speciesDetails.name} Details
                </>
              ) : (
                <>Species Details</>
              )}
            </DialogTitle>
            <CustomDialogClose />
          </DialogHeader>
          
          {!speciesDetails ? (
            <div className="bg-gray-800 p-4 rounded-md border border-orange-500/20">
              <p className="text-gray-300 mb-4 italic">Loading species details...</p>
            </div>
          ) : (
            <div className="space-y-4">
              {/* Header Info - matching tank readings style */}
              <div className="bg-gray-950/50 rounded-md p-3">
                <p className="text-orange-400 font-medium">Species ID: <span className="text-white font-mono">{speciesDetails.id}</span></p>
                <div className="flex items-center justify-between mt-1">
                  <p className="text-base font-medium">{speciesDetails.name}</p>
                  <p className="text-sm italic text-gray-300">{speciesDetails.scientific_name}</p>
                </div>
              </div>
              
              {/* Main Parameters - grid layout like tank readings */}
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div className="border border-gray-700 rounded-md p-3 bg-gray-850">
                  <div className="text-sm text-gray-400 mb-1 capitalize">Temperature Range</div>
                  <div className="flex justify-between items-center">
                    <span className="font-semibold text-lg">
                      {speciesDetails.parameters?.temperature || '22°C - 28°C'}
                    </span>
                  </div>
                </div>
                
                <div className="border border-gray-700 rounded-md p-3 bg-gray-850">
                  <div className="text-sm text-gray-400 mb-1 capitalize">pH Range</div>
                  <div className="flex justify-between items-center">
                    <span className="font-semibold text-lg">
                      {speciesDetails.parameters?.ph || '7.0 - 8.2'}
                    </span>
                  </div>
                </div>
                
                <div className="border border-gray-700 rounded-md p-3 bg-gray-850">
                  <div className="text-sm text-gray-400 mb-1 capitalize">Diet Type</div>
                  <div className="flex justify-between items-center">
                    <span className="font-semibold text-lg">
                      {speciesDetails.diet_type || 'Omnivore'}
                    </span>
                  </div>
                </div>
                
                {speciesDetails.tank_size && (
                  <div className="border border-gray-700 rounded-md p-3 bg-gray-850">
                    <div className="text-sm text-gray-400 mb-1 capitalize">Tank Size</div>
                    <div className="flex justify-between items-center">
                      <span className="font-semibold text-lg">
                        {speciesDetails.tank_size}
                      </span>
                    </div>
                  </div>
                )}
                
                {speciesDetails.care_level && (
                  <div className="border border-gray-700 rounded-md p-3 bg-gray-850">
                    <div className="text-sm text-gray-400 mb-1 capitalize">Care Level</div>
                    <div className="flex justify-between items-center">
                      <span className="font-semibold text-lg">
                        {speciesDetails.care_level}
                      </span>
                    </div>
                  </div>
                )}
              </div>
              
              {/* Description Panel */}
              {speciesDetails.description && (
                <div className="mt-4 border border-gray-700 rounded-md p-4 bg-gray-850">
                  <div className="text-sm text-gray-400 mb-2 capitalize">Description</div>
                  <p className="text-sm leading-relaxed">{speciesDetails.description}</p>
                </div>
              )}
              
              {/* Additional Parameters Panel */}
              {speciesDetails.parameters && Object.keys(speciesDetails.parameters).filter(k => k !== 'temperature' && k !== 'ph').length > 0 && (
                <div className="mt-6 border-t border-gray-700 pt-4">
                  <h3 className="text-orange-400 text-sm mb-3">Additional Parameters</h3>
                  <div className="grid grid-cols-2 gap-3 text-sm">
                    {Object.entries(speciesDetails.parameters)
                      .filter(([key]) => key !== 'temperature' && key !== 'ph')
                      .map(([key, value]) => (
                        <div key={key} className="border border-gray-700 rounded p-2 bg-gray-850">
                          <span className="text-gray-400 capitalize">{key.replace('_', ' ')}:</span>{" "}
                          <span className="font-medium">{value}</span>
                        </div>
                      ))
                    }
                  </div>
                </div>
              )}
            </div>
          )}
        </DialogContent>
      </Dialog>

      {/* Feeding Schedule Dialog */}
      <Dialog open={showFeedingScheduleDialog} onOpenChange={setShowFeedingScheduleDialog}>
        <DialogContent className="bg-gray-900 border-2 border-orange-500/40 text-white max-w-3xl" onInteractOutside={e => e.preventDefault()} onEscapeKeyDown={e => e.preventDefault()} hideCloseButton>
          <DialogHeader>
            <DialogTitle className="text-orange-400">Feeding Schedule</DialogTitle>
            <CustomDialogClose />
          </DialogHeader>
          
          {!feedingSchedule && (
            <div className="space-y-4 bg-gray-800 p-4 rounded-md">
              <Select value={feedingSpeciesId} onValueChange={setFeedingSpeciesId}>
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
                      {species.id} - {species.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              
              <Select value={tankType} onValueChange={setTankType}>
                <SelectTrigger className="w-full bg-gray-800 border-orange-500/30 text-white">
                  <SelectValue placeholder="Select tank type..." />
                </SelectTrigger>
                <SelectContent className="bg-gray-800 border-orange-500/30">
                  <SelectItem value="default" className="text-white hover:bg-gray-700">Default</SelectItem>
                  <SelectItem value="reef" className="text-white hover:bg-gray-700">Reef</SelectItem>
                  <SelectItem value="nano" className="text-white hover:bg-gray-700">Nano</SelectItem>
                  <SelectItem value="community" className="text-white hover:bg-gray-700">Community</SelectItem>
                </SelectContent>
              </Select>
              
              <Input
                placeholder="Custom diet (optional)"
                value={customDiet}
                onChange={(e) => setCustomDiet(e.target.value)}
                className="w-full bg-gray-800 border-orange-500/30 text-white"
              />
              
              <Button 
                variant="outline" 
                className="w-full bg-gray-800 border-orange-500/30 text-orange-400 hover:bg-gray-700"
                onClick={handleGetFeedingSchedule}
                disabled={scheduleLoading || !feedingSpeciesId}
              >
                <Clock className="mr-2 h-4 w-4" />
                {scheduleLoading ? "Loading Schedule..." : "Get Feeding Schedule"}
              </Button>
            </div>
          )}
          
          {feedingSchedule && (
            <ApiResultDisplay 
              data={feedingSchedule} 
              loading={scheduleLoading} 
              error={scheduleError}
              type="feeding-schedule" 
            />
          )}
        </DialogContent>
      </Dialog>
    </>
  );
};

export default SpeciesResultDialogs;
