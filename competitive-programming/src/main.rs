use std::env;
pub mod utils;
use utils::run_test;

include!(concat!(env!("OUT_DIR"), "/problems.rs"));

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("使い方: cargo run <問題名> <入力ファイルパス> <出力ファイルパス>");
        eprintln!("例: cargo run problem1 inputs/input1.txt outputs/output1.txt");
        return;
    }

    let problem = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];

    // 問題マップを取得
    let problem_map = get_problem_map();

    // 問題名に対応する関数を取得して実行
    if let Some(problem_func) = problem_map.get(problem.as_str()) {
        if let Err(e) = run_test(*problem_func, input_path, output_path) {
            eprintln!("エラー: {}", e);
        }
    } else {
        eprintln!("知らない問題名です: {}", problem);
    }
}