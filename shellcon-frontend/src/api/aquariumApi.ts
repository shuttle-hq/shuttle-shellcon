import { API_URLS } from '../config/api';

// System Status API - Fetch system component statuses from current challenges
export const getSystemStatus = async () => {
  try {
    // Check if we have saved status in localStorage
    const savedStatus = localStorage.getItem('system_status');
    
    // Initialize the system status object with default values or saved values
    const systemStatus = savedStatus ? JSON.parse(savedStatus) : {
      environmental_monitoring: 'degraded', // Challenge #1 - Async I/O
      species_database: 'degraded',        // Challenge #2 - Query Optimization
      feeding_system: 'error',             // Challenge #3 - Error Handling
      remote_monitoring: 'degraded',       // Challenge #4 - Resource Management
      overall_status: 'degraded',
      last_updated: new Date().toISOString()
    };

    // Fetch current challenges status
    try {
      const response = await fetch('/api/challenges/current');
      if (response.ok) {
        const data = await response.json();
        
        if (data.challenges && Array.isArray(data.challenges)) {
          // Map challenge statuses to system components
          data.challenges.forEach(challenge => {
            let componentStatus = challenge.status === 'solved' ? 'normal' : 
                                 (challenge.id === 3 ? 'error' : 'degraded');
            
            // Map challenge ID to system component
            switch (challenge.id) {
              case 1: // The Sluggish Sensor - Environmental Monitoring
                systemStatus.environmental_monitoring = componentStatus;
                break;
              case 2: // The Query Conundrum - Species Database
                systemStatus.species_database = componentStatus;
                break;
              case 3: // The Fragile Feeder - Feeding Schedule
                systemStatus.feeding_system = componentStatus;
                break;
              case 4: // The Leaky Connection - Remote Monitoring (Sensor Management)
                systemStatus.remote_monitoring = componentStatus;
                break;
            }
          });
        }
      }
    } catch (error) {
      console.error('Error fetching challenges status:', error);
      
      // If we can't get the challenges status, try to fetch individual validation statuses
      try {
        // Check if any challenges have been solved by validating each one
        const validationPromises = [
          validateChallenge(1).then(result => {
            if (result && result.system_component) {
              systemStatus.environmental_monitoring = result.system_component.status;
            }
          }),
          validateChallenge(2).then(result => {
            if (result && result.system_component) {
              systemStatus.species_database = result.system_component.status;
            }
          }),
          validateChallenge(3).then(result => {
            if (result && result.system_component) {
              systemStatus.feeding_system = result.system_component.status;
            }
          }),
          validateChallenge(4).then(result => {
            if (result && result.system_component) {
              systemStatus.remote_monitoring = result.system_component.status;
            }
          })
        ];
        
        await Promise.all(validationPromises);
      } catch (validationError) {
        console.error('Error validating challenges:', validationError);
      }
    }

    // Helper function to validate a challenge
    async function validateChallenge(challengeId) {
      try {
        const response = await fetch(`/api/challenges/${challengeId}/validate`);
        if (response.ok) {
          return await response.json();
        }
      } catch (error) {
        console.error(`Error validating challenge ${challengeId}:`, error);
      }
      return null;
    }

    // Calculate overall status
    const statusValues = [
      systemStatus.environmental_monitoring,
      systemStatus.species_database,
      systemStatus.feeding_system,
      systemStatus.remote_monitoring,
      systemStatus.analysis_engine
    ];

    if (statusValues.some(status => status === 'error' || status === 'critical' || status === 'offline')) {
      systemStatus.overall_status = 'critical';
    } else if (statusValues.some(status => status === 'degraded')) {
      systemStatus.overall_status = 'degraded';
    } else if (statusValues.every(status => status === 'normal' || status === 'online' || status === 'operational')) {
      systemStatus.overall_status = 'operational';
    } else {
      systemStatus.overall_status = 'unknown';
    }
    
    // Save the updated status to localStorage
    localStorage.setItem('system_status', JSON.stringify(systemStatus));

    return systemStatus;
  } catch (error) {
    console.error('Error aggregating system status:', error);
    throw error;
  }
};

