
import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Alert } from "@/components/ui/alert";
import { StatusBadge } from "@/components/ui/status-badge";
import { useSystemStatus } from '../hooks/useAquariumData';
import { AlertCircle, CheckCircle, Loader2, ServerCrash, Activity } from "lucide-react";

const SystemStatus: React.FC = () => {
  const { status, loading, error } = useSystemStatus();

  return (
    <Card className="mb-8 shadow-xl border-orange-500/50 overflow-hidden">
      <CardHeader className="bg-gradient-to-r from-gray-900 via-gray-800 to-gray-900 text-white border-b border-orange-500/30 pb-4">
        <div className="flex justify-between items-center">
          <CardTitle 
            className="card-header-title flex items-center gap-2 font-bold text-transparent bg-gradient-to-br from-orange-500 via-amber-400 to-amber-500 bg-clip-text shadow-sm"
          >
            <Activity className="h-5 w-5 text-orange-400" />
            System Status
          </CardTitle>
          
          {status && (
            <div className="flex items-center gap-2">
              <span className="text-sm font-medium text-gray-300">Overall:</span>
              <StatusBadge status={status.overall_status} />
            </div>
          )}
        </div>
      </CardHeader>
      <CardContent className="pt-6 bg-gradient-to-b from-gray-900/80 to-black">
        {loading && (
          <div className="flex justify-center items-center py-8">
            <Loader2 className="h-8 w-8 text-orange-500 animate-spin" />
          </div>
        )}

        {error && (
          <Alert variant="destructive" className="mb-4">
            <AlertCircle className="h-4 w-4" />
            <div className="ml-2">{error}</div>
          </Alert>
        )}

        {status && (
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3">
            {/* Each service card */}
            <StatusCard 
              title="Environmental Monitoring" 
              status={status.environmental_monitoring}
              icon={CheckCircle}
            />
            <StatusCard 
              title="Species Database" 
              status={status.species_database}
              icon={CheckCircle}
            />
            <StatusCard 
              title="Feeding System" 
              status={status.feeding_system}
              icon={CheckCircle}
            />
            <StatusCard 
              title="Remote Monitoring" 
              status={status.remote_monitoring}
              icon={CheckCircle}
            />
            {/* Updated timestamp */}
            <div className="col-span-1 sm:col-span-2 md:col-span-3 flex justify-end mt-2">
              <div className="text-xs text-orange-300/70 italic flex items-center gap-1">
                <Loader2 className="h-3 w-3 animate-spin" />
                Last updated: {status.last_updated ? new Date(status.last_updated).toLocaleString() : 'N/A'}
              </div>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
};

interface StatusCardProps {
  title: string;
  status: string;
  icon: React.ElementType;
}

const StatusCard: React.FC<StatusCardProps> = ({ title, status, icon: Icon }) => {
  const getBackgroundClass = () => {
    switch (status.toLowerCase()) {
      case 'online':
      case 'ok':
      case 'operational':
        return 'from-green-900/20 to-green-900/5 border-green-500/30 hover:border-green-500/50';
      case 'degraded':
        return 'from-yellow-900/20 to-yellow-900/5 border-yellow-500/30 hover:border-yellow-500/50';
      case 'critical':
      case 'offline':
        return 'from-red-900/20 to-red-900/5 border-red-500/30 hover:border-red-500/50';
      default:
        return 'from-gray-800/50 to-gray-900 border-gray-700/50 hover:border-gray-600';
    }
  };

  const getIconClass = () => {
    switch (status.toLowerCase()) {
      case 'online':
      case 'ok':
      case 'operational':
        return 'text-green-400';
      case 'degraded':
        return 'text-yellow-400';
      case 'critical':
      case 'offline':
        return 'text-red-400';
      default:
        return 'text-gray-400';
    }
  };

  return (
    <div className={`p-4 rounded-md bg-gradient-to-b ${getBackgroundClass()} border backdrop-blur-sm transition-all duration-300 flex justify-between items-center shadow-lg hover:shadow-xl`}>
      <h3 className="font-medium text-white">{title}</h3>
      <div className="flex items-center gap-2">
        <Icon className={`h-4 w-4 ${getIconClass()}`} />
        <StatusBadge status={status} />
      </div>
    </div>
  );
};

export default SystemStatus;
