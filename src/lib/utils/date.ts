export const calculateAgeInDays = (start: Date, end: Date): number => {
	const diff = Math.abs(start.getTime() - end.getTime());
	return Math.ceil(diff / (1000 * 60 * 60 * 24));
};
