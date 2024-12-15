# Unix Sort Tool Clone

This is a clone of unix `sort` tool

The tool is used through the command line with the command `ccsort`

## Steps to Create and Use the Tool

### 1. Compile the Program

First, build the program by running the following command:

```bash
cargo build --release
```

### 2. Move the executable to a directory in your PATH

To make the tool accessible from anywhere in your terminal, move the compiled executables to a directory that is included in your system's PATH.

```bash
sudo cp target/release/ccsort /usr/local/bin/
```

### 3. Verify the installation

To verify the installation, check if the executable is accessible from anywhere in your terminal:

```bash
which ccsort
```


### 4. Usage

```bash
ccsort [options] <filename>
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributions

Contributions are welcome! Feel free to open issues or submit pull requests to improve the tool. To contribute:

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Submit a pull request.

## Credits

Many thanks to [John Cricket](https://github.com/JohnCrickett) for his [Coding Challenges](https://codingchallenges.fyi/challenges/challenge-load-balancer), which inspired this solution.
