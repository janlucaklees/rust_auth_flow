# Simple Auth Workflow (Actix vs Rocket)

Here I built a simple login form and underlying authentication logic using Actix and Rocket to compare both against each
other.


## Actix

To start run:
- `cd actix`
- `cargo run`
- open [localhost:8080](http://localhost:8080/)


## Rocket

Rocket requires Rust nightly. To install on MacOS run:
- `brew remove rust` to remove your current rust installation. If you have installed rust in another way other than
  brew, this will probably not work.
- `brew install rustup-init` Install the rustup installer.
- `rustup-init` install rustup. Just use the recommended options.
- `cd rocket`
- `rustup override set nightly` to install and enable the nightly rust builds in the `rocket` directory.

To start run:
- (`cd rocket`)
- `cargo run`
- open [localhost:8000](http://localhost:8000/)


## The Setup

- When initially opening `/` one will be redirected to `/login`.
- At `/login` one can login with the user name `jlk` and any password.
- When logged in, one will be redirected to `/`.
- Trying to login again will just redirect the user back to `/`.
- One can logout at `/logout` and will be redirected to `/login`.
