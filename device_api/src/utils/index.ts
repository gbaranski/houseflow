export const getRandomShortUid = (): string => {
  return Math.random().toString(16).substr(2, 8);
};
