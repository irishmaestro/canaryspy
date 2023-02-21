# canaryspy
![Crates.io](https://img.shields.io/crates/d/canaryspy?color=white&style=plastic)
### monitor vpn canary pages from the command line

## Installation
### via crates.io
```bash
cargo install canaryspy
```
### build from source
```bash
git clone https://github.com/IrishMaestro/canaryspy.git
cargo install --path canaryspy
```

## Usage
### List canaries
```bash
canary list
```
### Retrieve canary
```bash
canary pull <provider>
```
```bash
# Providers -> nordvpn, surfshark, protonvpn
canary pull nordvpn
```
----

*disclaimer: This is not a recommendation of any particular vpn provider or vpn's in general. It is simply an informational/opsec tool. As always, do your own research. The author assumes no liability for the methods in which this software is utilized.
