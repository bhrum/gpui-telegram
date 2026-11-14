//! Keyboard Editing, Selection, Undo/Redo, Formatting Wrap & Trigger Detection (`editor_state`).

use std::path::PathBuf;
use super::ime_machine::ImeSafetyMachine;

#[derive(Debug, Clone)]
pub struct ComposerEditorState {
    pub selection_anchor: Option<usize>,
    pub undo_stack: Vec<(String, usize)>,
    pub redo_stack: Vec<(String, usize)>,
    pub attachments: Vec<PathBuf>,
    pub send_options_open: bool,
    pub send_silent: bool,
    pub scheduled_at_secs: Option<i64>,
    pub selector_search_focused: bool,
}

impl Default for ComposerEditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl ComposerEditorState {
    pub fn new() -> Self {
        Self {
            selection_anchor: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            attachments: Vec::new(),
            send_options_open: false,
            send_silent: false,
            scheduled_at_secs: None,
            selector_search_focused: false,
        }
    }

    pub fn push_undo(&mut self, ime: &ImeSafetyMachine) {
        let snap = (ime.text_buffer.clone(), ime.cursor_index);
        if self.undo_stack.last() != Some(&snap) {
            self.undo_stack.push(snap);
            if self.undo_stack.len() > 100 {
                self.undo_stack.remove(0);
            }
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self, ime: &mut ImeSafetyMachine) -> bool {
        if let Some((buf, cur)) = self.undo_stack.pop() {
            self.redo_stack.push((ime.text_buffer.clone(), ime.cursor_index));
            ime.text_buffer = buf;
            ime.cursor_index = clamp_char_boundary(&ime.text_buffer, cur);
            self.selection_anchor = None;
            return true;
        }
        false
    }

    pub fn redo(&mut self, ime: &mut ImeSafetyMachine) -> bool {
        if let Some((buf, cur)) = self.redo_stack.pop() {
            self.undo_stack.push((ime.text_buffer.clone(), ime.cursor_index));
            ime.text_buffer = buf;
            ime.cursor_index = clamp_char_boundary(&ime.text_buffer, cur);
            self.selection_anchor = None;
            return true;
        }
        false
    }

    pub fn selected_range(&self, cursor: usize) -> Option<(usize, usize)> {
        self.selection_anchor.and_then(|anc| {
            if anc == cursor {
                None
            } else {
                Some((anc.min(cursor), anc.max(cursor)))
            }
        })
    }

    pub fn selected_text<'a>(&self, ime: &'a ImeSafetyMachine) -> Option<&'a str> {
        let cur = clamp_char_boundary(&ime.text_buffer, ime.cursor_index);
        self.selected_range(cur).and_then(|(s, e)| ime.text_buffer.get(s..e))
    }

    pub fn select_all(&mut self, ime: &mut ImeSafetyMachine) {
        if ime.text_buffer.is_empty() {
            self.selection_anchor = None;
            ime.cursor_index = 0;
        } else {
            self.selection_anchor = Some(0);
            ime.cursor_index = ime.text_buffer.len();
        }
    }

    pub fn move_left(&mut self, ime: &mut ImeSafetyMachine, extend_selection: bool, by_word: bool) {
        let cur = clamp_char_boundary(&ime.text_buffer, ime.cursor_index);
        if extend_selection && self.selection_anchor.is_none() {
            self.selection_anchor = Some(cur);
        } else if !extend_selection {
            if let Some((s, _)) = self.selected_range(cur) {
                ime.cursor_index = s;
                self.selection_anchor = None;
                return;
            }
            self.selection_anchor = None;
        }
        ime.cursor_index = if by_word {
            prev_word_boundary(&ime.text_buffer, cur)
        } else {
            prev_char_boundary(&ime.text_buffer, cur)
        };
    }

    pub fn move_right(&mut self, ime: &mut ImeSafetyMachine, extend_selection: bool, by_word: bool) {
        let cur = clamp_char_boundary(&ime.text_buffer, ime.cursor_index);
        if extend_selection && self.selection_anchor.is_none() {
            self.selection_anchor = Some(cur);
        } else if !extend_selection {
            if let Some((_, e)) = self.selected_range(cur) {
                ime.cursor_index = e;
                self.selection_anchor = None;
                return;
            }
            self.selection_anchor = None;
        }
        ime.cursor_index = if by_word {
            next_word_boundary(&ime.text_buffer, cur)
        } else {
            next_char_boundary(&ime.text_buffer, cur)
        };
    }

    pub fn move_home(&mut self, ime: &mut ImeSafetyMachine, extend_selection: bool) {
        let cur = clamp_char_boundary(&ime.text_buffer, ime.cursor_index);
        if extend_selection && self.selection_anchor.is_none() {
            self.selection_anchor = Some(cur);
        } else if !extend_selection {
            self.selection_anchor = None;
        }
        ime.cursor_index = 0;
    }

    pub fn move_end(&mut self, ime: &mut ImeSafetyMachine, extend_selection: bool) {
        let cur = clamp_char_boundary(&ime.text_buffer, ime.cursor_index);
        if extend_selection && self.selection_anchor.is_none() {
            self.selection_anchor = Some(cur);
        } else if !extend_selection {
            self.selection_anchor = None;
        }
        ime.cursor_index = ime.text_buffer.len();
    }

    pub fn insert_at_cursor(&mut self, ime: &mut ImeSafetyMachine, text: &str) {
        self.push_undo(ime);
        let cur = clamp_char_boundary(&ime.text_buffer, ime.cursor_index);
        if let Some((s, e)) = self.selected_range(cur) {
            ime.text_buffer.replace_range(s..e, text);
            ime.cursor_index = s + text.len();
            self.selection_anchor = None;
        } else {
            ime.text_buffer.insert_str(cur, text);
            ime.cursor_index = cur + text.len();
        }
        ime.reset_commit_fence();
    }

    pub fn backspace_or_delete_sel(&mut self, ime: &mut ImeSafetyMachine) {
        self.push_undo(ime);
        let cur = clamp_char_boundary(&ime.text_buffer, ime.cursor_index);
        if let Some((s, e)) = self.selected_range(cur) {
            ime.text_buffer.replace_range(s..e, "");
            ime.cursor_index = s;
            self.selection_anchor = None;
        } else if cur > 0 {
            let prev = prev_char_boundary(&ime.text_buffer, cur);
            ime.text_buffer.replace_range(prev..cur, "");
            ime.cursor_index = prev;
        }
    }

    pub fn delete_forward(&mut self, ime: &mut ImeSafetyMachine) {
        self.push_undo(ime);
        let cur = clamp_char_boundary(&ime.text_buffer, ime.cursor_index);
        if let Some((s, e)) = self.selected_range(cur) {
            ime.text_buffer.replace_range(s..e, "");
            ime.cursor_index = s;
            self.selection_anchor = None;
        } else if cur < ime.text_buffer.len() {
            let next = next_char_boundary(&ime.text_buffer, cur);
            ime.text_buffer.replace_range(cur..next, "");
        }
    }

    pub fn wrap_selection(&mut self, ime: &mut ImeSafetyMachine, prefix: &str, suffix: &str) {
        self.push_undo(ime);
        let cur = clamp_char_boundary(&ime.text_buffer, ime.cursor_index);
        if let Some((s, e)) = self.selected_range(cur) {
            let inner = ime.text_buffer[s..e].to_string();
            let wrapped = format!("{}{}{}", prefix, inner, suffix);
            ime.text_buffer.replace_range(s..e, &wrapped);
            self.selection_anchor = Some(s + prefix.len());
            ime.cursor_index = s + prefix.len() + inner.len();
        } else {
            let marker = format!("{}{}", prefix, suffix);
            ime.text_buffer.insert_str(cur, &marker);
            ime.cursor_index = cur + prefix.len();
        }
    }
}

