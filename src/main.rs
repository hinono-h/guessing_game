use std::io; //io = 入出力. / io ライブラリは std という標準ライブラリに含まれる

fn main() {
    println!("Fucking Guess the number!"); // 数を当てていただければ幸いです

    println!("Fucking the fucker input your guess."); // 予想を入力してくださると嬉しいです

    let mut guess = String::new(); //let 文を用いて変数を作っている。mut がついている場合「可変」/ String：拡張可能な文字列型(strは拡張不能)

    io::stdin()
        .read_line(&mut guess) //「::」は関連関数/ ある型(上記だとString)に対して実装される関数のこと
        .expect("Failed to read line"); // 行の読み込みに失敗しました

    println!("You guessed: {}", guess); // 次のように予想しました: {}
} //「&」はこの引数が「参照」であることを示す/ コードの複数部が同データにアクセスしても、何度もメモリにコピーしなくて済む。
