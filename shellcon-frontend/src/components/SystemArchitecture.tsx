
import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Server, Database, Monitor, CircuitBoard } from "lucide-react";
import ServiceCard from './system-architecture/ServiceCard';
import AquaMonitorControls from './system-architecture/AquaMonitorControls';
import SpeciesHubControls from './system-architecture/SpeciesHubControls';
import AquaBrainControls from './system-architecture/AquaBrainControls';
import ResultDialogs from './system-architecture/ResultDialogs';
import SpeciesResultDialogs from './system-architecture/SpeciesResultDialogs';
import AquaBrainResultDialogs from './system-architecture/AquaBrainResultDialogs';
import { useAquaData } from './system-architecture/useAquaData';
import { useSpeciesData } from './system-architecture/useSpeciesData';
import { useAquaBrainData } from './system-architecture/useAquaBrainData';

const SystemArchitecture: React.FC = () => {
  // Extract all data and handlers from custom hooks
  const {
    tanksData,
    tanksLoading,
    tanksError,
    tankReadings,
    readingsLoading,
    readingsError,
    sensorStatus,
    sensorLoading,
    sensorError,
    tanksList,
    showTanksDialog,
    setShowTanksDialog,
    showReadingsDialog,
    setShowReadingsDialog,
    showSensorDialog,
    setShowSensorDialog,
    handleViewAllTanks,
    handleFetchTankReadings,
    handleCheckSensorStatus
  } = useAquaData();

  // Extract species hub data and handlers
  const {
    speciesList,
    speciesLoading,
    speciesError,
    searchResults,
    searchLoading,
    searchError,
    speciesDetails,
    detailsLoading,
    detailsError,
    feedingSchedule,
    scheduleLoading,
    scheduleError,
    showSpeciesListDialog,
    setShowSpeciesListDialog,
    showSearchResultsDialog,
    setShowSearchResultsDialog,
    showSpeciesDetailsDialog,
    setShowSpeciesDetailsDialog,
    showFeedingScheduleDialog,
    setShowFeedingScheduleDialog,
    handleFetchAllSpecies,
    handleSearchSpecies,
    handleGetSpeciesDetails,
    handleGetFeedingSchedule,
    loadSpeciesForDropdown
  } = useSpeciesData();
  
  // Extract aqua-brain data and handlers
  const {
    tankAnalysisSummary,
    summaryLoading,
    summaryError,
    tankAnalysisDetail,
    detailLoading,
    detailError,
    showAnalysisSummaryDialog,
    setShowAnalysisSummaryDialog,
    showAnalysisDetailDialog,
    setShowAnalysisDetailDialog,
    handleGetAllTankAnalysis,
    handleGetTankAnalysis
  } = useAquaBrainData();

  return (
    <>
      <Card className="mb-8 shadow-xl border-orange-500/50 overflow-hidden">
        <CardHeader className="bg-gradient-to-r from-gray-900 via-gray-800 to-gray-900 text-white border-b border-orange-500/30 pb-4">
          <div className="flex justify-between items-center">
            <CardTitle className="flex items-center gap-2">
              <CircuitBoard className="h-5 w-5 text-orange-400 flex-shrink-0" />
              <span 
                className="font-bold text-transparent bg-gradient-to-r from-orange-500 via-orange-400 to-amber-400 bg-clip-text inline-block"
              >
                ShellCon Smart Aquarium System Control Panel
              </span>
            </CardTitle>
          </div>
        </CardHeader>
        <CardContent className="pt-6 bg-gradient-to-b from-gray-900/80 to-black">
          <div className="grid md:grid-cols-3 gap-4">
            {/* Aqua Monitor Service Card */}
            <ServiceCard 
              title="aqua-monitor" 
              description="Real-time environmental monitoring service that collects sensor data from tanks including temperature, pH, oxygen levels, and salinity."
              icon={Monitor}
            >
              <AquaMonitorControls
                onViewAllTanks={handleViewAllTanks}
                onFetchTankReadings={handleFetchTankReadings}
                onCheckSensorStatus={handleCheckSensorStatus}
                tanksList={tanksList}
                tanksLoading={tanksLoading}
                readingsLoading={readingsLoading}
                sensorLoading={sensorLoading}
              />
            </ServiceCard>
            
            {/* Species Hub Service Card */}
            <ServiceCard 
              title="species-hub" 
              description="Species and feeding information service that manages the database of crustacean species, their optimal environmental parameters, and feeding schedules."
              icon={Database}
            >
              <SpeciesHubControls
                onFetchAllSpecies={handleFetchAllSpecies}
                onGetSpeciesDetails={handleGetSpeciesDetails}
                onGetFeedingSchedule={handleGetFeedingSchedule}
                setShowFeedingScheduleDialog={setShowFeedingScheduleDialog}
                loadSpeciesForDropdown={loadSpeciesForDropdown}
                speciesList={speciesList}
                speciesLoading={speciesLoading}
                detailsLoading={detailsLoading}
                scheduleLoading={scheduleLoading}
              />
            </ServiceCard>
            
            {/* Aqua Brain Service Card */}
            <ServiceCard 
              title="aqua-brain" 
              description="Analysis and challenge tracking service that performs system-wide analysis, detecting patterns, making predictions, and coordinating responses."
              icon={Server}
            >
              <AquaBrainControls
                onGetAllTankAnalysis={handleGetAllTankAnalysis}
                onGetTankAnalysis={handleGetTankAnalysis}
                tanksList={tanksList}
                summaryLoading={summaryLoading}
                detailLoading={detailLoading}
              />
            </ServiceCard>
          </div>
        </CardContent>
      </Card>

      {/* Dialogs for various API Results */}
      <ResultDialogs
        showTanksDialog={showTanksDialog}
        setShowTanksDialog={setShowTanksDialog}
        tanksData={tanksData}
        tanksLoading={tanksLoading}
        tanksError={tanksError}
        
        showReadingsDialog={showReadingsDialog}
        setShowReadingsDialog={setShowReadingsDialog}
        tankReadings={tankReadings}
        readingsLoading={readingsLoading}
        readingsError={readingsError}
        
        showSensorDialog={showSensorDialog}
        setShowSensorDialog={setShowSensorDialog}
        sensorStatus={sensorStatus}
        sensorLoading={sensorLoading}
        sensorError={sensorError}
      />

      {/* Dialogs for Species Hub API Results */}
      <SpeciesResultDialogs
        showSpeciesListDialog={showSpeciesListDialog}
        setShowSpeciesListDialog={setShowSpeciesListDialog}
        speciesList={speciesList}
        speciesLoading={speciesLoading}
        speciesError={speciesError}
        
        showSearchResultsDialog={showSearchResultsDialog}
        setShowSearchResultsDialog={setShowSearchResultsDialog}
        searchResults={searchResults}
        searchLoading={searchLoading}
        searchError={searchError}
        
        showSpeciesDetailsDialog={showSpeciesDetailsDialog}
        setShowSpeciesDetailsDialog={setShowSpeciesDetailsDialog}
        speciesDetails={speciesDetails}
        detailsLoading={detailsLoading}
        detailsError={detailsError}
        
        showFeedingScheduleDialog={showFeedingScheduleDialog}
        setShowFeedingScheduleDialog={setShowFeedingScheduleDialog}
        feedingSchedule={feedingSchedule}
        scheduleLoading={scheduleLoading}
        scheduleError={scheduleError}
        
        onSearchSpecies={handleSearchSpecies}
        onGetSpeciesDetails={handleGetSpeciesDetails}
        onGetFeedingSchedule={handleGetFeedingSchedule}
      />
      
      {/* Dialogs for Aqua Brain API Results */}
      <AquaBrainResultDialogs
        showAnalysisSummaryDialog={showAnalysisSummaryDialog}
        setShowAnalysisSummaryDialog={setShowAnalysisSummaryDialog}
        tankAnalysisSummary={tankAnalysisSummary}
        summaryLoading={summaryLoading}
        summaryError={summaryError}
        
        showAnalysisDetailDialog={showAnalysisDetailDialog}
        setShowAnalysisDetailDialog={setShowAnalysisDetailDialog}
        tankAnalysisDetail={tankAnalysisDetail}
        detailLoading={detailLoading}
        detailError={detailError}
        
        onGetTankAnalysis={handleGetTankAnalysis}
      />
    </>
  );
};

export default SystemArchitecture;
