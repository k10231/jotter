# Jotter

this is a tool which is literally on my mind since i started using pc (aggresively) and i know there are online tools which do these.
so after all why **why** shouldn't i build my own.

as name states you can jot down stuffs quickly while using shell
and see it later on website which is powered by Rust (server sided rendering)

## Usage

#### Requirements 
1. Rust installed
2. Linux (if you want to use bash for quick jot alternatively you can use any turing complete lang to make http requests :) )

### How to use

##### After cloning this project 
run 
```bash
bash bash-tool.sh "<args>"
```
to save some stuffs to database 

then to view it run
```rust
cargo run
```
> and keep in mind you need [supabase](https://supabase.io/) database to use this stuff (which is free and very cool) 

then visit http://localhost:3000 Voilà 

this is how it looks
<img src="https://github.com/akhmed-codes/jotter/blob/main/assets/website-screenshot.png" />

i know i am not the best designer but it works 
feel free to improve if you think you can do better then me
