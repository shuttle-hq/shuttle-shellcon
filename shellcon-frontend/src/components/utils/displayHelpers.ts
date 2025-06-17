
// Helper function for battery level color
export function getBatteryColorClass(level: number | undefined): string {
  if (!level && level !== 0) return "bg-gray-600";
  if (level < 20) return "bg-red-600";
  if (level < 50) return "bg-amber-600";
  return "bg-green-600";
}

// Helper function to process tanks data into a standard format
export function processTanksData(data: any): any[] {
  console.log("Processing tanks data:", data);
  
  let tanksArray: any[] = [];
  
  // Handle different possible data structures
  if (data && data.tanks && Array.isArray(data.tanks)) {
    tanksArray = data.tanks;
  } else if (Array.isArray(data)) {
    tanksArray = data;
  } else if (data && typeof data === 'object' && !Array.isArray(data)) {
    // If data is a single tank object
    tanksArray = [data];
  }
  
  console.log("Processed tanks array:", tanksArray);
  return tanksArray;
}
