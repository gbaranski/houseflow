import React, {useEffect, useState} from 'react';
import * as eva from '@eva-design/eva';
import {NavigationContainer} from '@react-navigation/native';
import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';
import {EvaIconsPack} from '@ui-kitten/eva-icons';
import {Alert} from 'react-native';
import messaging from '@react-native-firebase/messaging';
import {AppRegistry} from 'react-native';
import {
  BottomNavigation,
  BottomNavigationTab,
  ApplicationProvider,
  IconRegistry,
} from '@ui-kitten/components';
import Alarmclock from './screens/alarmclock';
import Watermixer from './screens/watermixer';
import Settings from './screens/settings';
import {
  AlarmIconFill,
  AlarmIcon,
  WaterIcon,
  WaterIconFill,
  SettingsIconFill,
  SettingsIcon,
} from './assets/icons';
import {getData, authMe} from './helpers';
import {Credentials} from './types';
import LoginPage from './screens/login';

const {Navigator, Screen} = createBottomTabNavigator();

async function requestUserPermission() {
  const authStatus = await messaging().requestPermission();
  const enabled =
    authStatus === messaging.AuthorizationStatus.AUTHORIZED ||
    authStatus === messaging.AuthorizationStatus.PROVISIONAL;

  if (enabled) {
    console.log('Authorization status:', authStatus);
  }
}

// Register background handler
messaging().setBackgroundMessageHandler(async (remoteMessage) => {
  console.log('Message handled in the background!', remoteMessage);
});

AppRegistry.registerComponent('app', () => App);

export default function App() {
  const [isLoggedIn, setLoggedIn] = useState<boolean>(false);

  useEffect(() => {
    requestUserPermission();

    getData().then(async (_credentials: Credentials) => {
      if (_credentials && _credentials.username && _credentials.password) {
        const headers = new Headers();
        headers.append('username', _credentials.username);
        headers.append('password', _credentials.password);

        const statusCode = await authMe(headers);
        if (statusCode === 200) {
          setLoggedIn(true);
          if (_credentials.username === 'gbaranski') {
            messaging()
              .subscribeToTopic('admin')
              .then(() => console.log('Subscribed to topic admin'));
          } else {
            messaging()
              .unsubscribeFromTopic('admin')
              .then(() => console.log('unsubscribed from topic admin'));
          }
        } else if (statusCode === 401) {
          Alert.alert('Info', "Sorry, you're not authenticated");
        } else {
          Alert.alert('Info', 'Error! Try again later');
        }
      }
    });

    const unsubscribe = messaging().onMessage(async (remoteMessage) => {
      console.log(remoteMessage);
      if (remoteMessage.data) {
        Alert.alert(remoteMessage.data.title, remoteMessage.data.body);
      }
    });
    return unsubscribe;
  }, []);

  return (
    <>
      <IconRegistry icons={EvaIconsPack} />
      <ApplicationProvider {...eva} theme={{...eva.dark}}>
        {!isLoggedIn && <LoginPage setLoggedIn={setLoggedIn} />}
        {isLoggedIn && (
          <NavigationContainer>
            <TabNavigator />
          </NavigationContainer>
        )}
      </ApplicationProvider>
    </>
  );
}

export function TabNavigator() {
  return (
    <Navigator tabBar={(props: any) => <BottomTabBar {...props} />}>
      <Screen name="Alarmclock" component={Alarmclock} />
      <Screen name="Watermixer" component={Watermixer} />
      <Screen name="Settings" component={Settings} />
    </Navigator>
  );
}

const BottomTabBar = ({navigation, state}: any) => (
  <BottomNavigation
    selectedIndex={state.index}
    onSelect={(index) => navigation.navigate(state.routeNames[index])}>
    <BottomNavigationTab
      icon={state.index === 0 ? AlarmIconFill : AlarmIcon}
      title="ALARMCLOCK"
    />
    <BottomNavigationTab
      icon={state.index === 1 ? WaterIconFill : WaterIcon}
      title="WATERMIXER"
    />
    <BottomNavigationTab
      icon={state.index === 2 ? SettingsIconFill : SettingsIcon}
      title="SETTINGS"
    />
  </BottomNavigation>
);