pub fn clamp_char_boundary(s: &str, mut idx: usize) -> usize {
    idx = idx.min(s.len());
    while idx > 0 && !s.is_char_boundary(idx) {
        idx -= 1;
    }
    idx
}

pub fn prev_char_boundary(s: &str, idx: usize) -> usize {
    if idx == 0 {
        return 0;
    }
    let mut p = (idx - 1).min(s.len());
    while p > 0 && !s.is_char_boundary(p) {
        p -= 1;
    }
    p
}

pub fn next_char_boundary(s: &str, idx: usize) -> usize {
    if idx >= s.len() {
        return s.len();
    }
    let mut n = idx + 1;
    while n < s.len() && !s.is_char_boundary(n) {
        n += 1;
    }
    n
}

fn prev_word_boundary(s: &str, idx: usize) -> usize {
    let mut i = prev_char_boundary(s, idx);
    while i > 0 && s[..i].chars().next_back().is_some_and(|c| c.is_whitespace()) {
        i = prev_char_boundary(s, i);
    }
    while i > 0 && s[..i].chars().next_back().is_some_and(|c| !c.is_whitespace()) {
        i = prev_char_boundary(s, i);
    }
    i
}

fn next_word_boundary(s: &str, idx: usize) -> usize {
    let mut i = next_char_boundary(s, idx);
    while i < s.len() && s[i..].chars().next().is_some_and(|c| c.is_whitespace()) {
        i = next_char_boundary(s, i);
    }
    while i < s.len() && s[i..].chars().next().is_some_and(|c| !c.is_whitespace()) {
        i = next_char_boundary(s, i);
    }
    i
}

/// Detects active `@mention`, `/command`, `#hashtag`, or `:emoji:` token immediately before cursor.
pub fn detect_autocomplete_trigger(text: &str, cursor: usize) -> Option<(char, String, usize)> {
    let cur = clamp_char_boundary(text, cursor);
    let prefix = &text[..cur];
    let token_start = prefix
        .char_indices()
        .rev()
        .find(|&(_, ch)| ch.is_whitespace())
        .map(|(i, ch)| i + ch.len_utf8())
        .unwrap_or(0);
    let token = &prefix[token_start..];
    let first = token.chars().next()?;
    if matches!(first, '@' | '/' | '#' | ':') && !token[first.len_utf8()..].contains(':') {
        Some((first, token.to_string(), token_start))
    } else {
        None
    }
}
