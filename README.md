# `ghdl`

A tool for **g**it**h**ub **d**own**l**oad.

## Motivation

I'm sure something else like this exists.  I have found some "working" ones
around the internet.  But many of them seemed to either be outdated or rate-
limited.  This should hopefully provide a better alternative.

## Usage

**Highly** recommended to generate an API key to use with this.  Either pass it
as `--api-key` or set an environment variable `GHDL_API_KEY`.

[Click here](https://github.com/settings/tokens)

```
ghdl 0.1.0
Tool to pull down specific paths from a github repo

USAGE:
    ghdl [OPTIONS] <repo path> --branch <branch> --repo <repo> [output dir]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --api-key <api-key>    Optional* GitHub API key (you will get rate-limited if you don't specify one)
    -b, --branch <branch>      Branch to pluck folder from
    -r, --repo <repo>          GitHub repo to pull from

ARGS:
    <repo path>     Path to file or folder to pull down
    <output dir>    Location to save to, defaulting to $PWD
```

## Examples

```
$ ghdl -r rawhat/phoenix_react --api-key ... -b master assets my_asset_folder
...
$ ls my_asset_folder
package.json package-lock.json ...

$ ghdl -r ryanoasis/nerd-fonts --api-key ... -b master patched-fonts/Iosevka iosevka-nerd-font
Downloading ...
$ ls iosekva-nerd-font
Bold Italic ...
```
