
import React, { useEffect } from 'react';
import { TanksTable } from './api-results/TanksTable';
import { ReadingsDisplay } from './api-results/ReadingsDisplay';
import { SensorsDisplay } from './api-results/SensorsDisplay';
import { SpeciesListDisplay } from './api-results/SpeciesListDisplay';
import { SpeciesDetailsDisplay } from './api-results/SpeciesDetailsDisplay';
import { FeedingScheduleDisplay } from './api-results/FeedingScheduleDisplay';
import { TankAnalysisSummaryDisplay } from './api-results/TankAnalysisSummaryDisplay';
import { TankAnalysisDetailDisplay } from './api-results/TankAnalysisDetailDisplay';
import { 
  LoadingState, 
  ErrorState, 
  NoDataState, 
  JsonView 
} from './api-results/CommonComponents';
import { processTanksData } from './utils/displayHelpers';

interface ApiResultDisplayProps {
  data: any;
  loading: boolean;
  error: string | null;
  type: 'tanks' | 'readings' | 'sensors' | 'species-list' | 'species-search' | 'species-details' | 'feeding-schedule' | 'tank-analysis-summary' | 'tank-analysis-detail';
}

export const ApiResultDisplay: React.FC<ApiResultDisplayProps> = ({ 
  data, 
  loading, 
  error, 
  type 
}) => {
  // Add debugging on component mount and when data changes
  useEffect(() => {
    console.log(`ApiResultDisplay received data for ${type}:`, data);
  }, [data, type]);

  if (loading) {
    return <LoadingState />;
  }

  if (error) {
    return <ErrorState error={error} />;
  }

  if (!data) {
    return <NoDataState />;
  }

  // Format each data type differently
  switch (type) {
    case 'tanks': {
      const tanksArray = processTanksData(data);
      return <TanksTable tanksArray={tanksArray} />;
    }
      
    case 'readings':
      return <ReadingsDisplay data={data} />;
      
    case 'sensors':
      return <SensorsDisplay data={data} />;
      
    case 'species-list':
    case 'species-search':
      return <SpeciesListDisplay data={Array.isArray(data) ? data : [data]} />;
      
    case 'species-details':
      return <SpeciesDetailsDisplay data={data} />;
      
    case 'feeding-schedule':
      return <FeedingScheduleDisplay data={data} />;
      
    case 'tank-analysis-summary':
      return <TankAnalysisSummaryDisplay data={data} />;
      
    case 'tank-analysis-detail':
      return <TankAnalysisDetailDisplay data={data} />;
      
    default:
      return <JsonView data={data} />;
  }
};
