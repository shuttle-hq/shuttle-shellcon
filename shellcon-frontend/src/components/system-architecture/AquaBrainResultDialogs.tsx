import React, { useState } from 'react';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { ApiResultDisplay } from "../ApiResultDisplay";
import { CustomDialogClose } from "./CustomDialogClose";
import { TankAnalysisSummary, TankAnalysisDetail } from './useAquaBrainData';

interface AquaBrainResultDialogsProps {
  // Tank Analysis Summary
  showAnalysisSummaryDialog: boolean;
  setShowAnalysisSummaryDialog: (show: boolean) => void;
  tankAnalysisSummary: TankAnalysisSummary[];
  summaryLoading: boolean;
  summaryError: string | null;
  
  // Tank Analysis Detail
  showAnalysisDetailDialog: boolean;
  setShowAnalysisDetailDialog: (show: boolean) => void;
  tankAnalysisDetail: TankAnalysisDetail | null;
  detailLoading: boolean;
  detailError: string | null;
  
  // Handlers
  onGetTankAnalysis: (tankId: string) => Promise<void>;
}

const AquaBrainResultDialogs: React.FC<AquaBrainResultDialogsProps> = ({
  // Tank Analysis Summary
  showAnalysisSummaryDialog,
  setShowAnalysisSummaryDialog,
  tankAnalysisSummary,
  summaryLoading,
  summaryError,
  
  // Tank Analysis Detail
  showAnalysisDetailDialog,
  setShowAnalysisDetailDialog,
  tankAnalysisDetail,
  detailLoading,
  detailError,
  
  // Handlers
  onGetTankAnalysis
}) => {
  return (
    <>
      {/* Tank Analysis Summary Dialog */}
      <Dialog open={showAnalysisSummaryDialog} onOpenChange={setShowAnalysisSummaryDialog}>
        <DialogContent className="bg-gray-900 border-2 border-orange-500/40 text-white max-w-3xl" onInteractOutside={e => e.preventDefault()} onEscapeKeyDown={e => e.preventDefault()} hideCloseButton>
          <DialogHeader>
            <DialogTitle className="text-orange-400">Tank Health Analysis</DialogTitle>
            <CustomDialogClose />
          </DialogHeader>
          
          <ApiResultDisplay 
            data={tankAnalysisSummary} 
            loading={summaryLoading} 
            error={summaryError}
            type="tank-analysis-summary" 
          />
        </DialogContent>
      </Dialog>

      {/* Tank Analysis Detail Dialog */}
      <Dialog open={showAnalysisDetailDialog} onOpenChange={setShowAnalysisDetailDialog}>
        <DialogContent className="bg-gray-900 border-2 border-orange-500/40 text-white max-w-3xl" onInteractOutside={e => e.preventDefault()} onEscapeKeyDown={e => e.preventDefault()} hideCloseButton>
          <DialogHeader>
            <DialogTitle className="text-orange-400">Tank Health Detail</DialogTitle>
            <CustomDialogClose />
          </DialogHeader>
          
          <ApiResultDisplay 
            data={tankAnalysisDetail} 
            loading={detailLoading} 
            error={detailError}
            type="tank-analysis-detail" 
          />
        </DialogContent>
      </Dialog>
    </>
  );
};

export default AquaBrainResultDialogs;
