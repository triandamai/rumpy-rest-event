use bson::{doc, Bson, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Filter {
    pub text: Option<Document>,
    pub and: Vec<Document>,
    pub or: Vec<Document>,
}

impl Filter {
    pub fn empty() -> Self {
        Filter {
            text: None,
            and: vec![],
            or: vec![],
        }
    }
    pub fn new_with(text: Document) -> Self {
        Filter {
            text: Some(text),
            and: vec![],
            or: vec![],
        }
    }
    pub fn new(text: Document, and: Vec<Document>, or: Vec<Document>) -> Self {
        Filter {
            text: Some(text),
            and,
            or,
        }
    }
    pub fn set_text(mut self, text: Document) -> Self {
        let mut doc = Document::new();
        doc.insert("$text", text);
        self.text = Some(doc);
        self
    }
    pub fn add_and(&mut self, and: Document) {
        self.and.push(and);
    }

    pub fn add_or(&mut self, or: Document) {
        self.or.push(or);
    }
}

pub fn search<T: Into<Bson>>(column: &str, value: T) -> Document {
    let mut doc = Document::new();
    doc.insert(column, value.into());
    doc
}

pub fn when<T: Into<Bson>>(column: &str, operator: Option<&str>, value: T) -> FilterGroup {
    let mut doc = Document::new();
    if let Some(opr) = operator {
        let mut filter = Document::new();
        filter.insert(opr, value.into());
        doc.insert(column, filter);
    } else {
        doc.insert(column.to_string(), value.into());
    }
    doc.into()
}

pub fn is<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
    when(column, None, value)
}
pub fn equal<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
    when(column, Some("$eq"), value)
}
pub fn not_equal<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
    when(column, Some("$ne"), value)
}
pub fn is_in<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
    when(column, Some("$in"), value)
}
pub fn is_not_in<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
    when(column, Some("$nin"), value)
}
pub fn lower<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
    when(column, Some("$le"), value)
}
pub fn greater<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
    when(column, Some("$ge"), value)
}
pub fn lower_than_equal<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
    when(column, Some("$lte"), value)
}
pub fn greater_than_equal<T: Into<Bson>>(column: &str, value: T) -> FilterGroup {
    when(column, Some("$gte"), value)
}

impl Into<FilterGroup> for Document {
    fn into(self) -> FilterGroup {
        FilterGroup::Filter(self)
    }
}

impl From<FilterGroup> for Document {
    fn from(value: FilterGroup) -> Self {
        let mut filter = Document::new();
        match value {
            FilterGroup::Filter(f) => f,
            FilterGroup::Or(or) => {
                filter.insert("$or", or);
                filter
            }
        }
    }
}

pub fn or(filter: &[FilterGroup]) -> FilterGroup {
    let collect = filter
        .into_iter()
        .map(|v| v.clone().into())
        .collect::<Vec<Document>>();
    FilterGroup::Or(collect)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FilterGroup {
    Filter(Document),
    Or(Vec<Document>),
}
