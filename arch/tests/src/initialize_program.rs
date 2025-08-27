use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct InitializeProgramData {
    discriminator: [u8; 8],
}

#[cfg(test)]
mod initialize_program_tests {
    use std::fs;
    use super::*;
    const INITIALIZE_PROGRAM_DISCRIMINATOR: [u8; 8] =[
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ];
    use serial_test::serial;

    use arch_program::{account::AccountMeta, bpf_loader::LoaderState, pubkey::Pubkey, sanitized::ArchMessage};
    use arch_sdk::{build_and_sign_transaction, Status};

    use crate::common::TestEnvironment;

    #[test]
    #[serial]
    fn test_deploy_program() {
        let env = TestEnvironment::new();

        let program_account_info = env.client
            .read_account_info(env.program_pubkey)
            .expect("read account info should not fail");

        let elf = fs::read("../../target/deploy/test.so")
            .expect("elf path should be available");

        assert!(program_account_info.data[LoaderState::program_data_offset()..] == elf);
        assert!(program_account_info.is_executable);
    }

    #[test]
    #[serial]
    fn test_deploy_or_get_program() {
        let env = TestEnvironment::get_or_deploy();

        let program_account_info = env.client
            .read_account_info(env.program_pubkey)
            .expect("read account info should not fail");

        let elf = fs::read("../../target/deploy/test.so")
            .expect("elf path should be available");


        assert!(program_account_info.data[LoaderState::program_data_offset()..] == elf);
        assert!(program_account_info.is_executable);
    }

    #[test]
    #[serial]
    fn test_initialize_program() {
        let env = TestEnvironment::get_or_deploy();

        let initialize_ix_data = InitializeProgramData {
            discriminator: INITIALIZE_PROGRAM_DISCRIMINATOR,
        };

        let data = borsh::to_vec(&initialize_ix_data).unwrap();


        let (program_config_pubkey, _) = Pubkey::find_program_address(
            &[b"config"],
            &env.program_pubkey,
        );

        println!("Config pubkey: {:?}", program_config_pubkey);

        let recent_blockhash = env.client.get_best_block_hash().unwrap();
        let initialize_program_tx = build_and_sign_transaction(
            ArchMessage::new(
                &[arch_program::instruction::Instruction {
                    program_id: env.program_pubkey,
                    accounts: vec![
                        AccountMeta::new(program_config_pubkey, false), // config
                        AccountMeta::new(env.signer_pubkey, true), // signer
                        AccountMeta::new_readonly(Pubkey::system_program(), false),
                    ],
                    data,
                }],
                Some(env.signer_pubkey),
                recent_blockhash,
            ),
            vec![env.signer],
            env.test_config.network,
        ).expect("Failed to build and sign initialize_program_tx");

        let txid = env.client.send_transaction(initialize_program_tx).unwrap();
        let processed_tx = env.client.wait_for_processed_transaction(&txid)
        .expect("Failed to confirm processed transaction");
        println!("Processed tx: {:?}", processed_tx.logs);
        println!("Processed status: {:?}", processed_tx.status);

        assert!(processed_tx.status == Status::Processed);
    }


}
