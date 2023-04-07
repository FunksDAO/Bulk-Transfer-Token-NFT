#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod bulk_transfer {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::{ownable::*, psp22::*, psp34::*},
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct BulkTransfer {
        #[storage_field]
        ownable: ownable::Data,
    }

    impl BulkTransfer {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn bulk_nft_transfer(
            &mut self,
            psp34_nft_contract_address: AccountId,
            transfer_to: Vec<AccountId>,
            token_id: Vec<Id>,
            _data: Vec<u8>,
        ) -> Result<(), PSP34Error> {
            for index in 0..transfer_to.len() as usize {
                PSP34Ref::transfer(
                    &psp34_nft_contract_address,
                    transfer_to[index],
                    token_id[index].clone(),
                    _data.clone(),
                )?;
            }
            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn bulk_token_transfer(
            &mut self,
            psp22_contract_address: AccountId,
            from: AccountId,
            transfer_to: Vec<AccountId>,
            token: Vec<Balance>,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            for index in 0..transfer_to.len() as usize {
                PSP22Ref::transfer_from(
                    &psp22_contract_address,
                    from,
                    transfer_to[index],
                    token[index].clone(),
                    _data.clone(),
                )?;
            }
            Ok(())
        }
    }
}
