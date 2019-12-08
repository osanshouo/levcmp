use levcmp::levenshtein_distance;

use std::fs;
use std::env;

fn main() {
    // 比較する 2 つのファイル名をコマンドライン引数から取得する.
    let (f1, f2) = {
        let mut args = env::args().skip(1).collect::<Vec<_>>();

        if args.len() != 2 {
            eprintln!("ファイル数が不正です. ちょうど 2 つのファイルを指定してください.");
            return;
        }

        (args.pop().unwrap(), args.pop().unwrap())
    };

    // 2 つのファイルをメモリに読み込む.
    let (f1, f2) = match read_files(&f1, &f2) {
        Ok((f1, f2)) => (f1, f2),
        Err(e) => {
            eprintln!("ファイルが読み込めませんでした: {}", e);
            return;
        }
    };

    // Levenshtein 距離を計算し, 一致度を出力する.
    let dist = levenshtein_distance(&f1, &f2);
    let n = f1.len().max(f2.len());
    println!("{:.3}", 1. - dist as f32/n as f32);
}


fn read_files(f1: &str, f2: &str) -> Result<(Vec<u8>, Vec<u8>), std::io::Error> {
    let f1 = fs::read(f1)?;
    let f2 = fs::read(f2)?;

    Ok((f1, f2))
}
