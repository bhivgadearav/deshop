use anchor_lang::prelude::*;

pub const GLOBAL_INFO_STATIC_SEED: &[u8] = b"global_info";
pub const SELLER_INFO_STATIC_SEED: &[u8] = b"seller_info";

#[account]
pub struct GlobalInfo {
    pub is_initialized: bool, // 1 Byte
    pub admin: Pubkey,        // 32 Bytes
    pub currency: Pubkey,     // 32 Bytes
}

impl GlobalInfo {
    pub const LEN: usize = 1 + 32 + 32;
}

#[account]
pub struct SellerInfo {
    pub is_initialized: bool, // 1 Byte
    pub seller: Pubkey,       // 32 Bytes
    pub balance: u64,         // 64 / 8 = 8 Bytes
}

impl SellerInfo {
    pub const LEN: usize = 1 + 32 + 8;
}
