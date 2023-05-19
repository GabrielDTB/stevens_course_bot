mod get_courses;

use std::env;

use anyhow::{bail, Result};
use async_once::AsyncOnce;
use get_courses::Course;
use lazy_static::lazy_static;
use serenity::async_trait;
use serenity::model::channel::*;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CourseQueryError {
    #[error("Course distribution \"{distribution}\" not found")]
    DistributionNotFound { distribution: String },
    #[error("Course \"{course_id}\" not found")]
    CourseNotFound { course_id: String },
}

struct Handler;

const PREFIX: &str = "!classy";
lazy_static! {
    static ref COURSES: AsyncOnce<HashMap<String, Course>> =
        AsyncOnce::new(async { get_courses::do_stuff().await.unwrap() });
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content.starts_with(PREFIX) {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);

                    return;
                }
            };
            let command = msg.content.split_once(PREFIX).unwrap().1.trim();
            if command.starts_with("query") {
                let body = command.split_once("query").unwrap().1.trim();
                match query_course(&*body).await {
                    Ok(course) => {
                        println!("{:#?}", course.clone());
                        if let Err(why) = msg
                            .channel_id
                            .send_message(&context.http, |m| {
                                m.embed(|e| {
                                    e.title(format!("{} {}", course.id, course.name))
                                        .url(format!("{}", course.link))
                                        .description(format!("{}", course.description))
                                        .fields({
                                            let mut fields = vec![];
                                            if !course.credits.is_empty() {
                                                fields.push(("Credits", course.credits, false));
                                            }
                                            if !course.prerequisites.is_empty() {
                                                fields.push((
                                                    "Prerequisites #IN PROGRESS",
                                                    format!("{:?}", course.prerequisites),
                                                    false,
                                                ));
                                            } else {
                                                fields.push((
                                                    "Prerequisites",
                                                    "None".into(),
                                                    false,
                                                ));
                                            }
                                            if !course.offered.is_empty() {
                                                fields.push((
                                                    "Offered",
                                                    course
                                                        .offered
                                                        .iter()
                                                        .map(|s| &**s)
                                                        .collect::<Vec<&str>>()
                                                        .join(", "),
                                                    false,
                                                ));
                                            }
                                            if !course.distribution.is_empty() {
                                                fields.push((
                                                    "Distribution",
                                                    course
                                                        .distribution
                                                        .iter()
                                                        .map(|s| &**s)
                                                        .collect::<Vec<&str>>()
                                                        .join(", "),
                                                    false,
                                                ));
                                            }
                                            fields
                                        })
                                        .footer(|f| f.text("Database updated"))
                                        .timestamp("2023-05-19T19:00:02Z")
                                })
                            })
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        };
                    }
                    Err(why) => {
                        println!("{:#?}", why);
                        if let Err(why) = msg
                            .channel_id
                            .send_message(&context.http, |m| {
                                m.embed(|e| e.title("CS 115 Introduction to Computer Science"))
                            })
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        };
                    }
                };
            } else {
                let response = MessageBuilder::new()
                    .push("User ")
                    .push_bold_safe(&msg.author.name)
                    .push(" used the 'ping' command in the ")
                    .mention(&channel)
                    .push(" channel")
                    .build();

                if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    COURSES.get().await;
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    Ok(())
}

async fn query_course(quarry: &str) -> Result<Course, CourseQueryError> {
    let courses = COURSES.get().await;
    let r;
    // Separate course prefix from numbers and reorder to allow for more acceptable queries
    let letters = quarry
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>();
    let numbers = quarry
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    if let Some(course) = courses.get(&(format!("{letters} {numbers}"))) {
        r = Ok(course.clone());
    } else if !courses.keys().any(|s| s.contains(&letters)) {
        r = Err(CourseQueryError::DistributionNotFound {
            distribution: letters,
        });
    } else {
        r = Err(CourseQueryError::CourseNotFound {
            course_id: letters + " " + &*numbers,
        });
    }
    r
}
