use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_sdk::{account::Account, signature::Signer, transaction::Transaction, msg, transport::TransportError},
    Testing_Hardcoded_Params_devnet_new::{process_instruction, state_merkle_tree::{MerkleTree,HashBytes,MERKLE_TREE_ACC_BYTES}, state_final_exp::INSTRUCTION_ORDER_VERIFIER_PART_2},
    std::str::FromStr,
};
use solana_program_test::ProgramTestError;
use solana_program::program_pack::Pack;
use solana_sdk::signer::keypair::Keypair;
use crate::tokio::runtime::Runtime;
use ark_ed_on_bn254::Fq;
use ark_std::{One};
use ark_ff::{PrimeField, BigInteger, Fp256};
use arkworks_gadgets::poseidon::{PoseidonError, PoseidonParameters, Rounds,circom::CircomCRH, sbox::PoseidonSbox};
use ark_crypto_primitives::{crh::{TwoToOneCRH}};
use ark_ff::bytes::{FromBytes, ToBytes};
use arkworks_gadgets::utils::{
	get_mds_poseidon_circom_bn254_x5_3, get_rounds_poseidon_circom_bn254_x5_3, parse_vec,
};
use ark_std::{UniformRand, test_rng};
use crate::tokio::time::timeout;

pub const instruction_order: [u8; 700] = [0,1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 10, 11, 12, 13, 14, 15, 19, 20, 20, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 20, 27, 28, 29, 30, 31, 32, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 27, 28, 29, 30, 31, 32, 20, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 27, 28, 29, 30, 31, 32, 20, 20, 27, 28, 29, 30, 31, 32, 20, 20, 27, 28, 29, 30, 31, 32, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 27, 28, 29, 30, 31, 32, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 20, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 21, 22, 23, 24, 25, 26, 20, 20, 20, 20, 20, 27, 28, 29, 30, 31, 32, 20, 20, 20, 20, 21, 22, 23, 24, 25, 26, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 42, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 42, 49, 50, 51, 52, 53, 54, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 49, 50, 51, 52, 53, 54, 42, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 49, 50, 51, 52, 53, 54, 42, 42, 49, 50, 51, 52, 53, 54, 42, 42, 49, 50, 51, 52, 53, 54, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 49, 50, 51, 52, 53, 54, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 42, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 43, 44, 45, 46, 47, 48, 42, 42, 42, 42, 42, 49, 50, 51, 52, 53, 54, 42, 42, 42, 42, 43, 44, 45, 46, 47, 48, 55, 56, 57, 57, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 57, 64, 65, 66, 67, 68, 69, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 64, 65, 66, 67, 68, 69, 57, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 64, 65, 66, 67, 68, 69, 57, 57, 64, 65, 66, 67, 68, 69, 57, 57, 64, 65, 66, 67, 68, 69, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 64, 65, 66, 67, 68, 69, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 57, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 58, 59, 60, 61, 62, 63, 57, 57, 57, 57, 57, 64, 65, 66, 67, 68, 69, 57, 57, 57, 57, 58, 59, 60, 61, 62, 63, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 71, 72, 73, 74, 75, 76, 112, 113, 114, 115, 116, 117, 118, 119, 120, 83, 84, 85, 86, 87, 88];
use solana_program_test::ProgramTestContext;

use std::{thread, time};

async fn create_and_start_program(account_init_bytes: Vec<u8>, final_exp_bytes_pubkey: Pubkey,program_id: Pubkey) -> ProgramTestContext {

    let mut program_test = ProgramTest::new(
        "Testing_Hardcoded_Params_devnet_new",
        program_id,
        processor!(process_instruction),
    );
    let merkle_tree_pubkey = Pubkey::new(&MERKLE_TREE_ACC_BYTES);

    let mut account_exp = Account::new(10000000000, 3772, &program_id);
    account_exp.data = account_init_bytes;
    program_test.add_account(
        final_exp_bytes_pubkey,
        account_exp,
    );
    program_test.add_account(
        merkle_tree_pubkey,
        Account::new(10000000000, 135057, &program_id),
    );
    let tmp = program_test.start_with_context().await;
    tmp

}