export const getChallenges = async () => {
  try {
    const response = await fetch(`${API_URLS.AQUA_BRAIN}/challenges/current`);
    if (!response.ok) throw new Error('Failed to fetch challenges');
    const data = await response.json();
    
    // Debug log to inspect challenge content
    if (data && data.challenges) {
      console.log('Challenge data received:', data);
      
      // Create a consistent function to normalize all markdown content from the API
      data.challenges = data.challenges.map(challenge => {
        // Helper function to normalize code blocks to use proper markdown syntax
        const normalizeMarkdownContent = (content) => {
          if (!content) return content;
          
          // Since the backend now handles triple quotes properly, we only need to
          // ensure proper formatting and spacing of existing code blocks
          let result = content;
          
          // Improve spacing around code blocks for better readability
          result = result
            // Ensure language specifier is followed by a newline
            .replace(/```([a-z]*)\s*/g, '```$1\n')
            
            // Ensure closing code fence has proper spacing
            .replace(/\s*```/g, '\n```');
          
          // Fix improper indentation and formatting inside code blocks
          const lines = result.split('\n');
          let inCodeBlock = false;
          
          for (let i = 0; i < lines.length; i++) {
            if (lines[i].trim().startsWith('```')) {
              inCodeBlock = !inCodeBlock;
              
              // Add a language identifier for code blocks without one (defaults to text)
              if (inCodeBlock && lines[i].trim() === '```') {
                lines[i] = '```text';
              }
            } else if (inCodeBlock) {
              // Normalize indentation inside code blocks (convert tabs to spaces)
              lines[i] = lines[i].replace(/\t/g, '  ');
            }
          }
          
          return lines.join('\n');
        };
        
        // Apply fixes to solution
        if (challenge.solution) {
          if (typeof challenge.solution === 'string') {
            challenge.solution = normalizeMarkdownContent(challenge.solution);
            console.log(`Fixed code blocks in solution for challenge ${challenge.id}`);
          } else if (challenge.solution.explanation) {
            challenge.solution.explanation = normalizeMarkdownContent(challenge.solution.explanation);
            console.log(`Fixed code blocks in solution explanation for challenge ${challenge.id}`);
          }
        }
        
        // Apply fixes to description
        if (challenge.description) {
          challenge.description = normalizeMarkdownContent(challenge.description);
          console.log(`Fixed code blocks in description for challenge ${challenge.id}`);
        }
        
        // Apply fixes to hint
        if (challenge.hint) {
          challenge.hint = normalizeMarkdownContent(challenge.hint);
          console.log(`Fixed code blocks in hint for challenge ${challenge.id}`);
        }
        
        return challenge;
      });
    }
    
    return data;
  } catch (error) {
    console.error('Error fetching challenges:', error);
    throw error;
  }
};

// Challenge validation function - confirmed working with our curl test
export const validateChallengeSolution = async (challengeId: number | string) => {
  try {
    // From our curl test, we know this endpoint is working correctly
    console.log(`Validating solution for challenge ${challengeId} at /api/challenges/${challengeId}/validate`);
    
    // Use direct endpoint path without service prefix as confirmed by curl test
    const response = await fetch(`/api/challenges/${challengeId}/validate`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json'
      }
    });
    
    if (!response.ok) {
      console.error(`Server returned status ${response.status} for challenge validation`);
      throw new Error(`Failed to validate solution for challenge ${challengeId}`);
    }
    
    const validationData = await response.json();
    
    // Format the response to match what the frontend expects
    return {
      implementation: {
        valid: validationData.valid,
        message: validationData.message
      },
      system_status: validationData.system_component ? {
        [validationData.system_component.name]: validationData.system_component.status
      } : null,
      details: validationData.details || null
    };
  } catch (error) {
    console.error(`Error validating solution for challenge ${challengeId}:`, error);
    throw error;
  }
};

export const getAllTankAnalysis = async () => {
  try {
    const response = await fetch(`${API_URLS.AQUA_BRAIN}/analysis/tanks`);
    if (!response.ok) throw new Error('Failed to fetch tank analysis summary');
    return await response.json();
  } catch (error) {
    console.error('Error fetching tank analysis summary:', error);
    throw error;
  }
};

