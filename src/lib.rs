#![no_std]

/// A 160 bit number representing an ethereum address.
pub type Address = &'static [u8; 20];
// A 128 bit number representing an amount of ether
pub type Amount = &'static [u8; 16];
// A 256 bit number representing data
pub type Data = &'static [u8; 32];
// A 256 bit number representing a node's hash
pub type Hash = Data;

// A result number for a call
#[repr(C)]
pub enum CallResult {
    Success = 0,
    Failure = 1,
    Revert = 2,
}

extern "C" {
    /// Subtracts an amount to the gas counter
    ///
    /// Arguments:
    /// * `amount`: the amount to subtract to the gas counter.
    pub fn ethereum_useGas(amount: i64);

    /// Get the address of currently executing account.
    ///
    /// Arguments:
    /// * `address_ptr`: the memory location at which the address is to be stored.
    pub fn ethereum_getAddress(address_ptr: *mut Address);

    /// Gets ethereum balance of the given account.
    ///
    /// Arguments:
    /// * `address_ptr`: The memory location to load the address from.
    /// * `ether_amount_ptr`:  The memory location to load the ethereum amount into.
    pub fn ethereum_getExternalBalance(address_ptr: *const Address, ether_amount_ptr: *mut Amount);

    /// Gets the hash of a recent block.
    ///
    /// Arguments:
    /// * `number`: The index of the last 256 blocks to load the hash of.
    /// * `hash_ptr`:  The memory location to load the hash into.
    /// Note: the hash memory value will not change if a failure occurs.
    pub fn ethereum_getBlockHash(number: i64, hash_ptr: *mut Hash);

    /// Send a message including data and/or ether to and address
    ///
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

    /// Get data from the current environment's input data.
    ///
    /// Arguments:
    /// * `result_ptr`: The memory location where to copy the result to.
    /// * `input_offset`: The memory location to load input data from.
    /// * `input_length`:  The length in bytes of input data to copy.
    pub fn ethereum_callDataCopy(result_ptr: *mut u8, input_offset: *mut u8, input_length: i32);

    /// Get size of input data in current environment's message or transaction.
    ///
    /// Returns the length in bytes.
    pub fn ethereum_getCallDataSize() -> i32;

    /// DEPRECATED, use callDelegate.
    pub fn ethereum_callCode(
        gas_limit: i64,
        address_ptr: *const Address,
        ether_amount_ptr: *const Amount,
        data_ptr: *const u8,
        data_length: i32,
    ) -> CallResult;

    /// Call another contract on behalf of the original caller.
    ///
    /// Arguments:
    /// * `gas_limit`: The gas limit of the message.
    /// * `address_ptr`: The memory location to load the address from.
    /// * `data_ptr`:  The memory location to load the data from.
    /// * `data_length`:  The length in bytes of data to send.
    pub fn ethereum_callDelegate(
        gas_limit: i64,
        address_ptr: *const Address,
        data_ptr: *const u8,
        data_length: i32,
    ) -> CallResult;

    /// Call a node by it's address, but do not modify it's state. This can be used for testing a transaction.
    ///
    /// Arguments:
    /// * `gas_limit`: The gas limit of the message.
    /// * `address_ptr`: The memory location to load the address from.
    /// * `data_ptr`:  The memory location to load the data from.
    /// * `data_length`:  The length in bytes of data to send.
    pub fn ethereum_callStatic(
        gas_limit: i64,
        address_ptr: *const Address,
        data_ptr: *const u8,
        data_length: i32,
    ) -> CallResult;

    /// Store data from memory
    ///
    /// Arguments:
    /// * `path_ptr`: The path in storage
    /// * `data_ptr`: The pointer to data
    pub fn ethereum_storeData(path_ptr: *const Data, data_ptr: *const Data);

    /// Load data from storage into memory.
    ///
    /// Arguments:
    /// * `path_ptr`: The path in storage
    /// * `data_ptr`: The pointer to memory where to put data
    pub fn ethereum_loadData(path_ptr: *const Data, data_ptr: *mut Data);

    /// Get the address of the caller and put it in memory.
    ///
    /// Arguments:
    /// * `address_ptr`: The pointer to memory where to put the address
    pub fn ethereum_getCaller(address_ptr: *mut Address);

    /// Get the deposited ether value of this transaction and puts the amount in memory.
    ///
    /// Arguments:
    /// * `amount_ptr`: The pointer to memory where to put the amount
    pub fn ethereum_getCallValue(amount_ptr: *mut Amount);
}
