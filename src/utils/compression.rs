use lz4_flex::decompress_size_prepended;
use oozle::decompress;

pub fn decompress_oodle(
    compressed_data: &[u8],
    compressed_len: usize,
    decompressed_data: &mut [u8],
    decompressed_len: usize,
) {
    unsafe {
        decompress(
            &compressed_data[..compressed_len],
            &mut decompressed_data[..decompressed_len],
        )
        .unwrap();
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
