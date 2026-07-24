export const truncate = (str: string, size: number) =>
	str.length > size ? str.substring(0, size) + '...' : str;

export const formatTimestamp = (timestamp: string) =>
	new Intl.DateTimeFormat('en-US', {
		dateStyle: 'medium',
		timeStyle: 'short',
	}).format(new Date(timestamp));
