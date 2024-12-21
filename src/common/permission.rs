pub(crate) mod permission{
    pub mod app{
        pub mod admin{
            pub const GROUP:&str="Admin";
            pub const ALL: &str ="app::admin::all";
        }
        pub mod user{
            pub const GROUP:&str="User";
            pub const CREATE: &str ="app::user::create";
            pub const READ: &str ="app::user::read";
            pub const UPDATE: &str ="app::user::update";
            pub const DELETE: &str ="app::user::delete";
        }
        pub mod branch{
            pub const GROUP:&str="Branch";
            pub const CREATE: &str ="app::branch::create";
            pub const READ: &str ="app::branch::read";
            pub const UPDATE: &str ="app::branch::update";
            pub const DELETE: &str ="app::branch::delete";
        }
        pub mod member{
            pub const GROUP:&str="Member";
            pub const CREATE: &str ="app::member::create";
            pub const READ: &str ="app::member::read";
            pub const UPDATE: &str ="app::member::update";
            pub const DELETE: &str ="app::member::delete";
        }
        pub mod coach{
            pub const GROUP:&str="Coach";
            pub const CREATE: &str ="app::coach::create";
            pub const READ: &str ="app::coach::read";
            pub const UPDATE: &str ="app::coach::update";
            pub const DELETE: &str ="app::coach::delete";
        }
        pub mod product{
            pub const GROUP:&str="Product";
            pub const CREATE: &str ="app::product::create";
            pub const READ: &str ="app::product::read";
            pub const UPDATE: &str ="app::product::update";
            pub const DELETE: &str ="app::product::delete";
        }
        pub mod discount{
            pub const GROUP:&str="Discount";
            pub const CREATE: &str ="app::discount::create";
            pub const READ: &str ="app::discount::read";
            pub const UPDATE: &str ="app::discount::update";
            pub const DELETE: &str ="app::discount::delete";
        }
        pub mod membership{
            pub const GROUP:&str="Membership";
            pub const CREATE: &str ="app::membership::create";
            pub const READ: &str ="app::membership::read";
            pub const UPDATE: &str ="app::membership::update";
            pub const DELETE: &str ="app::membership::delete";
        }
        pub mod stock{
            pub const GROUP:&str="Stock";
            pub const CREATE: &str ="app::stock::create";
            pub const READ: &str ="app::stock::read";
            pub const UPDATE: &str ="app::stock::update";
            pub const DELETE: &str ="app::stock::delete";
        }
        pub mod transaction{
            pub const GROUP:&str="Transaction";
            pub const CREATE: &str ="app::transaction::create";
            pub const READ: &str ="app::transaction::read";
            pub const UPDATE: &str ="app::transaction::update";
            pub const DELETE: &str ="app::transaction::delete";
        }
    }
}