#[tokio::test]
async fn test_final_exp_correct()-> Result<(), TransportError> {
    let program_id = Pubkey::from_str("TransferLamports111111111111111111111111111").unwrap();

    let final_exp_bytes_pubkey = Pubkey::new_unique();

    let mut program_context = create_and_start_program(INIT_BYTES_FINAL_EXP.to_vec(), final_exp_bytes_pubkey, program_id).await;

    //initialize MerkleTree account
    //
    // let mut transaction = Transaction::new_with_payer(
    //     &[Instruction::new_with_bincode(
    //         program_id,
    //         &[240, 0],
    //         vec![
    //             AccountMeta::new(program_context.payer.pubkey(),true),
    //             AccountMeta::new(merkle_tree_pubkey, false),
    //         ],
    //     )],
    //     Some(&program_context.payer.pubkey()),
    // );
    // transaction.sign(&[&program_context.payer], program_context.last_blockhash);
    //
    // program_context.banks_client.process_transaction(transaction).await.unwrap();
    //
    // let merkle_tree_data = program_context.banks_client
    //     .get_account(merkle_tree_pubkey)
    //     .await
    //     .expect("get_account").unwrap();
    //println!("merkletree: {:?}", merkle_tree_data);
    //assert_eq!(init_bytes11::INIT_BYTES_MERKLE_TREE_11, merkle_tree_data.data[0..769]);
    let init_data = program_context.banks_client
        .get_account(final_exp_bytes_pubkey)
        .await
        .expect("get_account").unwrap();
    assert_eq!(init_data.data, INIT_BYTES_FINAL_EXP);

    println!("initializing merkle tree success");


    //assert_eq!();

    //generating random commitment
    // let mut rng = test_rng();
    // let left_input = Fp256::<ark_ed_on_bn254::FqParameters>::rand(&mut rng).into_repr().to_bytes_le();
    // let right_input = Fp256::<ark_ed_on_bn254::FqParameters>::rand(&mut rng).into_repr().to_bytes_le();
    //let commit = vec![143, 120, 199, 24, 26, 175, 31, 125, 154, 127, 245, 235, 132, 57, 229, 4, 60, 255, 3, 234, 105, 16, 109, 207, 16, 139, 73, 235, 137, 17, 240, 2];//get_poseidon_ref_hash(&left_input[..], &right_input[..]);

    //padding to make every tx unique otherwise the test will not execute repeated instructions

    let mut i = 0usize;
    for (instruction_id) in INSTRUCTION_ORDER_VERIFIER_PART_2 {
        //println!("instruction data {:?}", [vec![*instruction_id, 0u8], left_input.clone(), right_input.clone(), [i as u8].to_vec() ].concat());
        //let instruction_data: Vec<u8> = [vec![*instruction_id, 1u8], commit.clone(), [i as u8].to_vec() ].concat();
        println!("INSTRUCTION_ORDER_VERIFIER_PART_2: {}", instruction_id);

        let mut success = false;
        let mut retries_left = 2;
        while(retries_left > 0 && success != true ) {
            println!("success: {}", success);
            let mut transaction = Transaction::new_with_payer(
                &[Instruction::new_with_bincode(
                    program_id,
                    &[vec![instruction_id, 2u8], usize::to_le_bytes(i).to_vec()].concat(),
                    vec![
                        AccountMeta::new(program_context.payer.pubkey(),true),
                        AccountMeta::new(final_exp_bytes_pubkey, false),
                        //AccountMeta::new(merkle_tree_pubkey, false),
                    ],
                )],
                Some(&program_context.payer.pubkey()),
            );
            transaction.sign(&[&program_context.payer], program_context.last_blockhash);
            //tokio::time::timeout(std::time::Duration::from_secs(2), self.process).await
            let res_request = timeout(time::Duration::from_millis(500), program_context.banks_client.process_transaction(transaction)).await;
            //let ten_millis = time::Duration::from_millis(400);

            //thread::sleep(ten_millis);
            //println!("res: {:?}", res_request);
            match res_request {
                Ok(_) => success = true,
                Err(e) => {

                    println!("retries_left {}", retries_left);
                    retries_left -=1;
                    let storage_account = program_context.banks_client
                        .get_account(final_exp_bytes_pubkey)
                        .await
                        .expect("get_account").unwrap();
                    //println!("data: {:?}", storage_account.data);
                    program_context = create_and_start_program(storage_account.data.to_vec(), final_exp_bytes_pubkey, program_id).await;                },
            }
        }

        i+=1;
    }

    //bytes resulting from successful final_exp_offchain test equivalent to respective VerifyingKey bytes
    let expected_result_bytes = vec![198, 242, 4, 28, 9, 35, 146, 101, 152, 133, 231, 128, 253, 46, 174, 170, 116, 96, 135, 45, 77, 156, 161, 40, 238, 232, 55, 247, 15, 79, 136, 20, 73, 78, 229, 119, 48, 86, 133, 39, 142, 172, 194, 67, 33, 2, 66, 111, 127, 20, 159, 85, 92, 82, 21, 187, 149, 99, 99, 91, 169, 57, 127, 10, 238, 159, 54, 204, 152, 63, 242, 50, 16, 39, 141, 61, 149, 81, 36, 246, 69, 1, 232, 157, 153, 3, 1, 25, 105, 84, 109, 205, 9, 78, 8, 26, 113, 240, 149, 249, 171, 170, 41, 39, 144, 143, 89, 229, 207, 106, 60, 195, 236, 5, 73, 82, 126, 170, 50, 181, 192, 135, 129, 217, 185, 227, 223, 0, 50, 203, 114, 165, 128, 252, 58, 245, 74, 48, 92, 144, 199, 108, 126, 82, 103, 46, 23, 236, 159, 71, 113, 45, 183, 105, 200, 135, 142, 182, 196, 3, 138, 113, 217, 236, 105, 118, 157, 226, 54, 90, 23, 215, 59, 110, 169, 133, 96, 175, 12, 86, 33, 94, 130, 8, 57, 246, 139, 86, 246, 147, 174, 17, 57, 27, 122, 247, 174, 76, 162, 173, 26, 134, 230, 177, 70, 148, 183, 2, 54, 46, 65, 165, 64, 15, 42, 11, 245, 15, 136, 32, 213, 228, 4, 27, 176, 63, 169, 82, 178, 89, 227, 58, 204, 40, 159, 210, 216, 255, 223, 194, 117, 203, 57, 49, 152, 42, 162, 80, 248, 55, 92, 240, 231, 192, 161, 14, 169, 65, 231, 215, 238, 131, 144, 139, 153, 142, 76, 100, 40, 134, 147, 164, 89, 148, 195, 194, 117, 36, 53, 100, 231, 61, 164, 217, 129, 190, 160, 44, 30, 94, 13, 159, 6, 83, 126, 195, 26, 86, 113, 177, 101, 79, 110, 143, 220, 57, 110, 235, 91, 73, 189, 191, 253, 187, 76, 214, 232, 86, 132, 6, 135, 153, 111, 175, 12, 109, 157, 73, 181, 171, 29, 118, 147, 102, 65, 153, 99, 57, 198, 45, 85, 153, 67, 208, 177, 113, 205, 237, 210, 233, 79, 46, 231, 168, 16, 11, 21, 249, 174, 127, 70, 3, 32, 60, 115, 188, 192, 101, 159, 85, 66, 193, 194, 157, 76, 121, 108, 222, 128, 27, 15, 163, 156, 8];
    //println!("storage_acc: {:?}", storage_account.data[700..(769+128)].to_vec());
    //assert_eq!(expected_root, storage_account.data[769..(769+32)]);
    let storage_account = program_context.banks_client
        .get_account(final_exp_bytes_pubkey)
        .await
        .expect("get_account").unwrap();
    let mut unpacked_data = vec![0u8;3772];

    unpacked_data = storage_account.data.clone();

    // for i in 0..2140 {
    //     print!("{}, ",unpacked_data[i]);
    // }
    // println!("Len data: {}", storage_account.data.len());

    assert_eq!(expected_result_bytes, unpacked_data[1756..2140]);

    //assert_eq!(unpacked_data[1..33], poseidon_hash_ref);

    //let data = <PoseidonHashMemory as Pack>::unpack_from_slice(&unpacked_data).unwrap();

    // let storage_account = program_context.banks_client
    //     .get_packed_account_data::<PoseidonHashMemory>(merkle_tree_pubkey)
    //     .await
    //     .expect("get_packed_account_data");
    //println!("{:?}",unpacked_data[1..33]);
    // let storage_account = program_context.banks_client
    //     .get_packed_account_data::<Testing_Hardcoded_Params::PoseidonHashMemory>(merkle_tree_pubkey)
    //     .await
    //     .expect("get_packed_account_data");
    // //let data = Testing_Hardcoded_Params::PoseidonHashMemory::unpack(&storage_account.data).unwrap();
    Ok(())
}



