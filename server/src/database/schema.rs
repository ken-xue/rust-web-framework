// @generated automatically by Diesel CLI.

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
    sys_config (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Varchar,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        config -> Nullable<Text>,
        #[max_length = 128]
        remark -> Nullable<Varchar>,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        #[max_length = 1]
        deleted -> Bool,
        #[max_length = 1]
        deletable -> Char,
        #[max_length = 1]
        editable -> Nullable<Char>,
    }
}

diesel::table! {
    sys_dept (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Varchar,
        #[max_length = 32]
        parent_uuid -> Nullable<Varchar>,
        #[max_length = 64]
        name -> Varchar,
        order -> Nullable<Integer>,
        #[max_length = 64]
        remark -> Nullable<Varchar>,
        #[max_length = 64]
        status -> Nullable<Varchar>,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        deleted -> Bool,
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
        uuid -> Varchar,
        #[max_length = 32]
        status -> Varchar,
        #[max_length = 32]
        parent_uuid -> Nullable<Varchar>,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        #[max_length = 200]
        path -> Nullable<Varchar>,
        #[max_length = 255]
        component -> Nullable<Varchar>,
        #[max_length = 255]
        redirect -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 50]
        icon -> Nullable<Varchar>,
        #[max_length = 200]
        permission -> Nullable<Varchar>,
        #[max_length = 1]
        menu_type -> Nullable<Varchar>,
        order -> Nullable<Integer>,
        #[max_length = 64]
        remark -> Nullable<Varchar>,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        deleted -> Bool,
    }
}

diesel::table! {
    sys_role (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Varchar,
        #[max_length = 64]
        name -> Nullable<Varchar>,
        #[max_length = 64]
        remark -> Nullable<Varchar>,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        deleted -> Bool,
    }
}

diesel::table! {
    sys_role_of_menu (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Varchar,
        #[max_length = 32]
        role_uuid -> Varchar,
        #[max_length = 32]
        menu_uuid -> Varchar,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        deleted -> Bool,
    }
}

diesel::table! {
    sys_user (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Varchar,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 64]
        password -> Varchar,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        status -> Integer,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        deleted -> Bool,
        #[max_length = 100]
        avatar -> Nullable<Varchar>,
    }
}

diesel::table! {
    sys_user_of_dept (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Varchar,
        #[max_length = 32]
        user_uuid -> Varchar,
        #[max_length = 32]
        dept_uuid -> Varchar,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        deleted -> Bool,
    }
}

diesel::table! {
    sys_user_of_role (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 32]
        uuid -> Varchar,
        #[max_length = 32]
        user_uuid -> Varchar,
        #[max_length = 32]
        role_uuid -> Varchar,
        #[max_length = 64]
        creator -> Nullable<Varchar>,
        #[max_length = 64]
        modifier -> Nullable<Varchar>,
        gmt_create -> Datetime,
        gmt_modified -> Datetime,
        deleted -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sys_captcha,
    sys_config,
    sys_dept,
    sys_dictionary,
    sys_log,
    sys_menu,
    sys_role,
    sys_role_of_menu,
    sys_user,
    sys_user_of_dept,
    sys_user_of_role,
);
