export const splitIntoWords = (text: string): string[] => text.trim().split(/\s+/).filter(Boolean);
