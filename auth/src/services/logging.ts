const logDate = (): string => {
  const now = new Date();
  const fixDate = (e: number) => e.toString().padStart(2, '0');

  return (
    [now.getFullYear(), now.getMonth(), now.getDate()].map(fixDate).join('/') +
    ' ' +
    [now.getHours(), now.getMinutes(), now.getSeconds()].map(fixDate).join(':')
  );
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const log = (...data: any[]): void =>
  console.log(logDate, ' - ', ...data);
