
import React from 'react';

const Header: React.FC = () => {
  return (
    <header className="header-with-mascot bg-black py-4 border-b-4 border-orange-500">
      <div className="container mx-auto px-4 flex items-center justify-between relative z-10">
        <div className="flex items-center gap-4">
          <img 
            src="/lovable-uploads/05b270df-e4c2-4bb9-8616-c60df68d5506.png" 
            alt="Shuttle Logo" 
            className="h-12 w-auto"
          />
          <div>
            <div className="text-2xl md:text-3xl font-sans font-extrabold tracking-tight">
              <span 
                style={{
                  background: 'linear-gradient(to right, #f97316, #fb923c, #fbbf24)',
                  WebkitBackgroundClip: 'text',
                  backgroundClip: 'text',
                  color: 'transparent',
                  display: 'inline-block'
                }}
              >
                ShellCon Smart Aquarium Dashboard
              </span>
            </div>
            <p className="text-gray-300 text-sm md:text-base">
              Real-time monitoring and optimization challenges for the world's premier crustacean convention
            </p>
          </div>
        </div>
      </div>
      <div className="header-mascot w-32 h-32">
        <img 
          src="/lovable-uploads/4adfa2cc-cffd-4463-8069-939658c80853.png" 
          alt="Ferris the Crab" 
          className="w-full h-full object-contain animate-float"
        />
      </div>
    </header>
  );
}

export default Header;
