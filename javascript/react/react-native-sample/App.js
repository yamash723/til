import React, { Component } from 'react';
import { Text, View, Image } from 'react-native';

export default class App extends Component {
  render() {
    return (
      <View>
        <Blink text='I love to blink.' />
        <Blink text='Yes blinking is so great.' />
        <Blink text='Why did they ever take this out of HTML.' />
        <Blink text='Look at me look at me look at me.' />
      </View>
    );
  }
}

class Blink extends Component {
  constructor(props) {
    super(props);
    this.state = {
      isShowingText: true,
    };

    // Toggle the state every second
    setInterval(() => {
      this.setState(previousState => {
        return { isShowingText: !previousState.isShowingText };
      });
    }, 1000);
  }

  render() {
    let display =this.state.isShowingText ? this.props.text: ' ';
    return (
      <Text>{display}</Text>
    );
  }
}

// class Bananas extends Component {
//     render() {
//         let pic = {
//             uri: 'https://upload.wikimedia.org/wikipedia/commons/d/de/Bananavarieties.jpg'
//         };
//         return (
//             <Image source={pic} style={{width: 193, height: 110}} />
//         );
//     }
// }

// class Greeting extends Component {
//   render() {
//     return (
//       <Text>Hello {this.props.name}</Text>
//     );
//   }
// }

// class LotsOfGreetings extends Component {
//   render() {
//     return (
//       <View style={{alignItems: 'center'}}>
//         <Greeting name='Rexxar' />
//         <Greeting name='Jaina' />
//         <Greeting name='Valeera' />
//       </View>
//     );
//   }
// }