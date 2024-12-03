use html2md;
use pulldown_cmark::{html, Options, Parser};
use reqwest::blocking::get;
use reqwest::Error;
use scraper::{Html, Selector};

// 传入 html 文本并解析
pub fn parse_html(html_content: &str) -> Result<String, Error> {
    // 解析 HTML 内容
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("body").unwrap(); // 选择 <body> 元素

    let mut extracted_content = String::new();

    // 提取 <body> 内容
    for element in document.select(&selector) {
        extracted_content.push_str(&element.inner_html());
    }
    println!("[INFO](html.rs) html content: {}", extracted_content);

    // 转换提取的 HTML 为 Markdown
    let markdown = html_to_markdown(&extracted_content);

    // 返回 Markdown 内容
    Ok(markdown)
}

/// 转换 HTML 为 Markdown
pub fn markdown_to_html(markdown_content: &str) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(markdown_content, options);

    let mut markdown_output = String::new();
    html::push_html(&mut markdown_output, parser);
    println!("[INFO](html.rs) markdown: {}", markdown_output);
    markdown_output
}

/// 将 html 转换为 markdown
pub fn html_to_markdown(html_content: &str) -> String {
    println!("[INFO](html.rs) 开始将 html 转换为 markdown");

    // 输出原始 HTML 的前 100 个字符
    let html_preview: String = html_content.chars().take(100).collect();
    println!("[INFO](html.rs) 原始 html: \n{}", &html_preview);

    // 使用 html2md 库解析 HTML
    let markdown_content = html2md::parse_html(html_content);

    // 输出转换后 Markdown 的前 100 个字符
    let markdown_preview: String = markdown_content.chars().take(100).collect();
    println!("[INFO](html.rs) 转换后的 markdown: \n{}", &markdown_preview);

    markdown_content
}
