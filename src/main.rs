use std::io;

fn main() {
    println!("Fucking Guess the number!"); // 数を当てていただければ幸いです

    println!("Fucking the fucker input your guess."); // 予想を入力してくださると嬉しいです

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // 行の読み込みに失敗しました

    println!("You guessed: {}", guess); // 次のように予想しました: {}
}
