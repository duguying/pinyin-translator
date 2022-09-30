use std::collections::HashMap;
use std::ops::{Bound, RangeBounds};

mod vars;

const SEP: char = 7 as char;

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn substring_len(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, end: usize) -> &str {
        self.substring_len(start, end - start)
    }

    fn substring_len(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }

    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

pub struct PinyinTranslator<'a> {
    max_word_len: usize,
    words: Vec<&'a str>,
    chars: Vec<&'a str>,
    word_dict: HashMap<String, String>,
    syllable_map: HashMap<&'a str, &'a str>,
}

impl PinyinTranslator<'static> {
    /// 翻译为拼音
    /// # 示例:
    /// ```rust
    /// let pt = pinyin_translator::PinyinTranslator::new();
    /// let result = pt.translate("下面是一段多音分词歧义测试，这个人无伤无臭味。".to_string());
    /// // 输出
    /// // xiàmiànshìyīduànduōyīnfēncíqíyìcèshì，zhègèrénwúshāngwúchòuwèi。
    /// ```
    pub fn translate(&self, content: String) -> String {
        let result = self.translate_raw(content);
        return result.replace(format!("{}", SEP).as_str(), "");
    }

    /// 翻译为拼音，返回vec
    /// # 示例:
    /// ```rust
    /// let pt = pinyin_translator::PinyinTranslator::new();
    /// let result = pt.translate_as_slice("下面是一段多音分词歧义测试，这个人无伤无臭味。".to_string());
    /// // 输出
    /// // ["xià", "miàn", "shì", "yī", "duàn", "duō", "yīn", "fēn", "cí", "qí", "yì", "cè", "shì", "，", "zhè", "gè", "rén", "wú", "shāng", "wú", "chòu", "wèi", "。"]
    /// ```
    pub fn translate_as_slice(&self, content: String) -> Vec<String> {
        let result = self.translate_raw(content);
        let vec: Vec<String> = result
            .split(format!("{}", SEP).as_str())
            .map(String::from)
            .collect();
        return vec;
    }

    /// 翻译为无声标拼音
    /// # 示例:
    /// ```rust
    /// let pt = pinyin_translator::PinyinTranslator::new();
    /// let result = pt.unmark_translate("下面是一段多音分词歧义测试，这个人无伤无臭味。".to_string());
    /// // 输出
    /// // xiamianshiyiduanduoyinfenciqiyiceshi，zhegerenwushangwuchouwei。
    /// ```
    pub fn unmark_translate(&self, content: String) -> String {
        let mut result = self.translate_raw(content);
        result = self.unmark(result);
        return result.replace(format!("{}", SEP).as_str(), "");
    }

    /// 翻译为无声标拼音，返回vec
    /// # 示例:
    /// ```rust
    /// let pt = pinyin_translator::PinyinTranslator::new();
    /// let result = pt.unmark_translate_as_slice("下面是一段多音分词歧义测试，这个人无伤无臭味。".to_string());
    /// // 输出
    /// // ["xia", "mian", "shi", "yi", "duan", "duo", "yin", "fen", "ci", "qi", "yi", "ce", "shi", "，", "zhe", "ge", "ren", "wu", "shang", "wu", "chou", "wei", "。"]
    /// ```
    pub fn unmark_translate_as_slice(&self, content: String) -> Vec<String> {
        let mut result = self.translate_raw(content);
        result = self.unmark(result);
        let vec: Vec<String> = result
            .split(format!("{}", SEP).as_str())
            .map(String::from)
            .collect();
        return vec;
    }

    fn unmark(&self, content: String) -> String {
        let mut output = content;
        for (key, value) in self.syllable_map.to_owned() {
            output = output.replace(key, value);
        }
        return output;
    }

    #[allow(unused_assignments)]
    fn translate_raw(&self, content: String) -> String {
        let mut result: String = "".to_string();
        let mut len: usize = 0;
        let mut tail_start_idx: usize = 0;
        let mut tail_end_idx: usize = 0;
        let mut left_end_idx: usize = 0;
        let mut tail: String = "".to_string();
        let mut left: String = content.to_string();

        while left.chars().count() > 0 {
            // outer
            if tail.chars().count() == 0 {
                len = left.chars().count();
                let cut_len: usize = if len > self.max_word_len {
                    self.max_word_len
                } else {
                    len
                };
                tail_start_idx = len - cut_len;
                tail_end_idx = len;
                left_end_idx = tail_start_idx;
                tail = content
                    .to_string()
                    .substring(tail_start_idx, tail_end_idx)
                    .to_string();
                left = content.to_string().substring(0, left_end_idx).to_string();
            }

            while tail.chars().count() > 1 {
                let value = self.word_dict.get(tail.as_str());
                let value = match value {
                    None => "",
                    Some(value) => value,
                };

                if value != "" {
                    result = format!("{}{}", value, result);
                    tail = "".to_string();
                    break;
                } else {
                    left = format!("{}{}", left, tail.chars().nth(0).unwrap());
                    tail = tail.substring(1, tail.len()).to_string()
                }
            }

            if tail.chars().count() > 0 {
                let value = self.word_dict.get(tail.as_str());
                let value = match value {
                    None => {
                        format!("{}{}", tail, SEP)
                    }
                    Some(value) => value.to_string(),
                };
                result = format!("{}{}", value, result);
                tail = "".to_string()
            }
        }

        if result.chars().last().unwrap() == SEP {
            result = result[..result.len() - 1].to_string()
        }

        return result;
    }

    fn parse(&mut self) {
        for char in self.chars.as_slice() {
            let char_elements: Vec<&str> = char.split(",").collect();
            let key = char_elements.get(0).unwrap();
            let value = char_elements.get(1).unwrap();
            self.word_dict
                .insert((*key).to_owned(), format!("{}{}", *value, SEP));
        }

        for word in self.words.as_slice() {
            let word_elements: Vec<&str> = word.split(",").collect();
            let key = word_elements.get(0).unwrap();
            let sub = word_elements[1..].to_vec();
            let v = sub.join(format!("{}", SEP).as_str());
            let value: &str = v.as_str();
            self.word_dict
                .insert((*key).to_owned(), format!("{}{}", value, SEP));
        }

        self.syllable_map = [
            ("ā", "a"),
            ("á", "a"),
            ("ǎ", "a"),
            ("à", "a"),
            ("ō", "o"),
            ("ó", "o"),
            ("ǒ", "o"),
            ("ò", "o"),
            ("ē", "e"),
            ("é", "e"),
            ("ě", "e"),
            ("è", "e"),
            ("ī", "i"),
            ("í", "i"),
            ("ǐ", "i"),
            ("ì", "i"),
            ("ū", "u"),
            ("ú", "u"),
            ("ǔ", "u"),
            ("ù", "u"),
            ("ǖ", "ü"),
            ("ǘ", "ü"),
            ("ǚ", "ü"),
            ("ǜ", "ü"),
        ]
        .iter()
        .cloned()
        .collect();
    }

    fn init(&mut self) {
        // 在这里运行初始化
        self.parse();
    }

    pub fn new<'a>() -> PinyinTranslator<'a> {
        let mut pt = PinyinTranslator {
            max_word_len: vars::WORD_LEN,
            words: vars::WORDS.to_vec(),
            chars: vars::CHARS.to_vec(),
            word_dict: HashMap::new(),
            syllable_map: HashMap::new(),
        };
        pt.init();
        return pt;
    }
}
