use crate::html::parse_html;
use epub::doc::{EpubDoc, NavPoint};
use std::error::Error;
use std::io::{Read, Seek};
use std::path::{self, Path};
/// 传入 epub 路径，并解析
pub fn parse_epub(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    println!("文件路径: {:?}", file_path);

    let mut doc = EpubDoc::new(file_path)?;

    // 打印各种信息
    // println!("获取资源长度: {}", doc.resources.len());
    // println!("获取页数: {:?}", doc.get_num_pages());

    for (id, _) in doc.resources.iter() {
        println!("可用资源 ID: {}", id);
    }
    let _ = read_page();
    println!("\n------");
    println!("获取封面 id: {:?}", doc.get_cover_id());
    println!("\n[INFO] 书籍资源:");
    for (key, (path, mime)) in doc.resources.iter() {
        println!("- 资源ID: {}, 路径: {:?}, MIME类型: {}", key, path, mime);
    }
    // epub 书脊 ID
    println!("\n[INFO]书脊 ID:");
    for id in doc.spine.iter() {
        println!("- {:?}", id);
    }

    println!("\n目录结构:");
    for nav_point in doc.toc.iter() {
        println!(
            "--|\n  + 标签: {:?}, \n  + 内容: {:?}, \n  + 顺序: {:?}",
            nav_point.label, nav_point.content, nav_point.play_order
        );
    }

    println!("\n目录结构和内容预览:");

    // 克隆 toc 以避免借用冲突
    let toc_clone = doc.toc.clone();
    // parse_toc_and_content(&mut doc, &toc_clone, 2)
    let html_content: Result<String, Box<dyn Error>> = read_page();
    // 返回到ui
    let markdown = match html_content {
        Ok(content) => {
            // 将 content 作为 &str 传递给 parse_html
            let markdown = parse_html(&content)?; // 处理 parse_html 返回的 Result
            println!("Markdown:\n{}", markdown);
            markdown // 返回 markdown
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            return Err(e); // 返回错误
        }
    };
    Ok(markdown)

    // content
}

/// 解析目录
fn parse_toc_and_content<R>(
    doc: &mut EpubDoc<R>,
    toc: &Vec<NavPoint>,
    indent: usize,
) -> Result<String, Box<dyn std::error::Error>>
where
    R: Read + Seek,
{
    let mut all_content = String::new();
    for nav_point in toc {
        let indent_str = " ".repeat(indent);
        println!(
            "{}- {} ({:?}), 顺序: {}",
            indent_str, nav_point.label, nav_point.content, nav_point.play_order
        );

        if let Some(content_str) = nav_point.content.to_str() {
            let id = "id_27";
            if let Some((content, _)) = doc.get_resource_str(id) {
                // 预览长度
                let preview_length = std::cmp::min(100, content.len());
                println!("{}内容预览: {}", indent_str, &content[..preview_length]);
                all_content.push_str(&content);
            } else {
                println!("{}警告: 无法读取资源 '{}'", indent_str, content_str);
                all_content.push_str(&format!("{{无法读取的内容: {}}}", content_str));
            }
        } else {
            println!(
                "{}警告: 无法转换路径为字符串: {:?}",
                indent_str, nav_point.content
            );
        }

        if !nav_point.children.is_empty() {
            match parse_toc_and_content(doc, &nav_point.children, indent + 2) {
                Ok(child_content) => all_content.push_str(&child_content),
                Err(e) => println!("{}警告: 解析子目录时出错: {}", indent_str, e),
            }
        }
    }
    Ok(all_content)
}

/// 读取页面
fn read_page() -> Result<String, Box<dyn std::error::Error>> {
    let epub_path = "/Users/simons/Documents/GitHub/epub-editor-slint/epub-test/零售的哲学.epub"; // EPUB 文件路径
    let mut doc = EpubDoc::new(Path::new(epub_path))?;

    // 获取资源 ID
    let resource_id = "id_21"; // 替换为你需要的有效资源 ID

    // 使用 if let 解构 Option
    if let Some((content, _mime)) = doc.get_resource_str(resource_id) {
        println!("文件内容:\n{}", content);
        Ok(content) // 返回内容
    } else {
        println!("无法找到资源 '{}'", resource_id);
        Err(Box::from(format!("无法找到资源 '{}'", resource_id))) // 返回错误
    }
}