export const getTankAnalysis = async (tankId: string) => {
  try {
    const response = await fetch(`${API_URLS.AQUA_BRAIN}/analysis/tanks/${tankId}`);
    if (!response.ok) throw new Error(`Failed to fetch analysis for tank ${tankId}`);
    return await response.json();
  } catch (error) {
    console.error(`Error fetching analysis for tank ${tankId}:`, error);
    throw error;
  }
};

// Aqua-Monitor API
export const getAllTanks = async () => {
  try {
    const response = await fetch(`${API_URLS.AQUA_MONITOR}/tanks`);
    if (!response.ok) throw new Error('Failed to fetch tanks');
    return await response.json();
  } catch (error) {
    console.error('Error fetching tanks:', error);
    throw error;
  }
};

export const getTankReadings = async (tankId: string) => {
  try {
    const response = await fetch(`${API_URLS.AQUA_MONITOR}/tanks/${tankId}/readings`);
    if (!response.ok) throw new Error('Failed to fetch tank readings');
    return await response.json();
  } catch (error) {
    console.error('Error fetching tank readings:', error);
    throw error;
  }
};

export const getSensorStatus = async () => {
  try {
    const response = await fetch(`${API_URLS.AQUA_MONITOR}/sensors/status`);
    if (!response.ok) throw new Error('Failed to fetch sensor status');
    return await response.json();
  } catch (error) {
    console.error('Error fetching sensor status:', error);
    throw error;
  }
};

// Species-Hub API
export const getAllSpecies = async () => {
  try {
    const response = await fetch(`${API_URLS.SPECIES_HUB}/species`);
    if (!response.ok) throw new Error('Failed to fetch species');
    return await response.json();
  } catch (error) {
    console.error('Error fetching species:', error);
    throw error;
  }
};

export const searchSpeciesByName = async (searchTerm: string) => {
  try {
    const response = await fetch(`${API_URLS.SPECIES_HUB}/species?name=${encodeURIComponent(searchTerm)}`);
    if (!response.ok) throw new Error('Failed to search species by name');
    return await response.json();
  } catch (error) {
    console.error('Error searching species by name:', error);
    throw error;
  }
};

export const searchSpeciesByScientificName = async (searchTerm: string) => {
  try {
    const response = await fetch(`${API_URLS.SPECIES_HUB}/species?scientific_name=${encodeURIComponent(searchTerm)}`);
    if (!response.ok) throw new Error('Failed to search species by scientific name');
    return await response.json();
  } catch (error) {
    console.error('Error searching species by scientific name:', error);
    throw error;
  }
};

export const getSpeciesDetails = async (id: string) => {
  try {
    const response = await fetch(`${API_URLS.SPECIES_HUB}/species/${id}`);
    if (!response.ok) throw new Error('Failed to fetch species details');
    return await response.json();
  } catch (error) {
    console.error('Error fetching species details:', error);
    throw error;
  }
};

export const getSpeciesFeedingSchedule = async (speciesId: string, tankType?: string, customDiet?: string) => {
  try {
    let url = `${API_URLS.SPECIES_HUB}/species/${speciesId}/feeding-schedule`;
    
    const params = new URLSearchParams();
    if (tankType) params.append('tank_type', tankType);
    if (customDiet) params.append('custom_diet', customDiet);
    
    if (params.toString()) {
      url += `?${params.toString()}`;
    }
    
    const response = await fetch(url);
    if (!response.ok) throw new Error('Failed to fetch feeding schedule');
    return await response.json();
  } catch (error) {
    console.error('Error fetching feeding schedule:', error);
    throw error;
  }
};

// Original getSpecies function for backwards compatibility
export const getSpecies = async () => {
  return getAllSpecies();
};

// Original getFeedingSchedule function for backwards compatibility
export const getFeedingSchedule = async () => {
  try {
    const response = await fetch(`${API_URLS.SPECIES_HUB}/feeding/schedule`);
    if (!response.ok) throw new Error('Failed to fetch feeding schedule');
    return await response.json();
  } catch (error) {
    console.error('Error fetching feeding schedule:', error);
    throw error;
  }
};
