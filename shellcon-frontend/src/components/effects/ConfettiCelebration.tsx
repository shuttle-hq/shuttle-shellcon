import React, { useEffect, useRef, useState, useCallback } from 'react';
import confetti from 'canvas-confetti';
import { useChallenges } from '@/hooks/useAquariumData';

interface ConfettiCelebrationProps {
  totalChallenges?: number;
}

const ConfettiCelebration: React.FC<ConfettiCelebrationProps> = ({ totalChallenges: propTotalChallenges = 4 }) => {
  const { challengesData } = useChallenges();
  const hasTriggeredConfetti = useRef(false);
  const confettiTriggeredKey = 'confetti_triggered';

  // State to hold the most up-to-date total number of challenges
  const [actualTotalChallenges, setActualTotalChallenges] = useState(propTotalChallenges);

  useEffect(() => {
    // Update actualTotalChallenges if challengesData.total is available and different
    if (challengesData?.total && challengesData.total !== actualTotalChallenges) {
      setActualTotalChallenges(challengesData.total);
    }
  }, [challengesData?.total, actualTotalChallenges, propTotalChallenges]);

  // Load initial confetti trigger state from localStorage
  useEffect(() => {
    try {
      const wasTriggered = localStorage.getItem(confettiTriggeredKey) === 'true';
      if (wasTriggered) {
        hasTriggeredConfetti.current = true;
      }
    } catch (error) {
      console.error('Error checking confetti trigger status from localStorage:', error);
    }
  }, [confettiTriggeredKey]);

  // The confetti firing function
  const triggerConfettiEffect = useCallback(() => {
    const myCanvas = document.createElement('canvas');
    myCanvas.style.position = 'fixed';
    myCanvas.style.top = '0';
    myCanvas.style.left = '0';
    myCanvas.style.width = '100vw';
    myCanvas.style.height = '100vh';
    myCanvas.style.pointerEvents = 'none';
    myCanvas.style.zIndex = '9999';
    document.body.appendChild(myCanvas);

    const myConfetti = confetti.create(myCanvas, {
      resize: true,
      useWorker: true
    });

    const duration = 5 * 1000; // 5 seconds
    const end = Date.now() + duration;

    myConfetti({
      particleCount: 100,
      spread: 160,
      origin: { y: 0, x: 0.5 },
      colors: ['#FF5733', '#33FF57', '#3357FF', '#F3FF33', '#FF33F3']
    });

    setTimeout(() => {
      myConfetti({
        particleCount: 50,
        angle: 60,
        spread: 80,
        origin: { x: 0, y: 0.5 },
        colors: ['#FF9933', '#33FFC1', '#8A33FF', '#FFFC33', '#FF33A1']
      });
    }, 250);

    setTimeout(() => {
      myConfetti({
        particleCount: 50,
        angle: 120,
        spread: 80,
        origin: { x: 1, y: 0.5 },
        colors: ['#33FFEC', '#FF5733', '#C133FF', '#33FF57', '#FFB533']
      });
    }, 400);

    const interval = setInterval(() => {
      if (Date.now() > end) {
        clearInterval(interval);
        setTimeout(() => {
          if (document.body.contains(myCanvas)) {
            document.body.removeChild(myCanvas);
          }
        }, 1000);
        return;
      }

      const x = Math.random();
      const y = Math.random() * 0.5;
      
      myConfetti({
        particleCount: 20,
        startVelocity: 30,
        spread: 100,
        origin: { x, y },
        colors: ['#FF5733', '#33FF57', '#3357FF', '#F3FF33', '#FF33F3', '#33FFEC', '#FFB533']
      });
    }, 300);
  }, []);

  const attemptToTriggerConfetti = useCallback(() => {
    if (hasTriggeredConfetti.current) return;

    let solvedCount = 0;
    const currentTotal = actualTotalChallenges;

    try {
      const solvedChallengesStr = localStorage.getItem('solved_challenges');
      if (solvedChallengesStr) {
        const solvedChallengesArray: number[] = JSON.parse(solvedChallengesStr);
        solvedCount = solvedChallengesArray.length;
      } else if (challengesData?.solved !== undefined) {
        solvedCount = challengesData.solved;
      }
    } catch (error) {
      console.error('Error reading solved_challenges from localStorage:', error);
      if (challengesData?.solved !== undefined) {
        solvedCount = challengesData.solved;
      }
    }
    
    if (solvedCount === currentTotal && currentTotal > 0) {
      console.log(`All ${currentTotal} challenges solved! Triggering confetti celebration.`);
      hasTriggeredConfetti.current = true;
      try {
        localStorage.setItem(confettiTriggeredKey, 'true');
      } catch (error) {
        console.error('Error saving confetti trigger status:', error);
      }
      triggerConfettiEffect();
    }
  }, [actualTotalChallenges, challengesData?.solved, confettiTriggeredKey, triggerConfettiEffect]);

  // Effect to attempt triggering confetti when challengesData changes
  useEffect(() => {
    if (challengesData) {
      attemptToTriggerConfetti();
    }
  }, [challengesData, attemptToTriggerConfetti]);

  // Effect to listen for localStorage changes to 'solved_challenges'
  useEffect(() => {
    const handleStorageChange = (event: Event) => {
      const isRelevantEvent = (e: Event): boolean => {
        if (e instanceof StorageEvent) {
          return e.key === 'solved_challenges';
        }
        if (e instanceof CustomEvent && e.type === 'storage') {
          // Ensure detail and detail.key exist before accessing
          return (e.detail as any)?.key === 'solved_challenges';
        }
        return false;
      };

      if (isRelevantEvent(event)) {
        console.log('localStorage event for solved_challenges detected.');
        attemptToTriggerConfetti();
      }
    };

    window.addEventListener('storage', handleStorageChange);
    // Custom events for same-tab also use type 'storage' and are caught by the above listener.
    // No need to add a duplicate listener for 'storage' of type CustomEvent explicitly.

    attemptToTriggerConfetti(); // Initial check

    return () => {
      window.removeEventListener('storage', handleStorageChange);
    };
  }, [attemptToTriggerConfetti]);

  return null;
};

export default ConfettiCelebration;
