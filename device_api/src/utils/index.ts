export const getRandomShortUid = (): string => {
  return Math.random().toString(16).substr(2, 8);
};

export const validateType = <T>(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  input: Record<any, any>,
): input is T => {
  const missingProperties = Object.keys(input)
    .filter((key) => input[key] === undefined)
    .map((key) => key as keyof T)
    .map((key) => new Error(`Document is missing ${key} ${input[key]}`));

  return missingProperties.length === 0;
};
