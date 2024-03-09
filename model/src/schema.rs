// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Integer,
        category_id -> Nullable<Integer>,
        title -> Text,
        thumb -> Nullable<Text>,
        author -> Nullable<Text>,
        descption -> Text,
        content -> Text,
        visit -> Integer,
        is_elite -> Integer,
        is_top -> Integer,
        is_hot -> Integer,
        status -> Integer,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    categories (id) {
        id -> Integer,
        pid -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        name -> Text,
        icon -> Nullable<Text>,
        status -> Integer,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    member_bills (id) {
        id -> Integer,
        member_id -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        pm -> Integer,
        number -> Double,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    member_teams (id) {
        id -> Integer,
        owner_uid -> Integer,
        parent_uid -> Integer,
        member_id -> Integer,
        level -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    members (id) {
        id -> Integer,
        unique_code -> Text,
        email -> Text,
        mobile -> Text,
        nickname -> Text,
        avatar -> Text,
        password -> Text,
        salt -> Text,
        sex -> Integer,
        balance -> Double,
        integral -> Integer,
        remark -> Text,
        status -> Integer,
        is_promoter -> Integer,
        last_login_ip -> Text,
        last_login_time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    system_action_logs (id) {
        id -> Integer,
        user_id -> Integer,
        menu_id -> Integer,
        menu_names -> Text,
        ip_address -> Text,
        ip_address_name -> Text,
        browser_agent -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    system_caches (id) {
        id -> Integer,
        key -> Text,
        #[sql_name = "type"]
        type_ -> Integer,
        value -> Text,
        attach -> Text,
        valid_time_length -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    system_depts (id) {
        id -> Integer,
        parent_id -> Integer,
        name -> Text,
        person_name -> Text,
        person_phone -> Text,
        person_email -> Text,
        describe -> Text,
        status -> Integer,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    system_dict_data (id) {
        id -> Integer,
        dict_id -> Integer,
        label -> Text,
        value -> Integer,
        remark -> Text,
        status -> Integer,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    system_dicts (id) {
        id -> Integer,
        name -> Text,
        sign -> Text,
        remark -> Text,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    system_login_logs (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        user_id -> Integer,
        ip_address -> Text,
        ip_address_name -> Text,
        browser_agent -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    system_menus (id) {
        id -> Integer,
        parent_id -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        title -> Text,
        icon -> Text,
        router_name -> Text,
        router_component -> Text,
        router_path -> Text,
        redirect -> Text,
        link -> Text,
        iframe -> Text,
        btn_auth -> Text,
        api_url -> Text,
        api_method -> Text,
        is_hide -> Integer,
        is_keep_alive -> Integer,
        is_affix -> Integer,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    system_role_menus (id) {
        id -> Integer,
        role_id -> Integer,
        menu_id -> Integer,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    system_roles (id) {
        id -> Integer,
        name -> Text,
        sign -> Text,
        describe -> Text,
        status -> Integer,
        sort -> Integer,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    system_users (id) {
        id -> Integer,
        username -> Text,
        nickname -> Text,
        role_id -> Nullable<Integer>,
        dept_id -> Nullable<Integer>,
        phone -> Text,
        email -> Text,
        sex -> Integer,
        password -> Text,
        salt -> Text,
        describe -> Text,
        expire_time -> Nullable<Timestamp>,
        status -> Integer,
        last_login_ip -> Text,
        last_login_time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(articles -> categories (category_id));
diesel::joinable!(member_bills -> members (member_id));
diesel::joinable!(system_action_logs -> system_menus (menu_id));
diesel::joinable!(system_action_logs -> system_users (user_id));
diesel::joinable!(system_dict_data -> system_dicts (dict_id));
diesel::joinable!(system_login_logs -> system_users (user_id));
diesel::joinable!(system_role_menus -> system_menus (menu_id));
diesel::joinable!(system_role_menus -> system_roles (role_id));
diesel::joinable!(system_users -> system_depts (dept_id));
diesel::joinable!(system_users -> system_roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    categories,
    member_bills,
    member_teams,
    members,
    system_action_logs,
    system_caches,
    system_depts,
    system_dict_data,
    system_dicts,
    system_login_logs,
    system_menus,
    system_role_menus,
    system_roles,
    system_users,
);
