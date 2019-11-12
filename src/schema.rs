table! {
    pageviews (id) {
        id -> Int4,
        view_time -> Timestamp,
        url -> Varchar,
        user_agent -> Varchar,
        referrer -> Varchar,
    }
}
