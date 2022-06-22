# Holidays

A simple CLI tool which displays the next 5 public holidays for any given locale
Behind the scenes it uses the `date.nager.at` API to fetch the public holidays. Also it caches the loaded results into the users cache directory.

## Run

To compile and to run the project it is advisable to use the standard Rust toolings.
It can easily be tested with `cargo run -- -l <LOCALE>`

## Dependencies

The dependencies used are mainly because of simplicity and they make some of the needed functions very easy.
Such as `structopt`, `chrono`, `serde`, `directories` and `anyhow`

## Thoughts

- The passed argument is an option, therefore it uses the dashes to pass the argument
- It is possible to have the data cache file, but empty. this edge case is not handled propperly
- The cache is a simple file, because we want to have a simple CLI tool which should optimally run without an DB server behind the scenes.
- Error handling is not all too sophisticated since the project isn't supposed to be huge
- It would even nicer to have the counties written out and also to filter for those too like for the locale
- Tests are also sparce, because we do not have all too much business logic and most funtions/methods depend on external systems (which makes tests more complex)
- There is no sophisticated project structure, to keep it very small and simple. No need for a abstraction which could scale, because this is not the intention
- Only one commit because it was developed in one go locally
