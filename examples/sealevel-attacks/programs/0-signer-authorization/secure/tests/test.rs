use trdelnik::*;
use fehler::throws;
use program_client::signer_authorization_secure_instruction;
use std::mem;

#[trdelnik_test]
async fn test_secure() {
    // initialize test fixture
    let reader = Reader::with_root("../../../");
    let mut fixture = Fixture {
        client: Client::new(system_keypair(0)),
        program: program_keypair(0),
        program_data: reader.program_data("signer_authorization_secure").await?,
        authority: system_keypair(3),
    };
    // deploy a tested program
    fixture.deploy().await?;

    // authority is required to be a signer. Try delete the authority
    // from the signers and see what happens. ('Transaction::sign failed with error NotEnoughSigners')

    // call an intstruction
    signer_authorization_secure_instruction::log_message(
        &fixture.client, 
        fixture.authority.pubkey(), 
        [fixture.authority],
    ).await?.print();
}

struct Fixture {
    client: Client,
    program: Keypair,
    program_data: Vec<u8>,
    authority: Keypair,
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