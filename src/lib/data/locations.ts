export interface City {
	name: string;
	latitude: number;
	longitude: number;
}

export interface Country {
	name: string;
	code: string;
	cities: City[];
}

export const countries: Country[] = [
	{
		name: 'Turkey',
		code: 'TR',
		cities: [
			{ name: 'Istanbul', latitude: 41.0082, longitude: 28.9784 },
			{ name: 'Ankara', latitude: 39.9334, longitude: 32.8597 },
			{ name: 'Izmir', latitude: 38.4192, longitude: 27.1287 },
			{ name: 'Bursa', latitude: 40.1826, longitude: 29.0665 },
			{ name: 'Antalya', latitude: 36.8969, longitude: 30.7133 },
			{ name: 'Adana', latitude: 37.0000, longitude: 35.3213 },
			{ name: 'Konya', latitude: 37.8746, longitude: 32.4932 },
			{ name: 'Gaziantep', latitude: 37.0662, longitude: 37.3833 },
			{ name: 'Mersin', latitude: 36.8121, longitude: 34.6415 },
			{ name: 'Kayseri', latitude: 38.7312, longitude: 35.4787 }
		]
	},
	{
		name: 'United States',
		code: 'US',
		cities: [
			{ name: 'New York', latitude: 40.7128, longitude: -74.0060 },
			{ name: 'Los Angeles', latitude: 34.0522, longitude: -118.2437 },
			{ name: 'Chicago', latitude: 41.8781, longitude: -87.6298 },
			{ name: 'Houston', latitude: 29.7604, longitude: -95.3698 },
			{ name: 'Phoenix', latitude: 33.4484, longitude: -112.0740 },
			{ name: 'Philadelphia', latitude: 39.9526, longitude: -75.1652 },
			{ name: 'San Antonio', latitude: 29.4241, longitude: -98.4936 },
			{ name: 'San Diego', latitude: 32.7157, longitude: -117.1611 },
			{ name: 'Dallas', latitude: 32.7767, longitude: -96.7970 },
			{ name: 'San Jose', latitude: 37.3382, longitude: -121.8863 }
		]
	},
	{
		name: 'United Kingdom',
		code: 'GB',
		cities: [
			{ name: 'London', latitude: 51.5074, longitude: -0.1278 },
			{ name: 'Birmingham', latitude: 52.4862, longitude: -1.8904 },
			{ name: 'Manchester', latitude: 53.4808, longitude: -2.2426 },
			{ name: 'Glasgow', latitude: 55.8642, longitude: -4.2518 },
			{ name: 'Liverpool', latitude: 53.4084, longitude: -2.9916 },
			{ name: 'Leeds', latitude: 53.8008, longitude: -1.5491 },
			{ name: 'Edinburgh', latitude: 55.9533, longitude: -3.1883 },
			{ name: 'Bristol', latitude: 51.4545, longitude: -2.5879 },
			{ name: 'Cardiff', latitude: 51.4816, longitude: -3.1791 },
			{ name: 'Belfast', latitude: 54.5973, longitude: -5.9301 }
		]
	},
	{
		name: 'Germany',
		code: 'DE',
		cities: [
			{ name: 'Berlin', latitude: 52.5200, longitude: 13.4050 },
			{ name: 'Hamburg', latitude: 53.5511, longitude: 9.9937 },
			{ name: 'Munich', latitude: 48.1351, longitude: 11.5820 },
			{ name: 'Cologne', latitude: 50.9375, longitude: 6.9603 },
			{ name: 'Frankfurt', latitude: 50.1109, longitude: 8.6821 },
			{ name: 'Stuttgart', latitude: 48.7758, longitude: 9.1829 },
			{ name: 'Dusseldorf', latitude: 51.2277, longitude: 6.7735 },
			{ name: 'Dortmund', latitude: 51.5136, longitude: 7.4653 },
			{ name: 'Essen', latitude: 51.4556, longitude: 7.0116 },
			{ name: 'Leipzig', latitude: 51.3397, longitude: 12.3731 }
		]
	},
	{
		name: 'Saudi Arabia',
		code: 'SA',
		cities: [
			{ name: 'Riyadh', latitude: 24.7136, longitude: 46.6753 },
			{ name: 'Jeddah', latitude: 21.5433, longitude: 39.1728 },
			{ name: 'Mecca', latitude: 21.3891, longitude: 39.8579 },
			{ name: 'Medina', latitude: 24.5247, longitude: 39.5692 },
			{ name: 'Dammam', latitude: 26.4207, longitude: 50.0888 },
			{ name: 'Khobar', latitude: 26.2172, longitude: 50.1971 },
			{ name: 'Tabuk', latitude: 28.3838, longitude: 36.5550 },
			{ name: 'Buraidah', latitude: 26.3260, longitude: 43.9750 },
			{ name: 'Abha', latitude: 18.2164, longitude: 42.5053 },
			{ name: 'Najran', latitude: 17.4924, longitude: 44.1277 }
		]
	},
	{
		name: 'United Arab Emirates',
		code: 'AE',
		cities: [
			{ name: 'Dubai', latitude: 25.2048, longitude: 55.2708 },
			{ name: 'Abu Dhabi', latitude: 24.4539, longitude: 54.3773 },
			{ name: 'Sharjah', latitude: 25.3463, longitude: 55.4209 },
			{ name: 'Ajman', latitude: 25.4052, longitude: 55.5136 },
			{ name: 'Ras Al Khaimah', latitude: 25.7894, longitude: 55.9432 },
			{ name: 'Fujairah', latitude: 25.1288, longitude: 56.3265 },
			{ name: 'Umm Al Quwain', latitude: 25.5647, longitude: 55.5530 },
			{ name: 'Al Ain', latitude: 24.2075, longitude: 55.7447 }
		]
	}
];
