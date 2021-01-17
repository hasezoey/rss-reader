# Rust RSS-Feed-Reader

Note: this project is currently only aimed at *nix systems (specifically linux)
Note: pidfile and anti-duplication will be handled by the "parent process" (like systemd or an docker-container)

## Motivation

I started creating this project because i couldnt find any rss reader that was capable of running as a service (on an 24/7 server) and be retrieved on an client, like transmission daemon & remote and i was an user of Lifera

## Goals

- [ ] service/daemon/server
- [ ] client
  - [ ] cli-client
  - [ ] graphical client (gtk?)
- [ ] both together

## Design thoughts

- make client-cli interactive (layout like Lifera, but in terminal form)
- fetch feeds in seperate threads
- create log file in all locations that are modifyable

## TODO

- setup clippy config, if there should be ever one
