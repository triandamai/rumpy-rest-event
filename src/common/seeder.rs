use crate::common::orm::orm::Orm;
use crate::common::orm::DB_NAME;
use crate::common::utils::{create_object_id_option, create_or_new_object_id};
use crate::entity::account::Account;
use crate::entity::account_permission::AccountPermission;
use crate::entity::branch::Branch;
use crate::entity::permission::Permission;
use bcrypt::DEFAULT_COST;
use bson::{doc, DateTime, Document};
use log::info;
use mongodb::options::IndexOptions;
use mongodb::{Client, IndexModel};

pub fn get_list_permission() -> Vec<Permission> {
    let current_time = DateTime::now();
    vec![
        //super admin role( ultimate role)
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06123"),
            value: "app::admin::all".to_string(),
            name: "Super Admin".to_string(),
            group: "Admin".to_string(),
            description: "Super Admin".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //read write member
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06124"),
            value: "app::member::write".to_string(),
            name: "Write Member".to_string(),
            group: "Member".to_string(),
            description: "Can Insert/Update Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06125"),
            value: "app::member::read".to_string(),
            name: "Read Member".to_string(),
            group: "Member".to_string(),
            description: "Can Read Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //read write branch
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06126"),
            value: "app::branch::write".to_string(),
            name: "Write Branch".to_string(),
            group: "Branch".to_string(),
            description: "Can Update Or Insert Branch".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06127"),
            value: "app::branch::read".to_string(),
            name: "Read Branch".to_string(),
            group: "Branch".to_string(),
            description: "Can Read Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //read write product
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06128"),
            value: "app::product::write".to_string(),
            name: "Write Product".to_string(),
            group: "Product".to_string(),
            description: "Can Insert Or Update Product".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06129"),
            value: "app::product::read".to_string(),
            name: "Read Product".to_string(),
            group: "Product".to_string(),
            description: "Can Read Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //read write membership
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612a"),
            value: "app::membership::write".to_string(),
            name: "Write Membership".to_string(),
            group: "Membership".to_string(),
            description: "Can Insert Or Update Membership".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612b"),
            value: "app::membership::read".to_string(),
            name: "Read Membership".to_string(),
            group: "Membership".to_string(),
            description: "Can Read Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //read write account
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612c"),
            value: "app::account::write".to_string(),
            name: "Write Account".to_string(),
            group: "Account".to_string(),
            description: "Can Create Or Update Account".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612d"),
            value: "app::account::read".to_string(),
            name: "Read Account".to_string(),
            group: "Account".to_string(),
            description: "Can Read Account".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //read write staff
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612e"),
            value: "app::account::staff::write".to_string(),
            name: "Write Staff".to_string(),
            group: "Account".to_string(),
            description: "Can Create Or Update Staff".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612f"),
            value: "app::account::staff::read".to_string(),
            name: "Read Staff".to_string(),
            group: "Account".to_string(),
            description: "Can Read Staff".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //read write transaction
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06130"),
            value: "app::transaction::write".to_string(),
            name: "Write Transaction".to_string(),
            group: "Transaction".to_string(),
            description: "Can Create Or Update Transaction".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06131"),
            value: "app::transaction::read".to_string(),
            name: "Read Transaction".to_string(),
            group: "Transaction".to_string(),
            description: "Can Read Transaction".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //read write promotion
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06132"),
            value: "app::promotion::write".to_string(),
            name: "Write Promotion".to_string(),
            group: "Promotion".to_string(),
            description: "Can Create Or Update Promotion".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06133"),
            value: "app::promotion::read".to_string(),
            name: "Read Promotion".to_string(),
            group: "Promotion".to_string(),
            description: "Can Read Promotion".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //read write coach
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06134"),
            value: "app::coach::write".to_string(),
            name: "Write Coach".to_string(),
            group: "Coach".to_string(),
            description: "Can Create Or Update Coach".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06135"),
            value: "app::coach::read".to_string(),
            name: "Read Coach".to_string(),
            group: "Coach".to_string(),
            description: "Can Read Coach".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06136"),
            value: "app::account::permission::read".to_string(),
            name: "Read Account Permission".to_string(),
            group: "Permission".to_string(),
            description: "Can Read Account Permission".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06137"),
            value: "app::account::permission::write".to_string(),
            name: "Read Account Permission".to_string(),
            group: "Permission".to_string(),
            description: "Can Read Account Permission".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06138"),
            value: "app::permission::read".to_string(),
            name: "Read Permission".to_string(),
            group: "Permission".to_string(),
            description: "Can Read Permission".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06139"),
            value: "app::permission::write".to_string(),
            name: "Read Permission".to_string(),
            group: "Permission".to_string(),
            description: "Can Read Permission".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
    ]
}

