use std::fs::File;
use std::io::Write;

use chrono::Utc;
use indoc::formatdoc;
use poise::ApplicationContext;
use reqwest::StatusCode;
use reqwest::header::HeaderMap;
use reqwest::multipart::Part;
use serenity::all::ButtonStyle;
use serenity::all::CreateButton;
use serenity::all::EditInteractionResponse;
use serenity::all::Message;
use serenity::json::Value;

use crate::Data;
use crate::cmd::Error;

#[derive(sqlx::FromRow)]
struct AlbumSize(i64);

/// Displays the author's account creation date
#[poise::command(context_menu_command = "Add this image to Album", owners_only)]
pub async fn add_image_to_album(ctx: ApplicationContext<'_, Data, Error>, msg: Message) -> Result<(), Error> {
    ctx.interaction.defer_ephemeral(ctx).await?;

    // obtain the author as user if not specified
    let _response: String = "https://ipp.dora343.dev/share/KEY".into();
    // println!("{}", response);
    // ctx.say(response).await?;

    let immich_url = std::env::var("IMMICH_URL").expect("missing DISCORD_TOKEN");
    let immich_api_key = std::env::var("IMMICH_API_KEY").expect("missing DISCORD_TOKEN");

    // println!("{}", immich_url);
    // println!("{}", immich_api_key);

    let mut headers = HeaderMap::new();

    headers.append("x-api-key", immich_api_key.parse().unwrap());

    let mut extracted_img_url: String = "".into();

    if !msg.embeds.is_empty() {
        let embed = &msg.embeds[0];

        if let Some(kind) = &embed.kind {
            match kind.as_str() {
                "image" => {
                    if let Some(img_url) = &embed.url {
                        extracted_img_url = img_url.to_string();
                    }
                }
                "rich" => {
                    if let Some(img) = &embed.image {
                        extracted_img_url = img.url.to_string();
                    }
                }
                _ => {}
            }
        }

        if &extracted_img_url != "" {
            let client = reqwest::ClientBuilder::new()
                .user_agent("RumbleBot/0.1.0")
                .build()
                .expect("err in building reqwest client");

            let api_response = client.get(extracted_img_url).send().await?;
            println!("{:?}", api_response);
            let image_bytes = match api_response.status() {
                StatusCode::OK => api_response.bytes().await?,
                _ => bytes::Bytes::new(),
            };

            if image_bytes.len() != 0 {
                let mut file = File::create("tmp.jpg")?;
                file.write_all(&image_bytes)?;
            }

            let asset_upload_url = format!("{}/api/assets", immich_url);

            let current_time = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);

            let multipart_form = reqwest::multipart::Form::new()
                .part("assetData", Part::file("tmp.jpg").await?)
                .text("deviceAssetId", "assetTest")
                .text("deviceId", "rumble")
                .text("fileCreatedAt", current_time.clone())
                .text("fileModifiedAt", current_time);

            let request_builder = client
                .post(asset_upload_url)
                .headers(headers.clone())
                .multipart(multipart_form);

            let api_response = request_builder.send().await?;

            std::fs::remove_file("tmp.jpg")?;

            println!("{:?}", api_response.status());

            match api_response.status() {
                StatusCode::OK | StatusCode::CREATED => {
                    let response = api_response.text().await?;
                    let response_json: Value = serde_json::from_str(&response)?;
                    let img_id = response_json["id"].as_str().unwrap().to_string();

                    // obtain share link and convert to ipp links

                    let request_data = formatdoc! {"
                        {{
                            \"type\": \"INDIVIDUAL\",
                            \"assetIds\": [
                                \"{}\"
                            ],
                            \"allowUpload\": false,
                            \"allowDownload\": true,
                            \"showMetadata\": false
                        }}", 
                        img_id
                    };

                    let request_data_value: Value = serde_json::from_str(&request_data)?;

                    let create_shared_links_url = format!("{}/api/shared-links", immich_url);

                    let api_response = client
                        .post(create_shared_links_url)
                        .headers(headers.clone())
                        .json(&request_data_value)
                        .send()
                        .await?;

                    match api_response.status() {
                        StatusCode::CREATED => {
                            let response: Value =
                                serde_json::from_str(&api_response.text().await?)?;
                            let img_key = response["key"].as_str().unwrap().to_string();
                            println!("{}", img_key);
                            // save this img id, key and author id to gallery table
                            println!("Inserting new image {} into gallery.album", img_id);
                            let res = sqlx::query(
                                r#"
                                insert into gallery.album 
                                (id, image_key, user_id)
                                values ($1, $2, $3)
                                on conflict(id)
                                do update set image_key = $2;
                                "#,
                            )
                            .bind(img_id)
                            .bind(img_key)
                            .bind(ctx.author().id.get() as i64)
                            .execute(&ctx.data().dbpool)
                            .await?;
                            println!("Affected rows: {}", res.rows_affected());

                            let res: AlbumSize = sqlx::query_as(
                                r#"
                                select count(*) from gallery.album
                                "#,
                            )
                            .fetch_one(&ctx.data().dbpool)
                            .await?;

                            let interaction_response_builder = 
                            // CreateInteractionResponse::UpdateMessage(
                                EditInteractionResponse::new()
                                    .content(format!("This image has been added to the album.\nThe album now contains {} images.", res.0))
                            // )
                            ;

                            ctx.interaction
                                .edit_response(ctx, interaction_response_builder)
                                .await?;
                            return Ok(());
                        }
                        _ => {
                            println!("{}", api_response.text().await?)
                        }
                    };
                }
                _ => {}
            };
        }
    }

    let _test_button = CreateButton::new("test")
        .style(ButtonStyle::Success)
        .label("click me");

    let interaction_response_builder = EditInteractionResponse::new().content("Failed");

    let _msg = ctx
        .interaction
        .edit_response(ctx, interaction_response_builder)
        .await?;

    // let interaction = match msg
    //     .await_component_interaction(&ctx.serenity_context.shard)
    //     .timeout(Duration::from_secs(60))
    //     .await
    // {
    //     Some(x) => x,
    //     None => return Ok(()),
    // };

    // // update original message when button is pressed
    // match interaction.data.kind {
    //     ComponentInteractionDataKind::Button => {
    //         let response_content = String::from("hello world!");

    //         let interaction_response_builder = CreateInteractionResponseMessage::new()
    //             .content(response_content)
    //             .components(vec![]);

    //         let interaction_response =
    //             CreateInteractionResponse::UpdateMessage(interaction_response_builder);

    //         interaction
    //             .create_response(ctx, interaction_response)
    //             .await?;
    //     }
    //     _ => return Ok(()),
    // };

    Ok(())
}
