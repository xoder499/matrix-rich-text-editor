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
    ComposerModel, ComposerUpdate, DomNode, Location, SuggestionPattern,
    UnicodeString,
};

impl<S> ComposerModel<S>
where
    S: UnicodeString,
{
    pub fn set_mention_from_suggestion(
        &mut self,
        url: S,
        text: S,
        suggestion: SuggestionPattern,
        attributes: Vec<(S, S)>,
    ) -> ComposerUpdate<S> {
        // This function removes the text between the suggestion start and end points, updates the cursor position
        // and then calls set_mention_with_text (for equivalence with stages for inserting a link)

        self.do_replace_text_in(S::default(), suggestion.start, suggestion.end);
        self.state.start = Location::from(suggestion.start);
        self.state.end = self.state.start;

        self.set_mention_with_text(url, text, attributes)
    }

    fn set_mention_with_text(
        &mut self,
        url: S,
        text: S,
        attributes: Vec<(S, S)>,
    ) -> ComposerUpdate<S> {
        // this function is similar to set_link_with_text, but now we call a new simpler insertion method
        self.push_state_to_history();
        self.insert_node_at_cursor(DomNode::new_mention(url, text, attributes))
    }

    /// Inserts the given node at the current cursor position
    fn insert_node_at_cursor(&mut self, node: DomNode<S>) -> ComposerUpdate<S> {
        let (s, e) = self.safe_selection();
        let range = self.state.dom.find_range(s, e);

        // manually determine where the cursor will be after we finish the amendment
        let new_cursor_position = s + node.text_len();

        let mut should_add_trailing_space = false;

        // do nothing if we don't have a cursor
        // TODO determine behaviour if we try to insert when we have a selection
        if range.is_selection() {
            ComposerUpdate::<S>::keep();
        }

        // manipulate the state of the dom as required
        if range.is_empty() {
            // this happens if we replace the whole of a single text node in the composer
            self.state.dom.append_at_end_of_document(node);
            should_add_trailing_space = true;
        } else if let Some(leaf) = range.leaves().next() {
            // when we have a leaf, the way we treat the insertion depends on the cursor position inside that leaf
            let cursor_at_end = leaf.start_offset == leaf.length;
            let cursor_at_start = leaf.start_offset == 0;

            if cursor_at_start {
                // insert the new node before a leaf that contains a cursor at the start
                self.state.dom.insert_at(&leaf.node_handle, node);
            } else if cursor_at_end {
                // insert the new node after a leaf that contains a cursor at the end
                let parent_node = self.state.dom.parent(&leaf.node_handle);
                self.state.dom.append(&parent_node.handle(), node);
                should_add_trailing_space = true;
            } else {
                // otherwise insert the new node in the middle of a text node
                self.state.dom.insert_into_text(
                    &leaf.node_handle,
                    leaf.start_offset,
                    node,
                );
            }
        } else {
            // TODO this is the case where we have some locations, but none of them are leaves
            // ie we are in a container node
            panic!("TODO implement inserting mention in container node")
        }

        // after manipulation, update cursor position, add a trailing space if desired, return
        self.state.start = Location::from(new_cursor_position);
        self.state.end = Location::from(new_cursor_position);

        if should_add_trailing_space {
            self.do_replace_text(" ".into());
        }

        self.create_update_replace_all()
    }
}