pub async fn init_seeder(db_client: &Client) {
    info!(target: "seeder","Spread the seed...");
    let branch_id_1 = create_object_id_option("6742c74a15e68b0e7ee06120");
    let branch_id_2 = create_object_id_option("6742c74a15e68b0e7ee06121");
    let account_id_1 = create_object_id_option("6742c74a15e68b0e7ee06122");

    //seed permission
    {
        info!(target: "seeder","20% seed permission");
        for mut permission in get_list_permission() {
            let exist = Orm::get("permission")
                .filter_object_id("_id", &permission.id.unwrap())
                .one::<Permission>(&db_client)
                .await;

            if exist.is_err() {
                let _index = Orm::insert("permission").one(permission, &db_client).await;
            } else {
                let id = &permission.id.unwrap();
                permission.id = None;
                let _index = Orm::update("permission")
                    .filter_object_id("_id", id)
                    .one(permission, &db_client)
                    .await;
            }
        }
    }

    //seed branch
    {
        info!(target: "seeder","50% seed branch");
        let current_date_time = DateTime::now();
        let branchs = vec![
            Branch {
                id: branch_id_1.clone(),
                branch_name: "Branch A".to_string(),
                branch_description: "Cabang utama pertama".to_string(),
                branch_email: Some(String::from("hq1@gmail.com")),
                branch_phone_number: Some(String::from("+62812269999")),
                branch_address: Some(String::from("Jakarta Selatan")),
                branch_owner: account_id_1.clone(),
                created_at: DateTime::parse_rfc3339_str("2024-12-09T16:27:32.002Z")
                    .unwrap_or(current_date_time.clone()),
                updated_at: DateTime::parse_rfc3339_str("2024-12-09T16:27:32.002Z")
                    .unwrap_or(current_date_time.clone()),
                deleted: false,
            },
            Branch {
                id: branch_id_2.clone(),
                branch_name: "Cabang B".to_string(),
                branch_description: "Cabang utama kedua".to_string(),
                branch_email: Some(String::from("hq2@gmail.com")),
                branch_phone_number: Some(String::from("+62812269998")),
                branch_address: Some(String::from("Jakarta Selatan")),
                branch_owner: account_id_1.clone(),
                created_at: DateTime::parse_rfc3339_str("2024-12-10T16:27:32.002Z")
                    .unwrap_or(current_date_time.clone()),
                updated_at: DateTime::parse_rfc3339_str("2024-12-10T16:27:32.002Z")
                    .unwrap_or(current_date_time.clone()),
                deleted: false,
            },
        ];

        for mut branch in branchs {
            let exist = Orm::get("branch")
                .filter_object_id("_id", &branch.id.unwrap())
                .one::<Branch>(&db_client)
                .await;
            if exist.is_err() {
                let _save = Orm::insert("branch").one(branch, &db_client).await;
            } else {
                let id = &branch.id.unwrap();
                branch.id = None;
                let _index = Orm::update("branch")
                    .filter_object_id("_id", &id)
                    .one(branch, &db_client)
                    .await;
            }
        }
    }

    //seed account
    {
        info!(target: "seeder","70% seed account");
        let pass = bcrypt::hash("12345678", DEFAULT_COST).unwrap_or(String::from("12345678"));
        let user_exist = Orm::get("account")
            .filter_object_id("_id", &account_id_1.unwrap())
            .one::<Account>(&db_client)
            .await;

        if user_exist.is_err() {
            let _index = Orm::insert("account")
                .one(
                    Account {
                        id: account_id_1,
                        full_name: "Owner Strong Teams".to_string(),
                        email: "owner@gmail.com".to_string(),
                        password: pass,
                        gender: "M".to_string(),
                        job_title: "OWNER".to_string(),
                        report_to: None,
                        branch_id: branch_id_1.clone(),
                        created_at: DateTime::now(),
                        updated_at: DateTime::now(),
                        deleted: false,
                    },
                    &db_client,
                )
                .await;
        }
    }

    //seed account-permission
    {
        info!(target: "seeder","90% seed account-permission");
        let account_permission: Vec<AccountPermission> = get_list_permission()
            .iter()
            .map(|e| {
                return AccountPermission {
                    id: e.id.clone(),
                    account_id: account_id_1.clone(),
                    permission_id: e.id.clone(),
                    name: e.name.to_string(),
                    value: e.value.to_string(),
                    created_at: DateTime::now(),
                    updated_at: DateTime::now(),
                    deleted: false,
                };
            })
            .collect();
        for mut permission in account_permission {
            let exist = Orm::get("account-permission")
                .filter_object_id("_id", &permission.id.unwrap())
                .one::<AccountPermission>(&db_client)
                .await;
            if exist.is_err() {
                let _save_account_permission = Orm::insert("account-permission")
                    .one(permission, &db_client)
                    .await;
            } else {
                let id = &permission.id.unwrap();
                permission.id = None;
                let _index = Orm::update("account-permission")
                    .filter_object_id("_id", &id)
                    .one(permission, &db_client)
                    .await;
            }
        }
    }
    //create index
    {
        info!(target: "seeder","99% seed index");

        let _index_account = &db_client
            .database(DB_NAME)
            .collection::<Document>("account")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "full_name": "text", "email":1 })
                    .options(
                        IndexOptions::builder()
                            .name("account-index".to_string())
                            .unique(false)
                            .build(),
                    )
                    .build(),
            )
            .await;

        let _index_permission = &db_client
            .database(DB_NAME)
            .collection::<Document>("permission")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "value":1 })
                    .options(
                        IndexOptions::builder()
                            .name("permission-value-index".to_string())
                            .unique(false)
                            .build(),
                    )
                    .build(),
            )
            .await;

        let _index_coach = &db_client
            .database(DB_NAME)
            .collection::<Document>("coach")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "full_name": "text", "email":1 })
                    .options(
                        IndexOptions::builder()
                            .name("coach-index".to_string())
                            .unique(false)
                            .build(),
                    )
                    .build(),
            )
            .await;

        let _index_branch = &db_client
            .database(DB_NAME)
            .collection::<Document>("branch")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "branch_name": "text" })
                    .options(
                        IndexOptions::builder()
                            .name("branch-index".to_string())
                            .unique(false)
                            .build(),
                    )
                    .build(),
            )
            .await;
    }
    info!(target: "seeder", "seeding completed application ready");
}
