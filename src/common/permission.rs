pub(crate) mod permission{
    pub mod app{
        pub mod admin{
            pub const GROUP:&str="Admin";
            pub const ALL: &str ="app::admin::ALL";
        }
        pub mod user{
            pub const GROUP:&str="User";
            pub const CREATE: &str ="app::user::CREATE";
            pub const READ: &str ="app::user::READ";
            pub const UPDATE: &str ="app::user::UPDATE";
            pub const DELETE: &str ="app::user::DELETE";
        }
        pub mod branch{
            pub const GROUP:&str="Branch";
            pub const CREATE: &str ="app::branch::CREATE";
            pub const READ: &str ="app::branch::READ";
            pub const UPDATE: &str ="app::branch::UPDATE";
            pub const DELETE: &str ="app::DELETE::DELETE";
        }
        pub mod member{
            pub const GROUP:&str="Member";
            pub const CREATE: &str ="app::member::CREATE";
            pub const READ: &str ="app::member::READ";
            pub const UPDATE: &str ="app::member::UPDATE";
            pub const DELETE: &str ="app::member::DELETE";
        }
        pub mod coach{
            pub const GROUP:&str="Coach";
            pub const CREATE: &str ="app::coach::CREATE";
            pub const READ: &str ="app::coach::READ";
            pub const UPDATE: &str ="app::coach::UPDATE";
            pub const DELETE: &str ="app::coach::DELETE";
        }
        pub mod product{
            pub const GROUP:&str="Product";
            pub const CREATE: &str ="app::product::CREATE";
            pub const READ: &str ="app::product::READ";
            pub const UPDATE: &str ="app::product::UPDATE";
            pub const DELETE: &str ="app::product::DELETE";
        }
        pub mod discount{
            pub const GROUP:&str="Discount";
            pub const CREATE: &str ="app::discount::CREATE";
            pub const READ: &str ="app::discount::READ";
            pub const UPDATE: &str ="app::discount::UPDATE";
            pub const DELETE: &str ="app::discount::DELETE";
        }
        pub mod membership{
            pub const GROUP:&str="Membership";
            pub const CREATE: &str ="app::membership::CREATE";
            pub const READ: &str ="app::membership::READ";
            pub const UPDATE: &str ="app::membership::UPDATE";
            pub const DELETE: &str ="app::membership::DELETE";
        }
        pub mod stock{
            pub const GROUP:&str="Stock";
            pub const CREATE: &str ="app::stock::CREATE";
            pub const READ: &str ="app::stock::READ";
            pub const UPDATE: &str ="app::stock::UPDATE";
            pub const DELETE: &str ="app::stock::DELETE";
        }
        pub mod transaction{
            pub const GROUP:&str="Transaction";
            pub const CREATE: &str ="app::transaction::CREATE";
            pub const READ: &str ="app::transaction::READ";
            pub const UPDATE: &str ="app::transaction::UPDATE";
            pub const DELETE: &str ="app::transaction::DELETE";
        }
    }
}