fn main() {
    let content = "わたしわ阿飞, and my English name is Rex Lee. 网名是独孤影！ ^_^。下面是一段多音分词歧义测试，这个人无伤无臭味。".to_string();
    let pt = pinyin_translator::PinyinTranslator::new();
    let result = pt.translate(content.clone());
    println!("{}", result);

    let vec = pt.translate_as_slice(content.clone());
    println!("{:?}", vec);

    let pt2 = pinyin_translator::PinyinTranslator::new();
    let result = pt2.unmark_translate("下面是一段多音分词歧义测试，这个人无伤无臭味。".to_string());
    println!("{}", result);
}