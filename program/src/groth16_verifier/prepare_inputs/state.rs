use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};
use std::convert::TryInto;

#[derive(Clone)]
pub struct PrepareInputsState {
    is_initialized: bool,
    pub found_root: u8,
    pub found_nullifier: u8,
    pub executed_withdraw: u8,
    pub signing_address: Vec<u8>, // is relayer address
    pub relayer_refund: Vec<u8>,
    pub to_address: Vec<u8>,
    pub amount: Vec<u8>,
    pub nullifier_hash: Vec<u8>,
    pub root_hash: Vec<u8>,
    pub data_hash: Vec<u8>,         // is commit hash until changed
    pub tx_integrity_hash: Vec<u8>, // is calculated on-chain from to_address, amount, signing_address,

    pub i_1_range: Vec<u8>,
    pub x_1_range: Vec<u8>,
    pub i_2_range: Vec<u8>,
    pub x_2_range: Vec<u8>,
    pub i_3_range: Vec<u8>,
    pub x_3_range: Vec<u8>,
    pub i_4_range: Vec<u8>,
    pub x_4_range: Vec<u8>,
    pub i_5_range: Vec<u8>,
    pub x_5_range: Vec<u8>,
    pub i_6_range: Vec<u8>,
    pub x_6_range: Vec<u8>,
    pub i_7_range: Vec<u8>,
    pub x_7_range: Vec<u8>,

    pub res_x_range: Vec<u8>,
    pub res_y_range: Vec<u8>,
    pub res_z_range: Vec<u8>,
    pub g_ic_x_range: Vec<u8>,
    pub g_ic_y_range: Vec<u8>,
    pub g_ic_z_range: Vec<u8>,
    pub current_instruction_index: usize,

    pub proof_a_b_c_leaves_and_nullifiers: Vec<u8>,

