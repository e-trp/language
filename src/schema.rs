// @generated automatically by Diesel CLI.

diesel::table! {
    verb_forms (id) {
        id -> Nullable<Integer>,
        word_id -> Integer,
        base_form -> Text,
        past_simple -> Text,
        past_participle -> Text,
    }
}

diesel::table! {
    words (id) {
        id -> Nullable<Integer>,
        source -> Text,
        description -> Nullable<Text>,
        phonetic -> Nullable<Text>,
        part_of_speech -> Nullable<Text>,
    }
}

diesel::joinable!(verb_forms -> words (word_id));

diesel::allow_tables_to_appear_in_same_query!(
    verb_forms,
    words,
);
