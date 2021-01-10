# Ping

A cross-platform and blazingly fast [Matrix](https://matrix.org/) client focused on group and gaming chat.

**Status:** Very much a work in progress.

## Goals

[Discord](https://discord.com/) is a fantastic chat application for communities and people who play games together. [Element](https://element.io/) is an awesome open source, E2E encrypted, and decentralized Matrix client. I would like to take aspects of both of these, mix in a native look-and-feel, fast start times, low memory usage, and create a great client for groups and gaming chat with easy audio/video calls.

In addition to those functional feats, I'd like to build Ping in a way that makes it relatively easy to support multiple platforms with shared code. This may be a fools errand, but it sounds like an interesting challenge! In doing so, I expect some boundaries will be pushed resulting in fixes and features upstreamed, helping the community overall.

### Roadmap

Short Term (tasks that could be done in the foreseeable future)
- [ ] Basic Linux GTK based app
    - [ ] Create a user
    - [ ] Send a message to another user
    - [ ] Receive a message from another user
    - [ ] Edit a message
    - [ ] Delete a message
- [ ] Basic Windows App
    - [ ] Create a user
    - [ ] Send a message to another user
    - [ ] Receive a message from another user
    - [ ] Edit a message
    - [ ] Delete a message

Long Term (aspirational but objective setting things)
- Full fledged apps for
    - GTK Linux
    - QT Linux
    - Windows
    - MacOS
    - Android
    - iOS
    - Web App
- App support
    - User to user chat E2EE
    - Group chat E2EE
    - Communities with multiple channels
    - Custom emoji/reactions
    - Single click audio and/or video calls
    - Screen sharing and game broadcasting
- Other
    - A native platform look and feel
    - Snappy start and load times
    - Low memory usage
    - Automated CI/CD with extensive testing

## Development Getting Started

### Installing

[Install Rust](https://www.rust-lang.org/tools/install) and optionally [Docker](https://docs.docker.com/engine/install/) and [Docker Compose](https://docs.docker.com/compose/install/) or [Podman](https://podman.io/getting-started/installation.html) and [Podman Compose](https://github.com/containers/podman-compose) for running the test Synapse server.

### Compiling

To build:

```bash
cargo build
```

To format and lint:

```bash
cargo fmt
cargo clippy
```

### Running

To start the test Synpase server, use Docker Compose or Podman Compose:

```bash
docker-compose -f containers/dev.docker-compose.yml up

podman-compose -f containers/dev.docker-compose.yml up
```

To stop the test Synpase server, use Docker Compose or Podman Compose:

```bash
docker-compose -f containers/dev.docker-compose.yml down

podman-compose -f containers/dev.docker-compose.yml down
```

<!-- ### Testing -->

## Built With

- [Rust](https://www.rust-lang.org/)
- [matrix-rust-sdk](https://github.com/matrix-org/matrix-rust-sdk

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for how to work on the project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
