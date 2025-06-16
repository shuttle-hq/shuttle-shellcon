
import { useState, useCallback, useEffect } from 'react';

// Define this type here instead of importing to avoid circular dependency
export type StoragePendingActionType = "solution" | "lecture";

interface StorageConfig {
  challengeId: string;
}

export interface ConfirmedActions {
  solution: boolean;
  lecture: boolean;
}

export const useChallengeStorage = ({ challengeId }: StorageConfig) => {
  // Ensure challengeId is always a string
  const normalizedId = String(challengeId);
  
  // Create unique keys for this challenge's localStorage items
  const confirmationKey = `shellcon_confirmed_${normalizedId}`;
  const solutionConfirmKey = `shellcon_solution_confirmed_${normalizedId}`;
  const lectureConfirmKey = `shellcon_lecture_confirmed_${normalizedId}`;
  
  // Debug localStorage values
  console.log(`[Storage Debug] Challenge ${normalizedId} solution key:`, solutionConfirmKey);
  console.log(`[Storage Debug] Current localStorage value for solution:`, localStorage.getItem(solutionConfirmKey));
  console.log(`[Storage Debug] Current localStorage value for lecture:`, localStorage.getItem(lectureConfirmKey));
  console.log(`[Storage Debug] Current localStorage value for shared confirmation:`, localStorage.getItem(confirmationKey));
  
  // Initialize confirmed actions from localStorage with direct access
  const [confirmedActions, setConfirmedActions] = useState<ConfirmedActions>(() => {
    // Check the shared confirmation first (this is the key issue!)
    const isConfirmed = localStorage.getItem(confirmationKey) === 'true';
    
    // If shared confirmation exists, both are confirmed
    if (isConfirmed) {
      console.log(`[Storage Debug] Shared confirmation found for challenge ${normalizedId}`);
      return {
        solution: true,
        lecture: true
      };
    }
    
    // Otherwise check individual confirmations (for backward compatibility)
    const solutionConfirmed = localStorage.getItem(solutionConfirmKey) === 'true';
    const lectureConfirmed = localStorage.getItem(lectureConfirmKey) === 'true';
    
    console.log(`[Storage Debug] Initial state - solution confirmed:`, solutionConfirmed);
    console.log(`[Storage Debug] Initial state - lecture confirmed:`, lectureConfirmed);
    
    return {
      solution: solutionConfirmed,
      lecture: lectureConfirmed
    };
  });
  
  // Use effect to sync state with localStorage when the component mounts or keys change
  useEffect(() => {
    const syncStorage = () => {
      // Check the shared confirmation first
      const isConfirmed = localStorage.getItem(confirmationKey) === 'true';
      
      if (isConfirmed) {
        console.log(`[Storage Debug] Storage sync - shared confirmation found`);
        // If shared confirmation exists, both are confirmed
        setConfirmedActions({
          solution: true,
          lecture: true
        });
        return;
      }
      
      // Otherwise check individual confirmations
      const solutionConfirmed = localStorage.getItem(solutionConfirmKey) === 'true';
      const lectureConfirmed = localStorage.getItem(lectureConfirmKey) === 'true';
      
      console.log(`[Storage Debug] Storage sync - solution confirmed:`, solutionConfirmed);
      console.log(`[Storage Debug] Storage sync - lecture confirmed:`, lectureConfirmed);
      
      // Only update state if the values are different
      setConfirmedActions(prev => {
        if (prev.solution !== solutionConfirmed || prev.lecture !== lectureConfirmed) {
          return {
            solution: solutionConfirmed,
            lecture: lectureConfirmed
          };
        }
        return prev;
      });
    };

    // Initial sync when component mounts
    syncStorage();
    
    // Set up event listener for storage changes from other tabs/windows
    window.addEventListener('storage', syncStorage);
    
    // Cleanup event listener
    return () => {
      window.removeEventListener('storage', syncStorage);
    };
  }, [confirmationKey, solutionConfirmKey, lectureConfirmKey]);
  
  const saveConfirmation = useCallback((action: StoragePendingActionType) => {
    // FIXED: When one action is confirmed, we confirm both by setting a shared confirmation
    // This is the key to fixing the issue - we ensure the shared key is set
    localStorage.setItem(confirmationKey, 'true');
    console.log(`[Storage Debug] Saved shared confirmation to localStorage:`, confirmationKey);
    
    // Also set individual confirmation for backward compatibility
    const storageKey = action === 'solution' ? solutionConfirmKey : lectureConfirmKey;
    localStorage.setItem(storageKey, 'true');
    console.log(`[Storage Debug] Also saved ${action} confirmation to localStorage:`, storageKey);
    
    // FIXED: Always set both individual keys as well for redundancy
    localStorage.setItem(solutionConfirmKey, 'true');
    localStorage.setItem(lectureConfirmKey, 'true');
    console.log(`[Storage Debug] Set both individual keys to true for complete syncing`);
    
    // Update the state to reflect both actions are confirmed
    setConfirmedActions({
      solution: true,
      lecture: true
    });
    console.log(`[Storage Debug] Updated state for both actions:`, true);
    
    // Force a storage event for synchronization across components
    try {
      window.dispatchEvent(new Event('storage'));
    } catch (e) {
      console.error('Failed to dispatch storage event:', e);
    }
  }, [confirmationKey, solutionConfirmKey, lectureConfirmKey]);

  return {
    confirmedActions,
    saveConfirmation
  };
};
