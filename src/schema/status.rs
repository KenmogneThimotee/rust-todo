// @generated automatically by Diesel CLI.

diesel::table! {
    /// Representation of the `status` table.
    ///
    /// (Automatically generated by Diesel.)
    status (id) {
        /// The `id` column of the `status` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Integer,
        /// The `name` column of the `status` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `color` column of the `status` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        color -> Text,
    }
}