export class DeviceOfflineException extends Error {
  constructor(m: string) {
    super(m);

    // Set the prototype explicitly.
    Object.setPrototypeOf(this, DeviceOfflineException.prototype);
  }
}
