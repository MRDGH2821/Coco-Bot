# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v0.4.0 - 2025-06-06
#### Continuous Integration
- add tmate step to help debug workflows - (4b436a9) - MRDGH2821
#### Features
- **(meme)** :sparkles: add new meme template image - (3b11ee3) - MRDGH2821

- - -

## v0.3.0 - 2025-06-06
#### Continuous Integration
- :adhesive_bandage: fix double quotes & changelog link - (f1e7ac1) - MRDGH2821
#### Features
- **(command)** :sparkles: add xkcd command - (a55aedb) - MRDGH2821
#### Style
- :art: remove blank line - (2ea3d87) - MRDGH2821

- - -

## v0.2.1 - 2025-06-06
#### Continuous Integration
- :green_heart: use run time env fetching instead compile time - (c008940) - MRDGH2821
- :bug: remove redundant quote - (ef0aa55) - MRDGH2821

- - -

## v0.2.0 - 2025-06-06
#### Build system
- **(deps)** update cargo lock file - (4c95bd2) - MRDGH2821
#### Continuous Integration
- :construction_worker: remove Mac OS targets - (fe1eee4) - MRDGH2821
- :green_heart: use alternate way to cross compile & generate release notes - (73e3816) - MRDGH2821
#### Features
- **(command)** :memo: add descriptions & remove prefix commands - (f8cdf27) - MRDGH2821
- **(command)** :sparkles: add new command about - (6af9cca) - MRDGH2821

- - -

## v0.1.5 - 2025-06-05
#### Build system
- :construction_worker: remove musl targets - (8556627) - MRDGH2821
#### Miscellaneous Chores
- **(devcontainer)** :sparkles: add trunk feature - (63d1815) - MRDGH2821

- - -

## v0.1.4 - 2025-06-05
#### Continuous Integration
- :green_heart: skip package tag for musl builds - (3c86ffd) - MRDGH2821

- - -

## v0.1.3 - 2025-06-05
#### Build system
- **(deps)** :package: remove openssl vendored package - (fd9a251) - MRDGH2821
- :wrench: add cross-rs config to help failing builds - (2adce85) - MRDGH2821
- :green_heart: use alternate way to build container images - (069cd78) - MRDGH2821
#### Continuous Integration
- **(megalinter)** :see_no_evil: ignore git ignored files during linting - (46b638d) - MRDGH2821
- **(megalinter)** :package: add perl package - (2efc14e) - MRDGH2821
- :truck: rename ci job name - (1c34052) - MRDGH2821
#### Miscellaneous Chores
- **(cspell)** :wrench: add words - (ae25d36) - MRDGH2821

- - -

## v0.1.2 - 2025-06-05
#### Build system
- **(deps)** :green_heart: add vendored openssl to fix failing builds - (c470e6f) - MRDGH2821
#### Miscellaneous Chores
- :see_no_evil: add markdownlint ignore file - (50d2bed) - MRDGH2821
- :rotating_light: fix & ignore linter errors - (31eb789) - MRDGH2821

- - -

## v0.1.1 - 2025-06-05
#### Build system
- :green_heart: fix failing ci docker builds - (e8acc4f) - MRDGH2821
#### Continuous Integration
- :green_heart: use different checksum command depending on os - (63b7494) - MRDGH2821
- :bug: install openssl packages for linux builders - (38521fa) - MRDGH2821
- :construction_worker: package non-windows builds with `.bin` extension - (dd5a827) - MRDGH2821
#### Miscellaneous Chores
- **(vscode)** :wrench: sync tags on git pull & push - (b15e3e2) - MRDGH2821
- :see_no_evil: update exclude list - (90b20d0) - MRDGH2821

- - -

