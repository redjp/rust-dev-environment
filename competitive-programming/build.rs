use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 出力ディレクトリを取得
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("problems.rs");

    // problemsディレクトリ内の.rsファイルを検出
    let problems_dir = Path::new("src/problems");
    let problem_files: Vec<_> = fs::read_dir(problems_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                Some(path.file_stem().unwrap().to_str().unwrap().to_string())
            } else {
                None
            }
        })
        .collect();

    // 検出した問題名からコードを生成
    let mut code = String::new();
    code.push_str("use std::collections::HashMap;\n");
    code.push_str("use std::io::BufRead;\n");
    code.push_str("pub type ProblemFunc = fn(&mut dyn BufRead) -> Vec<String>;\n");
    for problem in &problem_files {
        // src/problems/problemN.rs を OUT_DIR にコピー
        let src_path = problems_dir.join(format!("{}.rs", problem));
        let dst_path = Path::new(&out_dir).join(format!("{}.rs", problem));
        fs::copy(&src_path, &dst_path).unwrap();

        // 生成コード
        code.push_str(&format!("pub mod {};\n", problem));
    }
    code.push_str("pub fn get_problem_map() -> HashMap<&'static str, ProblemFunc> {\n");
    code.push_str("    let mut map = HashMap::new();\n");
    for problem in &problem_files {
        code.push_str(&format!("    map.insert(\"{}\", {}::solve as ProblemFunc);\n", problem, problem));
    }
    code.push_str("    map\n");
    code.push_str("}\n");

    // 生成したコードをファイルに書き込む
    fs::write(&dest_path, code).unwrap();

    // problemsディレクトリが変更された場合に再ビルドを指示
    println!("cargo:rerun-if-changed=src/problems");
}