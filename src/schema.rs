table! {
    users (id) {
        id -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
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

allow_tables_to_appear_in_same_query!(users,);
