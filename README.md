# seedphraser
Read and writes BIP39 seed phrases. This also supports an extended version on top of the 128-bit, 160-bit, 192-bit, 224-bit and 256-bit forms. There is also long sequence support.

## Installing
```bash
$ cargo install seedphraser
$ seedphraser --help
```
Alternatively, you may download a release from the release tab on GitHub. For example on Linux,
```bash
$ wget https://github.com/DiscordJim/seedphraser/releases/download/release/x86_64-unknown-linux-gnu
$ mv x86_64-unknown-linux-gnu seedphraser && chmod +x seedphraser
```

## Long Sequences
A long sequence is a sequence that exceeds 256-bit. It will be padded with 256-bit chunks for simplicity. If here is an amount that is not divisible by 256-bits, then padding will be used and a ` @ <trim>` will be appended onto the end which tells the program how many bytes to trim off the end. For instance consider,
```
moon topic gas diary boss siege among violin lumber expose trade obey @14
```
We first convert this into bytes and then can discard the last fourteen bytes. 

This means that the tool can work with odd sequence lengths such as 2072.


## Examples
Generate a new seed phrase of 256 bits and store it in a file named `example.txt`:
```bash
$ seedphraser generate -b 256 > example.txt
```
Read that file out and convert the output to base-64:
```bash
$ cat example.txt | seedphraser -i txt -o b64 > example.b64
```

