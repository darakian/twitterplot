use twitter_stream::rt::{self, lazy};
use minifb::{Key, WindowOptions, Window};
use structopt::StructOpt;
use std::{thread};
use std::sync::mpsc::channel;
use std::collections::hash_map::{HashMap, Entry};
use std::collections::VecDeque;
use twitterplot::{render_frame, twitter_sub};
const WIDTH: usize = 1000;
const HEIGHT: usize = 800;
const HISTORY: usize = 200;

#[derive(StructOpt, Debug)]
#[structopt(name = "Twitter Plot")]
struct Opt {
    #[structopt(long = "ca", help = "Twitter consumer api key")]
    consumer_apikey: String,
    #[structopt(long = "cs", help = "Twitter consumer secret")]
    consumer_secret: String,
    #[structopt(long = "at", help = "Twitter access token")]
    access_token: String,
    #[structopt(long = "as", help = "Twitter access secret")]
    access_secret: String,

}

fn main() {
    let opt = Opt::from_args();
    let consumer_apikey = opt.consumer_apikey;
    let consumer_secret = opt.consumer_secret;
    let access_token = opt.access_token;
    let access_secret = opt.access_secret;
    let subscription_string = "twitter,facebook,google,travel,art,music,photography,love,fashion,food";
    let mut window = Window::new("Press ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let (sender, receiver) = channel();
    let mut render_buffer: Vec<u8> = vec![0; WIDTH * HEIGHT * 3];
    let mut sentiments = HashMap::new();
    for token in vec!["twitter","facebook","google","travel","art","music","photography","love","fashion","food"].iter(){
        let history: VecDeque<f32> = VecDeque::with_capacity(HISTORY);
        sentiments.insert(
            token.to_string(),
            history
        );
    }


    thread::spawn(move || {
        rt::run(lazy(move || {
        let future = twitter_sub(
            consumer_apikey.to_string(),
            consumer_secret.to_string(),
            access_token.to_string(),
            access_secret.to_string(),
            subscription_string,
            sender
        );
        rt::spawn(future);
        Ok(())
        }));
    });

    while window.is_open() && !window.is_key_down(Key::Escape) { //Infinite loop
        let iter = receiver.try_iter();
        for element in iter{
            match sentiments.entry(element.0) {
                Entry::Vacant(_e) => {panic!{"I done goofed. Element classified to non-existent case."}},
                Entry::Occupied(mut e) => {
                    e.get_mut().push_front(element.1);
                    e.get_mut().truncate(HISTORY);
                }
             }
        }
        let disp_buff = render_frame(WIDTH, HEIGHT, sentiments.clone(), HISTORY, &mut render_buffer);
        window.update_with_buffer(&disp_buff).unwrap();
    }
}
