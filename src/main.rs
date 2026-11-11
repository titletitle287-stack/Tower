use macroquad::prelude::*;

struct Enemy {
    pos: Vec2,
    hp: f32,
    max_hp: f32,
    speed: f32,
    color: Color,
    reward: i32,
}

struct Projectile {
    pos: Vec2,
    target: Vec2,
    speed: f32,
    damage: f32,
}

#[macroquad::main("Days Clone")]
async fn main() {
    request_new_screen_size(450.0, 800.0);
    
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut projectiles: Vec<Projectile> = Vec::new();
    let mut gold = 0;
    let mut wave = 1;
    let mut castle_hp = 100.0;
    let mut spawn_timer = 0.0;
    let mut enemies_to_spawn = 5;
    let mut game_over = false;
    
    let castle_pos = vec2(50.0, screen_height() / 2.0);

    loop {
        let dt = get_frame_time();
        clear_background(Color::from_rgba(30, 40, 30, 255));

        if !game_over {
            spawn_timer -= dt;
            if spawn_timer <= 0.0 && enemies_to_spawn > 0 {
                let y = rand::gen_range(100.0, screen_height() - 100.0);
                let is_boss = wave % 5 == 0 && enemies_to_spawn == 1;
                let hp = if is_boss { 100.0 * (wave as f32) } else { 20.0 * (wave as f32) };
                
                enemies.push(Enemy {
                    pos: vec2(screen_width() + 50.0, y),
                    hp,
                    max_hp: hp,
                    speed: if is_boss { 20.0 } else { 40.0 + (wave as f32 * 2.0) },
                    color: if is_boss { RED } else { ORANGE },
                    reward: if is_boss { 10 } else { 2 },
                });
                
                enemies_to_spawn -= 1;
                spawn_timer = if is_boss { 2.0 } else { 1.0 };
            }

            if enemies.is_empty() && enemies_to_spawn == 0 {
                wave += 1;
                enemies_to_spawn = 5 + wave * 2;
                spawn_timer = 2.0;
            }

            for enemy in enemies.iter_mut() {
                let dir = (castle_pos - enemy.pos).normalize_or_zero();
                enemy.pos += dir * enemy.speed * dt;
                
                if enemy.pos.distance(castle_pos) < 30.0 {
                    castle_hp -= if enemy.max_hp > 50.0 { 20.0 } else { 5.0 };
                    enemy.hp = 0.0;
                }
            }

            for proj in projectiles.iter_mut() {
                let dir = (proj.target - proj.pos).normalize_or_zero();
                proj.pos += dir * proj.speed * dt;
            }

            for proj in projectiles.iter_mut() {
                for enemy in enemies.iter_mut() {
                    if enemy.hp > 0.0 && proj.pos.distance(enemy.pos) < 20.0 {
                        enemy.hp -= proj.damage;
                        proj.pos = vec2(-1000.0, -1000.0);
                        if enemy.hp <= 0.0 {
                            gold += enemy.reward;
                        }
                    }
                }
            }

            enemies.retain(|e| e.hp > 0.0);
            projectiles.retain(|p| p.pos.x > 0.0 && p.pos.y > 0.0 && p.pos.x < screen_width() + 100.0);

            if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_down(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if projectiles.len() < 10 {
                    projectiles.push(Projectile {
                        pos: castle_pos,
                        target: vec2(mx, my),
                        speed: 600.0,
                        damage: 10.0,
                    });
                }
            }

            for touch in touches() {
                if touch.phase == TouchPhase::Started || touch.phase == TouchPhase::Stationary {
                     if projectiles.len() < 10 {
                        projectiles.push(Projectile {
                            pos: castle_pos,
                            target: touch.position,
                            speed: 600.0,
                            damage: 10.0,
                        });
                    }
                }
            }

            if castle_hp <= 0.0 {
                game_over = true;
            }
        }

        draw_rectangle(castle_pos.x - 20.0, castle_pos.y - 30.0, 40.0, 60.0, BLUE);
        draw_rectangle(castle_pos.x - 10.0, castle_pos.y - 10.0, 8.0, 8.0, WHITE);
        draw_rectangle(castle_pos.x + 2.0, castle_pos.y - 10.0, 8.0, 8.0, WHITE);
        
        draw_rectangle(10.0, 10.0, 200.0, 20.0, DARKGRAY);
        draw_rectangle(10.0, 10.0, 200.0 * (castle_hp / 100.0), 20.0, GREEN);

        for enemy in &enemies {
            draw_circle(enemy.pos.x, enemy.pos.y, 15.0, enemy.color);
            let hp_ratio = enemy.hp / enemy.max_hp;
            draw_rectangle(enemy.pos.x - 15.0, enemy.pos.y - 25.0, 30.0 * hp_ratio, 4.0, RED);
        }

        for proj in &projectiles {
            draw_circle(proj.pos.x, proj.pos.y, 5.0, YELLOW);
        }

        draw_text(&format!("Gold: {}", gold), 10.0, 60.0, 30.0, GOLD);
        draw_text(&format!("Wave: {}", wave), 10.0, 100.0, 30.0, WHITE);

        if game_over {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.8));
            draw_text("GAME OVER", screen_width() / 2.0 - 100.0, screen_height() / 2.0, 50.0, RED);
            draw_text("Tap to restart", screen_width() / 2.0 - 80.0, screen_height() / 2.0 + 50.0, 30.0, WHITE);
            
            if is_mouse_button_pressed(MouseButton::Left) || touches().len() > 0 {
                enemies.clear();
                projectiles.clear();
                gold = 0;
                wave = 1;
                castle_hp = 100.0;
                enemies_to_spawn = 5;
                spawn_timer = 0.0;
                game_over = false;
            }
        }

        next_frame().await;
    }
                  }
