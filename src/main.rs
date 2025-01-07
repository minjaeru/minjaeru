use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let content = "자동으로 추가된 텍스트입니다.\n";
    let file_path = "README.md";

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .expect("readme.md 파일을 열 수 없습니다.");

    file.write_all(content.as_bytes())
        .expect("readme.md 파일에 내용을 추가할 수 없습니다.");
}
