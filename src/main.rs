use std::thread;
use std::time::Duration;
use termion::terminal_size;

const ROCKET_HEIGHT: usize = 21;

fn main() {
    // ロケットのパーツ
    let rocket = [
        "          ^           ",
        "         / \\         ",
        "        / _ \\        ",
        "       |     |        ",
        "       |     |        ",
        "       |     |        ",
        "      /|JAPAN|\\      ",
        "     / |     | \\     ",
        "    /  |     |  \\    ",
        "   |   |  O  |   |    ",
        "   |   |  o  |   |    ",
        "   |   |     |   |    ",
        "   |   |     |   |    ",
        "  /|##!#|   |#!##|\\  ",
        " | |##!#|   |#!##| |  ",
        " | |##!#|   |#!##| |  ",
        " | |##!#|   |#!##| |  ",
        " | |##!#|   |#!##| |  ",
        " \\_|____|___|____|_/ ",
        "     /  |   |  \\     ",
        "    /___|___|___\\    ",
        "       |  |  |       ",
        "       |  |  |       ",
        "       |  |  |       ",
    ];

    // ターミナルのクリア（ANSIエスケープコードを使用）
    fn clear_terminal() {
        print!("\x1B[2J\x1B[H");
    }

    // ターミナルの高さを取得
    let (_, terminal_height) = terminal_size().unwrap();

    // アニメーションのループ終了位置を計算
    let end_position = terminal_height as usize + ROCKET_HEIGHT;

    // アニメーションのループ
    for step in 0..end_position {
        clear_terminal();

        // ロケットの現在位置を描画
        if step < terminal_height as usize {
            // ロケット全体が表示される場合
            for _ in 0..(terminal_height as usize - step) {
                println!(); // 上方向のスペース
            }
            for line in &rocket {
                println!("{}", line);
            }
        } else {
            // ロケットの上部がターミナルの上部を超えた場合
            let start = step - terminal_height as usize;
            for line in &rocket[start..] {
                println!("{}", line);
            }
        }

        // エフェクト
        if step % 2 == 0 {
            println!("         |||");
            println!("        |||||");
            println!("       |||||||");
        } else {
            println!("        |||||");
            println!("       |||||||");
            println!("      |||||||||");
        }

        // アニメーションの間隔を調整
        thread::sleep(Duration::from_millis(170));
    }

    // 打ち上げ完了メッセージ
    clear_terminal();
    println!("Rocket launched successfully! 🚀");
}
