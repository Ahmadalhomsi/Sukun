import { Coordinates, CalculationMethod, PrayerTimes, Prayer } from 'adhan';
import type { City } from '$lib/data/locations';

export interface CalculatedPrayerTimes {
	fajr: Date;
	sunrise: Date;
	dhuhr: Date;
	asr: Date;
	maghrib: Date;
	isha: Date;
}

export type CalculationMethodName = 
	| 'Turkey'
	| 'MuslimWorldLeague'
	| 'Egyptian'
	| 'Karachi'
	| 'UmmAlQura'
	| 'Dubai'
	| 'Qatar'
	| 'Kuwait'
	| 'MoonsightingCommittee'
	| 'Singapore'
	| 'NorthAmerica'
	| 'Other';

/**
 * Calculate prayer times for a given city and date using adhan-js
 */
export function calculatePrayerTimes(
	city: City,
	date: Date = new Date(),
	methodName: CalculationMethodName = 'Turkey'
): CalculatedPrayerTimes {
	const coordinates = new Coordinates(city.latitude, city.longitude);
	
	// Get calculation method based on name
	let params;
	switch (methodName) {
		case 'Turkey':
			params = CalculationMethod.Turkey();
			break;
		case 'MuslimWorldLeague':
			params = CalculationMethod.MuslimWorldLeague();
			break;
		case 'Egyptian':
			params = CalculationMethod.Egyptian();
			break;
		case 'Karachi':
			params = CalculationMethod.Karachi();
			break;
		case 'UmmAlQura':
			params = CalculationMethod.UmmAlQura();
			break;
		case 'Dubai':
			params = CalculationMethod.Dubai();
			break;
		case 'Qatar':
			params = CalculationMethod.Qatar();
			break;
		case 'Kuwait':
			params = CalculationMethod.Kuwait();
			break;
		case 'MoonsightingCommittee':
			params = CalculationMethod.MoonsightingCommittee();
			break;
		case 'Singapore':
			params = CalculationMethod.Singapore();
			break;
		case 'NorthAmerica':
			params = CalculationMethod.NorthAmerica();
			break;
		default:
			params = CalculationMethod.Turkey();
	}
	
	const prayerTimes = new PrayerTimes(coordinates, date, params);

	return {
		fajr: prayerTimes.fajr,
		sunrise: prayerTimes.sunrise,
		dhuhr: prayerTimes.dhuhr,
		asr: prayerTimes.asr,
		maghrib: prayerTimes.maghrib,
		isha: prayerTimes.isha
	};
}

/**
 * Format Date to HH:MM string
 */
export function formatPrayerTime(date: Date): string {
	const hours = date.getHours().toString().padStart(2, '0');
	const minutes = date.getMinutes().toString().padStart(2, '0');
	return `${hours}:${minutes}`;
}

/**
 * Convert calculated prayer times to database format
 */
export function convertToDbFormat(
	prayerTimes: CalculatedPrayerTimes,
	dateStr: string
): Array<{ name: string; time: string; date: string }> {
	return [
		{ name: 'Fajr', time: formatPrayerTime(prayerTimes.fajr), date: dateStr },
		{ name: 'Sunrise', time: formatPrayerTime(prayerTimes.sunrise), date: dateStr },
		{ name: 'Dhuhr', time: formatPrayerTime(prayerTimes.dhuhr), date: dateStr },
		{ name: 'Asr', time: formatPrayerTime(prayerTimes.asr), date: dateStr },
		{ name: 'Maghrib', time: formatPrayerTime(prayerTimes.maghrib), date: dateStr },
		{ name: 'Isha', time: formatPrayerTime(prayerTimes.isha), date: dateStr }
	];
}

/**
 * Get next prayer from calculated times
 */
export function getNextPrayer(prayerTimes: CalculatedPrayerTimes): {
	name: string;
	time: Date;
} | null {
	const now = new Date();
	const prayers = [
		{ name: 'Fajr', time: prayerTimes.fajr },
		{ name: 'Sunrise', time: prayerTimes.sunrise },
		{ name: 'Dhuhr', time: prayerTimes.dhuhr },
		{ name: 'Asr', time: prayerTimes.asr },
		{ name: 'Maghrib', time: prayerTimes.maghrib },
		{ name: 'Isha', time: prayerTimes.isha }
	];

	for (const prayer of prayers) {
		if (prayer.time > now) {
			return prayer;
		}
	}

	return null; // All prayers have passed
}