pub const INIT_BYTES_FINAL_EXP : [u8;3772] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 164, 125, 219, 237, 181, 202, 195, 98, 55, 97, 232, 35, 147, 153, 23, 164, 70, 211, 144, 151, 9, 219, 197, 234, 13, 164, 242, 67, 59, 148, 5, 132, 108, 82, 161, 228, 167, 20, 24, 207, 201, 203, 25, 249, 125, 54, 96, 182, 231, 150, 215, 149, 43, 216, 0, 36, 166, 232, 13, 126, 3, 53, 0, 174, 209, 16, 242, 177, 143, 60, 247, 181, 65, 132, 142, 14, 231, 170, 52, 3, 34, 70, 49, 210, 158, 211, 173, 165, 155, 219, 80, 225, 32, 64, 8, 65, 139, 16, 138, 240, 218, 36, 220, 8, 100, 236, 141, 1, 223, 60, 59, 24, 38, 90, 254, 47, 91, 205, 228, 169, 103, 178, 30, 124, 141, 43, 9, 83, 155, 75, 140, 209, 26, 2, 250, 250, 20, 185, 78, 53, 54, 68, 178, 88, 78, 246, 132, 97, 167, 124, 253, 96, 26, 213, 99, 157, 155, 40, 9, 60, 139, 112, 126, 230, 195, 217, 125, 68, 169, 208, 149, 175, 33, 226, 17, 47, 132, 8, 154, 237, 156, 34, 97, 55, 129, 155, 64, 202, 54, 161, 19, 24, 1, 208, 104, 140, 149, 25, 229, 96, 239, 202, 24, 235, 221, 133, 137, 30, 226, 62, 112, 26, 58, 1, 85, 207, 182, 41, 213, 42, 72, 139, 41, 108, 152, 252, 164, 121, 76, 17, 62, 147, 226, 220, 79, 236, 132, 109, 130, 163, 209, 203, 14, 144, 180, 25, 216, 234, 198, 199, 74, 48, 62, 57, 0, 206, 138, 12, 130, 25, 12, 187, 216, 86, 208, 84, 198, 58, 204, 6, 161, 93, 63, 68, 121, 173, 129, 255, 249, 47, 42, 218, 214, 129, 29, 136, 7, 213, 160, 139, 148, 58, 6, 191, 11, 161, 114, 56, 174, 224, 86, 243, 103, 166, 151, 107, 36, 205, 170, 206, 196, 248, 251, 147, 91, 3, 136, 208, 36, 3, 51, 84, 102, 139, 252, 193, 9, 172, 113, 116, 50, 242, 70, 26, 115, 166, 252, 204, 163, 149, 78, 13, 255, 235, 222, 174, 120, 182, 178, 186, 22, 169, 153, 73, 48, 242, 139, 120, 98, 33, 101, 204, 204, 169, 57, 249, 168, 45, 197, 126, 105, 54, 187, 35, 241, 253, 4, 33, 70, 246, 206, 32, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
