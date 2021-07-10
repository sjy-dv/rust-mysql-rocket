table! {
    user (idx) {
        idx -> Nullable<Integer>,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(user,);
