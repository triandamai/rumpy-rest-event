use crate::common::mongo::get_db_name;
use crate::common::utils::create_object_id_option;
use bson::{doc, Document};
use log::info;
use mongodb::options::IndexOptions;
use mongodb::{Client, IndexModel};

pub async fn init_seeder(db_client: &Client) {
    info!(target: "seeder","Spread the seed...");
    let branch_id_1 = create_object_id_option("6742c74a15e68b0e7ee06120");
    let branch_id_2 = create_object_id_option("6742c74a15e68b0e7ee06121");
    let account_id_1 = create_object_id_option("6742c74a15e68b0e7ee06122");

    //CREATE index
    {
        info!(target: "seeder","5% seed index");

        let _index_user_full_name = &db_client
            .database(&get_db_name())
            .collection::<Document>("user")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "display_name": "text" })
                    .options(
                        IndexOptions::builder()
                            .name("user-index-full-name".to_string())
                            .unique(false)
                            .build(),
                    )
                    .build(),
            )
            .await;

        let _index_user_email = &db_client
            .database(&get_db_name())
            .collection::<Document>("user")
            .create_index(
                IndexModel::builder()
                    .keys(doc! {  "email":1 })
                    .options(
                        IndexOptions::builder()
                            .name("user-index-email".to_string())
                            .unique(true)
                            .build(),
                    )
                    .build(),
            )
            .await;

        let _index_user_email = &db_client
            .database(&get_db_name())
            .collection::<Document>("thread")
            .create_index(
                IndexModel::builder()
                    .keys(doc! {  "title":"text","content":"text" })
                    .options(
                        IndexOptions::builder()
                            .name("thread-index-fts".to_string())
                            .unique(false)
                            .build(),
                    )
                    .build(),
            )
            .await;

        let _index_topic = &db_client
            .database(&get_db_name())
            .collection::<Document>("topic")
            .create_index(
                IndexModel::builder()
                    .keys(doc! {  "name":"text","description":"text" })
                    .options(
                        IndexOptions::builder()
                            .name("topic-index-fts".to_string())
                            .unique(true)
                            .build(),
                    )
                    .build(),
            )
            .await;
    }
    info!(target: "seeder", "seeding completed application ready");
}
