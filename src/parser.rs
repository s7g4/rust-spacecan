pub fn decode_frame(data: &[u8]) -> Result<(), String> {
    // Stub implementation of decode_frame
    println!("Decoding frame: {:?}", data);
    Ok(())
}

pub fn encode_frame(frame: &impl std::fmt::Debug) -> Result<Vec<u8>, String> {
    // Stub implementation of encode_frame
    println!("Encoding frame: {:?}", frame);
    Ok(vec![])
}
