# rust-anonfiles-api
![build](https://img.shields.io/badge/build-success-green)

A simple project for uploading files to anonfiles.com


## Usage Example

```cpp
use rust_anonfiles_api;

fn main() {
    println!(
        "Link to download - {}",
        rust_anonfiles_api::upload_file("PATH_TO_FILE").unwrap()
    );
}

```


## Build Locally

Clone the project

```bash
  git clone https://github.com/dumitory-dev/rust-anonfiles-api.git
```

Go to the project directory

```bash
  cd rust-anonfiles-api
```

Build the project

```bash
  cargo build
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)

