# WBUSTER

# Description ❤
A simple tool for brute forcing web directories! This tool is designed to be used in a penetration testing environment and is not intended for malicious use.
Only supports GET requests and single sweep at the moment.
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
git clone https://github.com/0bject-0bject/wbuster.git
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

This tool works with the dirbuster word lists (txt only), and any other word list that is formatted the same way. The tool will automatically remove any comments and blank lines from the word list. The tool will also automatically add a "/" to the end of the URL if it is not already there. <br>
This tool also works with json files that are formatted like this 
```javascript
{
    "directories": [
        "/path1",
        "/path2",
        "/path3"
    ]
}
```
<br>
CLI example: 

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
