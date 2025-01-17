use std::{collections::HashSet, mem};

use serde::{Deserialize, Serialize};

use lindera_core::{error::LinderaErrorKind, LinderaResult};

use crate::{token::FilteredToken, token_filter::TokenFilter};

pub const JAPANESE_STOP_TAGS_TOKEN_FILTER_NAME: &str = "japanese_stop_tags";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct JapaneseStopTagsTokenFilterConfig {
    tags: HashSet<String>,
}

impl JapaneseStopTagsTokenFilterConfig {
    pub fn new(tags: HashSet<String>) -> Self {
        let mut formatted_tags: HashSet<String> = HashSet::new();
        for tag in tags.iter() {
            let mut formatted_tag = vec!["*", "*", "*", "*"];

            let tag_array: Vec<&str> = tag.split(',').collect();
            for (i, j) in tag_array.iter().enumerate() {
                formatted_tag[i] = j;
            }

            formatted_tags.insert(formatted_tag.join(","));
        }

        Self {
            tags: formatted_tags,
        }
    }

    pub fn from_slice(data: &[u8]) -> LinderaResult<Self> {
        let tmp_config = serde_json::from_slice::<JapaneseStopTagsTokenFilterConfig>(data)
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(err))?;

        Ok(Self::new(tmp_config.tags))
    }
}

/// Remove tokens with the specified part-of-speech tag.
///
#[derive(Clone, Debug)]
pub struct JapaneseStopTagsTokenFilter {
    config: JapaneseStopTagsTokenFilterConfig,
}

impl JapaneseStopTagsTokenFilter {
    pub fn new(config: JapaneseStopTagsTokenFilterConfig) -> Self {
        Self { config }
    }

    pub fn from_slice(data: &[u8]) -> LinderaResult<Self> {
        Ok(Self::new(JapaneseStopTagsTokenFilterConfig::from_slice(
            data,
        )?))
    }
}

