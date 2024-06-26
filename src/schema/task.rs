// @generated automatically by Diesel CLI.

diesel::table! {
    /// Representation of the `task` table.
    ///
    /// (Automatically generated by Diesel.)
    task (id) {
        /// The `id` column of the `task` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Integer,
        /// The `name` column of the `task` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `description` column of the `task` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Text,
        /// The `precedence` column of the `task` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        precedence -> Integer,
        /// The `status_id` column of the `task` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        status_id -> Integer,
        /// The `due_date` column of the `task` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        due_date -> Text,
        /// The `archive` column of the `task` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        archive -> Bool,
    }
}


