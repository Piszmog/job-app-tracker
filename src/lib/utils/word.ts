export const fromLowercaseToCapitalized = (word: string): String => {
	return word.charAt(0).toUpperCase() + word.slice(1);
};
