//! Full UTF-16 Message Entities & Rich Text Parser for Telegram Desktop ConversationStage.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityKind {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Code,
    Pre { language: String },
    Blockquote { collapsed: bool },
    Spoiler,
    Url,
    TextUrl { url: String },
    Mention,
    MentionName { user_id: u64 },
    Hashtag,
    BotCommand,
    Cashtag,
    CustomEmoji { document_id: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichEntity {
    pub offset_utf16: usize,
    pub length_utf16: usize,
    pub kind: EntityKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineStyleFlags {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub code: bool,
    pub spoiler: bool,
    pub link_url: Option<String>,
    pub mention: Option<String>,
    pub hashtag: Option<String>,
    pub bot_command: Option<String>,
}

impl Default for InlineStyleFlags {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            strike: false,
            code: false,
            spoiler: false,
            link_url: None,
            mention: None,
            hashtag: None,
            bot_command: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledSegment {
    pub text: String,
    pub flags: InlineStyleFlags,
    pub spoiler_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RichBlock {
    Paragraph(Vec<StyledSegment>),
    PreCode { language: String, code: String },
    Blockquote { lines: Vec<String>, collapsed_default: bool, block_id: usize },
}

pub fn utf16_to_byte_range(text: &str, offset_utf16: usize, length_utf16: usize) -> Option<(usize, usize)> {
    let mut cur_u16 = 0usize;
    let mut start_byte = if offset_utf16 == 0 { Some(0) } else { None };
    let mut end_byte = if offset_utf16 + length_utf16 == 0 { Some(0) } else { None };
    for (byte_idx, ch) in text.char_indices() {
        if start_byte.is_none() && cur_u16 >= offset_utf16 {
            start_byte = Some(byte_idx);
        }
        cur_u16 += ch.len_utf16();
        if end_byte.is_none() && cur_u16 >= offset_utf16 + length_utf16 {
            end_byte = Some(byte_idx + ch.len_utf8());
            break;
        }
    }
    if start_byte.is_none() && cur_u16 >= offset_utf16 {
        start_byte = Some(text.len());
    }
    if end_byte.is_none() {
        end_byte = Some(text.len());
    }
    let s = start_byte?;
    let e = end_byte?.max(s).min(text.len());
    Some((s, e))
}

pub fn byte_to_utf16_offset(text: &str, byte_idx: usize) -> usize {
    text[..byte_idx.min(text.len())].encode_utf16().count()
}

pub fn parse_rich_blocks(text: &str, explicit: &[RichEntity]) -> Vec<RichBlock> {
    if !explicit.is_empty() {
        return build_blocks_from_explicit(text, explicit);
    }
    parse_markdown_and_tokens(text)
}

fn build_blocks_from_explicit(text: &str, explicit: &[RichEntity]) -> Vec<RichBlock> {
    let mut pre_ranges: Vec<(usize, usize, String)> = Vec::new();
    let mut quote_ranges: Vec<(usize, usize, bool)> = Vec::new();
    for ent in explicit {
        if let Some((s, e)) = utf16_to_byte_range(text, ent.offset_utf16, ent.length_utf16) {
            match &ent.kind {
                EntityKind::Pre { language } => pre_ranges.push((s, e, language.clone())),
                EntityKind::Blockquote { collapsed } => quote_ranges.push((s, e, *collapsed)),
                _ => {}
            }
        }
    }
    let mut special: Vec<(usize, usize, bool, String, bool)> = Vec::new();
    for (s, e, lang) in pre_ranges {
        special.push((s, e, true, lang, false));
    }
    for (s, e, collapsed) in quote_ranges {
        special.push((s, e, false, String::new(), collapsed));
    }
    special.sort_by_key(|item| item.0);
    let mut blocks = Vec::new();
    let mut cursor = 0usize;
    let mut quote_id = 0usize;
    let mut spoiler_id = 0usize;
    for (s, e, is_pre, lang, collapsed) in special {
        if s > cursor {
            push_explicit_paragraph_lines(
                text,
                cursor,
                s,
                explicit,
                &mut spoiler_id,
                &mut blocks,
            );
        }
        if is_pre {
            blocks.push(RichBlock::PreCode {
                language: lang,
                code: text[s..e].trim_matches('\n').to_string(),
            });
        } else {
            let lines = text[s..e].lines().map(|l| l.to_string()).collect();
            blocks.push(RichBlock::Blockquote {
                lines,
                collapsed_default: collapsed,
                block_id: quote_id,
            });
            quote_id += 1;
        }
        cursor = e;
    }
    if cursor < text.len() {
        push_explicit_paragraph_lines(
            text,
            cursor,
            text.len(),
            explicit,
            &mut spoiler_id,
            &mut blocks,
        );
    }
    if blocks.is_empty() {
        blocks.push(RichBlock::Paragraph(Vec::new()));
    }
    blocks
}

fn push_explicit_paragraph_lines(
    text: &str,
    range_start: usize,
    range_end: usize,
    explicit: &[RichEntity],
    spoiler_id: &mut usize,
    blocks: &mut Vec<RichBlock>,
) {
    if range_start >= range_end {
        return;
    }
    let slice = &text[range_start..range_end];
    let mut rel = 0usize;
    for raw_line in slice.split('\n') {
        let line_byte_len = raw_line.len();
        let trimmed_line = raw_line.trim_end_matches('\r');
        let abs_start = range_start + rel;
        let abs_end = abs_start + trimmed_line.len();
        if trimmed_line.is_empty() {
            if abs_start < range_end {
                let prev_is_empty =
                    matches!(blocks.last(), Some(RichBlock::Paragraph(s)) if s.is_empty());
                if !blocks.is_empty() && !prev_is_empty {
                    blocks.push(RichBlock::Paragraph(Vec::new()));
                }
            }
        } else {
            blocks.push(RichBlock::Paragraph(slice_inline_with_entities(
                text,
                abs_start,
                abs_end,
                explicit,
                spoiler_id,
            )));
        }
        rel += line_byte_len + 1;
    }
}

fn slice_inline_with_entities(
    text: &str,
    range_start: usize,
    range_end: usize,
    explicit: &[RichEntity],
    spoiler_counter: &mut usize,
) -> Vec<StyledSegment> {
    let slice_end = range_end.min(text.len());
    let slice_start = range_start.min(slice_end);
    if slice_start >= slice_end {
        return Vec::new();
    }
    let mut cuts = vec![slice_start, slice_end];
    let mut mapped: Vec<(usize, usize, &EntityKind, Option<usize>)> = Vec::new();
    for ent in explicit {
        if let Some((s, e)) = utf16_to_byte_range(text, ent.offset_utf16, ent.length_utf16) {
            let cs = s.max(slice_start);
            let ce = e.min(slice_end);
            if cs < ce && !matches!(ent.kind, EntityKind::Pre { .. } | EntityKind::Blockquote { .. }) {
                let sp_id = if matches!(ent.kind, EntityKind::Spoiler) {
                    let id = *spoiler_counter;
                    *spoiler_counter += 1;
                    Some(id)
                } else {
                    None
                };
                cuts.push(cs);
                cuts.push(ce);
                mapped.push((cs, ce, &ent.kind, sp_id));
            }
        }
    }
    cuts.sort_unstable();
    cuts.dedup();
    let mut out = Vec::new();
    for w in cuts.windows(2) {
        let (a, b) = (w[0], w[1]);
        if a >= b {
            continue;
        }
        let chunk = &text[a..b];
        let mut flags = InlineStyleFlags::default();
        let mut sp_id = None;
        for (ms, me, kind, maybe_sp) in &mapped {
            if a >= *ms && b <= *me {
                match kind {
                    EntityKind::Bold => flags.bold = true,
                    EntityKind::Italic => flags.italic = true,
                    EntityKind::Underline => flags.underline = true,
                    EntityKind::Strikethrough => flags.strike = true,
                    EntityKind::Code => flags.code = true,
                    EntityKind::Spoiler => {
                        flags.spoiler = true;
                        sp_id = *maybe_sp;
                    }
                    EntityKind::Url => flags.link_url = Some(text[*ms..*me].to_string()),
                    EntityKind::TextUrl { url } => flags.link_url = Some(url.clone()),
                    EntityKind::Mention | EntityKind::MentionName { .. } => {
                        flags.mention = Some(text[*ms..*me].to_string());
                    }
                    EntityKind::Hashtag => flags.hashtag = Some(text[*ms..*me].to_string()),
                    EntityKind::BotCommand => flags.bot_command = Some(text[*ms..*me].to_string()),
                    _ => {}
                }
            }
        }
        out.push(StyledSegment {
            text: chunk.to_string(),
            flags,
            spoiler_id: sp_id,
        });
    }
    out
}

fn parse_markdown_and_tokens(text: &str) -> Vec<RichBlock> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = text.split('\n').collect();
    let mut i = 0usize;
    let mut quote_id = 0usize;
    let mut spoiler_id = 0usize;

    while i < lines.len() {
        let line = lines[i].trim_end_matches('\r');
        if let Some(rest) = line.strip_prefix("```") {
            let language = rest.trim().to_string();
            let mut code_lines = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].trim_end_matches('\r').starts_with("```") {
                code_lines.push(lines[i].trim_end_matches('\r'));
                i += 1;
            }
            if i < lines.len() {
                i += 1;
            }
            blocks.push(RichBlock::PreCode {
                language,
                code: code_lines.join("\n"),
            });
            continue;
        }
        if line.starts_with("> ") || line == ">" {
            let mut q_lines = Vec::new();
            while i < lines.len() {
                let q = lines[i].trim_end_matches('\r');
                if q.starts_with("> ") || q == ">" {
                    q_lines.push(q.strip_prefix("> ").unwrap_or("").to_string());
                    i += 1;
                } else {
                    break;
                }
            }
            let collapsed_default = q_lines.len() > 3;
            blocks.push(RichBlock::Blockquote {
                lines: q_lines,
                collapsed_default,
                block_id: quote_id,
            });
            quote_id += 1;
            continue;
        }
        if line.trim().is_empty() {
            let prev_is_empty =
                matches!(blocks.last(), Some(RichBlock::Paragraph(s)) if s.is_empty());
            if !blocks.is_empty() && !prev_is_empty {
                blocks.push(RichBlock::Paragraph(Vec::new()));
            }
        } else {
            blocks.push(RichBlock::Paragraph(parse_inline_markdown(
                line,
                &mut spoiler_id,
            )));
        }
        i += 1;
    }
    if blocks.is_empty() {
        blocks.push(RichBlock::Paragraph(Vec::new()));
    }
    blocks
}

fn parse_inline_markdown(text: &str, spoiler_id: &mut usize) -> Vec<StyledSegment> {
    let mut segs = Vec::new();
    let mut pos = 0usize;
    let bytes = text.as_bytes();
    while pos < bytes.len() {
        let rem = &text[pos..];
        let matched = [("**", 0), ("__", 1), ("~~", 2), ("||", 3), ("`", 4)]
            .into_iter()
            .find_map(|(delim, kind)| {
                rem.strip_prefix(delim).and_then(|after| {
                    after.find(delim).filter(|&end| end > 0).map(|end| {
                        (delim.len(), end, kind)
                    })
                })
            });
        if let Some((dlen, inner_len, kind)) = matched {
            let inner = &rem[dlen..dlen + inner_len];
            let mut flags = InlineStyleFlags::default();
            let mut sp = None;
            match kind {
                0 => flags.bold = true,
                1 => flags.italic = true,
                2 => flags.strike = true,
                3 => {
                    flags.spoiler = true;
                    sp = Some(*spoiler_id);
                    *spoiler_id += 1;
                }
                _ => flags.code = true,
            }
            segs.push(StyledSegment { text: inner.to_string(), flags, spoiler_id: sp });
            pos += dlen * 2 + inner_len;
            continue;
        }
        if rem.starts_with("http://") || rem.starts_with("https://") {
            let end = rem.find(char::is_whitespace).unwrap_or(rem.len());
            let clean = rem[..end].trim_end_matches(|c: char| matches!(c, '.' | ',' | '!' | '?' | ')' | ';'));
            let clean_len = clean.len().max(1);
            let mut flags = InlineStyleFlags::default();
            flags.link_url = Some(rem[..clean_len].to_string());
            segs.push(StyledSegment { text: rem[..clean_len].to_string(), flags, spoiler_id: None });
            pos += clean_len;
            continue;
        }
        if (rem.starts_with('@') || rem.starts_with('#') || (rem.starts_with('/') && (pos == 0 || text[..pos].ends_with(char::is_whitespace))))
            && rem.len() > 1
        {
            let prefix = rem.chars().next().unwrap();
            let token_len = 1 + rem[1..].find(|c: char| !c.is_alphanumeric() && c != '_').unwrap_or(rem.len() - 1);
            if token_len > 1 {
                let tok = rem[..token_len].to_string();
                let mut flags = InlineStyleFlags::default();
                match prefix {
                    '@' => flags.mention = Some(tok.clone()),
                    '#' => flags.hashtag = Some(tok.clone()),
                    _ => flags.bot_command = Some(tok.clone()),
                }
                segs.push(StyledSegment { text: tok, flags, spoiler_id: None });
                pos += token_len;
                continue;
            }
        }
        let ch = rem.chars().next().unwrap();
        let next = pos + ch.len_utf8();
        if let Some(last) = segs.last_mut() {
            if last.flags == InlineStyleFlags::default() {
                last.text.push(ch);
                pos = next;
                continue;
            }
        }
        segs.push(StyledSegment {
            text: ch.to_string(),
            flags: InlineStyleFlags::default(),
            spoiler_id: None,
        });
        pos = next;
    }
    segs
}
