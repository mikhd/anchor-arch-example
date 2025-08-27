use arch_program::{bitcoin::key::Keypair, pubkey::Pubkey};
use arch_sdk::{
    generate_new_keypair, with_secret_key_file, ArchRpcClient, Config, ProgramDeployer
};

pub struct TestEnvironment {
    pub client: ArchRpcClient,
    pub signer: Keypair,
    pub signer_pubkey: Pubkey,
    pub program_pubkey: Pubkey,
    pub test_config: Config,
}

impl TestEnvironment {
    pub const ELF_PATH: &str = "../../target/deploy/test.so";
    pub const PROGRAM_KEYPAIR_PATH: &str = "../../target/deploy/test-keypair.json";

    pub fn new() -> Self {
        let test_config = Config::localnet();
        let bitcoin_network = test_config.network;
        let node1_address = &test_config.arch_node_url;

        let client = ArchRpcClient::new(node1_address);

        let (program_keypair, _, _) = generate_new_keypair(bitcoin_network);
        let (program_keypair_from_file, program_pubkey_from_file) =
            with_secret_key_file(&TestEnvironment::PROGRAM_KEYPAIR_PATH.to_string()).expect("getting caller info should not fail");

        println!("program_pubkey_from_file: {}", program_pubkey_from_file.to_string());

        let (signer_keypair, signer_pubkey, _) = generate_new_keypair(bitcoin_network);

        let _res = client
            .create_and_fund_account_with_faucet(&signer_keypair, bitcoin_network).unwrap()
            ;

        let deployer = ProgramDeployer::new(node1_address, bitcoin_network);

        let program_pubkey = deployer
            .try_deploy_program(
                "Test Program".to_string(),
                program_keypair,
                signer_keypair,
                &TestEnvironment::ELF_PATH.to_string(),
            )
            .unwrap();


        TestEnvironment {
            client,
            signer: signer_keypair,
            signer_pubkey,
            program_pubkey,
            test_config,
        }
    }

    pub fn get_or_deploy() -> Self {
        let test_config = Config::localnet();
        let bitcoin_network = test_config.network;
        let node1_address = &test_config.arch_node_url;

        let client = ArchRpcClient::new(node1_address);

        let (program_keypair, program_pubkey) =
            with_secret_key_file(&TestEnvironment::PROGRAM_KEYPAIR_PATH.to_string()).expect("getting caller info should not fail");

        let (signer_keypair, signer_pubkey, _) = generate_new_keypair(bitcoin_network);


        let _res = client
            .create_and_fund_account_with_faucet(&signer_keypair, bitcoin_network).unwrap()
            ;

        println!("Using program pubkey: {}", program_pubkey.to_string());

        let program_account_info = client
        .read_account_info(program_pubkey).unwrap_or_else(|_| {
            println!("Program account not found, deploying program...");
            let deployer = ProgramDeployer::new(node1_address, bitcoin_network);

            let program_pubkey = deployer
                .try_deploy_program(
                    "Test Program".to_string(),
                    program_keypair,
                    signer_keypair,
                    &TestEnvironment::ELF_PATH.to_string(),
                )
                .unwrap();

            client.read_account_info(program_pubkey).unwrap()
        });


        TestEnvironment {
            client,
            signer: signer_keypair,
            signer_pubkey,
            program_pubkey,
            test_config
        }
    }

}