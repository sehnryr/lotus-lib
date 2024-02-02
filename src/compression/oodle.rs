use anyhow::{Error, Result};
use oodle_sys::{
    OodleLZ_CheckCRC_OodleLZ_CheckCRC_No, OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhaseAll,
    OodleLZ_Decompress, OodleLZ_FuzzSafe_OodleLZ_FuzzSafe_Yes,
    OodleLZ_Verbosity_OodleLZ_Verbosity_None,
};

pub fn decompress_oodle(
    compressed_data: &[u8],
    compressed_len: usize,
    decompressed_data: &mut [u8],
    decompressed_len: usize,
) -> Result<()> {
    let input = &compressed_data[..compressed_len];
    let output = &mut decompressed_data[..decompressed_len];

    let n = unsafe {
        OodleLZ_Decompress(
            input.as_ptr() as *const _,
            input.len() as isize,
            output.as_mut_ptr() as *mut _,
            output.len() as isize,
            OodleLZ_FuzzSafe_OodleLZ_FuzzSafe_Yes,
            OodleLZ_CheckCRC_OodleLZ_CheckCRC_No,
            OodleLZ_Verbosity_OodleLZ_Verbosity_None,
            std::ptr::null_mut(),
            0,
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhaseAll,
        )
    };

    if n < 0 {
        Err(Error::msg("Failed to decompress oodle data"))
    } else {
        Ok(())
    }
}
