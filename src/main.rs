use macroquad::prelude::*;
use ::rand::Rng;
// ゲームの状態（シーン）を定義
enum GameState {
    Start,
    Battle,
    ResultSuccess,
    ResultFailure, 
}

#[macroquad::main("転職クエスト")]
async fn main() {
    let texture: Texture2D = load_texture("assets/background.png").await.unwrap();
    let font = load_ttf_font("assets/msgothic.ttc").await.unwrap();

    let mut  hp = 100;
    let mut state = GameState::Start;

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // --- 共通の背景描画 ---
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // --- メッセージウィンドウ（黒い長方形）を描画 ---
        draw_rectangle(
            20.0, 
            screen_height() - 150.0, 
            screen_width() - 40.0, 
            130.0, 
            Color::new(0.0, 0.0, 0.0, 0.8)
        );

        // --- ウィンドウの白い枠線を描画 ---
        draw_rectangle_lines(
            20.0, 
            screen_height() - 150.0, 
            screen_width() - 40.0, 
            130.0, 
            3.0, 
            WHITE
        );

        // --- 状態ごとの表示と入力処理 ---
        match state {
            GameState::Start => {
                draw_msg(&font, "厳格な採用担当が現れた！", "スペースキーで対話開始");
                if is_key_pressed(KeyCode::Space) {
                    state = GameState::Battle;
                }
            }
            GameState::Battle => {
                draw_msg(&font, "「あなたの強みを教えてください」", &format!("1:熱意 2:逆質問 3:笑顔 | HP:{}", hp));
                if is_key_pressed(KeyCode::Key1) {
                    // 成功！内定へ
                    state = GameState::ResultSuccess;
                } 
                else if is_key_pressed(KeyCode::Key2) {
                    // 10～30のランダムなダメージ
                    let damage = ::rand::thread_rng().gen_range(10..31);
                    hp -= damage;

                    if hp <= 0 {
                        state = GameState::ResultFailure;    
                    }
                }
                else if is_key_pressed(KeyCode::Key3) {
                    // 笑顔で回復（最大100を超えないように）
                    hp += 15;
                    if hp > 100 { hp = 100; }
                }
            }
            
            GameState::ResultSuccess => {
                draw_msg(&font, "担当者「素晴らしい。ぜひ弊社で！」", "おめでとう！ 内定獲得です！（ESCで終了）");
            }
            GameState::ResultFailure => { // Fは大文字
                draw_msg(&font, "担当者「検討の結果、今回は…」", "残念！ お祈りされました…（ESCで終了）");
            }
        }

        next_frame().await
    }
}

// 文字表示を楽にするための「自作関数」
fn draw_msg(font: &Font, line1: &str, line2: &str) {
    let params = TextParams { font: Some(font), font_size: 30, color: WHITE, ..Default::default() };
    draw_text_ex(line1, 40.0, screen_height() - 100.0, params.clone());
    draw_text_ex(line2, 40.0, screen_height() - 60.0, params);
}