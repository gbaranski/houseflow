"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.OtherRequestsType = exports.WaterRequestType = exports.AlarmRequestType = exports.LocalIpAddress = exports.devicesSample = exports.DeviceList = void 0;
const alarmclock_1 = require("./alarmclock");
const watermixer_1 = require("./watermixer");
var DeviceList;
(function (DeviceList) {
    DeviceList["Alarmclock"] = "Alarmclock";
    DeviceList["Watermixer"] = "Watermixer";
    DeviceList["Gate"] = "Gate";
    DeviceList["Garage"] = "Garage";
})(DeviceList = exports.DeviceList || (exports.DeviceList = {}));
exports.devicesSample = {
    alarmclock: {
        status: false,
        data: alarmclock_1.alarmclockSampleData,
        ws: undefined,
        req: undefined,
    },
    watermixer: {
        status: false,
        data: watermixer_1.watermixerSampleData,
        ws: undefined,
        req: undefined,
    }
};
var LocalIpAddress;
(function (LocalIpAddress) {
    LocalIpAddress["Alarmclock"] = "192.168.1.110";
    LocalIpAddress["Watermixer"] = "192.168.1.120";
})(LocalIpAddress = exports.LocalIpAddress || (exports.LocalIpAddress = {}));
var AlarmRequestType;
(function (AlarmRequestType) {
    AlarmRequestType["GET_DATA"] = "/getESPData";
    AlarmRequestType["GET_TEMP_ARRAY"] = "/getTempArray";
    AlarmRequestType["GET_DEVICE_STATE"] = "/isDown";
    AlarmRequestType["SET_TIME"] = "/setAlarm";
    AlarmRequestType["SWITCH_STATE"] = "/setAlarmState";
    AlarmRequestType["TEST_ALARM"] = "/testAlarm";
})(AlarmRequestType = exports.AlarmRequestType || (exports.AlarmRequestType = {}));
var WaterRequestType;
(function (WaterRequestType) {
    WaterRequestType["GET_DATA"] = "/getESPData";
    WaterRequestType["START_MIXING"] = "/startMixing";
})(WaterRequestType = exports.WaterRequestType || (exports.WaterRequestType = {}));
var OtherRequestsType;
(function (OtherRequestsType) {
    OtherRequestsType["GET_DEVICES_STATUS"] = "/getDevicesStatus";
})(OtherRequestsType = exports.OtherRequestsType || (exports.OtherRequestsType = {}));
//# sourceMappingURL=other.js.map