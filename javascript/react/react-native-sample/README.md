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

### Handling Text Input

* 文字入力ハンドラを持つコンポーネント
* `onChangeText`で入力内容の変更をハンドリング
  * 別途`onSubmitEditing`もあり

```javascript
export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = { text: '' };
  }

  render() {
    return (
      <View style={{ padding: 10 }}>
        <TextInput
          style={{height: 40}}
          placeholder="Type here to translate!"
          onChangeText={(text) => this.setState({text})}
        />
        <Text style={{padding: 10, fontSize: 42}}>
          {this.state.text.split(' ').map((word) => word && '🍕').join(' ')}
        </Text>
      </View>
    );
  }
}
```

### Handling Touches

* `onPress`でタップ
* `color`で色の指定が可能
* `title`でボタンの名称

```javascript
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
```

* Buttonでなかなえない場合は`Touchable*****`のコンポーネントを使用
  * `TouchableHighlight`
  * `TouchableOpacity`
  * `TouchableNativeFeedback`
  * `TouchableWithoutFeedback`
  * `TouchableHighlight`
* 自分で好きな要素にタップイベントを設定できる

```javascript
<TouchableHighlight onPress={this._onPressButton} onLongPress={this._onLongPressButton} underlayColor="white">
  <View style={styles.button}>
    <Text style={styles.buttonText}>Touchable with Long Press</Text>
  </View>
</TouchableHighlight>
```

### Using a ScrollView

* 内部の要素をスクロール付きで表示
* ただし、画面に表示していなくても内部要素をすべてレンダリングしてしまうことに注意
  * 限られたコンテンツを表示するときに使用するのが望ましい

```javascript
<ScrollView>
  <Text style={{fontSize: 96}}>Scroll me plz.</Text>
  <Text style={{fontSize:96}}>If you like</Text>
  <Text style={{fontSize:96}}>Scrolling down</Text>
  <Text style={{fontSize:96}}>What's the best</Text>
  <Text style={{fontSize:96}}>Framework around?</Text>
  <Text style={{fontSize:80}}>React Native</Text>
</ScrollView>
```

### Using List Views

* リストの表現は通常`FlatList`か`SectionList`を使用する
* `FlatList`
  * 画面上に表示している内容のみをレンダリング
  * 時間経過と共に内容の数が変化するものに向いている
    * TwitterのTLなど？
  * `props`
    * `data`
      * リスト内に表示するリスト
    * `renderItem`
      * 各行内で表示するコンテンツ。JSX

```javascript
<View style={styles.containar}>
  <FlatList
    data={[
      {key: 'Devin'},
      {key: 'Jackson'},
      {key: 'James'},
      {key: 'Joel'},
      {key: 'John'},
      {key: 'Jillian'},
      {key: 'Jimmy'},
      {key: 'Julie'},
    ]}
    renderItem={({item}) => <Text style={styles.item}>{item.key}</Text>}
    />
</View>
```