impl TokenFilter for JapaneseStopTagsTokenFilter {
    fn name(&self) -> &'static str {
        JAPANESE_STOP_TAGS_TOKEN_FILTER_NAME
    }

    fn apply(&self, tokens: &mut Vec<FilteredToken>) -> LinderaResult<()> {
        let mut new_tokens = Vec::new();

        for token in tokens.iter_mut() {
            let mut formatted_tags = vec!["*", "*", "*", "*"];
            let tags_len = if token.details.len() >= 4 { 4 } else { 1 };
            for (i, j) in token.details[0..tags_len].iter().enumerate() {
                formatted_tags[i] = j;
            }
            if !self.config.tags.contains(&formatted_tags.join(",")) {
                new_tokens.push(token.clone());
            }
        }

        mem::swap(tokens, &mut new_tokens);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::token_filter::japanese_stop_tags::{
        JapaneseStopTagsTokenFilter, JapaneseStopTagsTokenFilterConfig,
    };
    #[cfg(all(feature = "ipadic", feature = "ipadic-filter",))]
    use crate::{token::FilteredToken, token_filter::TokenFilter};

    #[test]
    fn test_japanese_stop_tags_token_filter_config_from_slice() {
        let config_str = r#"
        {
            "tags": [
                "接続詞",
                "助詞",
                "助詞,格助詞",
                "助詞,格助詞,一般",
                "助詞,格助詞,引用",
                "助詞,格助詞,連語",
                "助詞,係助詞",
                "助詞,副助詞",
                "助詞,間投助詞",
                "助詞,並立助詞",
                "助詞,終助詞",
                "助詞,副助詞／並立助詞／終助詞",
                "助詞,連体化",
                "助詞,副詞化",
                "助詞,特殊",
                "助動詞",
                "記号",
                "記号,一般",
                "記号,読点",
                "記号,句点",
                "記号,空白",
                "記号,括弧閉",
                "その他,間投",
                "フィラー",
                "非言語音"
            ]
        }
        "#;
        let config = JapaneseStopTagsTokenFilterConfig::from_slice(config_str.as_bytes()).unwrap();

        assert_eq!(config.tags.len(), 25);
    }

    #[test]
    fn test_japanese_stop_tagss_token_filter_from_slice() {
        let config_str = r#"
        {
            "tags": [
                "接続詞",
                "助詞",
                "助詞,格助詞",
                "助詞,格助詞,一般",
                "助詞,格助詞,引用",
                "助詞,格助詞,連語",
                "助詞,係助詞",
                "助詞,副助詞",
                "助詞,間投助詞",
                "助詞,並立助詞",
                "助詞,終助詞",
                "助詞,副助詞／並立助詞／終助詞",
                "助詞,連体化",
                "助詞,副詞化",
                "助詞,特殊",
                "助動詞",
                "記号",
                "記号,一般",
                "記号,読点",
                "記号,句点",
                "記号,空白",
                "記号,括弧閉",
                "その他,間投",
                "フィラー",
                "非言語音"
            ]
        }
        "#;
        let result = JapaneseStopTagsTokenFilter::from_slice(config_str.as_bytes());

        assert_eq!(true, result.is_ok());
    }

    #[test]
    #[cfg(all(feature = "ipadic", feature = "ipadic-filter",))]
    fn test_japanese_stop_tags_token_filter_apply_ipadic() {
        let config_str = r#"
        {
            "tags": [
                "接続詞",
                "助詞",
                "助詞,格助詞",
                "助詞,格助詞,一般",
                "助詞,格助詞,引用",
                "助詞,格助詞,連語",
                "助詞,係助詞",
                "助詞,副助詞",
                "助詞,間投助詞",
                "助詞,並立助詞",
                "助詞,終助詞",
                "助詞,副助詞／並立助詞／終助詞",
                "助詞,連体化",
                "助詞,副詞化",
                "助詞,特殊",
                "助動詞",
                "記号",
                "記号,一般",
                "記号,読点",
                "記号,句点",
                "記号,空白",
                "記号,括弧閉",
                "その他,間投",
                "フィラー",
                "非言語音"
            ]
        }
        "#;
        let filter = JapaneseStopTagsTokenFilter::from_slice(config_str.as_bytes()).unwrap();

        let mut tokens: Vec<FilteredToken> = vec![
            FilteredToken {
                text: "すもも".to_string(),
                byte_start: 0,
                byte_end: 9,
                position: 0,
                position_length: 1,
                details: vec![
                    "名詞".to_string(),
                    "一般".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "すもも".to_string(),
                    "スモモ".to_string(),
                    "スモモ".to_string(),
                ],
            },
            FilteredToken {
                text: "も".to_string(),
                byte_start: 9,
                byte_end: 12,
                position: 1,
                position_length: 1,
                details: vec![
                    "助詞".to_string(),
                    "係助詞".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "も".to_string(),
                    "モ".to_string(),
                    "モ".to_string(),
                ],
            },
            FilteredToken {
                text: "もも".to_string(),
                byte_start: 12,
                byte_end: 18,
                position: 2,
                position_length: 1,
                details: vec![
                    "名詞".to_string(),
                    "一般".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "もも".to_string(),
                    "モモ".to_string(),
                    "モモ".to_string(),
                ],
            },
            FilteredToken {
                text: "も".to_string(),
                byte_start: 18,
                byte_end: 21,
                position: 3,
                position_length: 1,
                details: vec![
                    "助詞".to_string(),
                    "係助詞".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "も".to_string(),
                    "モ".to_string(),
                    "モ".to_string(),
                ],
            },
            FilteredToken {
                text: "もも".to_string(),
                byte_start: 21,
                byte_end: 27,
                position: 4,
                position_length: 1,
                details: vec![
                    "名詞".to_string(),
                    "一般".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "もも".to_string(),
                    "モモ".to_string(),
                    "モモ".to_string(),
                ],
            },
            FilteredToken {
                text: "の".to_string(),
                byte_start: 27,
                byte_end: 30,
                position: 5,
                position_length: 1,
                details: vec![
                    "助詞".to_string(),
                    "連体化".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "の".to_string(),
                    "ノ".to_string(),
                    "ノ".to_string(),
                ],
            },
            FilteredToken {
                text: "うち".to_string(),
                byte_start: 30,
                byte_end: 36,
                position: 6,
                position_length: 1,
                details: vec![
                    "名詞".to_string(),
                    "非自立".to_string(),
                    "副詞可能".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "*".to_string(),
                    "うち".to_string(),
                    "ウチ".to_string(),
                    "ウチ".to_string(),
                ],
            },
        ];

        filter.apply(&mut tokens).unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(&tokens[0].text, "すもも");
        assert_eq!(&tokens[1].text, "もも");
        assert_eq!(&tokens[2].text, "もも");
        assert_eq!(&tokens[3].text, "うち");
    }
}
