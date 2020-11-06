import { GeoPoint } from '@houseflow/types';
import { message } from 'antd';

export const getGeoPoint = async (): Promise<GeoPoint | undefined> => {
  const promise = () =>
    new Promise<GeoPoint>((resolve, reject) => {
      const successCallback = (position: Position) => {
        resolve({
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
        });
      };
      const errorCallback = (positionError: PositionError) => {
        reject(positionError);
      };
      navigator.geolocation.getCurrentPosition(successCallback, errorCallback);
    });
  try {
    return await promise();
  } catch (tempError) {
    const e: PositionError = tempError;
    if (e.code === 1) {
      message.error(
        'Please accept location services, we use it only to guarantee devices security',
      );
    } else if (e.code === 2) {
      message.error('Error occured when trying to retreive location, please try different browser');
    } else if (e.code === 2) {
      message.error('Timed out when trying to retreive location, please try different browser');
    } else {
      message.error('Unknown error when trying to retreive location');
    }
    console.log(e);
    return undefined;
  }
};
