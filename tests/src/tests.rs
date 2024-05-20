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

const ISSUE_DECIMAL: u8 = 8;
const ISSUE_NAME: &'static str = "XUDT Test C Token";
const ISSUE_SYMBOL: &'static str = "XTCT";

#[derive(PartialEq)]
enum UdtInfoError {
    NoError,
    UdtInfoTypeIdInvalid,
    InputUdtInfoCellForbidden,
    DecodeInfoError,
    OnlyOneUdtInfoOutputCellAllowed
}

fn create_test_context(udt_info_error: UdtInfoError) -> (Context, TransactionView) {
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

    let udt_info = encode_token_info();
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
    let (mut context, tx) = create_test_context(UdtInfoError::NoError);
    let tx = context.complete_tx(tx);
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

fn encode_token_info() -> Vec<u8> {
    [
        &[ISSUE_DECIMAL],
        &[ISSUE_NAME.len() as u8],
        ISSUE_NAME.as_bytes(),
        &[ISSUE_SYMBOL.len() as u8],
        ISSUE_SYMBOL.as_bytes(),
    ]
        .concat()
}
