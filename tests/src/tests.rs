// Include your tests here
// See https://github.com/xxuejie/ckb-native-build-sample/blob/main/tests/src/tests.rs for examples


use ckb_testtool::builtin::ALWAYS_SUCCESS;
use ckb_testtool::bytes::Bytes;
use ckb_testtool::ckb_types::core::{TransactionBuilder, TransactionView};
use ckb_testtool::ckb_types::packed::{CellDep, CellInput, CellOutput};
use ckb_testtool::ckb_types::prelude::{Builder, Entity, Pack};
use ckb_testtool::context::Context;
use crate::Loader;


const MAX_CYCLES: u64 = 10_000_000;

#[derive(PartialEq)]
enum UdtInfoError {
    NoError,
    UdtInfoTypeIdInvalid,
    InputUdtInfoCellForbidden,
    DecodeInfoError,
    OnlyOneUdtInfoOutputCellAllowed
}

fn create_test_context(
    issue_symbol: &str
) -> (Context, TransactionView) {
    // deploy contract
    let mut context = Context::default();
    let udt_info_bin: Bytes = Loader::default().load_binary("udt-info-lock");
    let udt_info_out_point = context.deploy_cell(udt_info_bin);

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());

    // prepare scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let udt_info = encode_token_info(
        issue_symbol,
    );
    let udt_info_script = context.build_script(&udt_info_out_point, udt_info.into()).expect("udt_info_script");
    let udt_info_script_dep = CellDep::new_builder().out_point(udt_info_out_point).build();

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
            Bytes::default(),
    );

    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let output = CellOutput::new_builder()
        .capacity(500u64.pack())
        .lock(lock_script.clone())
        .type_(Some(udt_info_script).pack())
        .build();


    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .output(output)
        .outputs_data(vec![Bytes::default()].pack())
        .cell_dep(lock_script_dep)
        .cell_dep(udt_info_script_dep)
        .build();


    (context, tx)
}

#[test]
fn test_create_udt_info_cell_success() {
    let (mut context, tx) = create_test_context(
        "XTCT",
    );
    let tx = context.complete_tx(tx);
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

#[test]
fn test_script_hash() {
    // deploy contract
    let mut context = Context::default();
    let udt_info_bin: Bytes = Loader::default().load_binary("udt-info-lock");
    let udt_info_out_point = context.deploy_cell(udt_info_bin);

    let udt_info1 = encode_token_info(
        "XTCT",
    );
    let udt_info_script1 = context.build_script(&udt_info_out_point, udt_info1.into()).expect("udt_info_script");

    let udt_info2 = encode_token_info(
        "XTCT",
    );
    let udt_info_script2 = context.build_script(&udt_info_out_point, udt_info2.into()).expect("udt_info_script");

    assert_ne!(udt_info_script1.calc_script_hash().as_bytes(), udt_info_script2.calc_script_hash().as_bytes())
}



fn encode_token_info(
    issue_symbol: &str
) -> Vec<u8> {
    issue_symbol.as_bytes().to_vec()
}

#[test]
fn test_load_infos() {

}