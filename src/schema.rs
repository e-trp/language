// @generated automatically by Diesel CLI.

diesel::table! {
    words (id) {
        id -> Nullable<Integer>,
        source -> Text,
        description -> Nullable<Text>,
        phonetic -> Nullable<Text>,
        part_of_speech -> Nullable<Text>,
    }
}
