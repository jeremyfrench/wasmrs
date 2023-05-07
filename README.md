#WebAsm In Rust

This is currently just a sandbox for trying to get WebAsm running with Rust.

I'm using it as an oppertunity to learn a bit more Rust while experementing with WASM. It's not intended to end up as a poduct or a library but it could be useful for someone else looking to do the same.

This code was developed with the assistance of ChatGPT, a large language model trained by OpenAI, based on the GPT-3.5 architecture. Some of the code in this project is based on guidance and suggestions provided by ChatGPT during development.

ChatGPT also "chose" that the code be released under an MIT licence.

This repo is basically a sandbox of me trying things out. It's good if you too are looking to tinker. If you actually want a rust wasm powered data tool, look at this https://www.quadratichq.com/
# Setup

```
cargo build
npm i
npm run serve
```
should get you a test server where you can check the benchmark yourself. 

# Backstory

In 2010 I was a not so young hot shot dev. I was asked if I could write something to analyse data sets, I looked into it and said JS can handle this. 
Narrator: JS could not handle that. 

It's very rare that I don't deliver software that I set out to. So that fact that this would lock up a browser and not deliver has never sat well with me. I've for years seen things about WebAsm so thought it could be a way to get this to work. But I have never had the time to delve into it. I've also wanted to write more Rust. 

I saw code generation with ChatGPT and whatever else that is it is a big productivity tool. Enough to help me to get something working with this. 

This has been great for the algorithms and direction, but also it has been completely wrong or misleading at times. It at least gives error messages that you can work on. 

# Work
## Phase 1
This was a POC and a benchmark to make sure that this was possible and faster than vanilla JS

## Phase 2
This enabled CSV data to be entered via a text area

## Phase 3
This runs statics on the data and display graphs.

## Phase 4
Make it look nice and display progress in some way when it is running.

## Phase 5
Allow different types in the data


After that who knows...

# Findings

You must must must optimise the rust code otherwise it will be slower than JS. 

The optimised code is about 8X faster than JS. That's with the function call overhead too.

Webpack is harder than Rust. Getting the rust to compile was easy, getting the benchmark to run took much longer.

Getting Webpack to make a CSV file rather than including the CSS in the JS was harder than expected.



# Reading list/Links

https://vincentarelbundock.github.io/Rdatasets/datasets.html
