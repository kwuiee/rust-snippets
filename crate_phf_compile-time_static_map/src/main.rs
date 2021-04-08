use phf::{phf_map, phf_set};

static MY_MAP: phf::Map<&'static str, u32> = phf_map! {
    "hello" => 1,
    "world" => 2,
};

static MY_SET: phf::Set<&'static str> = phf_set! {
    "hello world",
    "hola mundo",
};

fn main() {
    assert_eq!(MY_MAP["hello"], 1);
    assert!(MY_SET.contains("hello world"));
}
