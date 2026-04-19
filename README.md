# idc: A simple crate for error propagation  
Idc is a simple crate for propagating errors that implement std’s Error trait.  
Idc also supports no_std with the same functionality, but you have to provide a global allocator and disable default feature  
  
## examples:  
1. propagating multiple different errors:  
  
```rust
use std::fs;
use idc::*;
use serde_json::Value;

fn main() -> Result<()> {
    let foo = fs::read_to_string("foo.json").context("failed to read foo.", Some("maybe it doesn't exist?"))?;
    let json: Value = serde_json::from_str(&foo).context("failed to turn foo into json", Some("make sure foo.json is valid json."))?;
    println!("{}", json["important item"]);
    Ok(())
}

```
  
2. returning an one-time error:  
  
```rust
use std::env;
use idc::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::Args.collect();
    if args.len() < 2 {
        bail!("no argument provided!");
    }
    //...
    Ok(())
}
```
