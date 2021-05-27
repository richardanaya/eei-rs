#![no_std]

extern "C" {
    /// Subtracts an amount to the gas counter
    /// Arguments:
    /// * `amount`: the amount to subtract to the gas counter.
    pub fn ethereum_useGas(amount: i64);
}
