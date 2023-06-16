# WBUSTER

# Description ❤
A simple tool for brute forcing web directories! This tool is designed to be used in a penetration testing environment and is not intended for malicious use.

Note: This is one of my first Rust projects, so the code is not the best. I am still learning the language and will continue to improve the code as I learn more. This is also a very early version of the tool, so there are still many features that I plan to add 😅

# Screenshots 📸

![Screenshot](https://github.com/0bject-0bject/wbuster/blob/main/screenshots/wbuster.png)

# Installation 📥

<details>

<summary> <strong> Cargo Install </strong> </summary>

```bash
cargo install wbuster
``` 

</details>

<details>

<summary> <strong> From Source </strong> </summary>

```bash
git clone https://github.com/0bject-0bject/wbuster
```

```bash
cd wbuster
```

```bash
cargo build --release
```

```bash
./target/release/wbuster
```

</details>

# Usage 🧰

```bash
wbuster --path PATH/TO/WORDLIST.TXT --url URL --threads 10 --timeout 10
```

# Arguments 📝

| Argument | Description | Example | Required | Default |
| :---: | --- | --- | ---: | ---: |
| --path | Path to wordlist | --path /usr/share/wordlists/dirb/common.txt | True | Null |
| --url | URL to brute force | --url http://example.com | True | Null |
| --threads | Number of threads to use | --threads 10 | False | 1 |
| --timeout | Timeout for each request (seconds)| --timeout 10 | False | 10 |

# Examples 📚

```bash
wbuster --path /usr/share/wordlists/dirb/common.txt --url http://example.com
```

```bash
wbuster --path /usr/share/wordlists/dirb/common.txt --url http://example.com --threads 10 --timeout 10
```

# License 📜

Licensed under the MIT license ([LICENSE](https://github.com/0bject-0bject/wbuster/blob/main/LICENSE))