    pub changed_variables: [bool; 20],
    pub changed_constants: [bool; 12],
}
impl Sealed for PrepareInputsState {}
impl IsInitialized for PrepareInputsState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for PrepareInputsState {
    const LEN: usize = 3900; // 1020

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, PrepareInputsState::LEN];

        let (
            _is_initialized,
            found_root,
            found_nullifier,
            executed_withdraw,
            signing_address, // is relayer address
            relayer_refund,
            to_address,
            amount,
            nullifier_hash,
            root_hash,
            data_hash, // is commit hash until changed
            tx_integrity_hash,
            current_instruction_index,
            i_1_range, // 32b
            x_1_range, // 96b + constructor
            i_2_range,
            x_2_range,
            i_3_range,
            x_3_range,
            i_4_range,
            x_4_range,
            i_5_range,
            x_5_range,
            i_6_range,
            x_6_range,
            i_7_range,
            x_7_range,
            res_x_range,
            res_y_range,
            res_z_range,
            g_ic_x_range,
            g_ic_y_range,
            g_ic_z_range, // 3*32
            //until here 1084 bytes
            _unused_remainder,
            proof_a_b_c_leaves_and_nullifiers,
        ) = array_refs![
            input, 1, 1, 1, 1, 32, 8, 32, 8, 32, 32, 32, 32, 8, 32, 64, 32, 64, 32, 64, 32, 64, 32,
            64, 32, 64, 32, 64, 32, 32, 32, 32, 32, 32,
            2432,
            384
        ];

        Ok(PrepareInputsState {
            is_initialized: true,

            found_root: found_root[0],                     //0
            found_nullifier: found_nullifier[0],           //1
            executed_withdraw: executed_withdraw[0],       //2
            signing_address: signing_address.to_vec(),     //3
            relayer_refund: relayer_refund.to_vec(),       //4
            to_address: to_address.to_vec(),               //5
            amount: amount.to_vec(),                       //6
            nullifier_hash: nullifier_hash.to_vec(),       //7
            root_hash: root_hash.to_vec(),                 //8
            data_hash: data_hash.to_vec(),                 //9
            tx_integrity_hash: tx_integrity_hash.to_vec(), //10
            proof_a_b_c_leaves_and_nullifiers: proof_a_b_c_leaves_and_nullifiers.to_vec(), //11

            current_instruction_index: usize::from_le_bytes(*current_instruction_index),
            i_1_range: i_1_range.to_vec(),       //0
            x_1_range: x_1_range.to_vec(),       //1
            i_2_range: i_2_range.to_vec(),       //2
            x_2_range: x_2_range.to_vec(),       //3
            i_3_range: i_3_range.to_vec(),       //4
            x_3_range: x_3_range.to_vec(),       //5
            i_4_range: i_4_range.to_vec(),       //6
            x_4_range: x_4_range.to_vec(),       //7
            i_5_range: i_5_range.to_vec(),       //8
            x_5_range: x_5_range.to_vec(),       //9
            i_6_range: i_6_range.to_vec(),       //10
            x_6_range: x_6_range.to_vec(),       //11
            i_7_range: i_7_range.to_vec(),       //12
            x_7_range: x_7_range.to_vec(),       //13
            res_x_range: res_x_range.to_vec(),   //14
            res_y_range: res_y_range.to_vec(),   //15
            res_z_range: res_z_range.to_vec(),   //16
            g_ic_x_range: g_ic_x_range.to_vec(), //17
            g_ic_y_range: g_ic_y_range.to_vec(), //18
            g_ic_z_range: g_ic_z_range.to_vec(), //19
            changed_variables: [false; 20],
            changed_constants: [false; 12],
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, PrepareInputsState::LEN];

        let (
            //constants
            is_initialized_dst,
            found_root_dst,
            found_nullifier_dst,
            executed_withdraw_dst,
            signing_address_dst, // is relayer address
            relayer_refund_dst,
            to_address_dst,
            amount_dst,
            nullifier_hash_dst,
            root_hash_dst,
            data_hash_dst,
            tx_integrity_hash_dst,
            //variables
            current_instruction_index_dst,
            //220
            i_1_range_dst,
            x_1_range_dst,
            i_2_range_dst,
            x_2_range_dst,
            i_3_range_dst,
            x_3_range_dst,
            i_4_range_dst,
            x_4_range_dst,
            i_5_range_dst,
            x_5_range_dst,
            i_6_range_dst,
            x_6_range_dst,
            i_7_range_dst,
            x_7_range_dst,
            res_x_range_dst,
            res_y_range_dst,
            res_z_range_dst,
            g_ic_x_range_dst,
            g_ic_y_range_dst,
            g_ic_z_range_dst,
            _unused_remainder_dst,
            proof_a_b_c_leaves_and_nullifiers_dst,
        ) = mut_array_refs![
            dst, 1, 1, 1, 1, 32, 8, 32, 8, 32, 32, 32, 32, 8, 32, 64, 32, 64, 32, 64, 32, 64, 32,
            64, 32, 64, 32, 64, 32, 32, 32, 32, 32, 32,
            2432,
            384
        ];
        for (i, var_has_changed) in self.changed_variables.iter().enumerate() {
            if *var_has_changed {
                if i == 0 {
                    *i_1_range_dst = self.i_1_range.clone().try_into().unwrap();
                } else if i == 1 {
                    *x_1_range_dst = self.x_1_range.clone().try_into().unwrap();
                } else if i == 2 {
                    *i_2_range_dst = self.i_2_range.clone().try_into().unwrap();
                } else if i == 3 {
                    *x_2_range_dst = self.x_2_range.clone().try_into().unwrap();
                } else if i == 4 {
                    *i_3_range_dst = self.i_3_range.clone().try_into().unwrap();
                } else if i == 5 {
                    *x_3_range_dst = self.x_3_range.clone().try_into().unwrap();
                } else if i == 6 {
                    *i_4_range_dst = self.i_4_range.clone().try_into().unwrap();
                } else if i == 7 {
                    *x_4_range_dst = self.x_4_range.clone().try_into().unwrap();
                } else if i == 8 {
                    *i_5_range_dst = self.i_5_range.clone().try_into().unwrap();
                } else if i == 9 {
                    *x_5_range_dst = self.x_5_range.clone().try_into().unwrap();
                } else if i == 10 {
                    *i_6_range_dst = self.i_6_range.clone().try_into().unwrap();
                } else if i == 11 {
                    *x_6_range_dst = self.x_6_range.clone().try_into().unwrap();
                } else if i == 12 {
                    *i_7_range_dst = self.i_7_range.clone().try_into().unwrap();
                } else if i == 13 {
                    *x_7_range_dst = self.x_7_range.clone().try_into().unwrap();
                } else if i == 14 {
                    *res_x_range_dst = self.res_x_range.clone().try_into().unwrap();
                } else if i == 15 {
                    *res_y_range_dst = self.res_y_range.clone().try_into().unwrap();
                } else if i == 16 {
                    *res_z_range_dst = self.res_z_range.clone().try_into().unwrap();
                } else if i == 17 {
                    *g_ic_x_range_dst = self.g_ic_x_range.clone().try_into().unwrap();
                } else if i == 18 {
                    *g_ic_y_range_dst = self.g_ic_y_range.clone().try_into().unwrap();
                } else if i == 19 {
                    *g_ic_z_range_dst = self.g_ic_z_range.clone().try_into().unwrap();
                }
            }
        }

        for (i, const_has_changed) in self.changed_constants.iter().enumerate() {
            if *const_has_changed {
                if i == 0 {
                    *found_root_dst = [self.found_root.clone(); 1];
                } else if i == 1 {
                    *found_nullifier_dst = [self.found_nullifier.clone(); 1];
                } else if i == 2 {
                    *executed_withdraw_dst = [self.executed_withdraw.clone(); 1];
                } else if i == 3 {
                    *signing_address_dst = self.signing_address.clone().try_into().unwrap();
                } else if i == 4 {
                    *relayer_refund_dst = self.relayer_refund.clone().try_into().unwrap();
                } else if i == 5 {
                    *to_address_dst = self.to_address.clone().try_into().unwrap();
                } else if i == 6 {
                    *amount_dst = self.amount.clone().try_into().unwrap();
                } else if i == 7 {
                    *nullifier_hash_dst = self.nullifier_hash.clone().try_into().unwrap();
                } else if i == 8 {
                    *root_hash_dst = self.root_hash.clone().try_into().unwrap();
                } else if i == 9 {
                    *data_hash_dst = self.data_hash.clone().try_into().unwrap();
                } else if i == 10 {
                    *tx_integrity_hash_dst = self.tx_integrity_hash.clone().try_into().unwrap();
                } else if i == 11 {
                    *proof_a_b_c_leaves_and_nullifiers_dst = self
                        .proof_a_b_c_leaves_and_nullifiers
                        .clone()
                        .try_into()
                        .unwrap();
                }
            }
        }
        *current_instruction_index_dst = usize::to_le_bytes(self.current_instruction_index);
        //TODO: remove, rn crashes if removed
        if self.is_initialized {
            *is_initialized_dst = [1u8; 1];
        }

    }
}
