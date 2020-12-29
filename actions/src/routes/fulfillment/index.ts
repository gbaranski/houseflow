import {
  smarthome,
  SmartHomeV1ExecuteRequestExecution,
  SmartHomeV1ExecuteResponseCommands,
  SmartHomeV1SyncDevices,
} from 'actions-on-google';
import { extractJWTToken } from '@/utils';
import { verifyToken } from '@/utils/token';
import { fetchTokenUUID } from '@/database/redis';
import { findDevices, getUser } from '@/database/mongo';
import { sendDeviceMessage } from '@/services/mqtt';

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
  let payloadDevices = {};
  devices.forEach((device) => {
    payloadDevices = {
      ...payloadDevices,
      [device._id as string]: {
        ...device.data,
        status: 'SUCCESS',
      },
    };
  });

  return {
    requestId: body.requestId,
    payload: {
      devices: payloadDevices,
    },
  };
});

const executeOnDevice = (
  deviceID: string,
  executions: SmartHomeV1ExecuteRequestExecution[],
): Promise<SmartHomeV1ExecuteResponseCommands>[] =>
  executions.map(
    (execution): Promise<SmartHomeV1ExecuteResponseCommands> => {
      console.log(`Executing ${execution.command} on ${deviceID}`);
      return sendDeviceMessage(deviceID, 'OnOff', execution.params || {});
    },
  );

app.onExecute(async (body, headers) => {
  const commands = body.inputs[0].payload.commands
    .map((cmd) => {
      return cmd.devices.map((device) =>
        executeOnDevice(device.id, cmd.execution),
      );
    })
    .flat(2);

  return {
    requestId: body.requestId,
    payload: {
      commands: await Promise.all(commands),
    },
  };
});

export default app;
