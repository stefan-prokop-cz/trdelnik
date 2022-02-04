use trdelnik::*;
use fehler::throws;
use program_client::signer_authorization_insecure_instruction;
use std::mem;

#[trdelnik_test]
async fn test_insecure() {
    // initialize test fixture
    let reader = Reader::with_root("../../../");
    let mut fixture = Fixture {
        client: Client::new(system_keypair(0)),
        program: program_keypair(0),
        program_data: reader.program_data("signer_authorization_insecure").await?,
        authority: system_keypair(3),
    };
    // deploy a tested program
    fixture.deploy().await?;

    // ATTACK authority is not required to be a signer so anyone can call
    // this instruction. (try run this test)
    
    // call an intstruction
    signer_authorization_insecure_instruction::log_message(
        &fixture.client, 
        fixture.authority.pubkey(), 
        [],                                             
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