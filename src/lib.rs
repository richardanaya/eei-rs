#![no_std]

/// A 160 bit number, represented as a 20 bytes long little endian unsigned integer in memory.
pub type Address = &'static [u8; 20];
pub type Amount = &'static [u8; 16];
pub type Hash = &'static [u8; 32];

pub enum CallResult {
    Success = 0,
    Failure = 1,
    Revert = 2,
}

extern "C" {
    /// Subtracts an amount to the gas counter
    /// Arguments:
    /// * `amount`: the amount to subtract to the gas counter.
    pub fn ethereum_useGas(amount: i64);

    /// Get the address of currently executing account.
    /// Arguments:
    /// * `address_ptr`: the memory location at which the address is to be stored.
    pub fn ethereum_getAddress(address_ptr: *mut Address);

    /// Gets ethereum balance of the given account.
    /// Arguments:
    /// * `address_ptr`: The memory location to load the address from.
    /// * `ether_amount_ptr`:  The memory location to load the ethereum amount into.
    pub fn ethereum_getExternalBalance(address_ptr: *const Address, ether_amount_ptr: *mut Amount);

    /// Gets the hash of a recent block.
    /// Arguments:
    /// * `number`: The index of the last 256 blocks to load the hash of.
    /// * `hash_ptr`:  The memory location to load the hash into.
    /// Note: the hash memory value will not change if a failure occurs.
    pub fn ethereum_getBlockHash(number: i64, hash_ptr: *mut Hash);

    /// Send a message including data and/or ether to and address
    /// Arguments:
    /// * `gas_limit`: The gas limit of the message.
    /// * `address_ptr`: The memory location to load the address from.
    /// * `ether_amount_ptr`:  The memory location to load the ethereum amount from.
    /// * `data_ptr`:  The memory location to load the data from.
    /// * `data_length`:  The length in bytes of data to send.
    pub fn ethereum_call(
        gas_limit: i64,
        address_ptr: *const Address,
        ether_amount_ptr: *const Amount,
        data_ptr: *const u8,
        data_length: i32,
    ) -> CallResult;
}
