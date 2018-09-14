table! {
    use diesel::sql_types::*;
    use types::Job_status;

    /// Representation of the `accounts` table.
    ///
    /// (Automatically generated by Diesel.)
    accounts (id) {
        /// The `id` column of the `accounts` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `uri` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        uri -> Nullable<Varchar>,
        /// The `domain` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        domain -> Nullable<Varchar>,
        /// The `username` column of the `accounts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        username -> Varchar,
        /// The `display_name` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        display_name -> Nullable<Varchar>,
        /// The `summary` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        summary -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use types::Job_status;

    /// Representation of the `follows` table.
    ///
    /// (Automatically generated by Diesel.)
    follows (id) {
        /// The `id` column of the `follows` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `source_id` column of the `follows` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        source_id -> Int8,
        /// The `target_id` column of the `follows` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        target_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use types::Job_status;

    /// Representation of the `jobs` table.
    ///
    /// (Automatically generated by Diesel.)
    jobs (id) {
        /// The `id` column of the `jobs` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `created_at` column of the `jobs` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `status` column of the `jobs` table.
        ///
        /// Its SQL type is `Job_status`.
        ///
        /// (Automatically generated by Diesel.)
        status -> Job_status,
        /// The `kind` column of the `jobs` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        kind -> Varchar,
        /// The `job_data` column of the `jobs` table.
        ///
        /// Its SQL type is `Jsonb`.
        ///
        /// (Automatically generated by Diesel.)
        job_data -> Jsonb,
    }
}

table! {
    use diesel::sql_types::*;
    use types::Job_status;

    /// Representation of the `statuses` table.
    ///
    /// (Automatically generated by Diesel.)
    statuses (id) {
        /// The `id` column of the `statuses` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `text` column of the `statuses` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        text -> Text,
        /// The `content_warning` column of the `statuses` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        content_warning -> Nullable<Text>,
        /// The `created_at` column of the `statuses` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `account_id` column of the `statuses` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Int8,
        /// The `uri` column of the `statuses` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        uri -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use types::Job_status;

    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `email` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
        /// The `encrypted_password` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        encrypted_password -> Varchar,
        /// The `account_id` column of the `users` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Int8,
    }
}

joinable!(statuses -> accounts (account_id));
joinable!(users -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    follows,
    jobs,
    statuses,
    users,
);
