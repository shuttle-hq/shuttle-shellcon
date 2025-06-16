
import { useState, useCallback } from 'react';
import { 
  getAllSpecies, 
  searchSpeciesByName,
  searchSpeciesByScientificName,
  getSpeciesDetails,
  getSpeciesFeedingSchedule
} from '@/api/aquariumApi';
import { toast } from "sonner";

export interface Species {
  id: string;
  name: string;
  scientific_name: string;
  description?: string;
  parameters?: {
    temperature?: string;
    ph?: string;
    salinity?: string;
    [key: string]: any;
  };
  [key: string]: any;
}

export interface FeedingSchedule {
  species_id: string;
  species_name: string;
  schedule: {
    time: string;
    food_type: string;
    amount: string;
    notes?: string;
  }[];
  recommendations?: string[];
}

export const useSpeciesData = () => {
  // Species Data
  const [speciesList, setSpeciesList] = useState<Species[]>([]);
  const [speciesLoading, setSpeciesLoading] = useState<boolean>(false);
  const [speciesError, setSpeciesError] = useState<string | null>(null);
  
  // Search Results
  const [searchResults, setSearchResults] = useState<Species[]>([]);
  const [searchLoading, setSearchLoading] = useState<boolean>(false);
  const [searchError, setSearchError] = useState<string | null>(null);
  
  // Species Details
  const [speciesDetails, setSpeciesDetails] = useState<Species | null>(null);
  const [detailsLoading, setDetailsLoading] = useState<boolean>(false);
  const [detailsError, setDetailsError] = useState<string | null>(null);
  
  // Feeding Schedule
  const [feedingSchedule, setFeedingSchedule] = useState<FeedingSchedule | null>(null);
  const [scheduleLoading, setScheduleLoading] = useState<boolean>(false);
  const [scheduleError, setScheduleError] = useState<string | null>(null);
  
  // Dialog States
  const [showSpeciesListDialog, setShowSpeciesListDialog] = useState<boolean>(false);
  const [showSearchResultsDialog, setShowSearchResultsDialog] = useState<boolean>(false);
  const [showSpeciesDetailsDialog, setShowSpeciesDetailsDialog] = useState<boolean>(false);
  const [showFeedingScheduleDialog, setShowFeedingScheduleDialog] = useState<boolean>(false);

  // Fetch all species
  const handleFetchAllSpecies = useCallback(async () => {
    try {
      setSpeciesLoading(true);
      setSpeciesError(null);
      const data = await getAllSpecies();
      setSpeciesList(data.species || data);
      setShowSpeciesListDialog(true);
    } catch (error) {
      console.error("Failed to fetch species:", error);
      setSpeciesError("Failed to fetch species list");
      toast.error("Failed to fetch species list");
    } finally {
      setSpeciesLoading(false);
    }
  }, []);

  // Search species by name or scientific name
  const handleSearchSpecies = useCallback(async (searchTerm: string, searchType: 'name' | 'scientific_name') => {
    try {
      setSearchLoading(true);
      setSearchError(null);
      
      const searchFunction = searchType === 'name' ? searchSpeciesByName : searchSpeciesByScientificName;
      const data = await searchFunction(searchTerm);
      
      setSearchResults(data.species || data);
      setShowSearchResultsDialog(true);
    } catch (error) {
      console.error("Failed to search species:", error);
      setSearchError(`Failed to search species by ${searchType}`);
      toast.error(`Failed to search species by ${searchType}`);
    } finally {
      setSearchLoading(false);
    }
  }, []);

  // Get species details
  const handleGetSpeciesDetails = useCallback(async (speciesId: string) => {
    try {
      setDetailsLoading(true);
      setDetailsError(null);
      
      const data = await getSpeciesDetails(speciesId);
      setSpeciesDetails(data);
      setShowSpeciesDetailsDialog(true);
    } catch (error) {
      console.error("Failed to fetch species details:", error);
      setDetailsError("Failed to fetch species details");
      toast.error("Failed to fetch species details");
    } finally {
      setDetailsLoading(false);
    }
  }, []);

  // Get feeding schedule
  const handleGetFeedingSchedule = useCallback(async (speciesId: string, tankType?: string, customDiet?: string) => {
    try {
      setScheduleLoading(true);
      setScheduleError(null);
      
      const data = await getSpeciesFeedingSchedule(speciesId, tankType, customDiet);
      setFeedingSchedule(data);
      setShowFeedingScheduleDialog(true);
    } catch (error) {
      console.error("Failed to fetch feeding schedule:", error);
      setScheduleError("Failed to fetch feeding schedule");
      toast.error("Failed to fetch feeding schedule");
    } finally {
      setScheduleLoading(false);
    }
  }, []);

  // Load species list for dropdowns without displaying dialog
  const loadSpeciesForDropdown = useCallback(async () => {
    try {
      if (speciesList.length === 0) {
        setSpeciesLoading(true);
        const data = await getAllSpecies();
        setSpeciesList(data.species || data);
      }
      return speciesList;
    } catch (error) {
      console.error("Failed to load species for dropdown:", error);
      toast.error("Failed to load species list");
      return [];
    } finally {
      setSpeciesLoading(false);
    }
  }, [speciesList]);

  return {
    // Data
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
    
    // Dialog states
    showSpeciesListDialog,
    setShowSpeciesListDialog,
    showSearchResultsDialog,
    setShowSearchResultsDialog,
    showSpeciesDetailsDialog,
    setShowSpeciesDetailsDialog,
    showFeedingScheduleDialog,
    setShowFeedingScheduleDialog,
    
    // Handlers
    handleFetchAllSpecies,
    handleSearchSpecies,
    handleGetSpeciesDetails,
    handleGetFeedingSchedule,
    loadSpeciesForDropdown
  };
};
