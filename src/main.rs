use clap::{crate_description, crate_name, crate_version, App, Arg, SubCommand};
use solana_clap_utils::input_validators::{
    is_url_or_moniker, is_valid_pubkey, normalize_to_url_if_moniker,
};
use solana_client::rpc_client::RpcClient;
use solana_program::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::{Account, Mint};
use spl_token_metadata::{
    state::{
        Key, MasterEditionV2, Metadata, EDITION, MAX_MASTER_EDITION_LEN, MAX_METADATA_LEN, PREFIX,
    },
    utils::try_from_slice_checked,
};
use std::str::FromStr;

// Helper functions

fn show_mint(client: RpcClient, address: Pubkey) {
    let acc_data = client.get_account_data(&address).unwrap();
    let mint_data = Mint::unpack(acc_data.as_slice()).unwrap();
    println!("{:?}", mint_data);
}

fn show_account(client: RpcClient, address: Pubkey) {
    let acc_data = client.get_account_data(&address).unwrap();
    let account_data = Account::unpack(acc_data.as_slice()).unwrap();
    println!("{:?}", account_data);
}

fn show_metadata(client: RpcClient, address: Pubkey) {
    let acc_data = client.get_account_data(&address).unwrap();
    let account_data: Metadata =
        try_from_slice_checked(acc_data.as_slice(), Key::MetadataV1, MAX_METADATA_LEN).unwrap();
    println!("{:?}", account_data);
}

fn show_master_edition(client: RpcClient, address: Pubkey) {
    let acc_data = client.get_account_data(&address).unwrap();
    let account_data: MasterEditionV2 = try_from_slice_checked(
        acc_data.as_slice(),
        Key::MasterEditionV2,
        MAX_MASTER_EDITION_LEN,
    )
    .unwrap();
    println!("{:?}", account_data);
}

fn show_nft(client: RpcClient, mint_address: Pubkey) {
    let acc_data = client.get_account_data(&mint_address).unwrap();
    let mint_data = Mint::unpack(acc_data.as_slice()).unwrap();
    println!("{:?}", mint_data);

    // pda of ['metadata', program id, mint, 'edition']
    let me_key_seeds = &[
        PREFIX.as_bytes(),
        &spl_token_metadata::ID.as_ref(),
        &mint_address.as_ref(),
        EDITION.as_bytes(),
    ];
    let me_key = Pubkey::find_program_address(me_key_seeds, &spl_token_metadata::ID).0;
    let acc_data = client.get_account_data(&me_key).unwrap();
    let account_data: MasterEditionV2 = try_from_slice_checked(
        acc_data.as_slice(),
        Key::MasterEditionV2,
        MAX_MASTER_EDITION_LEN,
    )
    .unwrap();
    println!("{:?}", account_data);

    // pda of ['metadata', program id, mint id]
    let meta_key_seeds = &[
        PREFIX.as_bytes(),
        &spl_token_metadata::ID.as_ref(),
        &mint_address.as_ref(),
    ];
    let meta_key = Pubkey::find_program_address(meta_key_seeds, &spl_token_metadata::ID).0;

    let acc_data = client.get_account_data(&meta_key).unwrap();
    let meta_data: Metadata =
        try_from_slice_checked(acc_data.as_slice(), Key::MetadataV1, MAX_METADATA_LEN).unwrap();
    println!("{:?}", meta_data);
}

fn main() {
    let app_matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("json_rpc_url")
                .short("u")
                .long("url")
                .value_name("URL_OR_MONIKER")
                .takes_value(true)
                .global(true)
                .validator(is_url_or_moniker)
                .help(
                    "URL for Solana's JSON RPC or moniker (or their first letter): \
                       [mainnet-beta, testnet, devnet, localhost] \
                    Default is devnet",
                ),
        )
        .subcommand(
            SubCommand::with_name("mint")
                .arg(
                    Arg::with_name("address")
                        .validator(is_valid_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .help("Address"),
                )
                .about("Show Token Mint"),
        )
        .subcommand(
            SubCommand::with_name("account")
                .arg(
                    Arg::with_name("address")
                        .validator(is_valid_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .help("Address"),
                )
                .about("Show Token account"),
        )
        .subcommand(
            SubCommand::with_name("metadata")
                .arg(
                    Arg::with_name("address")
                        .validator(is_valid_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .help("Address"),
                )
                .about("Show Token metaplex metadata account"),
        )
        .subcommand(
            SubCommand::with_name("master-edition")
                .arg(
                    Arg::with_name("address")
                        .validator(is_valid_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .help("Address"),
                )
                .about("Show Token metaplex master edition account"),
        )
        .subcommand(
            SubCommand::with_name("nft")
                .arg(
                    Arg::with_name("address")
                        .validator(is_valid_pubkey)
                        .value_name("PUBKEY")
                        .takes_value(true)
                        .help("Address"),
                )
                .about("Show full NFT accounts pack"),
        )
        .get_matches();

    let json_rpc_url = normalize_to_url_if_moniker(
        app_matches
            .value_of("json_rpc_url")
            .unwrap_or(&"https://api.devnet.solana.com".to_owned()),
    );
    println!("RPC Client URL: {}", json_rpc_url);
    let client = RpcClient::new(json_rpc_url);

    let (sub_command, sub_matches) = app_matches.subcommand();
    match (sub_command, sub_matches) {
        ("mint", Some(arg_matches)) => {
            let address = arg_matches.value_of("address").unwrap();
            println!("Showing mint {}", address);
            show_mint(client, Pubkey::from_str(address).unwrap());
        }
        ("account", Some(arg_matches)) => {
            let address = arg_matches.value_of("address").unwrap();
            println!("Showing account {}", address);
            show_account(client, Pubkey::from_str(address).unwrap());
        }
        ("metadata", Some(arg_matches)) => {
            let address = arg_matches.value_of("address").unwrap();
            println!("Showing metaplex metadata {}", address);
            show_metadata(client, Pubkey::from_str(address).unwrap());
        }
        ("master-edition", Some(arg_matches)) => {
            let address = arg_matches.value_of("address").unwrap();
            println!("Showing metaplex master edition {}", address);
            show_master_edition(client, Pubkey::from_str(address).unwrap());
        }
        ("nft", Some(arg_matches)) => {
            let address = arg_matches.value_of("address").unwrap();
            println!("Showing NFT (by Mint address) {}", address);
            show_nft(client, Pubkey::from_str(address).unwrap());
        }
        _ => unreachable!(),
    }
}
