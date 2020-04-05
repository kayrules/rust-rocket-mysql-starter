table! {
    activities (id) {
        id -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        activity -> Varchar,
        status -> Tinyint,
        user_1 -> Nullable<Integer>,
        user_2 -> Nullable<Integer>,
        identifier_1 -> Nullable<Integer>,
        identifier_2 -> Nullable<Integer>,
        param_1 -> Nullable<Varchar>,
        param_2 -> Nullable<Varchar>,
        param_3 -> Nullable<Varchar>,
    }
}

table! {
    quests (id) {
        id -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        quest -> Varchar,
        status -> Tinyint,
        points -> Integer,
        level -> Varchar,
        icon_url -> Nullable<Varchar>,
        activity -> Varchar,
        activity_type -> Nullable<Varchar>,
        activity_count -> Integer,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        name -> Varchar,
        email -> Varchar,
        latitude -> Nullable<Decimal>,
        longitude -> Nullable<Decimal>,
        language -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
        age -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    activities,
    quests,
    users,
);
