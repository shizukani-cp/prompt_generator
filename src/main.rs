use arboard::Clipboard;
use regex::Regex;
use std::path::Path;
use std::{fs, io, process};

struct Source {
    filename: String,
    body: String,
}

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let mut sources: Vec<Source> = vec![];

    let re_command = Regex::new(r"^/\.+$").unwrap();
    let re_rm = Regex::new(r"^/rm\s+(?P<filename>.+)$").unwrap();
    let re_add = Regex::new(r"^/add\s+(?P<filename>.+)$").unwrap();

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).ok();
        command = command.trim().to_string();

        match command.as_str() {
            "/help" => {
                println!(
                    "コマンド一覧:\n\
                    \t/add <ファイル名>\tワークスペースにファイルを追加します\n\
                    \t/rm <ファイル名>\tワークスペースからファイルを削除します\n\
                    \t/help\tヘルプを表示します。\n\
                    \t/list\tワークスペースのファイル一覧を表示します。\n\
                    \t/exit\tアプリを終了します。\
                    "
                );
            }
            "/list" => {
                for s in &sources {
                    println!("{}", s.filename);
                }
            }
            "/exit" => {
                println!("Thank you for using!");
                process::exit(0);
            }
            _ => {
                if let Some(caps) = re_rm.captures(&command) {
                    let filename = caps.name("filename").unwrap().as_str();
                    sources.retain(|s| s.filename != filename);
                    println!("ファイルを削除しました: {}", filename);
                } else if let Some(caps) = re_add.captures(&command) {
                    let filename = caps.name("filename").unwrap().as_str();
                    if Path::new(filename).exists() {
                        match fs::read_to_string(filename) {
                            Ok(contents) => {
                                sources.push(Source {
                                    filename: filename.to_string(),
                                    body: contents,
                                });
                                println!("ファイルをワークスペースに追加しました: {}", filename);
                            }
                            Err(e) => {
                                eprintln!("ファイルの読み込みに失敗しました: {}", e);
                            }
                        }
                    } else {
                        eprintln!("ファイルが存在しません: {}", filename);
                    }
                } else if re_command.is_match(command.as_str()) {
                    println!("無効なコマンドです。/helpでヘルプを表示してください。");
                } else {
                    let mut codes = "".to_string();
                    for s in &sources {
                        codes.push_str(&format!(
                            "```{}\n\
                             {}\n```",
                            s.filename, s.body
                        ));
                    }
                    let prompt = format!(
                        "{c}\n\
                        # コード\n\
                        {cs}",
                        c = command,
                        cs = codes
                    );
                    let _ = clipboard.set_text(prompt).unwrap();
                    println!("Prompt Coppied!");
                }
            }
        }
    }
}
