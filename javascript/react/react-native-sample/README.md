React-native-sample
======================

ReactNativeの公式チュートリアル実施メモ

チュートリアル
-----------------

https://facebook.github.io/react-native/docs/tutorial.html

メモ
--------------------

### View

* 各コンポーネントのrenderは1つの親にぶら下がっている状態でなければいけない
  * 複数のコンポーネントを組み合わせるときはViewを使用する
    * Viewは入れ物
* divなどのDOMは使用することができない。すべてコンポーネントを組み合わせる

```javascript
class Greeting extends Component {
  render() {
    return (
      <Text>Hello {this.props.name}</Text>
    );
  }
}

class LotsOfGreetings extends Component {
  render() {
    return (
      <View style={{alignItems: 'center'}}>
        <Greeting name='Rexxar' />
        <Greeting name='Jaina' />
        <Greeting name='Valeera' />
      </View>
    );
  }
}
```

### Image

```javascript
class Bananas extends Component {
    render() {
        let pic = {
            uri: 'https://upload.wikimedia.org/wikipedia/commons/d/de/Bananavarieties.jpg'
        };
        return (
            <Image source={pic} style={{width: 193, height: 110}} />
        );
    }
}
```

### State

* `setState`を使用してパラメーターを更新

```javascript
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
```

### Style

* Styleの適応は`style`プロパティを使用して適応する
  * `style`に対して`StyleSheet.create`で作成したパラメータを渡す
  * その際に配列で複数渡せるが、同一パラメーターは後勝ちになる

```javascript
export default class App extends Component {
  render() {
    return (
      <View>
        <View style={{width:  50, height:  50, backgroundColor: 'powderblue'}} />
        <View style={{width: 100, height: 100, backgroundColor: 'skyblue'}} />
        <View style={{width: 150, height: 150, backgroundColor: 'steelblue'}} />
      </View>
    );
  }
}

const styles = StyleSheet.create({
  bigblue: {
    color: 'blue',
    fontWeight: 'bold',
    fontSize: 30,
  },
  red: {
    color:  'red',
  },
});
```

### Flex

* `flex`プロパティを使用したデザイン
  * 1つのView内にある要素は`flex`プロパティを使用して表示比率を変更できる
  * `flex: 1`と`flex: 2`の場合は表示エリアの比率が1:2になる
  * なお親コンポーネントが`flex``height``width`のいずれかのプロパティを指定していなければならない

```javascript
export default class App extends Component {
  render() {
    return (
      <View style={{flex: 1}}>
        {/*
        <View style={{width:  50, height:  50, backgroundColor: 'powderblue'}} />
        <View style={{width: 100, height: 100, backgroundColor: 'skyblue'}} />
        <View style={{width: 150, height: 150, backgroundColor: 'steelblue'}} />
        */}
        <View style={{flex: 1, backgroundColor: 'powderblue'}} />
        <View style={{flex: 2, backgroundColor: 'skyblue'}} />
        <View style={{flex: 3, backgroundColor: 'steelblue'}} />
      </View>
    );
  }
}
```

### Layout with Flex

* `flexDirection`
  * Flex内の配置順方向を指定
  * `row` `column`
  * 指定する場合は親コンポーネントから
* `justifyContent`
  * Flex内要素の配置方法
  * `flex-start` `center` `flex-end` `space-around` `space-between` `space-evenly`
  * 間にスペースを挟んだりなどの指定が可能
* `alignItems`
  * Flex内要素の配置場所
  * `flex-start` `center` `flex-end` `stretch`

```javascript
export default class App extends Component {
  render() {
    return (
      <View style={{ flex: 1, flexDirection: 'row' }}>
        <View style={{ flex: 1, flexDirection: 'row' }}>
          <View style={{ width: 50, height: 50, backgroundColor: 'powderblue' }} />
          <View style={{ width: 50, height: 50, backgroundColor: 'skyblue' }} />
          <View style={{ width: 50, height: 50, backgroundColor: 'steelblue' }} />
        </View>
        <View style={{ flex: 1, flexDirection: 'column', justifyContent: 'space-between' }}>
          <View style={{ width: 50, height: 50, backgroundColor: 'powderblue' }} />
          <View style={{ width: 50, height: 50, backgroundColor: 'skyblue' }} />
          <View style={{ width: 50, height: 50, backgroundColor: 'steelblue' }} />
        </View>
        <View style={{ flex: 1, flexDirection: 'column', justifyContent: 'center', alignItems: 'center' }}>
          <View style={{ width: 50, height: 50, backgroundColor: 'powderblue' }} />
          <View style={{ width: 50, height: 50, backgroundColor: 'skyblue' }} />
          <View style={{ width: 50, height: 50, backgroundColor: 'steelblue' }} />
        </View>
      </View>
    );
  }
}
```