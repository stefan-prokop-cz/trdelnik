use trdelnik::*;
use fehler::throws;
use program_client::owner_checks_secure_instruction;
use std::mem;
use anchor_lang::solana_program::program_option::COption;
use anchor_lang::solana_program::program_pack::Pack;

#[trdelnik_test]
async fn test_secure() {
    // initialize test fixture
    let reader = Reader::with_root("../../../");
    let mut fixture = Fixture {
        client: Client::new(system_keypair(0)),
        program: program_keypair(3),
        program_data: reader.program_data("type_cosplay_secure").await?,
        authority: system_keypair(3),
        token_account: keypair(4),
        _attacker: keypair(5),
    };
    // deploy a tested program
    fixture.deploy().await?;
    
}

struct Fixture {
    client: Client,
    program: Keypair,
    program_data: Vec<u8>,
    authority: Keypair,
    token_account: Keypair,
    _attacker: Keypair,
}

impl Fixture {
    #[throws]
    async fn deploy(&mut self) {
        self.client.airdrop(self.client.payer().pubkey(), 5_000_000_000).await?;
        self.client.deploy(
            self.program.clone(),
            mem::take(&mut self.program_data)
        ).await?;
    }
}