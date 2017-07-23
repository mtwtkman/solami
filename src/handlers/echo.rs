extern crate slack;
extern crate postgres;

use std::collections::HashMap;
use postgres::Connection;
use super::SolamiHandler;
use db::{Insert, Update, Query, Select};
use db::echo::D;

pub fn handle<'a>(p: &SolamiHandler, cmd: &'a str, rest: &'a str, pg: &Connection) {
    let body = match cmd {
        "help" => help(rest),
        "create" => create(rest, pg),
        "update" => update(rest, pg),
        "show" => show(rest, pg),
        "list" => list(pg),
        // "find" => "find",
        // "delete" => "delete",
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
        "create" => vec![
            ":apple: `!echo create`",
            "新しくパターンを登録します。",
            "--使い方--",
            "`!echo create [name] [pattern] [response]`",
            "*name*: パターン名",
            "*pattern*: パターン本体(正規表現使用可能)",
            "*response*: パターンに対応する返答",
            "例) `!echo create tsukareta 疲れた お疲れ`",
            "例) `!echo create curry ^カレー$ ライス`",
        ],
        "update" => vec![
            ":apple: `!echo update`",
            "既存のパターンを更新します。",
            "--使い方--",
            "`!echo update [name] [pattern] [response]`",
            "*name*: パターン名",
            "*pattern*: パターン本体(正規表現使用可能)",
            "*response*: パターンに対応する返答",
            "例) `!echo create tsukareta 疲れた お疲れ`",
            "例) `!echo create curry ^カレー$ ライス`",
        ],
        "show" => vec![
            ":apple: `!echo show`",
            "既存のパターン本体と返答の内容を表示します。",
            "--使い方--",
            "`!echo show [name]`",
            "*name*: パターン名",
            "例) `!echo show tsukareta`",
        ],
        "list" => vec![
            ":apple: `!echo list`",
            "全ての既存のパターン本体と返答の内容を表示します。",
            "--使い方--",
            "`!echo list`",
            "例) `!echo list`",
        ],
        // "find" => vec![
        //     ":apple: `!echo find`",
        //     "発言にマッチするパターンを見つけます。",
        //     "--使い方--",
        //     "`!echo find [text]`",
        //     "*text*: 検索キーとなる発言",
        //     "例) `!echo find 疲れた",
        // ],
        "delete" => vec![
            ":apple: `!echo delete`",
            "既存のパターンを削除します。一度削除したパターンは戻せません。",
            "--使い方--",
            "`!echo delete [name]`",
            "*name*: パターン名",
            "例) `!echo delete tsukareta`",
        ],
        _ => vec![
            ":green_apple: `!echo` はパターンにマッチする発言に対応した返答をする機能です。",
            "--使い方--",
            "`!echo [command] [args]`",
            "*command*: コマンド名 (help|create|update|show|list|delete)",
            "*args*: コマンドの引数",
            "--コマンドのヘルプ--",
            "`!echo help` への引数に各コマンドを渡すことで詳細なヘルプを確認できます。",
            "例) `!echo help create`",
        ]
    };
    Ok(body.as_slice().join("\n"))
}

fn create(rest: &str, pg: &Connection) -> Result<String, ()> {
    let mut splited = rest.split_whitespace();
    let mut obj: D = Default::default();

    if let Some(name) = splited.next() {
        obj.name = name.to_owned();
    } else {
        return Err(());
    }

    if let Some(pattern) = splited.next() {
        obj.pattern = pattern.to_owned();
    } else {
        return Err(());
    }

    if let Some(response) = splited.next() {
        obj.response = response.to_owned();
    } else {
        return Err(());
    }

    match obj.insert(pg) {
        Ok(_) => {
            println!("[echo] created.");
            Ok("登録しました".to_owned())
        },
        Err(e) => {
            println!("[echo] failed to create. ERROR: {}", e);
            Err(())
        }
    }
}

fn update(rest: &str, pg: &Connection) -> Result<String, ()> {
    let mut splited = rest.split_whitespace();
    let mut obj: D = Default::default();

    if let Some(name) = splited.next() {
        obj.name = name.to_owned();
    } else {
        return Err(());
    }

    if let Some(pattern) = splited.next() {
        obj.pattern = pattern.to_owned();
    } else {
        return Err(());
    }

    if let Some(response) = splited.next() {
        obj.response = response.to_owned();
    } else {
        return Err(());
    }

    match obj.update(pg) {
        Ok(_) => {
            println!("[echo] updated.");
            Ok("更新しました".to_owned())
        },
        Err(e) => {
            println!("[echo] failed to update. ERROR: {}", e);
            Err(())
        }
    }
}

fn show(rest: &str, pg: &Connection) -> Result<String, ()> {
    let mut splited = rest.split_whitespace();
    let mut query: Query = HashMap::new();
    let obj: D = Default::default();

    if let Some(name) = splited.next() {
        query.insert("name", name);
    } else {
        return Err(());
    }

    match obj.select(pg, Some(query)) {
        Ok(rows) => {
            let mut response: String = String::new();
            if rows.is_empty() {
                response.push_str("見つかりませんでした");
            } else {
                let selected = rows.get(0);
                response = [
                    format!("名称: {}", selected.get::<&str, String>("name")).as_str(),
                    format!("パターン: {}", selected.get::<&str, String>("pattern")).as_str(),
                    format!("返答: {}", selected.get::<&str, String>("response")).as_str(),
                ].join("\n");
            }
            println!("[echo] selected.");
            Ok(response)
        },
        Err(e) => {
            println!("[echo] failed to select. ERROR: {}", e);
            Err(())
        }
    }
}

fn list(pg: &Connection) -> Result<String, ()> {
    let obj: D = Default::default();
    match obj.select(pg, None) {
        Ok(rows) => {
            let mut response = vec!["名称: パターン -> 返答".to_owned()];
            for row in rows.iter() {
                response.push(
                    format!(
                        "{}: {} -> {}",
                        row.get::<&str, String>("name").as_str(),
                        row.get::<&str, String>("pattern").as_str(),
                        row.get::<&str, String>("response").as_str(),
                    ),
                )
            }
            println!("[echo] listed");
            Ok(response.iter().map(|x| x.as_str()).collect::<Vec<&str>>().as_slice().join("\n"))
        },
        Err(e) => {
            println!("[echo] failed to list. ERROR: {}", e);
            Err(())
        }
    }
}
