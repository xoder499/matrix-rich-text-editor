// Copyright 2023 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    tests::testutils_composer_model::{cm, tx},
    MenuAction,
};

#[test]
fn set_mention_replace_all_text() {
    let mut model = cm("|");
    let update = model.replace_text("@alic".into());
    let MenuAction::Suggestion(suggestion) = update.menu_action else {
        panic!("No suggestion pattern found")
    };
    model.set_mention_from_suggestion(
        "https://matrix.to/#/@alice:matrix.org".into(),
        "Alice".into(),
        suggestion,
        vec![],
    );
    assert_eq!(
        tx(&model),
        "<a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|",
    );
}

#[test]
fn set_mention_replace_end_of_text() {
    let mut model = cm("hello |");
    let update = model.replace_text("@ali".into());
    let MenuAction::Suggestion(suggestion) = update.menu_action else {
        panic!("No suggestion pattern found")
    };
    model.set_mention_from_suggestion(
        "https://matrix.to/#/@alice:matrix.org".into(),
        "Alice".into(),
        suggestion,
        vec![],
    );
    assert_eq!(
        tx(&model),
        "hello <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|",
    );
}

#[test]
fn set_mention_replace_start_of_text() {
    let mut model = cm("| says hello");
    let update = model.replace_text("@ali".into());
    let MenuAction::Suggestion(suggestion) = update.menu_action else {
        panic!("No suggestion pattern found")
    };
    model.set_mention_from_suggestion(
        "https://matrix.to/#/@alice:matrix.org".into(),
        "Alice".into(),
        suggestion,
        vec![],
    );
    assert_eq!(
        tx(&model),
        "<a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| says hello",
    );
}

#[test]
fn set_mention_replace_middle_of_text() {
    let mut model = cm("Like | said");
    let update = model.replace_text("@ali".into());

    let MenuAction::Suggestion(suggestion) = update.menu_action else {
        panic!("No suggestion pattern found")
    };
    model.set_mention_from_suggestion(
        "https://matrix.to/#/@alice:matrix.org".into(),
        "Alice".into(),
        suggestion,
        vec![],
    );
    assert_eq!(tx(&model),
    "Like <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| said");
}

#[test]
fn set_mention_replace_all_text_formatting_node() {
    let mut model = cm("<strong>|</strong>");
    let update = model.replace_text("@alic".into());
    let MenuAction::Suggestion(suggestion) = update.menu_action else {
        panic!("No suggestion pattern found")
    };
    model.set_mention_from_suggestion(
        "https://matrix.to/#/@alice:matrix.org".into(),
        "Alice".into(),
        suggestion,
        vec![],
    );
    assert_eq!(
        tx(&model),
        "<a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|",
    );
}

#[test]
fn set_mention_replace_end_of_text_formatting_node() {
    let mut model = cm("<strong>hello |</strong>");

    let update = model.replace_text("@ali".into());
    let MenuAction::Suggestion(suggestion) = update.menu_action else {
        panic!("No suggestion pattern found")
    };
    model.set_mention_from_suggestion(
        "https://matrix.to/#/@alice:matrix.org".into(),
        "Alice".into(),
        suggestion,
        vec![],
    );
    assert_eq!(
        tx(&model),
        "<strong>hello <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|</strong>",
    );
}

#[test]
#[ignore] // same issue here as the big stack trace above

fn set_mention_replace_start_of_text_formatting_node() {
    let mut model = cm("<strong>| says hello</strong>");
    let update = model.replace_text("@ali".into());
    let MenuAction::Suggestion(suggestion) = update.menu_action else {
        panic!("No suggestion pattern found")
    };
    model.set_mention_from_suggestion(
        "https://matrix.to/#/@alice:matrix.org".into(),
        "Alice".into(),
        suggestion,
        vec![],
    );

    dbg!(model.get_content_as_html());
    assert_eq!(
        tx(&model),
        "<strong><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a> says hello</strong>",
    );
}
