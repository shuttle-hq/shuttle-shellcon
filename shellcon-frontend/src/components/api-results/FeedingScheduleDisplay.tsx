
import React, { useEffect, useState } from 'react';
import { FeedingSchedule } from '../system-architecture/useSpeciesData';
import { getSpeciesDetails } from '@/api/aquariumApi';

interface FeedingScheduleDisplayProps {
  data: FeedingSchedule;
}

export const FeedingScheduleDisplay: React.FC<FeedingScheduleDisplayProps> = ({ data }) => {
  const [speciesName, setSpeciesName] = useState<string>('Loading species...');

  // Check if the data is in the expected format
  if (!data || typeof data !== 'object') {
    return (
      <div className="p-4 bg-red-900/30 border border-red-500 rounded-md">
        <h3 className="text-lg font-bold text-red-400 mb-2">Invalid Data Format</h3>
        <p className="text-white">The feeding schedule data is not in a valid format.</p>
      </div>
    );
  }
  
  // Use type assertion to handle API format
  const anyData = data as any;
  
  // Get species information
  const speciesId = anyData.species_id || 'Unknown ID';
  
  // Fetch species name from API
  useEffect(() => {
    const fetchSpeciesName = async () => {
      try {
        const speciesData = await getSpeciesDetails(speciesId);
        if (speciesData && speciesData.name) {
          setSpeciesName(speciesData.name);
        } else {
          setSpeciesName('Unknown Species');
        }
      } catch (error) {
        console.error('Error fetching species name:', error);
        setSpeciesName('Unknown Species');
      }
    };
    
    if (speciesId !== 'Unknown ID') {
      fetchSpeciesName();
    }
  }, [speciesId]);
  
  // Handle feeding schedule format from the API
  // The API returns an object with feeding_times array, food_type, and amount_grams
  const feedingTimes = anyData.feeding_times || [];
  const foodType = anyData.food_type || 'Standard';
  const amountGrams = anyData.amount_grams ? `${anyData.amount_grams}g` : 'As needed';
  
  // Create schedule entries from the feeding times
  const schedule = feedingTimes.map((time: string) => ({
    time,
    food_type: foodType,
    amount: amountGrams
  }));
  
  // Handle empty schedule
  if (!schedule.length) {
    return (
      <div className="p-4 bg-gray-800 border border-orange-500/30 rounded-md">
        <h3 className="text-lg font-bold text-orange-400 mb-2">
          {speciesName}
        </h3>
        <p className="text-gray-300">No feeding schedule information available.</p>
      </div>
    );
  }
  
  return (
    <div className="space-y-4">
      <div className="bg-gray-800 p-4 rounded-md">
        <h3 className="text-lg font-bold text-orange-400 mb-2">
          {speciesName}
        </h3>
        
        <div className="overflow-x-auto mt-4">
          <table className="w-full text-sm text-left text-gray-300">
            <thead className="text-xs text-orange-400 uppercase bg-gray-900">
              <tr>
                <th scope="col" className="px-4 py-2">Time</th>
                <th scope="col" className="px-4 py-2">Food Type</th>
                <th scope="col" className="px-4 py-2">Amount</th>
              </tr>
            </thead>
            <tbody>
              {schedule.map((item, index) => (
                <tr key={index} className="border-b border-gray-700 hover:bg-gray-700">
                  <td className="px-4 py-2">{item.time || item.feeding_time || '--:--'}</td>
                  <td className="px-4 py-2 font-medium">{item.food_type || item.foodType || 'Standard'}</td>
                  <td className="px-4 py-2">{item.amount || item.food_amount || 'As needed'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        
        <div className="mt-4 p-3 bg-gray-850 rounded">
          <h4 className="text-sm font-semibold text-orange-300 mb-2">Details</h4>
          <div className="text-sm text-gray-300">
            <p><span className="text-gray-400">Food Type:</span> {foodType}</p>
            <p><span className="text-gray-400">Standard Amount:</span> {amountGrams}</p>
            <p><span className="text-gray-400">Feedings Per Day:</span> {feedingTimes.length}</p>
          </div>
        </div>
      </div>
    </div>
  );
};
