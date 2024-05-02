use kaspa_addresses::{Address, AddressError};
use kaspa_wallet_core::{
    result::Result,
    tx::{get_consensus_params_by_address, MassCalculator, PaymentOutputs},
    utxo::{UtxoEntryReference, MAINNET_NETWORK_PARAMS, TESTNET11_NETWORK_PARAMS},
};

pub fn main() -> Result<()> {
    println!("test!");
    // params start
    let change_address = "kaspatest:qz7ulu4c25dh7fzec9zjyrmlhnkzrg4wmf89q7gzr3gfrsj3uz6xjceef60sd";
    // params end
    let change_address = Address::try_from(change_address)?;
    let params = get_consensus_params_by_address(&change_address);
    let mc = MassCalculator::new(&params, &TESTNET11_NETWORK_PARAMS);

    //println!("{}", change_address);

    let utxo_entries = if let Some(utxo_entries) = utxo_entry_source.dyn_ref::<js_sys::Array>() {
        utxo_entries.to_vec().iter().map(UtxoEntryReference::try_from).collect::<Result<Vec<_>, _>>()?
    } else {
        return Err(Error::custom("utxo_entries must be an array"));
    };
    /*
    let priority_fee: u64 = priority_fee.try_into().map_err(|err| Error::custom(format!("invalid fee value: {err}")))?;
    let payload = payload.try_as_vec_u8().ok().unwrap_or_default();
    let outputs: PaymentOutputs = outputs.try_into()?;
    let sig_op_count =
        if !sig_op_count.is_undefined() { sig_op_count.as_f64().expect("sigOpCount should be a number") as u8 } else { 1 };

    let minimum_signatures = if !minimum_signatures.is_undefined() {
        minimum_signatures.as_f64().expect("minimumSignatures should be a number") as u16
    } else {
        1
    };

    // ---

    let mut total_input_amount = 0;
    let mut entries = vec![];

    let inputs = utxo_entries
        .iter()
        .enumerate()
        .map(|(sequence, reference)| {
            let UtxoEntryReference { utxo } = reference;
            total_input_amount += utxo.amount();
            entries.push(reference.clone());
            TransactionInput::new(utxo.outpoint.clone(), vec![], sequence as u64, sig_op_count)
        })
        .collect::<Vec<TransactionInput>>();

    if priority_fee > total_input_amount {
        return Err(format!("priority fee({priority_fee}) > amount({total_input_amount})").into());
    }

    // TODO - Calculate mass and fees

    let outputs: Vec<TransactionOutput> = outputs.into();
    let transaction = Transaction::new(0, inputs, outputs, 0, SUBNETWORK_ID_NATIVE, 0, payload)?;
    let _fee = mc.calc_minimum_transaction_relay_fee(&transaction, minimum_signatures);
    let mtx = SignableTransaction::new(transaction, entries.into());

    Ok(mtx)*/
    Ok(())
}
