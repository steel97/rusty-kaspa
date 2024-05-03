use kaspa_addresses::{Address, AddressError};
use kaspa_consensus_core::{
    subnets::SUBNETWORK_ID_NATIVE,
    tx::{SignableTransaction, Transaction, TransactionInput, TransactionOutput},
};
use kaspa_txscript::pay_to_address_script;
use kaspa_wallet_core::{
    result::Result,
    tx::{get_consensus_params_by_address, MassCalculator, PaymentOutputs},
    utxo::{UtxoEntryReference, MAINNET_NETWORK_PARAMS, TESTNET11_NETWORK_PARAMS},
};
use serde_json::Value;

pub fn main() -> Result<()> {
    // params start
    let change_address = "kaspa:qrsatq67p8eu8kkjp80tedanhulmpcxevsj8rawmxmy75kpn3vrtuqyg7447h";
    //let utxo_entry_source = "[{\"address\":\"kaspa:qrsatq67p8eu8kkjp80tedanhulmpcxevsj8rawmxmy75kpn3vrtuqyg7447h\",\"outpoint\":{\"transactionId\":\"3556a2422ed1241617fe43a4212ad1d8e426ea56ebaf76c5ec6bc74181ae9d1d\",\"index\":1},\"utxo\":{\"amount\":\"5421907222\",\"scriptPublicKey\":{\"scriptPublicKey\":\"20e1d5835e09f3c3dad209debcb7b3bf3fb0e0d9642471f5db36c9ea58338b06beac\"},\"blockDaaScore\":\"78425374\"}},{\"address\":\"kaspa:qrsatq67p8eu8kkjp80tedanhulmpcxevsj8rawmxmy75kpn3vrtuqyg7447h\",\"outpoint\":{\"transactionId\":\"5024e1e3b12bd6a93822391e3f20f75e7c41762fdb21f799d4427e0805dfac0b\",\"index\":1},\"utxo\":{\"amount\":\"17317094711414\",\"scriptPublicKey\":{\"scriptPublicKey\":\"20e1d5835e09f3c3dad209debcb7b3bf3fb0e0d9642471f5db36c9ea58338b06beac\"},\"blockDaaScore\":\"76622688\"}},{\"address\":\"kaspa:qrsatq67p8eu8kkjp80tedanhulmpcxevsj8rawmxmy75kpn3vrtuqyg7447h\",\"outpoint\":{\"transactionId\":\"c2500ac1a9eb07e566f011f2a0b35ac09fd5d84f95f1245f695e15b064017596\",\"index\":1},\"utxo\":{\"amount\":\"5685368357389\",\"scriptPublicKey\":{\"scriptPublicKey\":\"20e1d5835e09f3c3dad209debcb7b3bf3fb0e0d9642471f5db36c9ea58338b06beac\"},\"blockDaaScore\":\"76622688\"}}]";
    let utxo_entry_source = "[{\"utxo\": {\"address\":\"kaspa:qrsatq67p8eu8kkjp80tedanhulmpcxevsj8rawmxmy75kpn3vrtuqyg7447h\",\"outpoint\": {\"inner\":{\"transactionId\":\"3556a2422ed1241617fe43a4212ad1d8e426ea56ebaf76c5ec6bc74181ae9d1d\",\"index\":1}},\"entry\":{\"amount\":5421907222,\"scriptPublicKey\":\"20e1d5835e09f3c3dad209debcb7b3bf3fb0e0d9642471f5db36c9ea58338b06beac\",\"blockDaaScore\":78425374, \"isCoinbase\": true}}}]";
    let outputs =
        "{\"outputs\": [{\"address\": \"kaspa:qqxh79sjz43dc2997ys0umrpjxup0v5vwkwm7k2cvz8gg89d857e5h2j6ku82\",\"amount\": 1}, {\"address\": \"kaspa:qrsatq67p8eu8kkjp80tedanhulmpcxevsj8rawmxmy75kpn3vrtuqyg7447h\",\"amount\": 2}]}";
    let priority_fee: u64 = 0;
    let payload = "woolypooly";
    let sig_op_count: u8 = 1;
    let minimum_signatures: u16 = 2;
    // params end
    let change_address = Address::try_from(change_address)?;
    let params = get_consensus_params_by_address(&change_address);
    let mc = MassCalculator::new(&params, &MAINNET_NETWORK_PARAMS);

    //println!("{}", change_address);

    /*let utxo_entries = if let Some(utxo_entries) = utxo_entry_source.dyn_ref::<js_sys::Array>() {
        utxo_entries.to_vec().iter().map(UtxoEntryReference::try_from).collect::<Result<Vec<_>, _>>()?
    } else {
        return Err(Error::custom("utxo_entries must be an array"));
    };*/
    /*let v: Value = serde_json::from_str(&utxo_entry_source).unwrap();
    if let Some(field) = v.get(0).unwrap().get("outpoint") {
        println!("field = {:?}", field);
    } else {
        println!("field is missing");
    }*/

    let utxo_entries = serde_json::from_str::<Vec<UtxoEntryReference>>(&utxo_entry_source).unwrap();
    //println!("{}", utxo_entries.len());

    //let priority_fee: u64 = priority_fee.try_into().map_err(|err| Error::custom(format!("invalid fee value: {err}")))?;
    let payload = payload.as_bytes().to_vec();
    let outputs: PaymentOutputs = serde_json::from_str::<PaymentOutputs>(&outputs).unwrap();
    /*let sig_op_count =
        if !sig_op_count.is_undefined() { sig_op_count.as_f64().expect("sigOpCount should be a number") as u8 } else { 1 };

    let minimum_signatures = if !minimum_signatures.is_undefined() {
        minimum_signatures.as_f64().expect("minimumSignatures should be a number") as u16
    } else {
        1
    };*/

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
            TransactionInput::new(utxo.outpoint.clone().into(), vec![], sequence as u64, sig_op_count)
        })
        .collect::<Vec<TransactionInput>>();

    if priority_fee > total_input_amount {
        return Err(format!("priority fee({priority_fee}) > amount({total_input_amount})").into());
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

    let txdata: Vec<u8> = bincode::serialize(&mtx.tx).unwrap();
    let hex: String = txdata.iter().map(|b| format!("{:02x}", b).to_string()).collect::<Vec<String>>().join("");
    println!("{}", hex);
    //Ok(mtx)
    Ok(())
}
