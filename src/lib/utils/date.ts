export const calculateAgeInDays = (date: Date): number => {
	const today = new Date();
	const timeDifference = today.getTime() - date.getTime();
	return Math.floor(timeDifference / (1000 * 3600 * 24));
};
