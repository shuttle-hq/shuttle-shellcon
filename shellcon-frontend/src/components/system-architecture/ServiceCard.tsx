
import React, { ReactNode } from 'react';
import { Card } from "@/components/ui/card";

interface ServiceCardProps {
  title: string;
  description: string;
  icon: React.ElementType;
  children?: ReactNode;
}

const ServiceCard: React.FC<ServiceCardProps> = ({ 
  title, 
  description, 
  icon: IconComponent,
  children
}) => {
  return (
    <div className="border border-orange-500/30 rounded-md p-4 bg-gray-900 text-white hover:border-orange-400 transition-all duration-300 shadow-md">
      <div className="flex justify-between items-center mb-2">
        <h3 className="font-bold text-orange-400">{title}</h3>
        <IconComponent className="h-5 w-5 text-orange-400" />
      </div>
      <p className="text-sm text-gray-300 mb-4">
        {description}
      </p>
      {children}
    </div>
  );
};

export default ServiceCard;
