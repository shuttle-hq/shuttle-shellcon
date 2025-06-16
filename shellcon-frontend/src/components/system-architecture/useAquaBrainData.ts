import { useState, useCallback } from 'react';
import { getAllTankAnalysis, getTankAnalysis } from '@/api/aquariumApi';
import { toast } from "sonner";

// Define interfaces for the API response types
export interface TankAnalysisSummary {
  tank_id: string;
  species_id: number;
  species_name: string;
  overall_health: 'good' | 'caution' | 'at_risk';
  timestamp: string;
}

export interface TankAnalysisDetail {
  tank_id: string;
  species_id: number;
  timestamp: string;
  temperature_status: 'normal' | 'high' | 'low';
  ph_status: 'normal' | 'high' | 'low';
  oxygen_status: 'normal' | 'high' | 'low';
  feeding_status: 'normal' | 'overdue' | 'excess';
  overall_health: 'good' | 'caution' | 'at_risk';
  recommendations: string[];
}

export const useAquaBrainData = () => {
  // Tank Analysis Summary
  const [tankAnalysisSummary, setTankAnalysisSummary] = useState<TankAnalysisSummary[]>([]);
  const [summaryLoading, setSummaryLoading] = useState<boolean>(false);
  const [summaryError, setSummaryError] = useState<string | null>(null);
  
  // Tank Analysis Detail
  const [tankAnalysisDetail, setTankAnalysisDetail] = useState<TankAnalysisDetail | null>(null);
  const [detailLoading, setDetailLoading] = useState<boolean>(false);
  const [detailError, setDetailError] = useState<string | null>(null);
  
  // Dialog States
  const [showAnalysisSummaryDialog, setShowAnalysisSummaryDialog] = useState<boolean>(false);
  const [showAnalysisDetailDialog, setShowAnalysisDetailDialog] = useState<boolean>(false);

  // Get all tank analysis summary
  const handleGetAllTankAnalysis = useCallback(async () => {
    try {
      setSummaryLoading(true);
      setSummaryError(null);
      
      const data = await getAllTankAnalysis();
      setTankAnalysisSummary(Array.isArray(data) ? data : []);
      setShowAnalysisSummaryDialog(true);
    } catch (error) {
      console.error("Failed to fetch tank analysis summary:", error);
      setSummaryError("Failed to fetch tank analysis summary");
      toast.error("Failed to fetch tank analysis summary");
    } finally {
      setSummaryLoading(false);
    }
  }, []);

  // Get tank analysis detail
  const handleGetTankAnalysis = useCallback(async (tankId: string) => {
    try {
      setDetailLoading(true);
      setDetailError(null);
      
      const data = await getTankAnalysis(tankId);
      setTankAnalysisDetail(data);
      setShowAnalysisDetailDialog(true);
    } catch (error) {
      console.error(`Failed to fetch analysis for tank ${tankId}:`, error);
      setDetailError(`Failed to fetch analysis for tank ${tankId}`);
      toast.error(`Failed to fetch analysis for tank ${tankId}`);
    } finally {
      setDetailLoading(false);
    }
  }, []);

  return {
    // Data
    tankAnalysisSummary,
    summaryLoading,
    summaryError,
    tankAnalysisDetail,
    detailLoading,
    detailError,
    
    // Dialog states
    showAnalysisSummaryDialog,
    setShowAnalysisSummaryDialog,
    showAnalysisDetailDialog,
    setShowAnalysisDetailDialog,
    
    // Handlers
    handleGetAllTankAnalysis,
    handleGetTankAnalysis
  };
};
