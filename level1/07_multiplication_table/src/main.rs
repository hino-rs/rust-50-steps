use std::io::{self, Write};

fn main() {
    pattern1();
    pattern2();
    pattern3();
}

// パターン1
// 可読性が高く、他言語での書き方に近い。
// Stringのメモリ確保と開放が頻出するため、メモリ効率が悪い。
// ちなみに、StringやVecは、キャパを指定できる。しない場合、容量の拡張が起こるため悪効率。
fn pattern1() {
    // 差が分かりやすいように、それぞれ時間計測をする
    let now = std::time::Instant::now();

    let mut line = String::new();

    for n in 1..=9 {
        for m in 1..=9 {
            // Stringに&strをプッシュする
            // format!で文字列を作れる
            line.push_str(&format!("{:2} ", n*m))
        }
        println!("{line}");
        // lineを空にする
        line.clear();
    }

    println!("🦀⌛パターン1: {:?}", now.elapsed());
}

// パターン2
// そもそも値を確保する必要が無いなら値を作る必要はない。
// 標準出力に直接書き込むことで、メモリ効率が最高に良い。
fn pattern2() {
    let now = std::time::Instant::now();

    // 標準出力の取得とロックを明示的に行う
    // 標準出力先はこの場合一つしかないので、プログラム上で競合を防ぐ必要があるから。
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for n in 1..=9 {
        for m in 1..=9 {
            // 書き込み
            write!(handle, "{:2} ", n * m).unwrap();
        }
        // 出力 (開放)
        writeln!(handle).unwrap();
    }

    println!("🦀⌛パターン2: {:?}", now.elapsed());
}

// パターン3
// 九九を値として保持したい場合はイテレータで作る
fn pattern3() {
    let now = std::time::Instant::now();

    // キャパを指定して、追加処理が発生しないようにする
    let mut table = String::with_capacity(252);

    // 1~9のイテレータを回す
    let t: String = (1..=9)
        // flat_mapは要素(n: 1~9)を受け取ってイテレータを返す
        // 遅延評価なので下のcollectが呼ばれるまで止まっている。
        .flat_map(|n| {
            (1..=9)
                // n本体を使う必要があるので、moveを付け所有権(本体)を渡す。
                // 遅延評価なので、外側のnがどうなってるか、参照だと分からなくなってしまうから。
                .map(move |m| format!("{:2} ", n * m))
                .chain(std::iter::once("\n".to_string()))
        })
        // 
        .collect();

    table.push_str(&t);

    print!("{table}");
    println!("🦀⌛パターン3: {:?}", now.elapsed());
}