# JayI

a Rust-based terminal AI chatbot because apparently talking to a terminal is more fun when it talks back.. and has opinions about donuts, apparently.. (based on personal experiences.... OH.)

## About

JayI is a Rust-based AI chatbot that runs directly in the terminal and communicates with a locally running `Ollama` model (yeah, specifically the `llama3.2` model!)

the idea is pretty simple: have a conversational AI that you can interact with right from your terminal, without needing to open a browser or build an entire web application just to talk to a model.

JayI uses Ollama to handle local model inference, with `llama3.2` as its current model. Rust handles the application logic, terminal interaction, conversation history, and communication with the AI backend.

and yes, apparently it can debate whether a goose constantly chasing and biting you is worse or not worse than a phone exposing your search history to everyone... very important scientific research, indeed..

## Features

- `terminal-based conversational AI`
- `local AI inference` through Ollama
- `streaming responses`, so the AI can respond as text is generated
- conversation history maintained throughout a `session`
- customizable `system instructions` to influence the AI's behavior and response style (you provide em!!)
- a `thinking animation` while waiting for the model to respond
- `elapsed-time display` for responses
- commands for `exiting`, `clearing` the terminal, and `resetting` conversation history
- a Rust implementation built around an AI client rather than manually handling the entire inference process

## How It Works

JayI acts as the interface between you and an Ollama model.

the general flow looks like this:

```text
User Input
    |
    v
JayI Terminal Interface
    |
    + -------> Conversation History
    |
    + -------> System Instructions
    |
    v
AI Client (ollama-rs)
    |
    v
  Ollama
    |
    v
Local Language Model
    |
    v
Generated Response
    |
    v
Streaming Output in the Terminal
```

when you enter a message, JayI sends it to the configured Ollama model along with the applicable conversation context and system instructions.

the model generates a response, which JayI displays in the terminal as it arrives.

this makes the interaction feel more like an actual conversation instead of waiting for one enormous response to appear all at once.

well, THAT IS, assuming the model doesn't suddenly decide that a pigeon is qualified to govern the entire planet..

## System Instructions

one of JayI's useful features is the ability to customize its system instructions.

system instructions help establish how the AI should behave throughout a conversation, including its tone, personality, and response style.

for example, you can ask JayI to use a chill personality and type entirely in lowercase:

```bash
jayi "be chill with a lowercase personality. i mean typing style"
# jayi <system instructions>?
# you might be familiar with this format or whatever (a <b> <c>?) if ONLY you actually read my source code from different repositories
# buuuut a '?' would mean optional!! which means you can leave it just blank
```

and then start a conversation with it normally.

this can be useful for experimenting with different conversational styles, testing instruction-following, or just making the AI feel less like a formal customer support representative.

because apparently, even a terminal chatbot deserves a personality.. (who'd want a BLAND one....)

## Command-Line Usage

JayI can be launched through its `jayi` launcher.

### Start a conversation

```bash
jayi
```

this starts JayI with its default system instructions!

### Customize the system instructions

```bash
jayi "be chill with a lowercase personality"
```

the supplied instruction is used to customize the AI's behavior for the session.

### Available commands

JayI currently supports the following commands during a conversation:

| Command          | Description                                                                              |
| ---------------- | ---------------------------------------------------------------------------------------- |
| `/exit`          | Exit JayI.                                                                               |
| `/clear-screen`  | Clear the terminal screen. (this actually just moves your terminal focus)                |
| `/clear-history` | Clear the current session's conversation history while retaining the system instruction. |

`/clear-history` is particularly useful when you want to start a completely different topic without restarting the program or losing your configured conversational style!! or for no reason at all

for example, you can go from discussing Rust lifetimes (mine's cut short because of Rust's) to debating about.. FOOD??? (in fact, i AM hangry right now)

## Requirements

you'll need:

- Rust and Cargo
- Ollama installed and running
- a compatible language model available through Ollama
- the `llama3.2` model for the current default setup

make sure Ollama is running and the required model is available before starting JayI.

for the current model, you can use:

```bash
ollama pull llama3.2
```

Ollama handles the model inference, while JayI provides the terminal interface and application logic!!

## Installation and Building

clone the repository:

```bash
git clone https://github.com/jayywashere/jay-i
cd jay-i
```

build the project:

```bash
cargo build --release
# as always... '--release' is optional!!
```

you can also run the application directly through Cargo:

```bash
cargo run --release
```

if you've configured the `jayi` launcher, you can invoke it directly:

```bash
jayi
```

you can also pass custom system instructions when launching it:

```bash
jayi "respond casually and use lowercase"
```

yeah.. just don't forget to wrap it in quotes or it will pass just the first word as the system instruction.. (which is `"respond"`.. LIKE WDYM???)

the launcher is intended to make JayI easier to invoke without having to type the full Cargo command every time heheheh

## Tech Stack

JayI is built using:

- **Rust** — application logic, terminal interaction, and overall program structure
- **Ollama** — local language model inference
- **ollama-rs** — Rust-side communication with the Ollama API
- **llama3.2** — the current language model used for generating responses

the goal is to keep the application itself relatively lightweight while letting Ollama handle the actual language generationnnn

## Current Scope

JayI currently focuses on terminal-based conversations with a locally hosted language model.

it isn't intended to be a complete AI platform with a graphical interface, cloud-hosted infrastructure, or a whole collection of unrelated features.

instead, it's a small Rust application for experimenting with local AI, streaming responses, conversation context, system instructions, and the general experience of interacting with an AI directly from the command line.. SIGH.. more command-line applications.. SIGH... SIGHHHHHH......

THOUGH, there's plenty of room to experiment with how the application handles conversations and presents its responses, but the terminal remains the main interface!

## Why I Made This

YESYESYESYESYESYESYESYES i wanted to build a terminal AI chatbot in Rust and see how all the pieces fit together (i sound like some villain.. or not?? anyway.. PUZZLE)

working with a local language model is interesting on its own, but i also wanted to explore the application side of things: sending messages, maintaining conversation history, streaming generated text, and making the whole interaction feel natural. because.. I FINISHED TASTING CYBERSECURITY AND I GUESS I DIDN'T LIKE IT, FOR SURE. so, i went on to taste.. this.... too........ actually, not taste... not at all.... WHY??..

the scientific value of these experiments is questionable. seriously.

the entertainment value, however, is undeniable. seriously.

not seriously.
seriously. not seriously.
seriously.
not seriously. seriously.
oh!

## License

See [LICENSE](LICENSE).
