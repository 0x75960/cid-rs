cid-rs
=======

Content identifier

usage
-----

1. add dependency into Cargo.toml

```toml
[dependencies]
cid = { git = "https://github.com/0x75960/cid-rs", branch = "master" }
```

2. import and use in your code

```rust
extern crate cid;

use cid::ContentIdentifier;

fn main() {
	let cid = ContentIdentifier::of_file("/path/to/file").unwrap();
	println!("SHA256: {}", cid.sha256);
	println!("SHA1:   {}", cid.sha1);
	println!("MD5:    {}", cid.md5);
}
```

### test

```sh
$ cargo test
```
