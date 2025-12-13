/**
 * Get user's current coordinates using browser Geolocation API
 */
export async function getCurrentPosition(): Promise<{
	latitude: number;
	longitude: number;
}> {
	return new Promise((resolve, reject) => {
		if (!navigator.geolocation) {
			reject(new Error('Geolocation is not supported by your browser'));
			return;
		}

		navigator.geolocation.getCurrentPosition(
			(position) => {
				resolve({
					latitude: position.coords.latitude,
					longitude: position.coords.longitude
				});
			},
			(error) => {
				let errorMessage = 'Failed to get location. ';
				
				switch (error.code) {
					case error.PERMISSION_DENIED:
						errorMessage += 'Please enable Location Services in Windows Settings:\n' +
							'Settings → Privacy & Security → Location → Turn ON "Location services"\n' +
							'Then allow this app to access location.';
						break;
					case error.POSITION_UNAVAILABLE:
						errorMessage += 'Location information is unavailable.';
						break;
					case error.TIMEOUT:
						errorMessage += 'Location request timed out.';
						break;
					default:
						errorMessage += error.message;
				}
				
				reject(new Error(errorMessage));
			},
			{
				enableHighAccuracy: true,
				timeout: 15000,
				maximumAge: 0
			}
		);
	});
}

/**
 * Get location from IP geolocation API (fallback)
 */
export async function getLocationFromIP(): Promise<{
	latitude: number;
	longitude: number;
	city: string;
	country: string;
}> {
	try {
		// Use ip-api.com which doesn't require API key and has no CORS issues
		const response = await fetch('http://ip-api.com/json/');
		if (!response.ok) {
			throw new Error('Failed to fetch location');
		}
		
		const data = await response.json();
		
		if (data.status === 'success' && data.lat && data.lon) {
			return {
				latitude: data.lat,
				longitude: data.lon,
				city: data.city || 'Unknown',
				country: data.country || 'Unknown'
			};
		}
		
		throw new Error(data.message || 'Failed to get location from IP');
	} catch (error) {
		throw new Error('IP geolocation failed: ' + (error instanceof Error ? error.message : 'Unknown error'));
	}
}

/**
 * Check if geolocation is available
 */
export function isGeolocationAvailable(): boolean {
	return 'geolocation' in navigator;
}
