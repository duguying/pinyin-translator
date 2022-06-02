# pinyin-translator [![Build Status](https://drone.duguying.net/api/badges/rust/pinyin-translator/status.svg)](https://drone.duguying.net/rust/pinyin-translator)

将中文翻译为拼音。内嵌字词库，支持多音词。

## 示例

```rust
fn main() {
    let content = "わたしわ阿飞, and my English name is Rex Lee. 网名是独孤影！ ^_^。下面是一段多音分词歧义测试，这个人无伤无臭味。".to_string();
    let pt = pinyin_translator::PinyinTranslator::new();
    let result = pt.translate(content.clone());
    println!("{}", result);

    let vec = pt.translate_as_slice(content.clone());
    println!("{:?}", vec);
}
```

输出

```text
わたしわāfēi, and my English name is Rex Lee. wǎngmíngshìdúgūyǐng！ ^_^。xiàmiànshìyīduànduōyīnfēncíqíyìcèshì，zhègèrénwúshāngwúchòuwèi。
["わ", "た", "し", "わ", "ā", "fēi", ",", " ", "a", "n", "d", " ", "m", "y", " ", "E", "n", "g", "l", "i", "s", "h", " ", "n", "a", "m", "e", " ", "i", "s", " ", "R", "e", "x", " ", "L", "e", "e", ".", " ", "wǎng", "míng", "shì", "dú", "gū", "yǐng", "！", " ", "^", "_", "^", "。", "xià", "miàn", "shì", "yī", "d"duō", "yīn", "fēn", "cí", "qí", "yì", "cè", "shì", "，", "zhè", "gè", "rén", "wú", "shāng", "wú", "chòu", "wèi", "。"]
```

## License

MIT License
