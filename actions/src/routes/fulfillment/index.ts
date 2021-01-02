import {
  smarthome,
  SmartHomeV1ExecuteResponseCommands,
  SmartHomeV1SyncDevices,
} from 'actions-on-google';
import { convertArrayToObject, extractJWTToken } from '@/utils';
import { verifyToken } from '@/utils/token';
import { fetchTokenUUID } from '@/database/redis';
import { findDevices, getUser } from '@/database/mongo';
import { sendCommand } from '@/services/mqtt';
import { ObjectId } from 'mongodb';

const serviceAccountKey = require('/app/service-account.json');

const app = smarthome({
  debug: true,
  jwt: serviceAccountKey,
});

app.onSync(async (body, headers) => {
  console.log('Received sync request');
  const token = extractJWTToken(headers);
  console.log({ token });
  const claims = await verifyToken(token);
  console.log({ claims });
  const userID = await fetchTokenUUID(claims.jti);
  console.log({ userID });
  const user = await getUser(userID);
  console.log({ user });
  const devices = await findDevices(user.devices);
  console.log({ devices });

  return {
    requestId: body.requestId,
    payload: {
      agentUserId: userID,
      devices: devices.map(
        (device): SmartHomeV1SyncDevices => ({
          id: device._id?.toHexString() as string, // that comes from DB, it must be defined
          type: device.type,
          traits: device.traits,
          name: device.name,
          willReportState: device.willReportState,
          deviceInfo: device.deviceInfo,
          roomHint: device.roomHint,
        }),
      ),
    },
  };
});

app.onQuery(async (body, headers) => {
  const token = extractJWTToken(headers);
  const claims = await verifyToken(token);
  const userID = await fetchTokenUUID(claims.jti);
  const user = await getUser(userID);
  const userDevices = await findDevices(user.devices);
  const requestedDevicesIDs = body.inputs[0].payload.devices.map(
    (device) => new ObjectId(device.id),
  );

  const payloadDevices = requestedDevicesIDs.map((deviceID) => {
    const dbDevice = userDevices.find((device) => device._id?.equals(deviceID));

    if (!dbDevice || !deviceID) {
      console.log({ dbDevice, deviceID, userDevices });
      return {
        status: 'ERROR',
        errorCode: 'relinkRequired',
      };
    }
    return {
      ...dbDevice.state,
      status: 'SUCCESS',
      id: deviceID,
    };
  });

  return {
    requestId: body.requestId,
    payload: {
      devices: convertArrayToObject(payloadDevices, 'id'),
    },
  };
});

const parseCommand = (cmd: string): string => cmd.split('.').reverse()[0];

app.onExecute(async (body, headers) => {
  console.log({ headers });
  const token = extractJWTToken(headers);
  const claims = await verifyToken(token);
  const userID = await fetchTokenUUID(claims.jti);
  const user = await getUser(userID);
  const userDevices = await findDevices(user.devices);

  const payload = body.inputs[0].payload;

  const commands = payload.commands
    .map(({ execution, devices }) =>
      execution.map((exec) =>
        devices.map(
          async (device): Promise<SmartHomeV1ExecuteResponseCommands> => {
            if (
              !userDevices.some(
                (userDevice) => userDevice._id?.toHexString() === device.id,
              )
            ) {
              console.log(
                `User ${userID} doesn't have devices with ID ${device.id}`,
              );
              return {
                ids: [device.id],
                status: 'ERROR',
                errorCode: 'relinkRequired',
              };
            }
            return sendCommand(
              device.id,
              parseCommand(exec.command),
              exec.params || {},
            );
          },
        ),
      ),
    )
    .flat(2);

  return {
    requestId: body.requestId,
    payload: {
      commands: await Promise.all(commands),
    },
  };
});

export default app;
