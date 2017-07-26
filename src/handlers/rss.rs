extern crate rss as rust_rss;
extern crate postgres;

use self::rust_rss::Channel;
use postgres::Connection;
use super::SolamiHandler;

pub fn handle<'a>(p: &SolamiHandler, cmd: &'a str, rest: &'a str, pg: &Connection) {
    let body = match cmd {
        "help" => help(rest),
        _ => {
            println!("Unknown command");
            return;
        },
    };
    if let Ok(b) = body {
        p.send(b.as_str());
    }
}

fn help(option: &str) -> Result<String, ()> {
    let body = match option {
        "add" => vec![
            ":apple: `!rss add`",
            "新しくRSSフィードを登録します。",
            "--使い方--",
            "`!rss add [name] [url]`",
            "*name*: RSSフィード名",
            "*url*: RSSのURL",
            "例) `!rss add hoge http://hoge.net/rss.xml`"
        ],
        "update" => vec![
            ":apple: `!rss update`",
            "RSSフィードのURLを更新します。",
            "--使い方--",
            "`!rss update [name] [url]`",
            "*name*: RSSフィード名",
            "*url*: RSSのURL",
            "例) `!rss update hoge http://hoge.net/rss.xml`"
        ],
        "show" => vec![
            ":apple: `!rss show`",
            "RSSフィードのURLを表示します。",
            "--使い方--",
            "`!rss show [name]`",
            "*name*: RSSフィード名",
            "例) `!rss show hoge`"
        ],
        "list" => vec![
            ":apple: `!rss list`",
            "全ての既存のRSSフィードとURLを表示します。",
            "--使い方--",
            "`!rss list`",
            "例) `!rss list`",
        ],
        "delete" => vec![
            ":apple: `!rss delete`",
            "RSSフィードを削除します。",
            "--使い方--",
            "`!rss delete [name]`",
            "*name*: RSSフィード名",
            "例) `!rss delete hoge`"
        ],
        _ => vec![
            ":green_apple: `!rss` はお気に入りのRSSフィードを購読する機能です。",
            "--使い方--",
            "`!rss [command] [args]`",
            "*command*: コマンド名 (help|add|update|show|list|delete)",
            "*args*: コマンドの引数",
            "--コマンドのヘルプ--",
            "`!rss help` への引数に各コマンドを渡すことで詳細なヘルプを確認できます。",
            "例) `!rss help add`",
        ]
    };
    Ok(body.as_slice().join("\n"))
}
