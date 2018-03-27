import React, { Component } from 'react';
import { Text, Button, Alert, View } from 'react-native';

export default class App extends Component {
  render() {
    return (
      <View style={{ padding: 10 }}>
        <Button
          onPress={() => { Alert.alert('You tapped the button!'); }}
          title="Press me"
        />
      </View>
    );
  }
}
