use ::serenity::all::CreateMessage;
use ::serenity::all::EditMessage;
use ::serenity::all::MessageBuilder;
use ::serenity::all::MessageFlags;
use regex::NoExpand;
use regex::Regex;
use reqwest::StatusCode;
use serde_json::Value;

use crate::event_handler::message::Error;
use crate::event_handler::message::Message;
use crate::serenity;

#[derive(Debug, Clone)]
struct Tweet {
    author: String,
    author_handle: String,
    author_url: String,
    text: String,
    image_url: Option<String>,
}

impl Tweet {
    fn new() -> Self {
        Tweet {
            author: String::from(""),
            author_handle: String::from(""),
            author_url: String::from(""),
            text: String::from(""),
            image_url: None,
        }
    }

    fn set_author(&mut self, author: String) -> &mut Self {
        self.author = author;
        self
    }

    fn set_author_handle(&mut self, author_handle: String) -> &mut Self {
        self.author_handle = author_handle;
        self
    }

    fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }

    fn set_image_url(&mut self, image_url: String) -> &mut Self {
        self.image_url = Some(image_url);
        self
    }

    fn set_author_url(&mut self, author_url: String) -> &mut Self {
        self.author_url = author_url;
        self
    }

    fn build(&mut self) -> Self {
        self.clone()
    }

    pub async fn _from_fxtwitter(tid: String) -> Result<Self, Error> {
        let url = String::from(format!("https://api.fxtwitter.com/i/status/{}", tid));
        let client = reqwest::ClientBuilder::new()
            .user_agent("RumbleBot/0.1.0")
            .build()
            .expect("err in building reqwest client");

        let response = client.get(url).send().await?;

        let query_result = match response.status() {
            StatusCode::OK => response.text().await?,
            _ => String::from("{}"),
        };

        let query_result: Value = serde_json::from_str(&query_result)?;

        let author = query_result["tweet"]["author"]["name"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let author_handle = query_result["tweet"]["author"]["screen_name"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let author_url = query_result["tweet"]["author"]["url"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let text = query_result["tweet"]["text"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let has_media = query_result["tweet"].get("media");

        match has_media {
            Some(_) => {
                let mosaic_photo_link = match query_result["tweet"]["media"].get("mosaic") {
                    Some(mosaic_object) => Some(String::from(
                        mosaic_object["formats"]["webp"].as_str().unwrap(),
                    )),
                    None => None,
                };

                let image_url = match mosaic_photo_link {
                    Some(link) => link,
                    None => {
                        let url = String::from(
                            query_result["tweet"]["media"]["all"][0]["url"]
                                .as_str()
                                .unwrap(),
                        );

                        let media_type = query_result["tweet"]["media"]["all"][0]["type"]
                            .as_str()
                            .unwrap();

                        match media_type {
                            "photo" => url.replace(".jpg", "?format=jpg&name=orig"),
                            _ => {
                                format!("[Video Link]({})", url)
                            }
                        }
                    }
                };

                Ok(Tweet::new()
                    .set_author(author)
                    .set_author_handle(author_handle)
                    .set_author_url(author_url)
                    .set_text(text)
                    .set_image_url(image_url)
                    .build())
            }
            None => Ok(Tweet::new()
                .set_author(author)
                .set_author_handle(author_handle)
                .set_author_url(author_url)
                .set_text(text)
                .build()),
        }
    }

    pub async fn from_vxtwitter(tid: String) -> Result<Self, Error> {
        let url = String::from(format!("https://api.vxtwitter.com/i/status/{}", tid));

        println!("Calling api: {}", url);

        let client = reqwest::ClientBuilder::new()
            .user_agent("RumbleBot/0.1.0")
            .build()
            .expect("err in building reqwest client");

        let response = client.get(url).send().await?;

        let query_result = match response.status() {
            StatusCode::OK => response.text().await?,
            _ => String::from("{}"),
        };

        let query_result: Value = serde_json::from_str(&query_result)?;

        let author = query_result["user_name"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let author_handle = query_result["user_screen_name"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let author_url = format!("https://x.com/{}", author_handle);

        let mut text = query_result["text"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let remove_ending_tco_link_regex =
            Regex::new(r"https:\/\/t.co\/\S+$").expect("invalid regex");

        text = remove_ending_tco_link_regex
            .replace(&text, NoExpand(""))
            .to_string();

        let has_media = query_result["hasMedia"].as_bool().unwrap_or_default();

        match has_media {
            true => {
                let image_url = match query_result["combinedMediaUrl"].as_str() {
                    Some(mosaic_link) => String::from(mosaic_link),
                    None => {
                        let url = String::from(query_result["mediaURLs"][0].as_str().unwrap());

                        let media_type =
                            query_result["media_extended"][0]["type"].as_str().unwrap();

                        match media_type {
                            "image" => url.replace(".jpg", "?format=jpg&name=orig"),
                            _ => {
                                format!("[Video Link]({})", url)
                            }
                        }
                    }
                };

                Ok(Tweet::new()
                    .set_author(author)
                    .set_author_handle(author_handle)
                    .set_author_url(author_url)
                    .set_text(text)
                    .set_image_url(image_url)
                    .build())
            }
            false => Ok(Tweet::new()
                .set_author(author)
                .set_author_handle(author_handle)
                .set_author_url(author_url)
                .set_text(text)
                .build()),
        }
    }

    pub fn create_msg(self) -> CreateMessage {
        // apply links to hashtag
        let capture_all_hashtags_regex = Regex::new(r"(#[\p{L}\p{N}]+)").expect("invalid regex");

        let caps = capture_all_hashtags_regex.captures_iter(&self.text);

        let hashtags_with_links: Vec<String> = caps
            .map(|tag| {
                let mut tag = String::from(tag.get(0).unwrap().as_str());
                let hashtag = tag.clone();
                tag.remove(0);

                format!(
                    "[{}](https://x.com/hashtag/{}?src=hashtag_click)",
                    hashtag, tag
                )
            })
            .collect();

        let mut tweet_msg = capture_all_hashtags_regex
            .replace_all(&self.text, NoExpand("HASHTAG_PLACEHOLDER"))
            .to_string();

        for tag in hashtags_with_links.iter() {
            tweet_msg = tweet_msg.replacen("HASHTAG_PLACEHOLDER", tag, 1);
        }

        let content = MessageBuilder::new()
            .push_line(format!(
                "### {} [(@{})]({})",
                self.author, self.author_handle, self.author_url
            ))
            .push_line("")
            .push_line(tweet_msg)
            .build();

        CreateMessage::new()
            .content(content)
            .flags(MessageFlags::SUPPRESS_EMBEDS)
    }
}

pub async fn respond_twitter_link(ctx: &serenity::Context, msg: &mut Message) -> Result<(), Error> {
    let typing = msg.channel_id.start_typing(&ctx.http);
    let tid_regex =
        Regex::new(r"https:\/\/(x|twitter)\.com\/[A-Za-z0-9_]{1,15}\/status\/(?<tid>[0-9]+)")
            .expect("invalid tid regex");

    let caps = tid_regex.captures(&msg.content).unwrap();

    let tid = String::from(&caps["tid"]);

    // let tweet = Tweet::from_fxtwitter(tid).await?;
    let tweet = Tweet::from_vxtwitter(tid).await?;
    println!("{:?}", tweet);
    let image_url = tweet.image_url.clone();

    msg.edit(&ctx, EditMessage::new().suppress_embeds(true))
        .await?;

    msg.channel_id
        .send_message(&ctx, tweet.create_msg())
        .await?;

    if let Some(image_url) = image_url {
        msg.channel_id.say(&ctx, image_url).await?;
    };

    typing.stop();
    Ok(())
}
