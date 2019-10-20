# Twitter plotter
This is a basic pure rust implementation of a twitter parser which plots its results. No dependencies other than a working rust development environment are required.

## Usage
A set of twitter keys will be necessary in order to use this code. They are all provided via command line arguments. `cargo run` will also interfere with the key word arguments so you will need to run the binary directly after compilation. Usage can be seen by calling `--help` ex.
```
jon~/i/t/t:master❯❯❯ ./target/release/twitterplot --help
twitter-plot 0.1.0

USAGE:
    twitterplot --as <access-secret> --at <access-token> --ca <consumer-apikey> --cs <consumer-secret>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --as <access-secret>      Twitter access secret
        --at <access-token>       Twitter access token
        --ca <consumer-apikey>    Twitter consumer api key
        --cs <consumer-secret>    Twitter consumer secret
```
Release mode compilation is recommended for optimal performance
```
cargo build --release
```
An end to end example of usage:
```
git clone
cargo build --release
./target/release/twitterplot --ca <your consumer api key> --cs <your consumer secret key> --at <your access token> --as <your access secret>
```
Twitter refers to these keys as `Consumer API keys` and `Access token & access token secret`

# Sentiment
At the moment sentiment is defined on a scale of (-1..1) and plotted as time progresses. Sentiment values are determined by counting "good" and "bad" words within a given tweet. Historical tweet count is configurable but set to 200 at the moment.

## Architecture
Built around the [Twitter Stream rs](https://github.com/tesaguri/twitter-stream-rs) library this project inherits the tokio futures based runtime with processing done on a per-message basis. An initial design choice was to register one future for each search term, however a limitation was discovered in the twitter api which caused a redesign. According to [some discussion online](https://stackoverflow.com/questions/34962677/twitter-streaming-api-limits) twitter limits streaming api connections to one per application. In testing I found two to be my personal limit, however the issue resulted in a single connection architecture. As a result tweet classification is non-trivial.

### Architecture Diagram
The logical data flow looks like
![ETL flow](https://raw.githubusercontent.com/darakian/twitterplot/master/docs/arch.png)
In code the stream handlers do the processing.

# Example output
Colors are randomly chosen on each run
![Graph](https://raw.githubusercontent.com/darakian/twitterplot/master/docs/Example%20output.png)
