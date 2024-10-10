# Download Temporary File (DTF)

A simple Rust command-line application that downloads a file from a given URL,
saves it to a temporary directory, and prints the file path
The program exits with an error code if any step fails.

Really useful for using in long commands, like:

```shell
cat (dtf http://example.com/test.txt) | grep 'foo'
```

### Installation

```shell
git clone https://github.com/cwill747/dtf
cd dtf
```

### Build

```shell
cargo build --release
```

## Usage

Once you've built the binary, you can use it with:

```shell
./target/release/dtf <URL>
```

For example:

```shell
./target/release/dtf https://example.com/file.txt
```

## Error Handling

* If the URL is missing or malformed, the program will print an error message and exit with status code 1.
* If the file download fails (e.g., due to network issues or a non-200 HTTP status), the program will print the corresponding error message and exit with status code 1.
* If the temporary directory or file creation fails, the program will print an error message and exit with status code 1.

## Developing

Lint code:

```shell
cargo clippy
```

## License
This project is licensed under the MIT License. See the LICENSE file for more details.