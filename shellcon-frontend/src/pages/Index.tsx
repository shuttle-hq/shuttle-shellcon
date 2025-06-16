
import React, { useState, useEffect } from 'react';
import SystemArchitecture from '../components/SystemArchitecture';
import SystemStatus from '../components/SystemStatus';
import ChallengeList from '../components/ChallengeList';
import FirstTimeModal from '../components/FirstTimeModal';
import Header from '../components/layout/Header';
import Footer from '../components/layout/Footer';
import WelcomeStory from '../components/layout/WelcomeStory';
import { toast } from '@/hooks/use-toast';
import ConfettiCelebration from '../components/effects/ConfettiCelebration';

const Index = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [showFirstTimeModal, setShowFirstTimeModal] = useState(false);

  useEffect(() => {
    // For testing purposes - uncomment to always show the modal
    // localStorage.removeItem('hasVisitedBefore');
    
    // Check if this is the user's first visit
    const hasVisitedBefore = localStorage.getItem('hasVisitedBefore');
    if (!hasVisitedBefore) {
      setShowFirstTimeModal(true);
    }
  }, []);

  const handleCloseFirstTimeModal = () => {
    localStorage.setItem('hasVisitedBefore', 'true');
    setShowFirstTimeModal(false);
  };

  const handleReadStory = () => {
    setIsOpen(true);
    handleCloseFirstTimeModal();
    // Scroll to the alert section
    setTimeout(() => {
      const alertElement = document.querySelector('.alert-story');
      if (alertElement) {
        alertElement.scrollIntoView({ behavior: 'smooth' });
      }
    }, 300);
  };

  return (
    <>
      <FirstTimeModal 
        isOpen={showFirstTimeModal} 
        onClose={handleCloseFirstTimeModal} 
        onReadStory={handleReadStory}
      />
      
      {/* Add confetti celebration component */}
      <ConfettiCelebration />
      
      <div className="min-h-screen bg-gray-900">
        <Header />
        <WelcomeStory isOpen={isOpen} setIsOpen={setIsOpen} />

        <main className="container mx-auto px-4 py-8 bg-gray-900 text-white">
          <SystemArchitecture />
          <SystemStatus />
          <ChallengeList />
        </main>
        
        <Footer />
      </div>
    </>
  );
};

export default Index;
