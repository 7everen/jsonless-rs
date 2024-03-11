# JsonLess <img src="https://raw.githubusercontent.com/7everen/jsonless-rs/main/icons/icon-32.png" alt="icon for jsonless compressor/obfuscator">

Is a Rust/C/C++ library that compresses and obfuscates JSON.

## Introduction
It was designed for performant compress large JSON which has duplicated data properties like in the example:
```json
{
 "player1": {
   "EfficiencyGameScore": 0,
   "FastBreakPointsAttempted": 0,
   "FastBreakPointsMade": 0
  },
  "player2": {
   "EfficiencyGameScore": 3,
   "FastBreakPointsAttempted": 2,
   "FastBreakPointsMade": 0
  },
   ...
}

```

Benefits:
- Can be achieved `2-10` times less size depending on data.
- More secure sending data, make it unreadable.
- Cost-efficient than compression.
- Support multiple languages:
  - [JS](https://github.com/7everen/jsonless-js)

  
## Encode

Example how to encode object to encoded object
```rust
use jsonless;

let object1 = json!({...});
let encoded = jsonless::encode(&object1);

// with options
let options = Options::new('$', None);
let encoded = jsonless::encode_with_options(&object1, options);

```
> **NOTE**
> No one property in JSON object should use character in `options.symbol`. By default, it is `$`.

Each encoded object consists `signature`. It is map of property names. It can be used as a key for decrypting encoded object.

Example how to remove signature from encoded object
```rust
use jsonless;

let encoded = json!([...]);
let encoded_wiothout_signature = jsonless::without_signature(&encoded);
```

Example how to get signature from encoded object
```rust
use jsonless;

let encoded = json!([...]);
let signature = jsonless::get_signature(&encoded);
```

## Decode

Example how to decode object
```rust
use jsonless;

let encoded = json!([...]);
let decoded = jsonless::decode(&encoded);

// with options
let signature = json!([...]);
let options = Options::new('$', Some(signature));
let decoded = jsonless::decode_with_options(&encoded, options);

```

## How to use in C/C++

[Integrating Rust into Existing C/C++ Projects](https://medium.com/@AlexanderObregon/integrating-rust-into-existing-c-c-projects-e0810dbddded)