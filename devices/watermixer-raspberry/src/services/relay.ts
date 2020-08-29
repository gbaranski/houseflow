const TOTAL_SECONDS = 600;

export let remainingSeconds = 0;
export let isTimerOn = false;

export const startMixing = (): void => {
    console.log("Start mixing");
    remainingSeconds = TOTAL_SECONDS;
    if (isTimerOn) return;

    isTimerOn = true;
    const decrementInterval = setInterval(() => {
        remainingSeconds -= 1;
        if (remainingSeconds < 1) {
            isTimerOn = false;
            clearInterval(decrementInterval);
        }
    }, 1000)

}