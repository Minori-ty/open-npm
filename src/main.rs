use std::env;
use webbrowser;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了足够的参数
    if args.len() < 2 {
        println!("usage: open <package>");
        return;
    }

    // 构建完整的URL
    let url = format!("https://www.npmjs.com/package/{}", args[1]);

    // 尝试在浏览器中打开URL
    if webbrowser::open(&url).is_err() {
        println!("Something error.");
    }
}
