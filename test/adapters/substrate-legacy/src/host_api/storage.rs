use crate::host_api::utils::{Decoder, ParsedInput, Runtime};
use parity_scale_codec::Encode;

fn str<'a>(input: &'a [u8]) -> &'a str {
    std::str::from_utf8(input).unwrap()
}

// Input: key, value
pub fn test_set_get_storage(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let key = input.get(0);
    let value = input.get(1);

    // Get invalid key
    let res = rtm
        .call("rtm_ext_get_allocated_storage", &key.encode())
        .decode_vec();
    assert_eq!(res, [0u8; 0]);

    // Set key/value
    let _ = rtm.call("rtm_ext_set_storage", &(key, value).encode());

    // Get valid key
    let res = rtm
        .call("rtm_ext_get_allocated_storage", &key.encode())
        .decode_vec();
    assert_eq!(res, value);

    println!("{}", str(&res));
}

// Input: key, value, offset
pub fn test_set_get_storage_into(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let key = input.get(0);
    let value = input.get(1);
    let offset = std::str::from_utf8(input.get(2))
        .unwrap()
        .parse::<usize>()
        .unwrap();

    // Prepare for comparison, set the min required length
    let empty = if offset > value.len() {
        vec![0u8; 0]
    } else {
        vec![0; value.len() - offset]
    };

    // Invalid access
    let res = rtm
        .call(
            "rtm_ext_get_storage_into",
            &(key, &empty, offset as u32).encode(),
        )
        .decode_vec();
    assert_eq!(res, empty);

    // Set key/value
    let _ = rtm.call("rtm_ext_set_storage", &(key, value).encode());

    // Get key with offset
    let res = rtm
        .call(
            "rtm_ext_get_storage_into",
            &(key, &empty, offset as u32).encode(),
        )
        .decode_vec();
    if offset > value.len() {
        assert_eq!(*res.as_slice(), [0u8; 0]);
    } else {
        assert_eq!(*res.as_slice(), value[(offset as usize)..]);
    };

    println!("{}", str(&res));
}

// Input: key, value
pub fn test_exists_storage(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let key = input.get(0);
    let value = input.get(1);

    // Check invalid key
    let res = rtm
        .call("rtm_ext_exists_storage", &key.encode())
        .decode_u32();
    assert_eq!(res, 0);

    // Set key/value
    let _ = rtm.call("rtm_ext_set_storage", &(key, value).encode());

    // Check valid key
    let res = rtm
        .call("rtm_ext_exists_storage", &key.encode())
        .decode_u32();
    assert_eq!(res, 1);

    println!("true");
}

// Input: key, value
pub fn test_clear_storage(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let key = input.get(0);
    let value = input.get(1);

    // Set key/value
    let _ = rtm.call("rtm_ext_set_storage", &(key, value).encode());

    // Get valid key
    let res = rtm
        .call("rtm_ext_get_allocated_storage", &key.encode())
        .decode_vec();
    assert_eq!(res, value);

    // Clear key
    let _ = rtm.call("rtm_ext_clear_storage", &key.encode());

    // Get invalid key
    let res = rtm
        .call("rtm_ext_get_allocated_storage", &key.encode())
        .decode_vec();
    assert_eq!(res, [0u8; 0]);
}

// Input: prefix, key1, value1, key2, value2
pub fn test_clear_prefix(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let prefix = input.get(0);
    let key1 = input.get(1);
    let value1 = input.get(2);
    let key2 = input.get(3);
    let value2 = input.get(4);

    // Set keys/values
    let _ = rtm.call("rtm_ext_set_storage", &(key1, value1).encode());
    let _ = rtm.call("rtm_ext_set_storage", &(key2, value2).encode());

    // Clear keys with specified prefix
    let _ = rtm.call("rtm_ext_clear_prefix", &prefix.encode());

    // Check deletions
    let res = rtm
        .call("rtm_ext_get_allocated_storage", &key1.encode())
        .decode_vec();
    if key1.starts_with(prefix) {
        assert_eq!(res, [0u8; 0]);
        println!("Key `{}` was deleted", str(key1));
    } else {
        assert_eq!(res, value1);
        println!("Key `{}` remains", str(key1));
    }

    let res = rtm
        .call("rtm_ext_get_allocated_storage", &key2.encode())
        .decode_vec();
    if key2.starts_with(prefix) {
        assert_eq!(res, [0u8; 0]);
        println!("Key `{}` was deleted", str(key2));
    } else {
        assert_eq!(res, value2);
        println!("Key `{}` remains", str(key2));
    }
}

