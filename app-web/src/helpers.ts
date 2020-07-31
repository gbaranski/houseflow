import {useRef, useEffect} from 'react';

export function useInterval(callback: any, delay: number) {
  const savedCallback = useRef();

  // Remember the latest callback.
  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  // Set up the interval.
  useEffect(() => {
    function tick() {
      // @ts-ignore
      savedCallback.current();
    }
    if (delay !== null) {
      let id = setInterval(tick, delay);
      return () => clearInterval(id);
    }
  }, [delay]);
}

export function formatOneDigitToDouble(string: number) {
  return ('0' + String(string)).slice(-2);
}
export function parseDateToDateString(date: Date) {
  const parsedDate = new Date(date);
  return (
    formatOneDigitToDouble(parsedDate.getHours()) +
    ':' +
    formatOneDigitToDouble(parsedDate.getMinutes()) +
    '  ' +
    formatOneDigitToDouble(parsedDate.getDate()) +
    '/' +
    formatOneDigitToDouble(parsedDate.getMonth() + 1) +
    '/' +
    parsedDate.getFullYear()
  );
}

export function capitalizeFirst(string: string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}
