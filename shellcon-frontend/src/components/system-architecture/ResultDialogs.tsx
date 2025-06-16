
import React from 'react';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { ApiResultDisplay } from "../ApiResultDisplay";
import { CustomDialogClose } from "./CustomDialogClose";

interface ResultDialogsProps {
  showTanksDialog: boolean;
  setShowTanksDialog: (show: boolean) => void;
  tanksData: any;
  tanksLoading: boolean;
  tanksError: string | null;
  
  showReadingsDialog: boolean;
  setShowReadingsDialog: (show: boolean) => void;
  tankReadings: any;
  readingsLoading: boolean;
  readingsError: string | null;
  
  showSensorDialog: boolean;
  setShowSensorDialog: (show: boolean) => void;
  sensorStatus: any;
  sensorLoading: boolean;
  sensorError: string | null;
}

const ResultDialogs: React.FC<ResultDialogsProps> = ({
  showTanksDialog,
  setShowTanksDialog,
  tanksData,
  tanksLoading,
  tanksError,
  
  showReadingsDialog,
  setShowReadingsDialog,
  tankReadings,
  readingsLoading,
  readingsError,
  
  showSensorDialog,
  setShowSensorDialog,
  sensorStatus,
  sensorLoading,
  sensorError
}) => {
  return (
    <>
      <Dialog open={showTanksDialog} onOpenChange={setShowTanksDialog}>
        <DialogContent className="bg-gray-900 border-2 border-orange-500/40 text-white max-w-md" onInteractOutside={e => e.preventDefault()} onEscapeKeyDown={e => e.preventDefault()} hideCloseButton>
          <DialogHeader>
            <DialogTitle className="text-orange-400 flex items-center">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
              Available Tanks
            </DialogTitle>
            <CustomDialogClose />
          </DialogHeader>
          <ApiResultDisplay 
            data={tanksData} 
            loading={tanksLoading} 
            error={tanksError}
            type="tanks" 
          />
        </DialogContent>
      </Dialog>

      <Dialog open={showReadingsDialog} onOpenChange={setShowReadingsDialog}>
        <DialogContent className="bg-gray-900 border-2 border-orange-500/40 text-white max-w-3xl" onInteractOutside={e => e.preventDefault()} onEscapeKeyDown={e => e.preventDefault()} hideCloseButton>
          <DialogHeader>
            <DialogTitle className="text-orange-400">Tank Readings</DialogTitle>
            <CustomDialogClose />
          </DialogHeader>
          <ApiResultDisplay 
            data={tankReadings} 
            loading={readingsLoading} 
            error={readingsError}
            type="readings" 
          />
        </DialogContent>
      </Dialog>

      <Dialog open={showSensorDialog} onOpenChange={setShowSensorDialog}>
        <DialogContent className="bg-gray-900 border-2 border-orange-500/40 text-white max-w-3xl" onInteractOutside={e => e.preventDefault()} onEscapeKeyDown={e => e.preventDefault()} hideCloseButton>
          <DialogHeader>
            <DialogTitle className="text-orange-400">Sensor Status</DialogTitle>
            <CustomDialogClose />
          </DialogHeader>
          <ApiResultDisplay 
            data={sensorStatus} 
            loading={sensorLoading} 
            error={sensorError}
            type="sensors" 
          />
        </DialogContent>
      </Dialog>
    </>
  );
};

export default ResultDialogs;
