// use chrono::{ Utc, Duration };
use teloxide::types::{ ChatPermissions, UserId };
use teloxide::prelude::*;
use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
struct Config {
    bot: BotConfig,
}

#[derive(Deserialize)]
struct BotConfig {
    token: String,
}

#[tokio::main]
async fn main() {
    // Чтение конфигурационного файла
    let file = File::open("config.yaml").expect("Failed to open config file");
    let config: Config = serde_yaml::from_reader(file).expect("Failed to read config");

    // Создание бота
    let bot = Bot::new(config.bot.token);

    // Определение списка разрешённых идентификаторов чатов и запрещённых слов
    let allowed_chat_ids = vec![ChatId(-924123638)];
    let forbidden_words = vec![
        "/",
        "поиске",
        "видео",
        "invite",
        "Видео",
        "Зaрaбaтывaю",
        "зaрaбaтывaю"
    ];

    // Реализация функции-обработчика сообщений
    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let allowed_chat_ids = allowed_chat_ids.clone();
        let forbidden_words = forbidden_words.clone();

        async move {
            if let Some(user) = msg.from() {
                println!("Chat ID: {}", msg.chat.id);

                if user.id == UserId(651726581) {
                    if allowed_chat_ids.contains(&msg.chat.id) {
                        match bot.get_chat(user.id).await {
                            Ok(chat) => {
                                let bio = chat.bio().unwrap_or_default();
                                let contains_forbidden_word = forbidden_words
                                    .iter()
                                    .any(|word| bio.contains(word));

                                if contains_forbidden_word {
                                    println!("User's bio contains forbidden words!");
                                    // Ваш код для обработки случая с запрещёнными словами
                                } else {
                                    println!("User's bio is clean.");
                                }
                            }
                            Err(err) => {
                                println!("Failed to get chat: {:?}", err);
                            }
                        }
                    } else {
                        println!("Message from non-allowed chat.");
                    }
                } else {
                    println!("No permissions, nothing is allowed for {}", user.id);
                    bot.restrict_chat_member(
                        msg.chat.id,
                        user.id,
                        ChatPermissions::empty()
                    ).await.unwrap();
                }
            }
            respond(())
        }
    }).await;
}
