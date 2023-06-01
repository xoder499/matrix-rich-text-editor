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

/**
 * TEXT NODE
 */
#[test]
fn text_node_replace_all() {
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
fn text_node_replace_start() {
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
fn text_node_replace_middle() {
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
fn text_node_replace_end() {
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

/**
 * LINEBREAK NODES
 */
#[test]
fn linebreak_insert_before() {
    let mut model = cm("|<br />");
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
        "<a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>|<br />",
    );
}

#[test]
fn linebreak_insert_after() {
    let mut model = cm("<br />|");
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
        "<br /><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|",
    );
}

/**
 * MENTION NODES
 */
#[test]
fn mention_insert_before() {
    let mut model = cm("|<a href=\"https://matrix.to/#/@test:example.org\" contenteditable=\"false\">test</a>");
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
        "<a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>|<a href=\"https://matrix.to/#/@test:example.org\" contenteditable=\"false\">test</a>",
    );
}

#[test]
fn mention_insert_after() {
    let mut model =
        cm("<a href=\"https://matrix.to/#/@test:example.org\" contenteditable=\"false\">test</a>|");
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
        "<a href=\"https://matrix.to/#/@test:example.org\" contenteditable=\"false\">test</a><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|",
    );
}

/**
 * CONTAINER NODES
 */

/**
 * FORMATTING NODES
 */
#[test]
fn formatting_node_replace_all() {
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
fn formatting_node_replace_start() {
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

    assert_eq!(
        tx(&model),
        "<strong><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| says hello</strong>",
    );
}

#[test]
fn formatting_node_replace_middle() {
    let mut model = cm("<strong>Like | said</strong>");
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
        "<strong>Like <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| said</strong>",
    );
}

#[test]
fn formatting_node_replace_end() {
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

/**
 * LINK NODES
 */
#[test]
fn link_insert_before() {
    let mut model =
        cm("| <a href=\"https://www.somelink.com\">regular link</a>");
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
        "<a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| <a href=\"https://www.somelink.com\">regular link</a>",
    );
}

// TODO - change behaviour to allow inserting mentions into links
// see issue https://github.com/matrix-org/matrix-rich-text-editor/issues/702
#[test]
#[should_panic]
fn link_insert_middle() {
    let mut model =
        cm("<a href=\"https://www.somelink.com\">regular | link</a>");
    let update = model.replace_text("@ali".into());
    let MenuAction::Suggestion(suggestion) = update.menu_action else {
        panic!("No suggestion pattern found")
    };
}

#[test]
fn link_insert_after() {
    let mut model =
        cm("<a href=\"https://www.somelink.com\">regular link|</a>");
    let update = model.replace_text(" @ali".into());
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
        "<a href=\"https://www.somelink.com\">regular link</a> <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|",
    );
}

/**
 * LIST ITEM
 */
#[test]
fn list_item_insert_into_empty() {
    let mut model = cm("<ol><li>|</li></ol>");
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
        "<ol><li><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|</li></ol>",
    );
}

#[test]
fn list_item_replace_start() {
    let mut model = cm("<ol><li>| says hello</li></ol>");
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
        "<ol><li><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| says hello</li></ol>",
    );
}

#[test]
fn list_item_replace_middle() {
    let mut model = cm("<ol><li>Like | said</li></ol>");
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
    "<ol><li>Like <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| said</li></ol>");
}

#[test]
fn list_item_replace_end() {
    let mut model = cm("<ol><li>hello |</li></ol>");
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
        "<ol><li>hello <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|</li></ol>",
    );
}

/**
 * CodeBlock
 */
#[test]
#[should_panic]
fn codeblock_insert_anywhere() {
    let mut model = cm("<code>regular | link</code>");
    let update = model.replace_text("@ali".into());
    let MenuAction::Suggestion(suggestion) = update.menu_action else {
        panic!("No suggestion pattern found")
    };
}

/**
 * Quote
 */
#[test]
fn quote_insert_into_empty() {
    let mut model = cm("<blockquote><p>|</p></blockquote>");
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
        "<blockquote><p><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|</p></blockquote>",
    );
}

#[test]
fn quote_replace_start() {
    let mut model = cm("<blockquote><p>| says hello</p></blockquote>");
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
        "<blockquote><p><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| says hello</p></blockquote>",
    );
}

#[test]
fn quote_replace_middle() {
    let mut model = cm("<blockquote><p>Like | said</p></blockquote>");
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
    "<blockquote><p>Like <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| said</p></blockquote>");
}

#[test]
fn quote_replace_end() {
    let mut model = cm("<blockquote><p>hello |</p></blockquote>");
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
        "<blockquote><p>hello <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|</p></blockquote>",
    );
}

/**
 * PARAGRAPH
 */
#[test]
fn paragraph_insert_into_empty() {
    let mut model = cm("<p>|</p>");
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
        "<p><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|</p>",
    );
}

#[test]
fn paragraph_replace_start() {
    let mut model = cm("<p>| says hello</p>");
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
        "<p><a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| says hello</p>",
    );
}

#[test]
fn paragraph_replace_middle() {
    let mut model = cm("<p>Like | said</p>");
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
    "<p>Like <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>| said</p>");
}

#[test]
fn paragraph_replace_end() {
    let mut model = cm("<p>hello |</p>");
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
        "<p>hello <a href=\"https://matrix.to/#/@alice:matrix.org\" contenteditable=\"false\">Alice</a>&nbsp;|</p>",
    );
}
