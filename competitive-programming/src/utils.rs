use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub fn run_test<F>(problem_func: F, input_path: &str, output_path: &str) -> io::Result<()>
where
    F: Fn(&mut dyn BufRead) -> Vec<String>,
{
    // 入力ファイルを読み込む
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    // 問題のメイン処理を実行
    let results = problem_func(&mut reader);

    // 期待される出力を読み込む
    let mut output_content = String::new();
    File::open(output_path)?.read_to_string(&mut output_content)?;
    let expected: Vec<String> = output_content.lines().map(|s| s.to_string()).collect();

    // 結果を比較
    if results == expected {
        println!("正解！出力が一致しました。");
    } else {
        println!("不正解。期待される出力と異なります。");
        println!("期待値: {:?}", expected);
        println!("実際: {:?}", results);
    }

    Ok(())
}