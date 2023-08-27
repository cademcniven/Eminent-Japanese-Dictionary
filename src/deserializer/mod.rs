use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Dictionary {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "$value")]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "$value")]
    pub forms: Vec<EntryChildren>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EntryChildren {
    Form {
        #[serde(rename = "@value")]
        value: String,
        tags: Option<Vec<Tag>>,
    },
    Reading {
        #[serde(rename = "@value")]
        value: String,
        tags: Option<Vec<Tag>>,
        #[serde(rename = "@value")]
        applies_to: Option<Vec<AppliesTo>>,
    },
    Frequency {
        #[serde(rename = "@value")]
        value: String,
        #[serde(rename = "@source")]
        source: Option<String>,
        #[serde(rename = "@value")]
        applies_to: Option<Vec<AppliesTo>>,
    },
    Pitch {
        #[serde(rename = "@value")]
        value: String,
        #[serde(rename = "@source")]
        source: Option<String>,
        #[serde(rename = "@value")]
        applies_to: Option<Vec<AppliesTo>>,
    },
    See {
        #[serde(rename = "$text")]
        value: String,
        #[serde(rename = "@reading")]
        reading: Option<String>,
    },
    Source {
        #[serde(rename = "$text")]
        source: String,
        #[serde(rename = "@url")]
        url: Option<String>,
        #[serde(rename = "@translation")]
        translation: Option<String>,
    },
    Usage {
        #[serde(rename = "@source")]
        source: Option<String>,
        #[serde(rename = "$value")]
        usage: Vec<UsageChildren>,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UsageChildren {
    Antonym {
        #[serde(rename = "$text")]
        value: String,
        #[serde(rename = "@reading")]
        reading: Option<String>,
    },
    Definition {
        #[serde(rename = "$text")]
        value: String,
    },
    Example {
        #[serde(rename = "$text")]
        value: String,
        #[serde(rename = "@source")]
        source: Option<String>,
        #[serde(rename = "@source_url")]
        source_url: Option<String>,
        #[serde(rename = "@translation")]
        translation: Option<String>,
    },
    Info {
        #[serde(rename = "$text")]
        value: String,
    },
    See {
        #[serde(rename = "$text")]
        value: String,
        #[serde(rename = "@reading")]
        reading: Option<String>,
    },
    Tag {
        #[serde(rename = "$text")]
        value: String,
    },
    Source {},
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    #[serde(rename = "$text")]
    pub tag: String,
}

#[derive(Deserialize, Debug)]
pub struct AppliesTo {
    #[serde(rename = "$text")]
    pub value: String,
    #[serde(rename = "@reading")]
    pub reading: Option<String>,
}

pub fn read_dictionary_xml() {
    let content = fs::read_to_string("src/test/eminent_dictionary_v0_7_6.xml").unwrap();
    let test_dict: Dictionary = match quick_xml::de::from_str(&content) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    println!("Serialized Data:\n------------------");
    println!("Name: {0}, Version: {1}", test_dict.name, test_dict.version);

    println!("{:#?}", test_dict);
}
