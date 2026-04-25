use regex::Regex;
use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use super::super::input;

pub fn handle(
    keyword: &Option<String>,
    mode: &Option<String>,
    case_sensitive: bool,
    storage: &DataBaseStorage,
    output: &Output,
) {
    let keyword = match keyword {
        Some(k) if !k.trim().is_empty() => k.clone(),
        _ => match input::prompt_text("请输入搜索关键词") {
            Some(k) => k,
            None => { output.error("已取消"); return; }
        }
    };

    let mode = mode.as_deref().unwrap_or("plain");

    let notes: Vec<_> = storage.list_notes().into_iter().filter(|n| {
        let mut read_content = String::new();
        if let Some(body) = storage.get_note(n.id) {
            read_content = body.content;
        }
        let tag_names: String = n.tags.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(" ");
        let haystack = format!("{} {} {} {}", n.title, read_content, n.category.name, tag_names);

        match mode {
            "regex" => search_regex(&haystack, &keyword, case_sensitive, output),
            "fuzzy" => search_fuzzy(&haystack, &keyword, case_sensitive),
            _ => search_plain(&haystack, &keyword, case_sensitive),
        }
    }).collect();

    if notes.is_empty() {
        output.empty(format!("未找到匹配 '{}' 的笔记", keyword));
        return;
    }

    let mut table = output.create_table();
    output.set_headers(&mut table, &["ID", "标题", "分类", "匹配", "修改时间"]);

    for n in &notes {
        let content = storage.get_note(n.id).map(|note| note.content).unwrap_or_default();
        let context = highlight_match(&content, &keyword, mode, case_sensitive);

        table.add_row(vec![
            output.cell_id(n.id),
            output.cell_title(&n.title),
            output.cell_category(&n.category.name),
            comfy_table::Cell::new(context),
            output.cell_date(&n.modified.format("%Y-%m-%d %H:%M").to_string()),
        ]);
    }

    output.print_table(&table);
    output.line(format!("共找到 {} 条匹配", notes.len()));
}

fn search_plain(haystack: &str, needle: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        haystack.contains(needle)
    } else {
        haystack.to_lowercase().contains(&needle.to_lowercase())
    }
}

fn search_regex(haystack: &str, pattern: &str, case_sensitive: bool, output: &Output) -> bool {
    let re = if case_sensitive {
        Regex::new(pattern)
    } else {
        Regex::new(&format!("(?i){}", pattern))
    };
    match re {
        Ok(re) => re.is_match(haystack),
        Err(_) => {
            output.warn("无效的正则表达式，回退到普通搜索");
            search_plain(haystack, pattern, case_sensitive)
        }
    }
}

fn search_fuzzy(haystack: &str, pattern: &str, case_sensitive: bool) -> bool {
    let (hay, pat) = if case_sensitive {
        (haystack.to_string(), pattern.to_string())
    } else {
        (haystack.to_lowercase(), pattern.to_lowercase())
    };

    let mut pat_chars = pat.chars().peekable();
    for c in hay.chars() {
        if pat_chars.peek() == Some(&c) {
            pat_chars.next();
        }
    }
    pat_chars.peek().is_none()
}

fn highlight_match(content: &str, keyword: &str, mode: &str, case_sensitive: bool) -> String {
    let max_chars = 40;
    let text = content.lines().next().unwrap_or("");
    if text.is_empty() {
        return "-".to_string();
    }

    let lower_text = if case_sensitive { text.to_lowercase() } else { text.to_lowercase() };
    let lower_kw = if case_sensitive { keyword.to_lowercase() } else { keyword.to_lowercase() };

    let char_pos = match mode {
        "regex" => {
            let re = Regex::new(&format!("(?i){}", keyword)).ok();
            re.and_then(|r| r.find(lower_text.as_str()))
                .map(|m| lower_text[..m.start()].chars().count())
        }
        "fuzzy" => lower_text.find(lower_kw.chars().next().unwrap_or('\0'))
            .map(|p| lower_text[..p].chars().count()),
        _ => lower_text.find(&lower_kw)
            .map(|p| lower_text[..p].chars().count()),
    };

    let snippet = match char_pos {
        Some(p) if p > 15 => {
            let start = p.saturating_sub(5);
            let end = (p + lower_kw.chars().count() + 20).min(text.chars().count());
            format!("…{}…", text.chars().skip(start).take(end - start).collect::<String>())
        }
        Some(_) => {
            if text.chars().count() > max_chars {
                format!("{}…", text.chars().take(max_chars).collect::<String>())
            } else {
                text.to_string()
            }
        }
        None => {
            if text.chars().count() > max_chars {
                format!("{}…", text.chars().take(max_chars).collect::<String>())
            } else {
                text.to_string()
            }
        }
    };

    snippet
}
