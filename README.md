# cycle
An MVP of E2EE functions based on RSA algorithm.

The project includes two members:
* `core`: library exposes functions interacts with encrypt, decrypt data.
* `cli`: a CLI application that users can interact with `core`

## `Core`
### Compile
Use `cargo` tool to build the project.
* Debug mode:
```bash
> cargo build -p core
```
* Release mode:
```bash
> cargo build --release -p core
```

### Test
Currently there are few test cases, to run them
* Without `println` information inside testcase:
```bash
> cargo test -p core
```
* With `println` information inside testcase:
```bash
> cargo test -p core -- --nocapture
```
Result:
```bash
> cargo test -p core -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/core-55275d6dd5ef4fdc)

running 3 tests
g/XiuJ+Qcmg3qJwRrXMH7nTvMyTO89fc6lbIQ/Koze0=
test test_encrypt_decrypt ... ok
MCgCIQDXI/ZOz9+xCYOWhXiWNxAdzPbepuatmWqun0vL99y4zwIDAQAB
MIGqAgEAAiEA1yP2Ts/fsQmDloV4ljcQHcz23qbmrZlqrp9Ly/fcuM8CAwEAAQIgO/PalLXa+UPUdqK1Ku8nPxORJb2cbXi7MzBo5L5DkfECEQD8yB1rCqmsGfm7RRdbzmIZAhEA2eEqBHI9IT2IKqilgrjfJwIRAJ7Ttfg92OIz5//Mp6KdiHECEDp7VKLi4wOkF4OTxvFCnCsCEF/gabc+36RqPukMutnALVo=
test keypair::test_gen_keypair ... ok
-----BEGIN RSA PUBLIC KEY-----
MCgCIQDVcWqPkhwjIMDZwr03Og6PfrcRDN/Y+nk1yIWbvZFmMQIDAQAB
-----END RSA PUBLIC KEY-----

-----BEGIN RSA PRIVATE KEY-----
MIGrAgEAAiEA1XFqj5IcIyDA2cK9NzoOj363EQzf2Pp5NciFm72RZjECAwEAAQIg
CHHpVm7ggB9Oog8Lb4DzjHVjIaVTj59jomhFRgtRyHUCEQDy9Nwkv/xAcbz9V+hy
jWcLAhEA4ObuEvOjLG3GAgDyH7D9MwIQPnJ7hyKlprbVWdTXlWINAwIRAJLIhXdf
Bg9YMdRa86uVYxsCEQCuPitAMZfOSwUphgFPfyhj
-----END RSA PRIVATE KEY-----

test keypair::test_gen_keypair_to_pem ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

## `CLI`
### Compile
```bash
> cargo build --release -p cli
```

### Usage
```bash
> ./target/release/cli --help
Usage: cli <COMMAND>

Commands:
  key-gen      Generate key pair
  encrypt, -e  Encrypt message
  decrypt, -d  Decrypt message
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

#### 1. Generate keypair
```bash
> ./target/release/cli key-gen --help
Generate key pair

Usage: cli key-gen [OPTIONS]

Options:
  -p, --pem   Print .pem format
  -h, --help  Print help
```

#### 2. Encrypt message
```bash
> ./target/release/cli encrypt --help
Encrypt message

Usage: cli {encrypt|-e} --public-key <PUBLIC_KEY> --message <MESSAGE>

Options:
  -p, --public-key <PUBLIC_KEY>  Public key
  -m, --message <MESSAGE>        Message to encrypt
  -h, --help                     Print help
```

#### 3. Decrypt cipher
```bash
> ./target/release/cli decrypt --help
Decrypt message

Usage: cli {decrypt|-d} --private-key <PRIVATE_KEY> --cipher <CIPHER>

Options:
  -p, --private-key <PRIVATE_KEY>  Private key
  -c, --cipher <CIPHER>            Ciphertext to decrypt
  -h, --help                       Print help
```

### Example
#### 1. Generate keypair
```bash
> ./target/release/cli key-gen
Public key: MCgCIQDTTzxur09AtuIrhURsy7RjtGemwCBBBpS6e/DhrCWM8QIDAQAB

Private key: MIGrAgEAAiEA0088bq9PQLbiK4VEbMu0Y7RnpsAgQQaUunvw4awljPECAwEAAQIhALy/zk9gMP6xXZ1aUSCNZPb6wt+xU5MteD3Kuw5k93HxAhEA81mAjwWPuHtwvr66tuWOKwIRAN5LWBQ6SBEWj/4fqGsj31MCEBN7o/ZPbK1JfMJYxNwlztcCEQDQXET56MsmIUTkyN2/V0d7AhBlw1r9guSvZabOTsntPTC1
```

#### 2. Encrypt message
```bash
> ./target/release/cli encrypt -p MCgCIQDTTzxur09AtuIrhURsy7RjtGemwCBBBpS6e/DhrCWM8QIDAQAB -m "hello wolrd"
VhyZZrBgxnSouQWo5QHVlYZlnhMjs6cMbopLc9Nhn34=
```

#### 3. Decrypt message
```bash
> ./target/release/cli decrypt -p MIGrAgEAAiEA0088bq9PQLbiK4VEbMu0Y7RnpsAgQQaUunvw4awljPECAwEAAQIhALy/zk9gMP6xXZ1aUSCNZPb6wt+xU5MteD3Kuw5k93HxAhEA81mAjwWPuHtwvr66tuWOKwIRAN5LWBQ6SBEWj/4fqGsj31MCEBN7o/ZPbK1JfMJYxNwlztcCEQDQXET56MsmIUTkyN2/V0d7AhBlw1r9guSvZabOTsntPTC1 -c VhyZZrBgxnSouQWo5QHVlYZlnhMjs6cMbopLc9Nhn34=
hello wolrd
```