## v0.1.0 - 2025-06-05
#### Bug Fixes
- **(command)** :rotating_light: fix return type - (544e7c0) - MRDGH2821
- **(event)** :bug: use correct syntax to handle events - (be04aa3) - MRDGH2821
- :bug: wrap generated text in the meme - (9682cb4) - MRDGH2821
#### Build system
- **(deps)** :package: switch to git version of serenity & poise - (291cfa6) - MRDGH2821
- **(deps)** :package: downgrade to stable release - (135e95c) - MRDGH2821
- **(deps)** :package: add dependencies for new commands - (3f6fdb5) - MRDGH2821
- **(deps)** :package: update packages - (261f5dd) - MRDGH2821
- **(deps)** :package: use git versions of dependencies - (81f88c8) - MRDGH2821
- **(deps)** :heavy_plus_sign: add dotenv dependency - (fdb1723) - MRDGH2821
- **(deps)** :package: change poise version to wildcard - (0c9eb14) - MRDGH2821
- **(deps)** :package: add serenity & poise package - (31f4153) - MRDGH2821
- :adhesive_bandage: use CI provided env vars to apply version label - (72d79ee) - MRDGH2821
#### Continuous Integration
- **(megalinter)** :wrench: add pre commands for rust clippy linter - (aff3e4c) - MRDGH2821
- :construction_worker: add builds generator - (3171b57) - MRDGH2821
- :construction_worker: auto handle latest tag - (3477ebf) - MRDGH2821
- :wrench: add top level permissions - (d543a62) - MRDGH2821
- :rotating_light: add mega linter config - (6ac06e5) - MRDGH2821
#### Documentation
- :construction: add instructions - (9e490c6) - MRDGH2821
#### Features
- **(command)** :recycle: send meme as embed - (62cd357) - MRDGH2821
- **(command)** :sparkles: add autocomplete for template - (42483dc) - MRDGH2821
- **(command)** :sparkles: add meme generator logic - (2ffd313) - MRDGH2821
- **(command)** :sparkles: add new commands - (74b9875) - MRDGH2821
- **(devcontainer)** :wrench: add dev container config - (17b5f43) - MRDGH2821
- **(event)** :loud_sound: update logging statements & use tracing library - (28b3bf5) - MRDGH2821
- **(event)** :sparkles: add events - (f06ca6d) - MRDGH2821
- :construction_worker: add dockerfile & ci image builder - (43f039e) - MRDGH2821
- add ping command - (6125ce3) - MRDGH2821
- send an informative embed - (e716736) - MRDGH2821
- read env file - (9b461d4) - MRDGH2821
- add poise starter code - (94aaf67) - MRDGH2821
#### Miscellaneous Chores
- **(cocogitto)** :adhesive_bandage: fix cargo version command - (402d8e5) - MRDGH2821
- **(cspell)** :wrench: update words - (f52cd25) - MRDGH2821
- **(cspell)** :wrench: update word list - (c7a7c6b) - MRDGH2821
- **(cspell)** :wrench: add words - (df78343) - MRDGH2821
- **(cspell)** :wrench: add ignore paths & words - (0661172) - MRDGH2821
- **(cspell)** :wrench: add cspell config - (43166c9) - MRDGH2821
- **(devcontainer)** :package: add cargo-edit package - (25ff30d) - MRDGH2821
- **(devcontainer)** :sparkles: add hadolint feature & extension suggestion - (2848e81) - MRDGH2821
- **(devcontainer)** :truck: run cocogitto bootstrap only on post create, not post attach - (bac78a8) - MRDGH2821
- **(devcontainer)** :sparkles: add more features & sort keys - (9ac5a44) - MRDGH2821
- **(devcontainer)** :wrench: use docker-in-docker - (2895dcf) - MRDGH2821
- **(jscpd)** :rotating_light: ignore code block - (dfdc076) - MRDGH2821
- **(megalinter)** :wrench: remove apt command - (1be0837) - MRDGH2821
- **(vscode)** :wrench: enable rust analyzer style hints - (54b289b) - MRDGH2821
- **(vscode)** :wrench: update vscode config - (37fbec3) - MRDGH2821
- **(vscode)** :wrench: update extensions list - (48c4fe7) - MRDGH2821
- **(vscode)** :package: update extension recommendations - (d4e4799) - MRDGH2821
- **(vscode)** :hammer: add megalinter runner task - (0c37548) - MRDGH2821
- **(vscode)** :technologist: add extension recommendations - (d23ca49) - MRDGH2821
- **(vscode)** add cargo clean task - (498577a) - MRDGH2821
- **(vscode)** add new commit scope - (2800cf1) - MRDGH2821
- **(vscode)** :wrench: add tasks - (221b037) - MRDGH2821
- **(vscode)** :wrench: add commit scope - (dfe2184) - MRDGH2821
- **(vscode)** :wrench: update commit scopes - (23e5c5b) - MRDGH2821
- **(vscode)** :wrench: add vscode config files - (f9d285b) - MRDGH2821
- :pencil2: use en-GB spelling - (a900a39) - MRDGH2821
- :wrench: update cocogitto config - (1976acc) - MRDGH2821
- change version to 0.0.0 - (f5397c7) - MRDGH2821
- :adhesive_bandage: overwrite all git hooks whe cocogitto installs it - (ffc3f08) - MRDGH2821
- :wrench: add cocogitto related configs - (4e16072) - MRDGH2821
- :see_no_evil: ignore megalinter reports folder - (5a93869) - MRDGH2821
- :see_no_evil: add v8r ignore list - (230f155) - MRDGH2821
- :wrench: remove unused manifest key - (c8d6874) - MRDGH2821
- :bulb: remove comments - (a18edba) - MRDGH2821
- :wrench: add dependabot config - (6987eac) - MRDGH2821
- :hammer: add cargo scripts - (e882f03) - MRDGH2821
- :loud_sound: add log statements - (dce9843) - MRDGH2821
- add cargo metadata - (bd6e1e5) - MRDGH2821
- :memo: remove outdated comment - (3282a92) - MRDGH2821
- :see_no_evil: ignore .env file - (ee5d54a) - MRDGH2821
- initial commit - (2ff3439) - MRDGH2821
#### Refactoring
- **(command)** :recycle: directly register global commands - (8def06f) - MRDGH2821
- **(event)** :recycle: use corrent syntax for git version of libraries - (2996c76) - MRDGH2821
- :recycle: use an early return - (6b36d54) - MRDGH2821
- :truck: rename file - (b1992cd) - MRDGH2821
- :recycle: use std lib to get env variable - (5ba310f) - MRDGH2821
- :recycle: use correct syntax for next version of libraries - (08ce6df) - MRDGH2821
- :truck: rename command - (eda5311) - MRDGH2821
- :recycle: split commands into a sub directory - (ddf6fed) - MRDGH2821
#### Style
- :art: format files - (8f5005b) - MRDGH2821
- :art: format using prettier - (bb8a96e) - MRDGH2821

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).