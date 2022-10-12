use base64::alphabet::Alphabet;
use base64::engine::fast_portable::{FastPortable, FastPortableConfig};
use once_cell::sync::Lazy;

pub static B64_ALPHABET: Lazy<Alphabet> = Lazy::new(|| {
    // base64, like URL_SAFE, but '-' is replaced with '+' for better interaction
    // with shell programs, which don't like file names which start with '-'
    Alphabet::from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+_").unwrap()
});

pub static B64_ENGINE: Lazy<FastPortable> = Lazy::new(|| {
    FastPortable::from(
        &B64_ALPHABET,
        FastPortableConfig::new().with_encode_padding(false),
    )
});

pub use base64;
