import * as React from 'react';
import {Layout, Text, Modal, Card, Button} from '@ui-kitten/components';
import styleSheet from '../../styles';
import {useEffect} from 'react';
import {
  useInterval,
  formatTotalSeconds,
  getData,
  fetchUrl,
  getRemoteData,
} from '../../helpers';
import {switchIconOn, switchIconOff} from '../../assets/icons';
import {DeviceTypes} from '../../types';

export default function Watermixer() {
  const [remoteData, setRemoteData] = React.useState({
    remainingSeconds: 0,
    isTimerOn: 0,
  });
  const [isModalVisible, setModalVisiblity] = React.useState(false);

  useEffect(() => {
    getData()
      .then((credentials) => {
        if (!credentials || !credentials.username || !credentials.password) {
          console.warn('Login or password are undefined values');
        }
      })
      .catch(console.error);
  }, []);

  useInterval(() => {
    getRemoteData(DeviceTypes.WATERMIXER).then(async (response) => {
      if (response.ok) {
        setRemoteData(JSON.parse(await response.json()));
      } else {
        console.warn('Wrong request');
      }
    });
    // Alert.alert(String(remoteData.remainingSeconds));
  }, 1000);

  return (
    <Layout style={styleSheet.basicLayout}>
      <Text category="h1">REMAINING TIME</Text>
      <Text category="h5">
        {formatTotalSeconds(remoteData.remainingSeconds)}
      </Text>
      <Button
        style={styleSheet.switchStateButton}
        accessoryRight={remoteData.isTimerOn ? switchIconOn : switchIconOff}
        onPress={async () => {
          setModalVisiblity(true);
          const headers = new Headers();
          fetchUrl('/watermixer/startMixing', headers).then(() => {
            setModalVisiblity(false);
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
    </Layout>
  );
}
