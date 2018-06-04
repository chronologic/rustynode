use web3::types::Bytes;

pub fn split_n_chars(s: &str, n: usize) -> Vec<&str> {
    let mut result = vec![];

    let mut i = 0;
    while i < s.len() {
        result.push(&s[i..i+n]);
        i += n;
    }

    result
}

use ethabi::token::{Token, Tokenizer, LenientTokenizer};

pub fn decode_raw_data(b: &[u8]) {
    
    let param_types: Vec<ethabi::ParamType> = vec![
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Address,
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Address,
        ethabi::ParamType::Bytes,
        ethabi::ParamType::Bytes,
    ];

    let decoded = ethabi::decode(
        &param_types,
        b,
    );

    println!("{:?}", decoded);
}