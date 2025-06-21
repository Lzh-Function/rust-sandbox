// ユーザー入力を受け付けるためのioライブラリをスコープに取り込む
// 標準ライブラリstdに含まれる
// デフォルトで取り込まれているライブラリ群（prelude）でない場合にuseで明示
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // guessという変数を定義
    // Rustでは変数はデフォルトでimmutable(不変)なので、mutable(可変)にするためにmutをつける
    // String::new()という関数を呼び出してString型の新しいインスタンスを得る
    let mut guess = String::new();

    // ioのstdin関数を呼び出して標準入力ハンドル型(std::io::Stdin)を得る
    io::stdin()
        // Stdin型のread_lineメソッド呼び出し・入力を得る
        // ユーザー入力を可変変数guessに格納
        // &は引数が参照であることを示すが、参照もデフォルトはimmutableなので、mutを挟む必要がある
        .read_line(&mut guess)
        // read_lineの返り値(Result型)に応じた分岐
        // 列挙子Ok/ErrのうちErrの場合にmsgを表示
        // .expectでResultを利用しない場合、コンパイルは成功するがwarningが出力される
        .expect("Failed to read line...");

    // プレースホルダーでguessを表示
    println!("You guessed: {}", guess);
}
