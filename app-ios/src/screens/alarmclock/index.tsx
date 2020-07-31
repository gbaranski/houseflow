import * as React from 'react';
import MaterialIcons from 'react-native-vector-icons/MaterialCommunityIcons';
import DateTimePicker from '@react-native-community/datetimepicker';
import {
  Button,
  ButtonGroup,
  Layout,
  Text,
  Modal,
  Card,
} from '@ui-kitten/components';
import styleSheet from '../../styles';
import {View, Alert} from 'react-native';

import {useEffect, useState} from 'react';
import {useInterval, getData, fetchUrl, getRemoteData} from '../../helpers';
import {
  switchIconOn,
  switchIconOff,
  testAlarmIcon,
  addAlarmIcon,
  refreshIcon,
} from '../../assets/icons';
import {DeviceTypes} from '../../types';

export default function Alarmclock() {
  const [remoteData, setRemoteData] = React.useState({
    currentTime: '',
    alarmTime: '',
    remainingTime: '',
    alarmState: 0,
    temperature: 0,
    humidity: 0,
    heatIndex: 0,
  });
  const [isModalVisible, setModalVisiblity] = useState(false);

  async function getAndSetData() {
    setModalVisiblity(true);
    getRemoteData(DeviceTypes.ALARMCLOCK).then(async (response) => {
      if (response.ok) {
        setModalVisiblity(false);
        setRemoteData(JSON.parse(await response.json()));
      } else {
        Alert.alert('Response is not okay');
      }
    });
  }

  const [isTimePickerVisible, setTimePickerVisiblity] = useState(false);
  const [outputTime, setOutputTime] = useState({
    hour: '',
    minute: '',
  });
  const [time, setTime] = useState(new Date());
  const onTimeChange = (event: Event, selectedTime: any) => {
    setTime(selectedTime || time);
    setOutputTime({
      hour: ('0' + selectedTime.getHours()).slice(-2),
      minute: ('0' + selectedTime.getMinutes()).slice(-2),
    });
  };

  useInterval(() => {
    getRemoteData(DeviceTypes.ALARMCLOCK).then(async (response) => {
      if (response.ok) {
        setRemoteData(JSON.parse(await response.json()));
      } else {
        console.warn('Wrong response');
      }
    });
  }, 1000);

  useEffect(() => {
    getData()
      .then((credentials) => {
        if (!credentials || !credentials.username || !credentials.password) {
          Alert.alert('Alert!', 'Login or password are undefined values');
        }
      })
      .catch(console.error);
  }, []);

  return (
    <Layout style={styleSheet.basicLayout}>
      <View style={styleSheet.rowFlex}>
        <Text category="h6" style={styleSheet.aboveRemainingTimeText}>
          {'  '}
          <MaterialIcons name="clock" size={18} /> REMAINING TIME
        </Text>
        <Text category="h6" style={styleSheet.aboveAlarmTimeText}>
          ALARM TIME <MaterialIcons name="clock-outline" size={18} />{' '}
        </Text>
      </View>

      <View style={styleSheet.rowFlex}>
        <Text style={styleSheet.remainingTimeText}>
          {remoteData.remainingTime}
        </Text>
        <Text style={styleSheet.alarmTimeText}>{remoteData.alarmTime}</Text>
      </View>

      <View style={styleSheet.rowFlex}>
        <Text category="h6" style={styleSheet.aboveTemperatureText}>
          <MaterialIcons name="thermometer" size={20} />
          TEMPERATURE
        </Text>
        <Text category="h6" style={styleSheet.aboveHumidityText}>
          HUMIDITY
          <MaterialIcons name="water-outline" size={20} />
        </Text>
      </View>
      <View style={styleSheet.rowFlex}>
        <Text style={styleSheet.temperatureText}>
          {remoteData.temperature}°C
        </Text>
        <Text style={styleSheet.humidityText}>{remoteData.humidity}%</Text>
      </View>

      <ButtonGroup style={styleSheet.buttonGroup}>
        <Button
          accessoryRight={testAlarmIcon}
          onPress={async () => {
            setModalVisiblity(true);
            const headers = new Headers();
            fetchUrl('/alarmclock/testSiren', headers).then(() => {
              setModalVisiblity(false);
            });
          }}>
          Test alarm
        </Button>
        <Button
          accessoryRight={addAlarmIcon}
          onPress={() => {
            setTimePickerVisiblity(true);
          }}>
          Set Alarm
        </Button>
        <Button
          accessoryRight={refreshIcon}
          onPress={async () => {
            getAndSetData();
          }}>
          Fetch
        </Button>
      </ButtonGroup>

      <Button
        style={styleSheet.switchStateButton}
        accessoryRight={remoteData.alarmState ? switchIconOn : switchIconOff}
        onPress={async () => {
          setModalVisiblity(true);
          const headers = new Headers();
          headers.append('state', `${remoteData.alarmState ? 0 : 1}`);
          fetchUrl('/alarmclock/switchState', headers).then(() => {
            setModalVisiblity(false);
            getAndSetData();
          });
        }}>
        Switch alarm state
      </Button>

      <Modal visible={isModalVisible} backdropStyle={styleSheet.modalBackdrop}>
        <Card disabled={true}>
          <Text>Please wait for request to complete.{'\n'}</Text>
          <Button onPress={() => setModalVisiblity(false)}>CANCEL</Button>
        </Card>
      </Modal>

      {isTimePickerVisible && (
        <View style={styleSheet.timePickerView}>
          <DateTimePicker
            testID="dateTimePicker"
            timeZoneOffsetInMinutes={120}
            value={time}
            onChange={onTimeChange}
            mode={'time'}
            is24Hour={true}
            display="default"
          />
          <Button
            onPress={async () => {
              setTimePickerVisiblity(false);
              setModalVisiblity(true);
              const headers = new Headers();
              headers.append('time', `${outputTime.hour}:${outputTime.minute}`);
              fetchUrl('/alarmclock/setTime', headers).then(async () => {
                getAndSetData().then(() => setModalVisiblity(false));
              });
            }}>
            Done
          </Button>
        </View>
      )}
    </Layout>
  );
}
