use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct IconData {
    #[serde(rename = "icon_name")]
    name: String,
    #[serde(rename = "icon_code")]
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IconTypeData {
    #[serde(rename = "type")]
    type_name: String,
    font_family: String,
    title: String,
    filename: String,
    icons: Vec<IconData>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 定义不同类型的图标配置
    let icon_types = vec![
        (
            "MaterialIcons-Regular.codepoints",
            "filled",
            "MaterialIcons-Regular",
            "filled",
            "MaterialIcons-Regular.ttf",
        ),
        (
            "MaterialIconsOutlined-Regular.codepoints",
            "outlined",
            "MaterialIconsOutlined-Regular",
            "outlined",
            "MaterialIconsOutlined-Regular.otf",
        ),
        (
            "MaterialIconsRound-Regular.codepoints",
            "round",
            "MaterialIconsRound-Regular",
            "round",
            "MaterialIconsRound-Regular.otf",
        ),
        (
            "MaterialIconsSharp-Regular.codepoints",
            "sharp",
            "MaterialIconsSharp-Regular",
            "sharp",
            "MaterialIconsSharp-Regular.otf",
        ),
        (
            "MaterialIconsTwoTone-Regular.codepoints",
            "two-tone",
            "MaterialIconsTwoTone-Regular",
            "two-tone",
            "MaterialIconsTwoTone-Regular.otf",
        ),
    ];

    let mut result: Vec<IconTypeData> = Vec::new();

    // 遍历每种类型的图标
    for (codepoints_file, type_name, font_family, title, font_file) in icon_types.iter() {
        let file_path = format!("material-design-icons-font/{}", codepoints_file);

        // 检查文件是否存在
        if !Path::new(&file_path).exists() {
            println!("文件 {} 不存在，跳过", file_path);
            continue;
        }

        // 读取文件内容
        let content = fs::read_to_string(&file_path)?;

        // 解析 codepoints 文件
        let mut icons: Vec<IconData> = Vec::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                icons.push(IconData {
                    name: parts[0].to_string(),
                    icon: parts[1].to_string(),
                });
            }
        }

        // 添加到结果中
        result.push(IconTypeData {
            type_name: type_name.to_string(),
            font_family: font_family.to_string(),
            title: title.to_string(),
            filename: font_file.to_string(),
            icons,
        });
    }

    // 将结果序列化为 JSON
    let json_result = serde_json::to_string(&result)?;

    // 同时写入文件
    fs::write("material_icons.json", json_result)?;
    println!("结果已保存到 material_icons.json 文件中");

    Ok(())
}
