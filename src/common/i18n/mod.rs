use crate::common::env_config::EnvConfig;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::sync::Once;

#[derive(Debug, Default)]
pub struct I18n {
    pub locales: HashMap<String, HashMap<String, String>>
}

impl I18n {
    pub async fn load_locales() {
        let env = EnvConfig::init();
        if !env.mode.eq("DEV") {
            let minio = MinIO::new()
                .await
                .get_file("assets".to_string(), "locales.json".to_string())
                .await;
            if minio.is_ok() {
                let file = minio.unwrap();

                // Read the file contents
                let data = file.bytes();

                // Save the file to the "uploads" folder
                let mut file = File::create(format!("locales/{}", "locales.json")).unwrap();
                file.write_all(&data).unwrap();
            }
        }
    }
}

// Declare a static global variable
static mut GLOBAL_I18N: Option<Mutex<I18n>> = None;
static INIT: Once = Once::new();

// Function to initialize and retrieve the global I18n instance
pub fn get_global_i18n() -> &'static Mutex<I18n> {
    unsafe {
        INIT.call_once(|| {
            info!(target: "INIT i18n","opening file...");
            let file = File::open("./locales/locales.json").unwrap();

            let value: HashMap<String, HashMap<String, String>> =
                serde_json::from_reader(file).unwrap_or(HashMap::new());

            // Safely assign to GLOBAL_I18N
            GLOBAL_I18N = Some(Mutex::new(I18n { locales: value }));
        });

        // Safely access GLOBAL_I18N
        GLOBAL_I18N.as_ref().expect("Global I18n not initialized")
    }
}
pub fn replace_placeholders(
    template: &str,
    values: &HashMap<&str, &str>,
) -> String {
    let mut result = template.to_string();

    for (key, value) in values {
        let placeholder = format!("${{{}}}", key); // Create a placeholder like ${key}
        result = result.replace(&placeholder, value);
    }

    result
}

pub fn tr(name: &str, lang: &str, params: HashMap<String, String>) -> String {
    let mut str = String::new();
    let i18n = get_global_i18n().lock().unwrap();
    if let Some(locale) = i18n.locales.get(name) {
        if let Some(greeting) = locale.get(lang) {
            str = greeting.to_string();
            info!(target: "i18n::translating","{}",name);
            for (key, value) in params {
                let placeholder = format!("${{{}}}", key); // Create placeholder like ${key}
                str = str.replace(&placeholder, value.as_str());
            }
        }
    }
    str
}

pub fn tr_ty_lang(name: &str, lang: Lang, params: HashMap<String, String>) -> String {
    let mut str = String::new();
    let i18n = get_global_i18n().lock().unwrap();
    if let Some(locale) = i18n.locales.get(name) {
        if let Some(greeting) = locale.get(&lang.locale_code.to_string()) {
            str = greeting.to_string();
            info!(target: "i18n::translating","{}",name);
            for (key, value) in params {
                let placeholder = format!("${{{}}}", key); // Create placeholder like ${key}
                str = str.replace(&placeholder, value.as_str());
            }
        }
    }
    str
}

// Define a macro to create the placeholder replacement map and perform replacements
#[macro_export]
macro_rules! translate {
    //
    ($name:expr,{$($key:expr => $value:expr),* $(,)? }) => {{
        let mut values = std::collections::HashMap::new();
        $(
            values.insert($key.to_string(), $value.to_string());
        )*
        $crate::common::i18n::tr($name,"id-ID",values)
    }};
    //
    ($name:expr)=> {{

        $crate::common::i18n::tr($name,"id-ID",std::collections::HashMap::<String,String>::new())
    }};
    //
    ($name:expr,$lang:ident)=> {{

        &$crate::common::i18n::tr_ty_lang($name,$lang.clone(),std::collections::HashMap::<String,String>::new())
    }};
    //
    ($name:expr,$lang:ident, {$($key:expr => $value:expr),* $(,)? }) => {{
        let mut values = std::collections::HashMap::new();
        $(
            values.insert($key.to_string(), $value.to_string());
        )*

        $crate::common::i18n::tr_ty_lang($name,$lang.clone(),values)
    }};
}
