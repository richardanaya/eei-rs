#![no_std]

/// A 160 bit number, represented as a 20 bytes long little endian unsigned integer in memory.
type Address = &'static [u8; 20];

extern "C" {
    /// Subtracts an amount to the gas counter
    /// Arguments:
    /// * `amount`: the amount to subtract to the gas counter.
    pub fn ethereum_useGas(amount: i64);

    /// Get the address of currently executing account and stores it in memory at the given offset.
    /// Arguments:
    /// * `resultOffset`: the memory offset at which the address is to be stored.
    pub fn ethereum_getAddress(resultOffset: *const Address);
}
