// DO NOT EDIT - automatically generated file
pub mod signer_authorization_insecure_instruction {
    use trdelnik::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn log_message(
        client: &Client,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                signer_authorization_insecure::instruction::LogMessage {},
                signer_authorization_insecure::accounts::LogMessage {
                    authority: a_authority,
                },
                signers,
            )
            .await?)
    }
    pub fn log_message_ix(a_authority: anchor_lang::solana_program::pubkey::Pubkey) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: signer_authorization_insecure::instruction::LogMessage {}.data(),
            accounts: signer_authorization_insecure::accounts::LogMessage {
                authority: a_authority,
            }
            .to_account_metas(None),
        }
    }
}
pub mod account_data_matching_insecure_instruction {
    use trdelnik::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 204u8, 101u8, 166u8, 163u8, 239u8, 244u8, 13u8, 110u8, 64u8, 106u8, 230u8,
        81u8, 141u8, 186u8, 208u8, 155u8, 78u8, 83u8, 194u8, 215u8, 103u8, 17u8, 94u8, 15u8, 137u8,
        68u8, 170u8, 153u8, 74u8, 59u8,
    ]);
    pub async fn log_message(
        client: &Client,
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                account_data_matching_insecure::instruction::LogMessage {},
                account_data_matching_insecure::accounts::LogMessage {
                    token: a_token,
                    authority: a_authority,
                },
                signers,
            )
            .await?)
    }
    pub fn log_message_ix(
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: account_data_matching_insecure::instruction::LogMessage {}.data(),
            accounts: account_data_matching_insecure::accounts::LogMessage {
                token: a_token,
                authority: a_authority,
            }
            .to_account_metas(None),
        }
    }
}
pub mod account_data_matching_secure_instruction {
    use trdelnik::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 204u8, 101u8, 166u8, 163u8, 239u8, 244u8, 13u8, 110u8, 64u8, 106u8, 230u8,
        81u8, 141u8, 186u8, 208u8, 155u8, 78u8, 83u8, 194u8, 215u8, 103u8, 17u8, 94u8, 15u8, 137u8,
        68u8, 170u8, 153u8, 74u8, 59u8,
    ]);
    pub async fn log_message(
        client: &Client,
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                account_data_matching_secure::instruction::LogMessage {},
                account_data_matching_secure::accounts::LogMessage {
                    token: a_token,
                    authority: a_authority,
                },
                signers,
            )
            .await?)
    }
    pub fn log_message_ix(
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: account_data_matching_secure::instruction::LogMessage {}.data(),
            accounts: account_data_matching_secure::accounts::LogMessage {
                token: a_token,
                authority: a_authority,
            }
            .to_account_metas(None),
        }
    }
}
pub mod owner_checks_insecure_instruction {
    use trdelnik::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 210u8, 199u8, 34u8, 74u8, 85u8, 157u8, 68u8, 66u8, 165u8, 143u8, 204u8, 94u8,
        197u8, 81u8, 54u8, 83u8, 232u8, 47u8, 209u8, 232u8, 119u8, 226u8, 102u8, 194u8, 188u8,
        254u8, 117u8, 135u8, 234u8, 243u8,
    ]);
    pub async fn log_message(
        client: &Client,
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                owner_checks_insecure::instruction::LogMessage {},
                owner_checks_insecure::accounts::LogMessage {
                    token: a_token,
                    authority: a_authority,
                },
                signers,
            )
            .await?)
    }
    pub fn log_message_ix(
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: owner_checks_insecure::instruction::LogMessage {}.data(),
            accounts: owner_checks_insecure::accounts::LogMessage {
                token: a_token,
                authority: a_authority,
            }
            .to_account_metas(None),
        }
    }
}
pub mod owner_checks_secure_instruction {
    use trdelnik::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 210u8, 199u8, 34u8, 74u8, 85u8, 157u8, 68u8, 66u8, 165u8, 143u8, 204u8, 94u8,
        197u8, 81u8, 54u8, 83u8, 232u8, 47u8, 209u8, 232u8, 119u8, 226u8, 102u8, 194u8, 188u8,
        254u8, 117u8, 135u8, 234u8, 243u8,
    ]);
    pub async fn log_message(
        client: &Client,
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                owner_checks_secure::instruction::LogMessage {},
                owner_checks_secure::accounts::LogMessage {
                    token: a_token,
                    authority: a_authority,
                },
                signers,
            )
            .await?)
    }
    pub fn log_message_ix(
        a_token: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: owner_checks_secure::instruction::LogMessage {}.data(),
            accounts: owner_checks_secure::accounts::LogMessage {
                token: a_token,
                authority: a_authority,
            }
            .to_account_metas(None),
        }
    }
}
pub mod signer_authorization_secure_instruction {
    use trdelnik::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn log_message(
        client: &Client,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                signer_authorization_secure::instruction::LogMessage {},
                signer_authorization_secure::accounts::LogMessage {
                    authority: a_authority,
                },
                signers,
            )
            .await?)
    }
    pub fn log_message_ix(a_authority: anchor_lang::solana_program::pubkey::Pubkey) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: signer_authorization_secure::instruction::LogMessage {}.data(),
            accounts: signer_authorization_secure::accounts::LogMessage {
                authority: a_authority,
            }
            .to_account_metas(None),
        }
    }
}
pub mod type_cosplay_insecure_instruction {
    use trdelnik::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 215u8, 1u8, 41u8, 54u8, 230u8, 115u8, 153u8, 67u8, 189u8, 136u8, 246u8, 187u8,
        201u8, 55u8, 37u8, 168u8, 236u8, 76u8, 147u8, 174u8, 37u8, 128u8, 234u8, 92u8, 119u8,
        171u8, 79u8, 75u8, 69u8, 196u8,
    ]);
    pub async fn add_user(
        client: &Client,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                type_cosplay_insecure::instruction::AddUser {},
                type_cosplay_insecure::accounts::AddUser {
                    user: a_user,
                    vault: a_vault,
                    authority: a_authority,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn add_user_ix(
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: type_cosplay_insecure::instruction::AddUser {}.data(),
            accounts: type_cosplay_insecure::accounts::AddUser {
                user: a_user,
                vault: a_vault,
                authority: a_authority,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn withdraw(
        client: &Client,
        i_amount: u64,
        a_meta: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_source_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_destination_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                type_cosplay_insecure::instruction::Withdraw { amount: i_amount },
                type_cosplay_insecure::accounts::Withdraw {
                    meta: a_meta,
                    user: a_user,
                    authority: a_authority,
                    source_vault: a_source_vault,
                    destination_vault: a_destination_vault,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn withdraw_ix(
        i_amount: u64,
        a_meta: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_source_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_destination_vault: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: type_cosplay_insecure::instruction::Withdraw { amount: i_amount }.data(),
            accounts: type_cosplay_insecure::accounts::Withdraw {
                meta: a_meta,
                user: a_user,
                authority: a_authority,
                source_vault: a_source_vault,
                destination_vault: a_destination_vault,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}
pub mod type_cosplay_secure_instruction {
    use trdelnik::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 214u8, 215u8, 1u8, 41u8, 54u8, 230u8, 115u8, 153u8, 67u8, 189u8, 136u8, 246u8, 187u8,
        201u8, 55u8, 37u8, 168u8, 236u8, 76u8, 147u8, 174u8, 37u8, 128u8, 234u8, 92u8, 119u8,
        171u8, 79u8, 75u8, 69u8, 196u8,
    ]);
    pub async fn update_user(
        client: &Client,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                type_cosplay_secure::instruction::UpdateUser {},
                type_cosplay_secure::accounts::UpdateUser {
                    user: a_user,
                    authority: a_authority,
                },
                signers,
            )
            .await?)
    }
    pub fn update_user_ix(
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_authority: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: type_cosplay_secure::instruction::UpdateUser {}.data(),
            accounts: type_cosplay_secure::accounts::UpdateUser {
                user: a_user,
                authority: a_authority,
            }
            .to_account_metas(None),
        }
    }
}