// Input: key1, value1, key2, value2
pub fn test_storage_root(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let key1 = input.get(0);
    let value1 = input.get(1);
    let key2 = input.get(2);
    let value2 = input.get(3);

    let _ = rtm.call("rtm_ext_set_storage", &(key1, value1).encode());
    let _ = rtm.call("rtm_ext_set_storage", &(key2, value2).encode());

    let root = rtm.call("rtm_ext_storage_root", &[]).decode_vec();

    println!("{}", hex::encode(root));
}

// Input: hash
pub fn test_storage_changes_root(input: ParsedInput) {
    let parent_hash_data = input.get(0);

    let mut rtm = Runtime::new();
    let root = rtm
        .call("rtm_ext_storage_changes_root", &parent_hash_data.encode())
        .decode_vec();

    println!("{}", hex::encode(root));
}

// Input: key, value
pub fn test_set_get_local_storage(input: ParsedInput) {
    let mut rtm = Runtime::new_offchain();

    let key = input.get(0);
    let value = input.get(1);

    // Test invalid persistant storage
    let res = rtm
        .call("rtm_ext_local_storage_get", &(1, key).encode())
        .decode_vec();
    assert_eq!(res, [0u8; 0]);

    // Test valid persistant storage
    let _ = rtm.call("rtm_ext_local_storage_set", &(1, key, value).encode());

    let res = rtm
        .call("rtm_ext_local_storage_get", &(1, key).encode())
        .decode_vec();
    assert_eq!(res.as_slice(), value);

    print!("{},", str(&res)); // Result of persistant storage

    // Test invalid local storage
    let res = rtm
        .call("rtm_ext_local_storage_get", &(2, key).encode())
        .decode_vec();
    assert_eq!(res, [0u8; 0]);

    // Test valid local storage
    let _ = rtm.call("rtm_ext_local_storage_set", &(2, key, value).encode());
    let res = rtm
        .call("rtm_ext_local_storage_get", &(2, key).encode())
        .decode_vec();

    assert_eq!(res.as_slice(), value);

    println!("{}", str(&res)); // Result of local storage

    // Invalid cross access
    // -> make sure keys set in persistant storage cannot be access by local storage (and vice-versa)
    let key1 = "somekey1".as_bytes();
    let value1 = "somevalue1".as_bytes();

    let key2 = "somekey2".as_bytes();
    let value2 = "somevalue2".as_bytes();

    let _ = rtm.call("rtm_ext_local_storage_set", &(1, key1, value1).encode());
    let _ = rtm.call("rtm_ext_local_storage_set", &(2, key2, value2).encode());

    let res = rtm
        .call("rtm_ext_local_storage_get", &(1, key2).encode())
        .decode_vec();
    assert_eq!(res, [0u8; 0]);

    let res = rtm
        .call("rtm_ext_local_storage_get", &(2, key1).encode())
        .decode_vec();
    assert_eq!(res, [0u8; 0]);
}

// Input: key, old_value, new_value
pub fn test_local_storage_compare_and_set(input: ParsedInput) {
    let mut rtm = Runtime::new_offchain();

    let key = input.get(0);
    let old_value = input.get(1);
    let new_value = input.get(2);

    // Test invalid key
    let res = rtm
        .call(
            "rtm_ext_local_storage_compare_and_set",
            &(1, key, old_value, new_value).encode(),
        )
        .decode_u32();

    assert_eq!(res, 1);

    let _ = rtm.call("rtm_ext_local_storage_set", &(1, key, old_value).encode());

    // Test invalid value
    let res = rtm
        .call(
            "rtm_ext_local_storage_compare_and_set",
            &(1, key, new_value, new_value).encode(),
        )
        .decode_u32();

    assert_eq!(res, 1);

    // Test valid value
    let res = rtm
        .call(
            "rtm_ext_local_storage_compare_and_set",
            &(1, key, old_value, new_value).encode(),
        )
        .decode_u32();

    assert_eq!(res, 0);

    let res = rtm
        .call("rtm_ext_local_storage_get", &(1, key).encode())
        .decode_vec();
    assert_eq!(res, new_value);

    println!("{}", str(new_value));

    // Invalid cross access
    // -> make sure keys set in persistant storage cannot be access by local storage (and vice-versa)
    let key1 = "somekey1".as_bytes();
    let value1 = "somevalue1".as_bytes();

    let key2 = "somekey2".as_bytes();
    let value2 = "somevalue2".as_bytes();

    let _ = rtm.call("rtm_ext_local_storage_set", &(1, key1, value1).encode());
    let _ = rtm.call("rtm_ext_local_storage_set", &(2, key2, value2).encode());

    let res = rtm
        .call(
            "rtm_ext_local_storage_compare_and_set",
            &(1, key2, value1, new_value).encode(),
        )
        .decode_u32();

    assert_eq!(res, 1);

    let res = rtm
        .call(
            "rtm_ext_local_storage_compare_and_set",
            &(2, key1, value2, new_value).encode(),
        )
        .decode_u32();

    assert_eq!(res, 1);
}
