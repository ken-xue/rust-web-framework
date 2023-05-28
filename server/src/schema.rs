// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Bigint,
        #[max_length = 256]
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    sys_captcha (id) {
        #[max_length = 36]
        uuid -> Char,
        #[max_length = 6]
        code -> Varchar,
        expire_time -> Nullable<Datetime>,
        id -> Bigint,
    }
}

diesel::table! {
    sys_dictionary (id) {
        id -> Bigint,
        #[max_length = 200]
        key -> Nullable<Varchar>,
        value -> Nullable<Text>,
        fixed -> Nullable<Bool>,
    }
}

diesel::table! {
    sys_log (id) {
        id -> Bigint,
        #[max_length = 50]
        username -> Nullable<Varchar>,
        #[max_length = 50]
        operation -> Nullable<Varchar>,
        #[max_length = 200]
        method -> Nullable<Varchar>,
        #[max_length = 5000]
        params -> Nullable<Varchar>,
        execute_time -> Bigint,
        #[max_length = 64]
        ip -> Nullable<Varchar>,
        occur_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    sys_menu (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Nullable<Varchar>,
        #[max_length = 32]
        menu_parent_uuid -> Nullable<Varchar>,
        #[max_length = 64]
        menu_name -> Nullable<Varchar>,
        #[max_length = 200]
        menu_url -> Nullable<Varchar>,
        #[max_length = 200]
        menu_perms -> Nullable<Varchar>,
        #[max_length = 1]
        menu_type -> Nullable<Varchar>,
        #[max_length = 20]
        menu_icon -> Nullable<Varchar>,
        menu_order -> Nullable<Integer>,
        #[max_length = 64]
        menu_remark -> Nullable<Varchar>,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        #[max_length = 1]
        deleted -> Char,
    }
}

diesel::table! {
    sys_role (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Nullable<Varchar>,
        #[max_length = 64]
        role_name -> Nullable<Varchar>,
        #[max_length = 64]
        role_remark -> Nullable<Varchar>,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        #[max_length = 1]
        deleted -> Char,
    }
}

diesel::table! {
    sys_role_of_menu (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Nullable<Varchar>,
        #[max_length = 32]
        role_uuid -> Nullable<Varchar>,
        #[max_length = 32]
        menu_uuid -> Nullable<Varchar>,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        #[max_length = 1]
        deleted -> Char,
    }
}

diesel::table! {
    sys_user (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Nullable<Varchar>,
        #[max_length = 64]
        account -> Nullable<Varchar>,
        #[max_length = 64]
        password -> Nullable<Varchar>,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        #[max_length = 128]
        email -> Nullable<Varchar>,
        status -> Nullable<Integer>,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        #[max_length = 1]
        deleted -> Char,
        #[max_length = 100]
        avatar -> Nullable<Varchar>,
    }
}

diesel::table! {
    sys_user_of_role (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Nullable<Varchar>,
        #[max_length = 32]
        user_uuid -> Nullable<Varchar>,
        #[max_length = 32]
        role_uuid -> Nullable<Varchar>,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        #[max_length = 1]
        deleted -> Char,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    sys_captcha,
    sys_dictionary,
    sys_log,
    sys_menu,
    sys_role,
    sys_role_of_menu,
    sys_user,
    sys_user_of_role,
);
