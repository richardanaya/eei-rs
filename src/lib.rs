#![no_std]

/// A 160 bit number, represented as a 20 bytes long little endian unsigned integer in memory.
type Address = &'static [u8; 20];

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
    /// * `amount_ptr`:  The memory location to load the balance into.
    pub fn ethereum_getExternalBalance(address_ptr: *const Address, amount_ptr: *mut u128);
}
