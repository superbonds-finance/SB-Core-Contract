#![allow(clippy::too_many_arguments)]
#![allow(non_snake_case)]

use {
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

    add_remove_liq{
        action_type: u8,
        amount: u64,
    },
    /*
        type = 0 --> lp token
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

    claimNFT{

    },

    trade{
        usdc_in: u64
    },

    redeem{
    },

    request_farming_reward{
    },

    distribute_farming_reward{
        farm_id: u8,
        amount: u64
    },

    cleanup_farming_reward_request{

    },

    set_superbonds{
        status: u8
    },

    settle_redemption{

    },
    //Withdraw USDC/SUPERB Treasury and SuperB Fee Pool
    withdraw_fund{
        amount: u64
    }

}
impl SuperBondsInstruction {

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                let (pool_length, _rest) = Self::unpack_u32(rest)?;
                Self::initPool {
                    pool_length
                }
            },

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

            11 => {
                let (action_type, rest) = Self::unpack_u8(rest)?;
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::add_remove_liq{
                    action_type,
                    amount
                }
            }
            /*          TRADE       */
            15 =>{
                let (usdc_in, _rest) = Self::unpack_u64(rest)?;
                Self::trade{
                    usdc_in
                }
            },
            /*          REDEEM       */
            17 =>{
                Self::redeem{
                }
            },
            /*          STAKE  and CLAIM
                    When amount = 0; mean claiming rewards
                */
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
            23 => {
                Self::claimNFT{

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
                Self::request_farming_reward{
                }
            },
            37 => {
                let (farm_id, rest) = Self::unpack_u8(rest)?;
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::distribute_farming_reward{
                    farm_id,
                    amount
                }
            },

            38 => {
                Self::cleanup_farming_reward_request{

                }
            },

            39 => {
                Self::settle_redemption{
                }
            },
            41 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::withdraw_fund{
                    amount
                }
            }

            _ => return Err(ProgramError::InvalidInstructionData.into())
        })
    }
    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }
    fn unpack_u32(input: &[u8]) -> Result<(u32, &[u8]), ProgramError> {
        if input.len() >= 4 {
            let (amount, rest) = input.split_at(4);
            let amount = amount
                .get(..4)
                .and_then(|slice| slice.try_into().ok())
                .map(u32::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }
    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.len() >= 1 {
            let (amount, rest) = input.split_at(1);
            let amount = amount
                .get(..1)
                .and_then(|slice| slice.try_into().ok())
                .map(u8::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }

}
