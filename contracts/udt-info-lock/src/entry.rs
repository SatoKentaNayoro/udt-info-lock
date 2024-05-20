use alloc::vec::Vec;
use ckb_std::ckb_types::prelude::Unpack;
use ckb_std::debug;
use ckb_std::high_level::load_script;
use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let udt_data:Vec<u8> = script.args().unpack();
    if udt_data.is_empty() {
        return Err(Error::LengthNotEnough);
    }
    debug!("udt_data: {:?}", udt_data);
    let result = parse_udt_data(udt_data)?;
    debug!("result: {:?}", result);

    Ok(())
}

fn parse_udt_data(data: Vec<u8>) -> Result<(u8, Vec<u8>, Vec<u8>), Error> {
    let decimal = data.first().ok_or_else(||Error::DecodeInfoError)?;
    let name_len = data.get(1).ok_or_else(||Error::DecodeInfoError)?;
    let (name, next_start) = slice_vec(&data, 2, *name_len as usize)?;
    let (symbol_len, next_start) = slice_vec(&data, next_start, 1)?;
    let (symbol, _next_start) = slice_vec(&data, next_start, symbol_len[0] as usize)?;
    Ok((*decimal, name, symbol))
}

fn slice_vec(vec: &Vec<u8>, start: usize, length: usize) -> Result<(Vec<u8>, usize), Error> {
    if start + length > vec.len() {
        return Err(Error::IndexOutOfBound);
    }

    let next_start = start + length;

    let sliced_vec: Vec<u8> = vec.iter().skip(start).take(length).cloned().collect();
    Ok((sliced_vec, next_start))
}