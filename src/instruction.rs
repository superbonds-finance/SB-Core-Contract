#![allow(clippy::too_many_arguments)]
#![allow(non_snake_case)]

use {
    crate::{
        error::SB_Error
    },
    solana_program::{
        program_error::ProgramError,
    }
};
use std::convert::TryInto;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SuperBondsInstruction {
    /*
        Created only ONCE
    */
    initStakingPool{
    },
    
    initPool {
        pool_length: u32,
    },

    updateParameter{
        para_id: u8,
        value_u64: u64,
    },

    external_farm {
    },

    deposit_trader_pool{
        amount: u64
    },

    updateGovernanceOperatorAdmin {
        isGov: u8,
        index: u8
    },

    claimTreasury {
    },

    addLiquidity{
        liquidity_n_fee_usdt: u64
    },

    removeLiquidity{
        lp_token_amount: u64
    },
    /*
        type = 0 --> lp token
        type = 1 --> sol_sb_lp token
        type = 2 --> superB token
    */
    stake{
        stake_type: u8,
        amount: u64
    },

    unstake{
        unstake_type: u8,
        amount: u64
    },

    claim{
    },

    check_reward{
    },

    trade{
        usdt_in: u64
    },

    redeem{
    },

    add_farming_reward{
        amount: u64,
        farm_id: u8
    },

    claim_farming_reward{
        farm_id: u8,
        reward_data_size: u8
    },

    set_superbonds{
        status: u8
    }

}
impl SuperBondsInstruction {

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(SB_Error::InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (pool_length, _rest) = Self::unpack_u32(rest)?;
                Self::initPool {
                    pool_length
                }
            },
            /*          UPDATE YIELD       */

            2 =>{
                let (isGov, rest) = Self::unpack_u8(rest)?;
                let (index, _rest) = Self::unpack_u8(rest)?;
                Self::updateGovernanceOperatorAdmin{
                    isGov,
                    index
                }
            },
            3 =>{
                Self::initStakingPool{
                }
            },

            5 => {
                Self::claimTreasury {
                }
            },

            7 => {
                let (para_id, rest) = Self::unpack_u8(rest)?;
                let (value_u64, _rest) = Self::unpack_u64(rest)?;
                Self::updateParameter{
                    para_id,
                    value_u64
                }
            },
            9 => {
                let (status, _rest) = Self::unpack_u8(rest)?;
                Self::set_superbonds{
                    status
                }
            },

            /*          ADD Liquidity       */
            11 =>{
                let (liquidity_n_fee_usdt, _rest) = Self::unpack_u64(rest)?;
                Self::addLiquidity{
                    liquidity_n_fee_usdt
                }
            },
            /*          REMOVE Liquidity       */
            13 =>{
                let (lp_token_amount, _rest) = Self::unpack_u64(rest)?;
                Self::removeLiquidity{
                    lp_token_amount
                }
            },
            /*          TRADE       */
            15 =>{
                let (usdt_in, _rest) = Self::unpack_u64(rest)?;
                Self::trade{
                    usdt_in
                }
            },
            /*          REDEEM       */
            17 =>{
                Self::redeem{
                }
            },
            /*          STAKE      */
            19 =>{
                let (stake_type, rest) = Self::unpack_u8(rest)?;
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::stake{
                    stake_type,
                    amount
                }
            },
            /*          UNSTAKE     */
            21 =>{
                let (unstake_type, rest) = Self::unpack_u8(rest)?;
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::unstake{
                    unstake_type,
                    amount
                }
            },

            /*          CLAIM SB     */
            27 =>{
                Self::claim{
                }
            },
            /*          CHECK SB REWARDS    */
            29 =>{
                Self::check_reward{
                }
            },
            31 => {
                Self::external_farm{

                }
            },
            33 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::deposit_trader_pool{
                    amount
                }
            },
            35 => {
                let (amount, rest) = Self::unpack_u64(rest)?;
                let (farm_id, _rest) = Self::unpack_u8(rest)?;
                Self::add_farming_reward{
                    amount,
                    farm_id
                }
            },
            37 => {
                let (farm_id, rest) = Self::unpack_u8(rest)?;
                let (reward_data_size, _rest) = Self::unpack_u8(rest)?;
                Self::claim_farming_reward{
                    farm_id,
                    reward_data_size
                }
            }

            _ => return Err(SB_Error::InvalidInstruction.into())
        })
    }
    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(SB_Error::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(SB_Error::InvalidInstruction.into())
        }
    }
    fn unpack_u32(input: &[u8]) -> Result<(u32, &[u8]), ProgramError> {
        if input.len() >= 4 {
            let (amount, rest) = input.split_at(4);
            let amount = amount
                .get(..4)
                .and_then(|slice| slice.try_into().ok())
                .map(u32::from_le_bytes)
                .ok_or(SB_Error::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(SB_Error::InvalidInstruction.into())
        }
    }
    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.len() >= 1 {
            let (amount, rest) = input.split_at(1);
            let amount = amount
                .get(..1)
                .and_then(|slice| slice.try_into().ok())
                .map(u8::from_le_bytes)
                .ok_or(SB_Error::InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(SB_Error::InvalidInstruction.into())
        }
    }

}
