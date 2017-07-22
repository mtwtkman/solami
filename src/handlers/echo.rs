extern crate slack;
extern crate postgres;

use postgres::Connection;
use super::SolamiHandler;

pub fn handle<'a>(p: &SolamiHandler, cmd: &'a str, rest: &'a str) {
    let body = match cmd {
        "help" => help(rest),
        // "create" => "create",
        // "update" => "update",
        // "show" => "show",
        // "list" => "list",
        // "find" => "find",
        // "delete" => "delete",
        _ => {
            println!("Unknown cmd");
            return;
        },
    };
    p.send(body.as_str());
}

fn help(option: &str) -> String {
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
        "find" => vec![
            ":apple: `!echo find`",
            "発言にマッチするパターンを見つけます。",
            "--使い方--",
            "`!echo find [text]`",
            "*text*: 検索キーとなる発言",
            "例) `!echo find 疲れた",
        ],
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
            "*command*: コマンド名 (help|create|update|show|list|find|delete)",
            "*args*: コマンドの引数",
            "--コマンドのヘルプ--",
            "`!echo help` への引数に各コマンドを渡すことで詳細なヘルプを確認できます。",
            "例) `!echo help create`",
        ]
    };
    body.as_slice().join("\n")
}
