use plotters::prelude::*;
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::rt::{self};
use serde_json::{Value};
use twitter_stream::rt::{Future, Stream};
use rand::seq::SliceRandom;
use std::sync::mpsc::Sender;
use std::collections::HashMap;
use std::collections::VecDeque;

const GOOD_WORDS: [&'static str; 10] = ["winning","love","best","taco","hooray","wonderful","❤️","😍","beautiful","fantastic"];
const BAD_WORDS: [&'static str; 10] = ["losing","hate","worst","tamale","boo","terrible","💔","😢","horrible","dystopian"];

pub fn render_frame(w: usize, h: usize, trends: HashMap<String, VecDeque<f32>>, hist: usize) -> Vec<u32>{
    let WIDTH = w;
    let HEIGHT = h;
    let mut bit_buffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 4];
    let root = BitMapBackend::with_buffer(&mut bit_buffer, (WIDTH as u32, HEIGHT as u32)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Selected Twitter Sentiment", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0f32..hist as f32, -0.8f32..0.8f32).unwrap();

    chart.configure_mesh().draw().unwrap();
    for (id, tuple) in trends.into_iter().enumerate(){
        let points = tuple.1.into_iter().enumerate().map(|x| (x.0 as f32, x.1));
        chart
            .draw_series(LineSeries::new(
                points.rev(),
                &Palette99::pick(id),
            )).unwrap()
            .label(tuple.0)
            .legend(move |(x, y)| Path::new(vec![(x, y), (x + 20, y)], &Palette99::pick(id)));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();
    drop(chart);
    drop(root);
    let disp_buff: Vec<u32> = bit_buffer.chunks_exact(3).map(|rgb| from_u8_rgb(
        rgb[0],
        rgb[1],
        rgb[2])).collect();
    return disp_buff
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn twitter_sub(c_key: String, c_secret: String, a_token: String, a_secret: String, track_id: &str, s: Sender<(String, f32)>) -> impl Future<Item = (), Error =()>{
    let owned_id = track_id.to_owned();
    let tracker = Some(owned_id.as_str());
    TwitterStreamBuilder::filter(Token::new(
        c_key,
        c_secret,
        a_token,
        a_secret
    ))
        .track(tracker)
        .listen()
	.unwrap()
        .flatten_stream()
        .for_each(move |json| {
            let value: Value = serde_json::from_str(&json).unwrap();
            process_tweet(value, owned_id.clone(), s.clone());
            Ok(())
        })
        .map_err(|e| println!("error: {}\n 420 => Rate limit", e))
}

fn process_tweet(tweet: Value, tags: String, s: Sender<(String, f32)>) -> (){
    let mut good_count = 0;
    let mut bad_count = 0;
    let mut classifier = String::new();
    let tag_list: Vec<&str> = tags.split(',').collect();
    for token in tweet["text"].to_string().split(' '){
        if tag_list.contains(&token){
            classifier=token.to_string();
        }
        if GOOD_WORDS.contains(&token){
            good_count+=1;
        }
        if BAD_WORDS.contains(&token){
            bad_count+=1;
        }
    }
    if classifier==""{ //Give it a random classifier for now
        classifier = tag_list.choose(&mut rand::thread_rng()).unwrap().to_string();
    }
    let score: f32 = (good_count as f32 - bad_count as f32) / (good_count+bad_count+1) as f32;
    s.send((classifier.to_string(),score));
}
