use crate::common::env_config::EnvConfig;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Default,Clone)]
pub struct I18n {
    pub languages: HashMap<String, HashMap<String, String>>,
    pub locale: String,
}

impl I18n {
    pub fn load(feature: &str, lang: Lang) -> Self {
        let file = File::open(format!("locales/{}.json", feature));
        let hash = match file {
            Ok(lang) => serde_json::from_reader(lang).unwrap_or(HashMap::new()),
            Err(_) => {
                info!(target:"i18n::failed","failed to load file {}.json",feature);
                HashMap::new()
            }
        };
        I18n {
            languages: hash,
            locale: lang.locale_code,
        }
    }
    pub async fn sync_locales(features: &[&str]) {
        info!(target:"app::i18n","sync locales");
        let env = EnvConfig::init();
        if !env.mode.eq("DEV") {
            info!(target:"i18n","total feature {}",features.len());
            for (index, feature) in features.iter().enumerate() {
                info!(target:"load::locale::loading","attempt {} feature {}",index+1,feature);
                let minio = MinIO::new()
                    .get_file("assets".to_string(), format!("{}.json", feature))
                    .await;
                if minio.is_ok() {
                    let file = minio.unwrap();

                    // Read the file contents
                    let data = file.bytes();

                    // Save the file to the "uploads" folder
                    let mut file =
                        File::create(format!("locales/{}", format!("{}.json", feature))).unwrap();
                    file.write_all(&data).unwrap();
                }
                info!(target:"load::locale::finish","atempt {} feature {}",index+1,feature);
            }
        }
    }

    pub fn translate_with_args(&self, name: &str, params: HashMap<String, String>) -> String {
        if let Some(locale) = self.languages.get(name) {
            if let Some(greeting) = locale.get(&self.locale.clone()) {
                let mut str = greeting.to_string();
                info!(target: "i18n::translating","{}",name);
                for (key, value) in params {
                    let placeholder = format!("${{{}}}", key); // Create placeholder like ${key}
                    str = str.replace(&placeholder, value.as_str());
                }
                str
            } else {
                name.to_string()
            }
        } else {
            name.to_string()
        }
    }

    pub fn translate(&self, name: &str) -> String {
        if let Some(locale) = self.languages.get(name) {
            if let Some(greeting) = locale.get(&self.locale.clone()) {
                let str = greeting.to_string();
                info!(target: "i18n::translating","{}",name);
                str
            } else {
                name.to_string()
            }
        } else {
            name.to_string()
        }
    }
}

pub fn replace_placeholders(template: &str, values: &HashMap<&str, &str>) -> String {
    let mut result = template.to_string();

    for (key, value) in values {
        let placeholder = format!("${{{}}}", key); // Create a placeholder like ${key}
        result = result.replace(&placeholder, value);
    }
    result
}

// Define a macro to create the placeholder replacement map and perform replacements
#[macro_export]
macro_rules! i18n {
    ($feature:expr,$lang:ident) => {{
        $crate::common::i18n::I18n::load($feature, $lang.clone())
    }};
}

// Define a macro to create the placeholder replacement map and perform replacements
#[macro_export]
macro_rules! t {
    ({$($key:expr => $value:expr),* $(,)? }) => {{
        let mut values = std::collections::HashMap::new();
        $(
            values.insert($key.to_string(), $value.to_string());
        )*
        values
    }};
}
