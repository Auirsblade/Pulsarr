export const formatDate = (dateStr: string): string => {
    const date = new Date(dateStr);
    return date.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
};

export const formatRatingValue = (value: string): string => {
    const num = parseFloat(value);
    if (isNaN(num)) return value;
    return parseFloat(num.toFixed(2)).toString();
};
