Steps to create a json parser Unix Command (json_parser)

- Compile the program
```bash 
cargo build --release
```

- Move the executable to a directory in your PATH
```bash
sudo cp ~/projects/json_parser/target/release/json_parser /usr/local/bin/
```

- Verify the installation
```bash
json_parser test.txt
```

- Optional: Create a symbolic link
```bash
sudo ln -s ~/projects/json_parser/target/release/json_parser /usr/local/bin/json_parser
```

- Check if it is working
```bash
which json_parser
```