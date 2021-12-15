use std::borrow::Borrow;
use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
// use crate::processors::mint_contract;
use std::convert::TryFrom;
use std::marker::PhantomData;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Method {
    MINT = 0
}

pub trait ConvertProgramData<T> {
    fn convert(&self) -> T;
}

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct ProgramArgument {
//     pub value: u64
// }

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ProgramData {
    pub method: Method,
    pub args: MintProgramData
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MintProgramData {
    pub amount: u64
}

// impl From<&ProgramData> for MintProgramData {
//     fn from(data: &ProgramData) -> Self {
//         MintProgramData::try_from_slice(&data.args.borrow()).expect("PIZDEC")
//     }
// }

pub struct OVNToken {
    pub token_pub: Pubkey,
    pub owner_pub: Pubkey,
    pub mint_pub: Pubkey,
    pub token_program_pub: Pubkey,
    sysvar_pub: Pubkey,

    pub decimals: u32
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountTokenData {
    pub token_amount: u64
}

impl Default for OVNToken {
    fn default() -> Self {
        OVNToken {
            token_pub: Pubkey::try_from("AV8U839Ysa7WnVzk7BHQTJMNN3eLSf6qDazZGUdCpP4G").unwrap(),
            owner_pub: Pubkey::try_from("5aeAsopdEKRXXiKVn52iRRA1x3oXiaU1qyJEMzZ8g9YR").unwrap(),
            mint_pub: Pubkey::try_from("4JapSRqYT3K3BQkgqqRrGnWEXtpNb9p5MoWj8EmTyNha").unwrap(),
            token_program_pub: Pubkey::try_from("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
            sysvar_pub: Pubkey::try_from("SysvarRent111111111111111111111111111111111").unwrap(),

            decimals: 9
        }
    }
}


pub struct OVNProcessor {

}

pub struct Exchange<'a> {
    pub ovn: OVNToken,
    pub program_data: ProgramData,
    pub account_infos: Vec<AccountInfo<'a>>,
}