use std::{fs::File, io::Read};

use bevy::prelude::*;
use rand::Rng;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_startup_system(setup_camera);
    app.add_startup_system(setup_game);
    app.add_system(game_wordle);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, mut query: Query<&mut Word>,) {
    commands.spawn().insert(generate_word());

    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    for y in 0..6 {
        for x in 0..5 {
            let transform = Transform {
                translation: Vec3::new((60.0 * x as f32) - 50.0, (120.0 * y as f32) - 300.0, 0.0),
                ..Default::default()
            };
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.25, 0.75),
                        custom_size: Some(Vec2::new(50.0, 100.0)),
                        ..default()
                    },
                    transform: transform,
                    ..default()
                })
                .insert(Position { x: x, y: y });
            commands
                .spawn()
                .insert_bundle(Text2dBundle {
                    text: Text::with_section(
                        " ",
                        TextStyle {
                            font: asset_server.load("fonts/PlayfairDisplay-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                        text_alignment,
                    ),
                    transform: Transform {
                        translation: Vec3::new(
                            transform.translation.x,
                            transform.translation.y,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Position { x: x, y: y });
        }
    }
}

fn game_wordle(
    mut char_input_events: EventReader<ReceivedCharacter>,
    mut query: Query<&mut Word>,
    mut text: Query<(&mut Text, &Position)>,
    mut sprite: Query<(&mut Sprite, &Position)>,
) {
    for mut word in query.iter_mut() {
        for event in char_input_events.iter() {
            info!("{:?}: '{}'", event, event.char);
            if word.current_word.len() == 5 {
                word.current_word_string = word.current_word.iter().cloned().collect::<String>();
            } else if event.char.is_alphabetic() {
                let letter = event.char.to_lowercase().next().unwrap();

                word.current_word.push(letter);

                for (mut text, position) in text.iter_mut() {
                    if position.x as usize == word.current_word.len() - 1 {
                        if position.y as i32 == word.tried_words + 5 {
                            text.sections[0].value = letter.to_string();
                            println!("{}", word.tried_words + 5);
                            println!("Yep {}", text.sections[0].value);
                            word.current_word.len();
                        }
                    }
                }
            }
            if event.char == '\r' {
                if word.correct_word == word.current_word_string {
                    for current_letter in 0..word.current_word.len() {
                        let correct_word: Vec<char> = word.correct_word.chars().collect();
                        if word.current_word[current_letter] == correct_word[current_letter] {
                            for (mut sprite, position) in sprite.iter_mut() {
                                if position.x == current_letter as i8 {
                                    if position.y as i32 == word.tried_words + 5 {
                                        sprite.color = Color::GREEN;
                                        println!("You win!");
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if is_a_word(&word) {
                        println!(
                            "Its the wrong word you put '{}' and the correct word is '{}'",
                            word.current_word_string, word.correct_word
                        );

                        for current_letter in 0..word.current_word.len() {
                            let correct_word: Vec<char> = word.correct_word.chars().collect();
                            if word.current_word[current_letter] == correct_word[current_letter] {
                                for (mut sprite, position) in sprite.iter_mut() {
                                    if position.x == current_letter as i8 {
                                        if position.y as i32 == word.tried_words + 5 {
                                            sprite.color = Color::GREEN;
                                        }
                                    }
                                }
                            }
                        }

                        for x in 0..word.current_word.len() {
                            word.current_word.pop();
                        }

                        word.tried_words -= 1;
                        if word.tried_words == -6 {
                            println!("Game over");
                        }
                    } else {
                        println!(
                            "This is not a word '{}' the word is '{}'",
                            word.current_word_string, word.correct_word
                        );
                    }
                }
            }
            if event.char == '\u{8}' {
                word.current_word_string = word.current_word.iter().cloned().collect::<String>();
                word.current_word.pop();
                for (mut text, position) in text.iter_mut() {
                    if position.x as usize == word.current_word.len() {
                        if position.y as i32 == word.tried_words + 5 {
                            text.sections[0].value = " ".to_string();
                        }
                    }
                }
                println!("{}", word.current_word_string);
            }
        }
    }
}

fn is_a_word(word: &Word) -> bool {
    if word.current_word.len() == 5 {
        for testss in word.word_list.iter() {
            let tests = testss.clone();
            if word.current_word_string == tests {
                return true;
            }
        }
    }
    false
}

fn generate_word() -> Word {
    let mut data: String = String::new();
    let mut f =
        File::open("C:/dev/vuudra/client/assets/word-list.txt").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");

    let mut vec: Vec<String> = Vec::new();

    for testzz in data.lines() {
        vec.push(testzz.to_string());
    }

    println!("{}", vec.len());

    Word {
        correct_word: vec[rand::thread_rng().gen_range(0..5756)].to_string(),
        current_word: Vec::new(),
        current_word_string: String::new(),
        word_list: vec,
        tried_words: 0,
    }
}

#[derive(Component)]
struct Word {
    correct_word: String,
    current_word: Vec<char>,
    current_word_string: String,
    word_list: Vec<String>,
    tried_words: i32,
}

#[derive(Component)]
struct Position {
    x: i8,
    y: i8,
}
