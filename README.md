TCP server should be protected from DDOS attacks with the Prof of Work (https://en.wikipedia.org/wiki/Proof_of_work), the challenge-response protocol should be used.
After Prof Of Work verification, server should send one of the quotes from “word of wisdom” book or any other collection of the quotes.

Target should be built in nightly channel
$ rustup default nightly
Build
$ cargo build
Run server
$ ./target/debug/word_of_wisdom
Test may be run in another tab using
$ cargo test

Server is configured to listen 0:0:0:0:5555
