import {
  smarthome,
  SmartHomeV1ExecuteRequestCommands,
  SmartHomeV1ExecuteResponseCommands,
} from 'actions-on-google';
import { extractJWTToken } from '@/utils';
import { verifyToken } from '@/utils/token';
import { fetchTokenUUID } from '@/database/redis';
import { DeviceData, findDevices } from '@/database/mongo';

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
          id: device._id as string, // that comes from DB, it must be defined
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
  const deviceIDs = body.inputs[0].payload.devices.map((device) => device.id);
  const devices = await findDevices(deviceIDs);
  const payloadDevices = new Map<string, DeviceData>();
  devices.forEach((device) => {
    payloadDevices.set(device._id as string, device.data);
  });
  console.log({ payloadDevices });

  return {
    requestId: body.requestId,
    payload: {
      devices: payloadDevices,
    },
  };
});

const executeCommand = (
  cmd: SmartHomeV1ExecuteRequestCommands,
): SmartHomeV1ExecuteResponseCommands => {
  return {
    ids: cmd.devices.map((device) => device.id),
    status: 'SUCCESS',
    states: {
      on: true,
      online: true,
    },
  };
};

app.onExecute(async (body, headers) => {
  body.inputs[0].payload.commands;
  const responses = body.inputs[0].payload.commands.map((cmd) =>
    executeCommand(cmd),
  );

  return {
    requestId: body.requestId,
    payload: {
      commands: responses,
    },
  };
});

export default app;
