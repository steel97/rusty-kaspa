extern crate self as kaspa_bindings;

// enums
const OP_OK: i32 = 0;
const E_CHANGEADDRESS_DECODE: i32 = -1;
const E_UTXOLIST_DECODE: i32 = -2;
const E_OUTPUTS_DECODE: i32 = -3;
const E_PAYLOAD_DECODE: i32 = -4;

fn u8_array_to_str(data: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(data)
}

#[no_mangle]
pub extern "C" fn build_transaction(
    change_address_data: *const u8,
    change_address_len: usize,
    utxo_entry_source_data: *const u8,
    utxo_entry_source_len: usize,
    outputs_data: *const u8,
    outputs_len: usize,
    priority_fee: u64,
    payload_data: *const u8,
    payload_len: usize,
    sig_op_count: u8,
    minimum_signatures: u16,
) -> i32 {
    let change_address = unsafe { std::slice::from_raw_parts(change_address_data, change_address_len) };
    let utxo_entry_source = unsafe { std::slice::from_raw_parts(utxo_entry_source_data, utxo_entry_source_len) };
    let outputs = unsafe { std::slice::from_raw_parts(outputs_data, outputs_len) };
    let payload = unsafe { std::slice::from_raw_parts(payload_data, payload_len) };

    // params start
    let change_address = match u8_array_to_str(&change_address) {
        Ok(s) => s,
        Err(e) => {
            return E_CHANGEADDRESS_DECODE;
        }
    };
    println!("={}=", change_address);
    let utxo_entry_source = match u8_array_to_str(&utxo_entry_source) {
        Ok(s) => s,
        Err(e) => {
            return E_UTXOLIST_DECODE;
        }
    };
    println!("={}=", utxo_entry_source);
    let outputs = match u8_array_to_str(&outputs) {
        Ok(s) => s,
        Err(e) => {
            return E_OUTPUTS_DECODE;
        }
    };
    println!("={}=", outputs);
    println!("={}=", priority_fee);
    let payload = match u8_array_to_str(&payload) {
        Ok(s) => s,
        Err(e) => {
            return E_PAYLOAD_DECODE;
        }
    };
    println!("={}=", payload);
    println!("={}=", sig_op_count);
    println!("={}=", minimum_signatures);
    // params end

    OP_OK
}
