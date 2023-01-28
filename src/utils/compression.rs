use lz4_flex::decompress_size_prepended;
use ooz_sys::Kraken_Decompress;

pub fn decompress_oodle(
    compressed_data: &[u8],
    compressed_len: usize,
    decompressed_data: &mut [u8],
    decompressed_len: usize,
) {
    unsafe {
        Kraken_Decompress(
            compressed_data.as_ptr(),
            compressed_len,
            decompressed_data.as_mut_ptr(),
            decompressed_len,
        );
    }
}

pub fn decompress_lz(
    compressed_data: &[u8],
    compressed_len: usize,
    decompressed_data: &mut [u8],
    decompressed_len: usize,
) {
    decompressed_data[..decompressed_len]
        .copy_from_slice(&decompress_size_prepended(&compressed_data[..compressed_len]).unwrap());
}
