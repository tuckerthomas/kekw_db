table! {
    periods (id) {
        id -> Int4,
        start_day -> Int8,
        end_day -> Nullable<Int8>,
        vote_message -> Nullable<Varchar>,
    }
}

table! {
    rolls (id) {
        id -> Int4,
        selection_1 -> Int4,
        selection_2 -> Int4,
        period_id -> Int4,
        selection_1_emote -> Nullable<Varchar>,
        selection_2_emote -> Nullable<Varchar>,
    }
}

table! {
    submissions (id) {
        id -> Int4,
        dis_user_id -> Varchar,
        title -> Varchar,
        link -> Varchar,
        period_id -> Int4,
    }
}

joinable!(rolls -> periods (period_id));
joinable!(submissions -> periods (period_id));

allow_tables_to_appear_in_same_query!(
    periods,
    rolls,
    submissions,
);
