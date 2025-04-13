use bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lookup {
    pub doc: Document,
    pub unwind: Option<Document>,
    pub set: Option<Document>,
}

pub fn raw(doc: Document) -> Lookup {
    Lookup {
        doc,
        unwind: None,
        set: None,
    }
}

pub fn create_lookup_doc(collection: &str, local: &str, foreign: &str, alias: &str) -> Document {
    let mut lookup = Document::new();
    lookup.insert("from", collection);
    lookup.insert("localField", local);
    lookup.insert("foreignField", foreign);
    lookup.insert("as", alias);
    lookup
}

pub fn one(collection: &str, local: &str, foreign: &str, alias: &str) -> Lookup {
    let mut doc = Document::new();

    doc.insert(
        "$lookup",
        create_lookup_doc(collection, local, foreign, alias),
    );

    let mut unwind_doc = Document::new();
    let mut unwind = Document::new();
    unwind.insert("path", format!("${}", alias));
    unwind.insert("preserveNullAndEmptyArrays", true);
    unwind_doc.insert("$unwind", unwind);

    Lookup {
        doc,
        unwind: Some(unwind_doc),
        set: None,
    }
}

pub fn one_merge_to(
    collection: &str,
    local: &str,
    foreign: &str,
    alias: &str,
    parent: &str,
) -> Lookup {
    let parent_alias = format!("{}{}", parent, alias);
    let mut doc = Document::new();
    doc.insert(
        "$lookup",
        create_lookup_doc(collection, local, foreign, alias),
    );

    let f_alias = format!("${}", parent_alias);
    let f_parent = format!("${}", parent);
    let set = doc! {
        "$set": {
            parent: {
                "$cond": {
                    "if": {
                        "$ifNull": [
                            f_parent.clone(),
                            false
                        ]
                    },
                    "then": {
                        "$mergeObjects": [
                            f_parent,
                            {
                                alias: {
                                    "$cond": {
                                        "if": {
                                            "$gt": [
                                                {
                                                    "$size": [f_alias.clone()]
                                                },
                                                0
                                            ]
                                        },
                                        "then": {
                                            "$arrayElemAt": [
                                                f_alias.clone(),
                                                0
                                            ]
                                        },
                                        "else": "$$REMOVE"
                                    }
                                }
                            }
                        ]
                    },
                    "else": "$$REMOVE"
                }
            }
        }
    };
    Lookup {
        doc,
        unwind: None,
        set: Some(set),
    }
}

pub fn many(collection: &str, foreign: &str, local: &str, alias: &str) -> Lookup {
    let mut doc = Document::new();
    doc.insert(
        "$lookup",
        create_lookup_doc(collection, local, foreign, alias),
    );

    Lookup {
        doc,
        unwind: None,
        set: None,
    }
}
