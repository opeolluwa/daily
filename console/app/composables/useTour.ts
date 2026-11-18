
import { driver, type DriveStep, type Driver, type Config } from "driver.js";
import "driver.js/dist/driver.css";

export interface TourOptions {
  steps: DriveStep[];
  onComplete?: () => void;
}

export function useTour(options: TourOptions) {
  let instance: Driver | undefined;

  function start(stepIndex = 0) {
    if (import.meta.server) return;

    instance?.destroy();

    const config: Config = {
      showProgress: true,
      allowClose: true,
      overlayOpacity: 0.65,
      smoothScroll: true,
      stagePadding: 8,
      stageRadius: 8,
      nextBtnText: "Continue",
      prevBtnText: "Back",
      doneBtnText: "Finish",
      steps: options.steps,
      onDestroyed: () => {
        options.onComplete?.();
      },
    };

    instance = driver(config);
    instance.drive(stepIndex);
  }

  function destroy() {
    instance?.destroy();
    instance = undefined;
  }

  return {
    start,
    destroy,
  };
}
