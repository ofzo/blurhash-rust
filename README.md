# BlurHash encoder in portable Rust

This code implements an encoder for the BlurHash algorithm in Rust.

# encode 
```Rust
use image::{open, GenericImageView};
use std::path::Path;                                                              ```
use blurhash_rs::encode;
                                                                                  
let img = open(&Path::new("../Swift/BlurHashTest/pic1.png")).expect("not found");
let width = img.dimensions().0;
let height = img.dimensions().1;
let mut data = Vec::with_capacity((width * height) as usize);
                                                                                  
for p in img.pixels() {
    let p2 = p.2;
    data.push(p2.0[0]);
    data.push(p2.0[1]);
    data.push(p2.0[2]);
    data.push(p2.0[3]);
}
let v = encode(&data, width, height, 4, 3);
                                                                                  
assert_eq!(v.unwrap(), "LaJHjmVu8_~po#smR+a~xaoLWCRj");
```


# decode
```Rust
use blurhash_rs;
                                                          
let hash = "LEHV6nWB2yk8pyo0adR*.7kCMdnj";
let v = blurhash_rs::decode(hash, 32, 32, None).unwrap();

assert_eq!(v.len(), 4096);
```
