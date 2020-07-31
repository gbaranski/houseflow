import React, {useState} from 'react';
import {Layout, Input, Text, Button, Icon} from '@ui-kitten/components';
import {AlertIcon} from '../../assets/icons';
import {TouchableWithoutFeedback, StyleSheet, Alert} from 'react-native';
import {authMe, saveData} from '../../helpers';

export default function LoginPage(props: any) {
  const [username, setUsername] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [secureTextEntry, setSecureTextEntry] = useState<boolean>(true);

  const renderIcon = (_props: any) => (
    <TouchableWithoutFeedback
      onPress={() =>
        secureTextEntry ? setSecureTextEntry(false) : setSecureTextEntry(true)
      }>
      <Icon {..._props} name={secureTextEntry ? 'eye-off' : 'eye'} />
    </TouchableWithoutFeedback>
  );

  const saveEntry = async () => {
    const headers = new Headers();
    headers.append('username', username);
    headers.append('password', password);
    const statusCode = await authMe(headers);

    if (statusCode === 200) {
      if (
        await saveData({
          username,
          password,
        })
      ) {
        props.setLoggedIn(true);
      } else {
        Alert.alert('Info', 'Error occured with saving, try again');
      }
    } else if (statusCode === 401) {
      Alert.alert('Info', "Sorry, you're not authenticated");
    } else {
      Alert.alert('Info', 'Error! Try again later');
    }
  };
  const settingsLayout = StyleSheet.create({
    settingsLayout: {
      flex: 1,
      paddingTop: 60,
      alignItems: 'center',
      padding: 10,
    },
    loginButton: {
      marginTop: 20,
      width: '50%',
    },
  });

  return (
    <Layout style={settingsLayout.settingsLayout}>
      <Text category="h1">Log in</Text>
      <Input
        placeholder="Your username here"
        label="  USERNAME"
        onChangeText={setUsername}
        value={username}
      />
      <Input
        value={password}
        label="PASSWORD"
        placeholder="Your password here"
        caption="Long mixed password"
        accessoryRight={renderIcon}
        captionIcon={AlertIcon}
        secureTextEntry={secureTextEntry}
        onChangeText={(nextValue) => setPassword(nextValue)}
      />
      <Button style={settingsLayout.loginButton} onPress={saveEntry}>
        LOGIN
      </Button>
    </Layout>
  );
}
