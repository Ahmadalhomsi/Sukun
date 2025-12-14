export function formatTime(time: string): string {
	try {
		const [hours, minutes] = time.split(':');
		const hour = parseInt(hours);
		const period = hour >= 12 ? 'PM' : 'AM';
		const displayHour = hour % 12 || 12;
		return `${displayHour}:${minutes} ${period}`;
	} catch {
		return time;
	}
}

export function formatDate(date: string, locale: string = 'en-US'): string {
	try {
		const d = new Date(date);
		return d.toLocaleDateString(locale, {
			weekday: 'long',
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	} catch {
		return date;
	}
}

export function isTimeInPast(time: string): boolean {
	try {
		const now = new Date();
		const [hours, minutes] = time.split(':').map(Number);
		const prayerTime = new Date();
		prayerTime.setHours(hours, minutes, 0, 0);
		return prayerTime < now;
	} catch {
		return false;
	}
}

export function getTimeUntil(time: string, lang: 'en' | 'tr' = 'en'): string {
	try {
		const now = new Date();
		const [hours, minutes] = time.split(':').map(Number);
		const prayerTime = new Date();
		prayerTime.setHours(hours, minutes, 0, 0);

		if (prayerTime < now) {
			return lang === 'tr' ? 'Geçti' : 'Passed';
		}

		const diff = prayerTime.getTime() - now.getTime();
		const hoursLeft = Math.floor(diff / (1000 * 60 * 60));
		const minutesLeft = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
		const secondsLeft = Math.floor((diff % (1000 * 60)) / 1000);

		if (hoursLeft === 0 && minutesLeft === 0) {
			return lang === 'tr' ? `${secondsLeft} saniye içinde` : `in ${secondsLeft}s`;
		}
		if (hoursLeft === 0) {
			return lang === 'tr' ? `${minutesLeft} dk ${secondsLeft} sn içinde` : `in ${minutesLeft}m ${secondsLeft}s`;
		}
		return lang === 'tr'
			? `${hoursLeft} sa ${minutesLeft} dk ${secondsLeft} sn içinde`
			: `in ${hoursLeft}h ${minutesLeft}m ${secondsLeft}s`;
	} catch {
		return '';
	}
}

export function getTodayDate(): string {
	const now = new Date();
	return now.toISOString().split('T')[0];
}
