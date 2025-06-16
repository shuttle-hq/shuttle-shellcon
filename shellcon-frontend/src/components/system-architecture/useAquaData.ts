
import { useState, useEffect } from 'react';
import { getAllTanks, getTankReadings, getSensorStatus } from "@/api/aquariumApi";
import { toast } from "@/components/ui/sonner";

export const useAquaData = () => {
  const [tanksData, setTanksData] = useState<any>(null);
  const [tanksLoading, setTanksLoading] = useState(false);
  const [tanksError, setTanksError] = useState<string | null>(null);
  
  const [tankReadings, setTankReadings] = useState<any>(null);
  const [readingsLoading, setReadingsLoading] = useState(false);
  const [readingsError, setReadingsError] = useState<string | null>(null);
  
  const [sensorStatus, setSensorStatus] = useState<any>(null);
  const [sensorLoading, setSensorLoading] = useState(false);
  const [sensorError, setSensorError] = useState<string | null>(null);
  
  // Start with an empty array that will be populated on initialization
  const [tanksList, setTanksList] = useState<string[]>([]);
  
  // Fetch tank list on component initialization
  useEffect(() => {
    const fetchInitialTanks = async () => {
      try {
        const data = await getAllTanks();
        if (Array.isArray(data)) {
          setTanksList(data);
          console.log("Initial tank list loaded:", data);
        }
      } catch (error) {
        console.error("Failed to load initial tank list:", error);
        // Fallback to known tank IDs if the API fails
        setTanksList(["Tank-A1", "Tank-B2", "Tank-C3"]);
      }
    };
    
    fetchInitialTanks();
  }, []);
  const [showTanksDialog, setShowTanksDialog] = useState(false);
  const [showReadingsDialog, setShowReadingsDialog] = useState(false);
  const [showSensorDialog, setShowSensorDialog] = useState(false);
  
  const handleViewAllTanks = async () => {
    setTanksLoading(true);
    setTanksError(null);
    try {
      const data = await getAllTanks();
      console.log("Raw tanks data received:", data);
      
      // The API returns an array of tank IDs
      if (Array.isArray(data)) {
        // Store the raw array of tank IDs
        setTanksData({ tanks: data });
        setTanksList(data);
        console.log("Tank IDs set in state:", data);
      } else {
        // Handle unexpected data format
        console.warn("Unexpected data format received from API", data);
        setTanksData({ tanks: [] });
        setTanksList([]);
      }
      
      setShowTanksDialog(true);
    } catch (error) {
      console.error("Error fetching tanks:", error);
      setTanksError("Failed to fetch tanks");
      toast.error("Failed to fetch tanks");
    } finally {
      setTanksLoading(false);
    }
  };
  
  const handleFetchTankReadings = async (tankId: string) => {
    if (!tankId) {
      toast.error("Please select a tank first");
      return;
    }
    
    // Add the tank ID to the list if it's not already there
    if (!tanksList.includes(tankId)) {
      setTanksList(prev => [...prev, tankId]);
    }
    
    setReadingsLoading(true);
    setReadingsError(null);
    try {
      const data = await getTankReadings(tankId);
      console.log("Tank readings raw data:", data);
      
      // Extract the readings array from the response structure
      if (data && data.readings && Array.isArray(data.readings)) {
        setTankReadings(data.readings);
      } else {
        // Fallback in case the expected structure changes
        setTankReadings(Array.isArray(data) ? data : []);
      }
      
      setShowReadingsDialog(true);
    } catch (error) {
      console.error("Error fetching tank readings:", error);
      setReadingsError("Failed to fetch tank readings");
      toast.error("Failed to fetch tank readings");
    } finally {
      setReadingsLoading(false);
    }
  };
  
  const handleCheckSensorStatus = async () => {
    setSensorLoading(true);
    setSensorError(null);
    try {
      const data = await getSensorStatus();
      setSensorStatus(data);
      setShowSensorDialog(true);
      console.log("Sensor status:", data);
    } catch (error) {
      console.error("Error fetching sensor status:", error);
      setSensorError("Failed to fetch sensor status");
      toast.error("Failed to fetch sensor status");
    } finally {
      setSensorLoading(false);
    }
  };

  return {
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
  };
};
