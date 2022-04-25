# Trunk - Seed template

A `cargo-generate` template for setting up a [Trunk](https://github.com/thedodd/trunk) + [Seed](https://seed-rs.org/) project.

## Usage

If you haven't installed `cargo generate` run `cargo install cargo-generate`. If
you don't have OpenSSL installed on the system run
`cargo install cargo-generate --features vendored-openssl`.

1. Install Trunk: `cargo install --locked trunk` (see the
[instructions](https://github.com/thedodd/trunk#install) for more details)
2. Run `cargo generate --git https://github.com/cfsamson/templ-trunk-seed -n "my-project-name"

To serve run `trunk serve --open`