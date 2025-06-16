
import React from 'react';
import { cn } from "@/lib/utils";

interface StatusBadgeProps extends React.HTMLAttributes<HTMLDivElement> {
  status: string;
  type?: string;
}

export const StatusBadge: React.FC<StatusBadgeProps> = ({ 
  status, 
  type = '',
  className,
  ...props 
}) => {
  // Helper function to get status color that won't change on hover
  const getStatusColor = (status: string, type: string) => {
    if (status === 'normal' || status === 'online' || status === 'operational' || status === 'ok') {
      return 'bg-green-500/20 text-green-400 border-green-500/30 hover:bg-green-500/30';
    }
    
    if (type === 'overall_health' || type === '') {
      switch (status.toLowerCase()) {
        case 'good':
          return 'bg-green-500/20 text-green-400 border-green-500/30 hover:bg-green-500/30';
        case 'caution':
        case 'degraded':
          return 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30 hover:bg-yellow-500/30';
        case 'at_risk':
        case 'critical':
          return 'bg-red-500/20 text-red-400 border-red-500/30 hover:bg-red-500/30';
        case 'offline':
          return 'bg-gray-500/20 text-gray-400 border-gray-500/30 hover:bg-gray-500/30';
        default:
          return 'bg-gray-500/20 text-gray-400 border-gray-500/30 hover:bg-gray-500/30';
      }
    }
    
    if (status === 'high' || status === 'excess') {
      return 'bg-orange-500/20 text-orange-400 border-orange-500/30 hover:bg-orange-500/30';
    }
    
    if (status === 'low' || status === 'overdue') {
      return 'bg-blue-500/20 text-blue-400 border-blue-500/30 hover:bg-blue-500/30';
    }
    
    return 'bg-gray-500/20 text-gray-400 border-gray-500/30 hover:bg-gray-500/30';
  };

  const displayText = status.replace(/_/g, ' ').toUpperCase();
  
  return (
    <div
      className={cn(
        "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-all duration-300 whitespace-nowrap shadow-sm backdrop-blur-sm",
        getStatusColor(status, type),
        className
      )}
      {...props}
    >
      {displayText}
    </div>
  );
};
