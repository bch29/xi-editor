#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate serde_macros;

use std::io;
use std::collections::BTreeMap;
use std::io::Write;
use std::error;
use std::fmt;
use serde_json::builder::ObjectBuilder;
use serde_json::Value;

// =============================================================================
//  Request handling
// =============================================================================

pub fn send(v: &Value) -> Result<(), io::Error> {
    let mut s = serde_json::to_string(v).unwrap();
    s.push('\n');
    //print_err!("from core: {}", s);
    io::stdout().write_all(s.as_bytes())
}

pub fn respond(result: &Value, id: Option<Value>)
{
    if let Some(id) = id {
        if let Err(e) = send(&ObjectBuilder::new()
                             .insert("id", id)
                             .insert("result", result)
                             .unwrap()) {
            // print_err!("error {} sending response to RPC {:?}", e, id);
        }
    } else {
        // print_err!("tried to respond with no id");
    }
}

// =============================================================================
//  Command types
// =============================================================================

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename="request")]
pub enum Request {
    #[serde(rename="tab_command")]
    TabCommand {
        #[serde(default="None")]
        id: Option<Value>,

        tab_command: TabCommand
    }
}

/// An enum representing a tab command, parsed from JSON.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename="tab_command")]
pub enum TabCommand {
    #[serde(rename="edit")]
    Edit { tab_name: String, edit_command: EditCommand },
    #[serde(rename="new_tab")]
    NewTab,
    #[serde(rename="delete_tab")]
    DeleteTab { tab_name: String },
}

/// An enum representing an edit command, parsed from JSON.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename="edit_command")]
pub enum EditCommand {
    #[serde(rename="render_lines")]
    RenderLines { first_line: usize, last_line: usize },
    #[serde(rename="key")]
    Key { chars: String, flags: u64 },
    #[serde(rename="insert")]
    Insert { chars: String },
    #[serde(rename="insert_newline")]
    InsertNewline,
    #[serde(rename="delete")]
    Delete { motion: EditMotion },
    #[serde(rename="move")]
    Move { motion: EditMotion, modify_selection: bool },
    #[serde(rename="scroll_page_up")]
    ScrollPageUp,
    #[serde(rename="page_up_and_modify_selection")]
    PageUpAndModifySelection,
    #[serde(rename="scroll_page_down")]
    ScrollPageDown,
    #[serde(rename="page_down_and_modify_selection")]
    PageDownAndModifySelection,
    #[serde(rename="open")]
    Open { file_path: String },
    #[serde(rename="save")]
    Save { file_path: String },
    #[serde(rename="scroll")]
    Scroll { first: i64, last: i64 },
    #[serde(rename="yank")]
    Yank,
    #[serde(rename="transpose")]
    Transpose,
    #[serde(rename="click")]
    Click { line: u64, column: u64, flags: u64, click_count: u64 },
    #[serde(rename="drag")]
    Drag { line: u64, column: u64, flags: u64 },
    #[serde(rename="undo")]
    Undo,
    #[serde(rename="redo")]
    Redo,
    #[serde(rename="cut")]
    Cut,
    #[serde(rename="copy")]
    Copy,
    #[serde(rename="debug_rewrap")]
    DebugRewrap,
    #[serde(rename="debug_test_fg_spans")]
    DebugTestFgSpans,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename="edit_motion")]
pub enum EditMotion {
    #[serde(rename="prev_char")]
    PrevChar,
    #[serde(rename="next_char")]
    NextChar,
    #[serde(rename="prev_line")]
    PrevLine,
    #[serde(rename="next_line")]
    NextLine,
    #[serde(rename="start_of_line")]
    StartOfLine,
    #[serde(rename="start_of_document")]
    StartOfDocument,
    #[serde(rename="end_of_line")]
    EndOfLine,
    #[serde(rename="end_of_document")]
    EndOfDocument,

    // TODO: Also implement these motions:
    // PrevWordStart,
    // NextWordStart,
    // PrevWordEnd,
    // NextWordEnd,
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
