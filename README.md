![](public/logo.png)

# WLRS
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FZusier%2Fwlrs.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FZusier%2Fwlrs?ref=badge_shield)


WLRS (_[/julərs/](http://ipa-reader.xyz/?text=jul%C9%99rs)_) is a WIP workout tracker.

# Installation

## Docker

1. Clone the repository: `git clone https://github.com/zusier/wlrs.git`
2. Configure the `docker-compose.yml`
3. Build and launch!: `docker compose up --build -d`

## TODO

- [x] JSON for data retrieval
- [x] Learn Svelte
- [x] Workout View
- [x] Status Codes
- [x] Workout create
  - [x] When adding exercises form a json, render off the json. (Editing/deleting exerices will be possible then)
- [x] Front page
- [x] Login/Auth
  - [x] Register Account
  - [x] Login
    - [ ] Remember Login?
    - [ ] Add validation (on frontend, backend has validation)
  - [ ] Delete
- [x] User Home Page
  - [x] Summaries of workouts
    - [x] Add "New Workout" panel for users with less than 3 workouts
    - [ ] View All page
  - [ ] Progression statistics/graphcs
  - [ ] Body weight graph?
- [ ] Account Page
  - [ ] Global unit preference
- [ ] Add Support for other DB types (diesel?) such as MariaDB, Postgres, MySQL
- [x] Extend error handling with global error codes and don't handle `Result<>`s with `unwrap()`s
- [ ] Organize api
- [ ] [Ratelimiting](https://lib.rs/crates/rocket-governor)
- [x] Compression
  - [x] Decrease latency/prcoessing times, by pre-compressing assets ( On The Fly compression adds +/-16ms of response time!!!)
- [x] Docker!

### Ideas for the future

- [ ] GPX Support
- [ ] Exercise lists, with muscle group visualization (like wger)
- [ ] Native mobile applications (Svelte-native?)


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FZusier%2Fwlrs.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FZusier%2Fwlrs?ref=badge_large)