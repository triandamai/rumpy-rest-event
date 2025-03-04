use bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Lookup {
    pub lookup: Document,
    pub unwind: Option<Document>,
    pub set: Option<Document>,
}

pub fn raw(doc:Document)->Lookup{
    Lookup{
        lookup:doc,
        unwind:None,
        set:None
    }
}

pub fn one(collection: &str, local: &str, foreign: &str, alias: &str) -> Lookup {
    let mut doc = Document::new();
    let mut lookup = Document::new();
    lookup.insert("from", collection);
    lookup.insert("localField", local);
    lookup.insert("foreignField", foreign);
    lookup.insert("as", alias);
    doc.insert("$lookup", lookup);

    let mut unwind_doc = Document::new();
    let mut unwind = Document::new();
    unwind.insert("path", format!("${}",alias));
    unwind.insert("preserveNullAndEmptyArrays", true);
    unwind_doc.insert("$unwind", unwind);

    Lookup {
        lookup: doc,
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
    let mut lookup = Document::new();
    lookup.insert("from", collection);
    lookup.insert("localField", local);
    lookup.insert("foreignField", foreign);
    lookup.insert("as", parent_alias.clone());
    doc.insert("$lookup", lookup);

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
        lookup: doc,
        unwind: None,
        set: Some(set),
    }
}

pub fn many(collection: &str, foreign: &str, local: &str, alias: &str) -> Lookup {
    let mut doc = Document::new();
    let mut lookup = Document::new();
    lookup.insert("from", collection);
    lookup.insert("localField", local);
    lookup.insert("foreignField", foreign);
    lookup.insert("as", alias);
    doc.insert("$lookup", lookup);

    Lookup {
        lookup: doc,
        unwind: None,
        set: None,
    }
}
