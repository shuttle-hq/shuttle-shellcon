
import React, { useState } from 'react';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import { Button } from '@/components/ui/button';
import { ArrowDown, Sparkles } from 'lucide-react';
import { Alert, AlertDescription } from '@/components/ui/alert';

interface WelcomeStoryProps {
  isOpen: boolean;
  setIsOpen: (isOpen: boolean) => void;
}

const WelcomeStory: React.FC<WelcomeStoryProps> = ({ isOpen, setIsOpen }) => {
  return (
    <div className="container mx-auto px-4 py-3 mt-4">
      <Alert 
        variant="highlight" 
        className="border-orange-500 shadow-xl alert-story"
      >
        <Collapsible
          open={isOpen}
          onOpenChange={setIsOpen}
          className="w-full"
        >
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <CollapsibleTrigger asChild>
                <Button 
                  variant="outline" 
                  size="sm" 
                  className={`
                    relative overflow-hidden bg-gradient-to-r from-orange-500 via-purple-500 to-orange-500 
                    bg-size-200 text-white font-bold px-6 py-3 shadow-lg 
                    hover:shadow-orange-500/50 border-none
                    flex items-center gap-2 transition-all duration-300
                    hover:scale-105
                  `}
                >
                  <Sparkles className="h-4 w-4 animate-sparkle-spin" />
                  Read Story
                  <ArrowDown className={`h-4 w-4 transition-transform duration-300 animate-arrow-bounce ${isOpen ? 'transform rotate-180' : ''}`} />
                  <span className="absolute inset-0 bg-gradient-to-r from-orange-400 to-purple-500 opacity-50 blur-sm"></span>
                </Button>
              </CollapsibleTrigger>
              <h2 className="text-lg font-semibold text-orange-300">
                Welcome to the Shuttle onboarding experience!
                <div className="text-sm font-normal mt-1 text-white">You are now a member of the emergency technical response team for ShellCon, where Rustaceans and crustaceans unite!</div>
              </h2>
            </div>
          </div>
          
          <CollapsibleContent className="text-sm mt-2 space-y-3 text-gray-300 animate-fade-in">
            
            <div className="border-l-4 border-orange-500 pl-4 py-2">
              <p className="font-bold text-orange-400">🦀 Congratulations on Your Deployment! 🦀</p>
              <p>
                First and foremost, excellent work on successfully deploying the microservices that power the ShellCon platform! This is no small feat, and demonstrates your technical prowess. Before diving into the challenges, take a moment to verify that all systems are operational by using the API call buttons in the Control Panel below. Each button connects to a different service endpoint, allowing you to confirm that all components are communicating properly.
              </p>
            </div>
            
            <div>
              <p className="font-semibold text-orange-300">The Situation</p>
              <p>
                This year's ShellCon features a revolutionary Smart Aquarium system built with Rust and deployed on Shuttle. 
                The system uses advanced sensors and analytics to maintain perfect environments for rare and valuable crab, 
                lobster, and shrimp specimens from around the world.
              </p>
            </div>
            
            <p>
              However, just hours before the convention opens, several performance issues have been detected in the backend services. 
              As a newly recruited Rustacean engineer, you've been called in to optimize these systems before the doors open to 
              the public. The organizers are in a pinch—quite literally, as the convention's mascot, a giant Coconut Crab 
              named Ferris, is anxiously clicking his claws at the mounting technical issues!
            </p>
            
            <div>
              <p className="font-semibold text-orange-300">The Smart Aquarium Architecture</p>
              <ul className="list-disc pl-5 space-y-1">
                <li><span className="font-medium text-orange-200">aqua-monitor:</span> Collects real-time environmental data from tank sensors (currently moving at a snail's pace instead of crab-like efficiency)</li>
                <li><span className="font-medium text-orange-200">species-hub:</span> Manages the species database and feeding requirements (currently as disorganized as a lobster molting season)</li>
                <li><span className="font-medium text-orange-200">aqua-brain:</span> Performs analysis and coordinates system responses (currently experiencing more bottlenecks than a hermit crab trying to fit into a new shell)</li>
              </ul>
            </div>
            
            <div>
              <p className="font-semibold text-orange-300">Your Mission</p>
              <p>
                Your mission is to solve a series of challenges that will optimize these services, demonstrating your 
                Rust backend expertise while learning valuable patterns for high-performance distributed systems. 
                Each fix brings you one step closer to ensuring the safety of these incredible creatures and saving 
                the reputation of ShellCon.
              </p>
            </div>
            
            <p>
              Examine each challenge, implement the suggested optimizations, and watch as the system's performance improves under "System Status". 
              The convention opens soon—the clock is ticking and the crustaceans are getting crabby!
            </p>
            
            <p>
              Remember, in the world of Rust performance optimization, you're not just a developer—you're a Rustacean on a mission!
            </p>
          </CollapsibleContent>
        </Collapsible>
      </Alert>
    </div>
  );
}

export default WelcomeStory;
