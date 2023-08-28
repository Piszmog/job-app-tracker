export const calculateAgeInDays = (date: Date): number => {
	const today = new Date();
	const timeDifference = today.getTime() - date.getTime();
	const value = Math.floor(timeDifference / (1000 * 3600 * 24));
	if (value < 0) {
		return 0;
	}
	return value;
};
