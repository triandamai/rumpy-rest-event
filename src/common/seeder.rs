use crate::common::constant::{COLLECTION_EVENTS, COLLECTION_USERS};
use crate::common::mongo::get_db_name;
use crate::common::utils::create_object_id_option;
use bson::{Document, doc};
use log::info;
use mongodb::options::IndexOptions;
use mongodb::{Client, IndexModel};

pub async fn init_seeder(db_client: &Client) {
    info!(target: "seeder","Spread the seed...");
    let _branch_id_1 = create_object_id_option("6742c74a15e68b0e7ee06120");
    let _branch_id_2 = create_object_id_option("6742c74a15e68b0e7ee06121");
    let _account_id_1 = create_object_id_option("6742c74a15e68b0e7ee06122");

    //CREATE index
    info!(target: "seeder","5% seed index");

    let _index_user_full_name = &db_client
        .database(&get_db_name())
        .collection::<Document>(COLLECTION_USERS)
        .create_index(
            IndexModel::builder()
                .keys(doc! { "display_name": "text" })
                .options(
                    IndexOptions::builder()
                        .name("user-index-display-name".to_string())
                        .unique(false)
                        .build(),
                )
                .build(),
        )
        .await;

    let _index_user_email = &db_client
        .database(&get_db_name())
        .collection::<Document>(COLLECTION_USERS)
        .create_index(
            IndexModel::builder()
                .keys(doc! {  "phone_number":1 })
                .options(
                    IndexOptions::builder()
                        .name("user-index-phone-number".to_string())
                        .unique(true)
                        .build(),
                )
                .build(),
        )
        .await;

    let _index_fts_event = &db_client
        .database(&get_db_name())
        .collection::<Document>(COLLECTION_EVENTS)
        .create_index(
            IndexModel::builder()
                .keys(doc! { "event_name": "text","event_description":"text" })
                .options(
                    IndexOptions::builder()
                        .name("event-index-fulltext".to_string())
                        .unique(false)
                        .build(),
                )
                .build(),
        )
        .await;

    info!(target: "seeder", "seeding completed application ready");
}
