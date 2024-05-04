extern crate self as kaspa_bindings;

use kaspa_addresses::Address;
use kaspa_bip32::secp256k1::ffi::types::c_uchar;
use kaspa_consensus_core::{
    subnets::SUBNETWORK_ID_NATIVE,
    tx::{SignableTransaction, Transaction, TransactionInput, TransactionOutput},
};
use kaspa_txscript::pay_to_address_script;
use kaspa_wallet_core::{
    result::Result,
    tx::{get_consensus_params_by_address, MassCalculator, PaymentOutputs},
    utxo::{UtxoEntryReference, MAINNET_NETWORK_PARAMS},
};

// enums
//const OP_OK: i32 = 0;
const E_CHANGEADDRESS_DECODE: i32 = -1;
const E_UTXOLIST_DECODE: i32 = -2;
const E_OUTPUTS_DECODE: i32 = -3;
const E_PAYLOAD_DECODE: i32 = -4;
const E_UTXOLIST_JSON_DECODE: i32 = -5;
const E_OUTPUTS_JSON_DECODE: i32 = -6;
const E_PRIORITY_FEE_MORE_AMOUNT: i32 = -7;
const E_TXDATA_SERIALIZE_ERROR: i32 = -8;
const E_OUTBUF_OVERFLOW: i32 = -9;

fn u8_array_to_str(data: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(data)
}

fn copy_vec_to_c_uchar(data: &mut [c_uchar], vec_data: &Vec<u8>) {
    assert!(data.len() >= vec_data.len(), "Destination buffer is too small");

    for (dest, src) in data.iter_mut().zip(vec_data.iter()) {
        *dest = *src as c_uchar;
    }
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
    outbuf: *mut c_uchar,
    outbuf_len: usize,
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
    let utxo_entry_source = match u8_array_to_str(&utxo_entry_source) {
        Ok(s) => s,
        Err(e) => {
            return E_UTXOLIST_DECODE;
        }
    };
    let outputs = match u8_array_to_str(&outputs) {
        Ok(s) => s,
        Err(e) => {
            return E_OUTPUTS_DECODE;
        }
    };
    let payload = match u8_array_to_str(&payload) {
        Ok(s) => s,
        Err(e) => {
            return E_PAYLOAD_DECODE;
        }
    };
    // params end

    let change_address = match Address::try_from(change_address) {
        Ok(val) => val,
        _ => {
            return E_CHANGEADDRESS_DECODE;
        }
    };
    let params = get_consensus_params_by_address(&change_address);
    let mc = MassCalculator::new(&params, &MAINNET_NETWORK_PARAMS);

    let utxo_entries = match serde_json::from_str::<Vec<UtxoEntryReference>>(&utxo_entry_source) {
        Ok(val) => val,
        _ => {
            return E_UTXOLIST_JSON_DECODE;
        }
    };

    let payload = payload.as_bytes().to_vec();
    let outputs: PaymentOutputs = match serde_json::from_str::<PaymentOutputs>(&outputs) {
        Ok(val) => val,
        _ => {
            return E_OUTPUTS_JSON_DECODE;
        }
    };

    let mut total_input_amount = 0;
    let mut entries = vec![];

    let inputs = utxo_entries
        .iter()
        .enumerate()
        .map(|(sequence, reference)| {
            let UtxoEntryReference { utxo } = reference;
            total_input_amount += utxo.amount();
            entries.push(reference.clone());
            TransactionInput::new(utxo.outpoint.clone().into(), vec![], sequence as u64, sig_op_count)
        })
        .collect::<Vec<TransactionInput>>();

    if priority_fee > total_input_amount {
        return E_PRIORITY_FEE_MORE_AMOUNT;
    }

    // TODO - Calculate mass and fees

    let outputs: Vec<TransactionOutput> = outputs
        .iter()
        .enumerate()
        .map(|(sequence, reference)| TransactionOutput::new(reference.amount, pay_to_address_script(&reference.address)))
        .collect::<Vec<TransactionOutput>>(); //outputs.into();
    let transaction = Transaction::new(0, inputs, outputs, 0, SUBNETWORK_ID_NATIVE, 0, payload); //?
    let _fee = mc.calc_minium_transaction_relay_fee(&transaction, minimum_signatures);
    let mtx = SignableTransaction::new(transaction); //, entries.into()

    let txdata: Vec<u8> = match bincode::serialize(&mtx.tx) {
        Ok(val) => val,
        _ => {
            return E_TXDATA_SERIALIZE_ERROR;
        }
    };

    if txdata.len() > outbuf_len {
        return E_OUTBUF_OVERFLOW;
    }

    unsafe {
        copy_vec_to_c_uchar(std::slice::from_raw_parts_mut(outbuf, txdata.len()), &txdata);
    }

    (txdata.len() as i32).into()
}
