use crate::common::constant::DEFAULT_ID_NON_MEMBER;
use crate::common::orm::orm::Orm;
use crate::common::orm::DB_NAME;
use crate::common::permission::permission::app;
use crate::common::utils::{create_object_id_option, create_or_new_object_id};
use crate::entity::account::Account;
use crate::entity::account_permission::AccountPermission;
use crate::entity::branch::Branch;
use crate::entity::membership::Membership;
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
            value: app::admin::ALL.to_string(),
            name: "Super Admin".to_string(),
            group: app::admin::GROUP.to_string(),
            description: "Super Admin".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //READ write member
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06124"),
            value: app::member::CREATE.to_string(),
            name: "Create Member".to_string(),
            group: app::member::GROUP.to_string(),
            description: "Can Insert Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06125"),
            value: app::member::READ.to_string(),
            name: "Read Member".to_string(),
            group: app::member::GROUP.to_string(),
            description: "Can Read Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06126"),
            value: app::member::UPDATE.to_string(),
            name: "Update Member".to_string(),
            group: app::member::GROUP.to_string(),
            description: "Can Update Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06127"),
            value: app::member::DELETE.to_string(),
            name: "Delete Member".to_string(),
            group: app::member::GROUP.to_string(),
            description: "Can Delete Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //READ write branch
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06128"),
            value: app::branch::CREATE.to_string(),
            name: "Create Branch".to_string(),
            group: app::branch::GROUP.to_string(),
            description: "Can Insert Branch".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06129"),
            value: app::branch::READ.to_string(),
            name: "Read Branch".to_string(),
            group: app::branch::GROUP.to_string(),
            description: "Can Read Branch".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612a"),
            value: app::branch::UPDATE.to_string(),
            name: "Update Branch".to_string(),
            group: app::branch::GROUP.to_string(),
            description: "Can Update Branch".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612b"),
            value: app::branch::DELETE.to_string(),
            name: "Delete Branch".to_string(),
            group: app::branch::GROUP.to_string(),
            description: "Can Delete Branch".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //READ write product
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612c"),
            value: app::product::CREATE.to_string(),
            name: "Create Product".to_string(),
            group: app::product::GROUP.to_string(),
            description: "Can Insert Product".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612d"),
            value: app::product::READ.to_string(),
            name: "Read Product".to_string(),
            group: app::product::GROUP.to_string(),
            description: "Can Read Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612e"),
            value: app::product::UPDATE.to_string(),
            name: "Update Product".to_string(),
            group: app::product::GROUP.to_string(),
            description: "Can Update Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0612f"),
            value: app::product::DELETE.to_string(),
            name: "Delete Product".to_string(),
            group: app::product::READ.to_string(),
            description: "Can Delete Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //READ write membership
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06130"),
            value: app::membership::CREATE.to_string(),
            name: "Insert Membership".to_string(),
            group: app::membership::GROUP.to_string(),
            description: "Can Insert Membership".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06131"),
            value: app::membership::READ.to_string(),
            name: "Read Membership".to_string(),
            group: app::membership::GROUP.to_string(),
            description: "Can Read Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06132"),
            value: app::membership::UPDATE.to_string(),
            name: "Update Membership".to_string(),
            group: app::membership::GROUP.to_string(),
            description: "Can Update Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06133"),
            value: app::membership::DELETE.to_string(),
            name: "Delete Membership".to_string(),
            group: app::membership::GROUP.to_string(),
            description: "Can Delete Member".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //READ write account
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06134"),
            value: app::user::CREATE.to_string(),
            name: "Create Account".to_string(),
            group: app::user::GROUP.to_string(),
            description: "Can Create User".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06135"),
            value: app::user::READ.to_string(),
            name: "Read Account".to_string(),
            group: app::user::GROUP.to_string(),
            description: "Can Read User".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06136"),
            value: app::user::UPDATE.to_string(),
            name: "Update Account".to_string(),
            group: app::user::GROUP.to_string(),
            description: "Can Update User".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06134"),
            value: app::user::DELETE.to_string(),
            name: "Delete Account".to_string(),
            group: app::user::GROUP.to_string(),
            description: "Can Delete User".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //READ write transaction
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06135"),
            value: app::transaction::CREATE.to_string(),
            name: "Write Transaction".to_string(),
            group: app::transaction::GROUP.to_string(),
            description: "Can Create Or Update Transaction".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06136"),
            value: app::transaction::READ.to_string(),
            name: "Read Transaction".to_string(),
            group: app::transaction::GROUP.to_string(),
            description: "Can Read Transaction".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06137"),
            value: app::transaction::UPDATE.to_string(),
            name: "Update Transaction".to_string(),
            group: app::transaction::GROUP.to_string(),
            description: "Can Update Transaction".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06138"),
            value: app::transaction::DELETE.to_string(),
            name: "Delete Transaction".to_string(),
            group: app::transaction::GROUP.to_string(),
            description: "Can Delete Transaction".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //READ write coach
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06139"),
            value: app::coach::CREATE.to_string(),
            name: "Create Coach".to_string(),
            group: app::coach::GROUP.to_string(),
            description: "Can Create Coach".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0613a"),
            value: app::coach::READ.to_string(),
            name: "Rad Coach".to_string(),
            group: app::coach::GROUP.to_string(),
            description: "Can Read Coach".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0613b"),
            value: app::coach::UPDATE.to_string(),
            name: "Update Coach".to_string(),
            group: app::coach::GROUP.to_string(),
            description: "Can Update Coach".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0613c"),
            value: app::coach::DELETE.to_string(),
            name: "Delete Coach".to_string(),
            group: app::coach::GROUP.to_string(),
            description: "Can Delete Coach".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //discount
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0613d"),
            value: app::discount::CREATE.to_string(),
            name: "Create Discount".to_string(),
            group: app::discount::GROUP.to_string(),
            description: "Can Insert Discount".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0613e"),
            value: app::discount::READ.to_string(),
            name: "Read Discount".to_string(),
            group: app::discount::GROUP.to_string(),
            description: "Can Read Discount".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee0613f"),
            value: app::discount::UPDATE.to_string(),
            name: "Update Discount".to_string(),
            group: app::discount::GROUP.to_string(),
            description: "Can Update Discount".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06140"),
            value: app::discount::DELETE.to_string(),
            name: "Delete Discount".to_string(),
            group: app::discount::GROUP.to_string(),
            description: "Can Delete Discount".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        //stock
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06141"),
            value: app::stock::CREATE.to_string(),
            name: "Create Stock".to_string(),
            group: app::stock::GROUP.to_string(),
            description: "Can Create Stock".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06142"),
            value: app::stock::READ.to_string(),
            name: "Read Stock".to_string(),
            group: app::stock::GROUP.to_string(),
            description: "Can Read Stock".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06143"),
            value: app::stock::UPDATE.to_string(),
            name: "Update Stock".to_string(),
            group: app::stock::GROUP.to_string(),
            description: "Can Update Stock".to_string(),
            created_at: current_time.clone(),
            updated_at: current_time,
            deleted: false,
        },
        Permission {
            id: create_or_new_object_id("6742c74a15e68b0e7ee06144"),
            value: app::stock::DELETE.to_string(),
            name: "Delete Stock".to_string(),
            group: app::stock::GROUP.to_string(),
            description: "Can Delete Stock".to_string(),
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

    {
        info!(target: "seeder","25% seed membership");
        let id = create_or_new_object_id(DEFAULT_ID_NON_MEMBER);
        let exist = Orm::get("membership")
            .filter_object_id("_id", &id.clone().unwrap())
            .one::<Permission>(&db_client)
            .await;

        let membership = Membership {
            id: id.clone(),
            branch_id: branch_id_1,
            name: "FOR NON MEMBER".to_string(),
            price: 0.0,
            price_per_item: 0.0,
            quota: 0,
            description: "FOR NON MEMBER".to_string(),
            kind: Some("NON-MEMBER".to_string()),
            created_by_id: account_id_1,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            deleted: false,
        };
        if exist.is_err() {
            let _index = Orm::insert("permission").one(membership, &db_client).await;
        } else {
            let _index = Orm::update("permission")
                .filter_object_id("_id", &id.unwrap())
                .one(membership, &db_client)
                .await;
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
                        identity_number: Some("6403131788890001".to_string()),
                        gender: "M".to_string(),
                        job_title: "OWNER".to_string(),
                        report_to_id: None,
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
    //CREATE index
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

        let _index_member = &db_client
            .database(DB_NAME)
            .collection::<Document>("member")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "full_name": "text","member_code": "text", "email":1 })
                    .options(
                        IndexOptions::builder()
                            .name("member-index".to_string())
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
