
import React from 'react';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { AlertTriangle } from 'lucide-react';
import { PendingActionType } from './hooks/useChallengeCard';

interface ConfirmationDialogProps {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
  pendingAction: PendingActionType;
}

const ConfirmationDialog: React.FC<ConfirmationDialogProps> = ({
  isOpen,
  onOpenChange,
  onConfirm,
  pendingAction
}) => {
  return (
    <AlertDialog open={isOpen} onOpenChange={onOpenChange}>
      <AlertDialogContent className="bg-gray-900 border border-gray-700 text-white">
        <AlertDialogHeader>
          <AlertDialogTitle className="text-transparent bg-clip-text bg-gradient-to-r from-orange-400 via-purple-400 to-orange-400 text-left">
            {pendingAction === 'solution' ? '🦀 View Solution? 🦀' : '🦀 View Information? 🦀'}
          </AlertDialogTitle>
          <AlertDialogDescription asChild>
            {pendingAction === 'solution' ? (
              <p className="text-gray-300">
                <span className="font-medium text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-purple-400">Ferris says:</span> Looking at the solution will prevent you from experiencing the challenge of solving this problem yourself. Are you sure you want to continue?
              </p>
            ) : (
              <p className="text-gray-300">
                <span className="font-medium text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-purple-400">Ferris says:</span> This section contains educational information about the concepts involved in this challenge. It may contain hints or explanations that could make the challenge easier.
              </p>
            )}
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter className="flex !flex-row !justify-center gap-2 sm:!justify-center">
          <AlertDialogCancel className="text-gray-400 border-gray-700 hover:bg-gray-800 hover:text-white">
            I'll try again
          </AlertDialogCancel>
          <AlertDialogAction 
            onClick={onConfirm} 
            className="bg-gradient-to-r from-orange-500 via-purple-500 to-orange-500 animate-gradient bg-size-200 text-white hover:bg-orange-700"
          >
            {pendingAction === 'solution' ? "Show me the solution" : "Show me the information"}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
};

export default ConfirmationDialog;
