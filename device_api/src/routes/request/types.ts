export interface DeviceRequestUser {
  token: string;
}
export interface DeviceRequestDevice {
  uid: string;
  gpio: number;
  action: 'trigger' | 'toggle';
  data: string | undefined; // json encoded
}

export interface DeviceRequest {
  user: DeviceRequestUser;
  device: DeviceRequestDevice;
}

export const isValidDeviceRequest = (
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  input: Record<any, any>,
): input is DeviceRequest => {
  const missingProperties = Object.keys(input)
    .filter((key) => input[key] === undefined)
    .map((key) => key as keyof DeviceRequest)
    .map((key) => new Error(`Document is missing ${key} ${input[key]}`));

  return missingProperties.length === 0;
};
