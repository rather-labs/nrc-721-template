use super::*;
use ckb_testtool::ckb_hash::Blake2bBuilder;
use ckb_testtool::ckb_types::{
    bytes::Bytes,
    core::{TransactionBuilder, TransactionView},
    packed::*,
    prelude::*,
};
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};

const MAX_CYCLES: u64 = 70_000_000;
const TYPE: u8 = 1;

fn create_test_context() -> (Context, TransactionView) {
    // deploy contract
    let mut context = Context::default();

    let nft_bin: Bytes = Loader::default().load_binary("custom_nft");
    let nft_out_point = context.deploy_cell(nft_bin);
    let nft_type_script_dep = CellDep::new_builder()
        .out_point(nft_out_point.clone())
        .build();

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    // funding cell
    let normal_input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let normal_input = CellInput::new_builder()
        .previous_output(normal_input_out_point.clone())
        .build();
    let inputs = vec![normal_input.clone()];

    // factory type script and inputs
    let factory_input_data =
        Bytes::from(hex::decode("000200000000000203E80002000200020000").unwrap());

    // Type Id script for factory
    let mut blake2b = Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    blake2b.update(b"random hash");
    let mut factory_type_args = [0; 32];
    blake2b.finalize(&mut factory_type_args);

    let mut factory_type_code_hash = [0u8; 32];
    hex::decode_to_slice(
        "00000000000000000000000000000000000000000000000000545950455f4944",
        &mut factory_type_code_hash as &mut [u8],
    )
    .unwrap();

    let factory_type_script = Script::new_builder()
        .code_hash(Byte32::from_slice(&factory_type_code_hash[..]).unwrap())
        .args(Bytes::copy_from_slice(&factory_type_args[..]).pack())
        .hash_type(Byte::new(TYPE))
        .build();
    let factory_cell_dep_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(2000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(factory_type_script.clone()).pack())
            .build(),
        factory_input_data,
    );
    let factory_cell_dep = CellDep::new_builder()
        .out_point(factory_cell_dep_out_point.clone())
        .build();

    // nft type script and inputs
    let mut nft_type_args = factory_type_script.code_hash().as_slice().to_vec();
    nft_type_args.append(&mut factory_type_script.hash_type().as_slice().to_vec());
    nft_type_args.append(&mut factory_type_script.args().unpack());

    let mut blake2b = Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    blake2b.update(normal_input.as_slice());
    blake2b.update(&(0u64).to_le_bytes());
    let mut ret = [0; 32];
    blake2b.finalize(&mut ret);

    nft_type_args.append(&mut ret.to_vec());

    let nft_type_script = context
        .build_script(&nft_out_point, Bytes::copy_from_slice(&nft_type_args[..]))
        .expect("script");

    let outputs = vec![CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(lock_script.clone())
        .type_(Some(nft_type_script.clone()).pack())
        .build()];

    let outputs_data: Vec<_> = vec![Bytes::from(hex::decode("").unwrap())];

    let mut witnesses = vec![];
    witnesses.push(Bytes::from(hex::decode("5500000010000000550000005500000041000000b69c542c0ee6c4b6d8350514d876ea7d8ef563e406253e959289457204447d2c4eb4e4a993073f5e76d244d2f93f7c108652e3295a9c8d72c12477e095026b9500").unwrap()));

    let cell_deps = vec![lock_script_dep, factory_cell_dep, nft_type_script_dep];

    // build transaction
    let tx = TransactionBuilder::default()
        .inputs(inputs)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_deps(cell_deps)
        .witnesses(witnesses.pack())
        .build();
    (context, tx)
}

#[test]
fn test_create_nft_cells_success() {
    let (mut context, tx) = create_test_context();

    let tx = context.complete_tx(tx);
    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}
