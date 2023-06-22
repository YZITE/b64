use base64::alphabet::Alphabet;
use base64::engine::general_purpose::{GeneralPurpose, GeneralPurposeConfig};
use base64::engine::DecodePaddingMode;
use once_cell::sync::Lazy;

pub static B64_ALPHABET: Lazy<Alphabet> = Lazy::new(|| {
    // base64, like URL_SAFE, but '-' is replaced with '+' for better interaction
    // with shell programs, which don't like file names which start with '-'
    Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+_").unwrap()
});

pub static B64_ENGINE: Lazy<GeneralPurpose> = Lazy::new(|| {
    GeneralPurpose::new(
        &B64_ALPHABET,
        GeneralPurposeConfig::new()
            .with_encode_padding(false)
            .with_decode_padding_mode(DecodePaddingMode::RequireNone),
    )
});

pub use base64;
