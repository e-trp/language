// @generated automatically by Diesel CLI.

diesel::table! {
    quiz (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    quiz_history (id) {
        id -> Nullable<Integer>,
        quiz_id -> Integer,
        result -> Bool,
        errors -> Integer,
        context -> Text,
        quiz_date -> Nullable<Double>,
    }
}

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

diesel::joinable!(quiz_history -> quiz (quiz_id));
diesel::joinable!(verb_forms -> words (word_id));

diesel::allow_tables_to_appear_in_same_query!(
    quiz,
    quiz_history,
    verb_forms,
    words,
);
