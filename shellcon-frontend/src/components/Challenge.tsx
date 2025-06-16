
import React from 'react';
import ChallengeCard from './challenge/ChallengeCard';
import { Challenge as ChallengeType } from '../hooks/useAquariumData';

interface ChallengeProps {
  challenge: ChallengeType;
  onSystemStatusUpdate?: (status: any) => void;
}

const Challenge: React.FC<ChallengeProps> = ({ challenge, onSystemStatusUpdate }) => {
  return <ChallengeCard challenge={challenge} onSystemStatusUpdate={onSystemStatusUpdate} />;
};

export default Challenge;
