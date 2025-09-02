# werd

a simple thesaurus command line interface utilizing [WordsApi](https://www.wordsapi.com/)

## Install

0. Requirements
  - rustc (cargo)
  - a [WordsApi API key](https://rapidapi.com/dpventures/api/wordsapi/pricing) (via rapidapi)

1. Clone the [repo](https://github.com/laffed/werd)

```
git clone https://github.com/laffed/werd.git
```

2. Build and install binary

```
cd werd
cargo install --path .
```

3. Run setup command & enter your WordsApi API key

- will create a `$HOME/.werd.toml` config file containing your API key
```
werd setup
```

## Basic operations
