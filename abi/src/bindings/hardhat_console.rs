pub use hardhat_console::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod hardhat_console {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"}],\"name\":\"logAddress\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"}],\"name\":\"logBool\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"p0\",\"type\":\"bytes\"}],\"name\":\"logBytes\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes1\",\"name\":\"p0\",\"type\":\"bytes1\"}],\"name\":\"logBytes1\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes10\",\"name\":\"p0\",\"type\":\"bytes10\"}],\"name\":\"logBytes10\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes11\",\"name\":\"p0\",\"type\":\"bytes11\"}],\"name\":\"logBytes11\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes12\",\"name\":\"p0\",\"type\":\"bytes12\"}],\"name\":\"logBytes12\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes13\",\"name\":\"p0\",\"type\":\"bytes13\"}],\"name\":\"logBytes13\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes14\",\"name\":\"p0\",\"type\":\"bytes14\"}],\"name\":\"logBytes14\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes15\",\"name\":\"p0\",\"type\":\"bytes15\"}],\"name\":\"logBytes15\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes16\",\"name\":\"p0\",\"type\":\"bytes16\"}],\"name\":\"logBytes16\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes17\",\"name\":\"p0\",\"type\":\"bytes17\"}],\"name\":\"logBytes17\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes18\",\"name\":\"p0\",\"type\":\"bytes18\"}],\"name\":\"logBytes18\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes19\",\"name\":\"p0\",\"type\":\"bytes19\"}],\"name\":\"logBytes19\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes2\",\"name\":\"p0\",\"type\":\"bytes2\"}],\"name\":\"logBytes2\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes20\",\"name\":\"p0\",\"type\":\"bytes20\"}],\"name\":\"logBytes20\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes21\",\"name\":\"p0\",\"type\":\"bytes21\"}],\"name\":\"logBytes21\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes22\",\"name\":\"p0\",\"type\":\"bytes22\"}],\"name\":\"logBytes22\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes23\",\"name\":\"p0\",\"type\":\"bytes23\"}],\"name\":\"logBytes23\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes24\",\"name\":\"p0\",\"type\":\"bytes24\"}],\"name\":\"logBytes24\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes25\",\"name\":\"p0\",\"type\":\"bytes25\"}],\"name\":\"logBytes25\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes26\",\"name\":\"p0\",\"type\":\"bytes26\"}],\"name\":\"logBytes26\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes27\",\"name\":\"p0\",\"type\":\"bytes27\"}],\"name\":\"logBytes27\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes28\",\"name\":\"p0\",\"type\":\"bytes28\"}],\"name\":\"logBytes28\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes29\",\"name\":\"p0\",\"type\":\"bytes29\"}],\"name\":\"logBytes29\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes3\",\"name\":\"p0\",\"type\":\"bytes3\"}],\"name\":\"logBytes3\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes30\",\"name\":\"p0\",\"type\":\"bytes30\"}],\"name\":\"logBytes30\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes31\",\"name\":\"p0\",\"type\":\"bytes31\"}],\"name\":\"logBytes31\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"p0\",\"type\":\"bytes32\"}],\"name\":\"logBytes32\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"p0\",\"type\":\"bytes4\"}],\"name\":\"logBytes4\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes5\",\"name\":\"p0\",\"type\":\"bytes5\"}],\"name\":\"logBytes5\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes6\",\"name\":\"p0\",\"type\":\"bytes6\"}],\"name\":\"logBytes6\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes7\",\"name\":\"p0\",\"type\":\"bytes7\"}],\"name\":\"logBytes7\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes8\",\"name\":\"p0\",\"type\":\"bytes8\"}],\"name\":\"logBytes8\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes9\",\"name\":\"p0\",\"type\":\"bytes9\"}],\"name\":\"logBytes9\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"p0\",\"type\":\"int256\"}],\"name\":\"logInt\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"}],\"name\":\"logString\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"}],\"name\":\"logUint\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"p0\",\"type\":\"int256\"}],\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"log\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"int256\",\"name\":\"p1\",\"type\":\"int256\"}],\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"log\"}]";
    ///The parsed JSON ABI of the contract.
    pub static HARDHATCONSOLE_ABI: ::corebc_contract::Lazy<::corebc_core::abi::Abi> = ::corebc_contract::Lazy::new(||
    ::corebc_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct HardhatConsole<M>(::corebc_contract::Contract<M>);
    impl<M> ::core::clone::Clone for HardhatConsole<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HardhatConsole<M> {
        type Target = ::corebc_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HardhatConsole<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HardhatConsole<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(HardhatConsole)).field(&self.address()).finish()
        }
    }
    impl<M: ::corebc_providers::Middleware> HardhatConsole<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `corebc::Contract` object.
        pub fn new<T: Into<::corebc_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::corebc_contract::Contract::new(
                    address.into(),
                    HARDHATCONSOLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `log` (0xf5518d02) function
        pub fn log_23(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 81, 141, 2], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb7b720f1) function
        pub fn log_87(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 183, 32, 241], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x697ca861) function
        pub fn log_24(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 124, 168, 97], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc7b92562) function
        pub fn log_88(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 185, 37, 98], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4c448553) function
        pub fn log_89(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 68, 133, 83], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xaa818e86) function
        pub fn log_90(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 129, 142, 134], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcab56040) function
        pub fn log_91(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 181, 96, 64], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2c4fc75b) function
        pub fn log_25(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 79, 199, 91], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbf4ad3f2) function
        pub fn log_92(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 74, 211, 242], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x51632250) function
        pub fn log_93(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 99, 34, 80], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1014f2c1) function
        pub fn log_94(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 20, 242, 193], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6d5bf117) function
        pub fn log_95(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 91, 241, 23], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6b885c5b) function
        pub fn log_96(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 136, 92, 91], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf9d77a15) function
        pub fn log_26(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 215, 122, 21], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6407b4d8) function
        pub fn log_97(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 7, 180, 216], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x053bf2f0) function
        pub fn log_98(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 59, 242, 240], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x19a2eca8) function
        pub fn log_99(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 162, 236, 168], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0f44fef8) function
        pub fn log_100(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 68, 254, 248], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbe28811c) function
        pub fn log_101(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 40, 129, 28], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x25dc7e21) function
        pub fn log_102(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 220, 126, 33], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x516f1022) function
        pub fn log_27(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 111, 16, 34], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc1d8bdfd) function
        pub fn log_28(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 216, 189, 253], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdc74f6fe) function
        pub fn log_103(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 116, 246, 254], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8dc489a6) function
        pub fn log_29(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 196, 137, 166], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe1c37768) function
        pub fn log_104(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 195, 119, 104], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x10a511e4) function
        pub fn log_105(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 165, 17, 228], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4b0fe5b2) function
        pub fn log_106(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 15, 229, 178], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x65f7f895) function
        pub fn log_107(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 247, 248, 149], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x38490c03) function
        pub fn log_108(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 73, 12, 3], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x642f1c8f) function
        pub fn log_109(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 47, 28, 143], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd0001512) function
        pub fn log_110(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 0, 21, 18], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc23055b4) function
        pub fn log_111(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 48, 85, 180], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb54ffdc4) function
        pub fn log_30(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 79, 253, 196], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa3fcc8db) function
        pub fn log_31(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 252, 200, 219], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbca7d815) function
        pub fn log_112(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 167, 216, 21], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x93a1d6e4) function
        pub fn log_113(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 161, 214, 228], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xead3ed41) function
        pub fn log_114(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 211, 237, 65], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdd3bab2e) function
        pub fn log_115(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 59, 171, 46], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8c3dc7fd) function
        pub fn log_116(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 61, 199, 253], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x69bd1047) function
        pub fn log_32(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 189, 16, 71], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbec0d1ab) function
        pub fn log_6(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 192, 209, 171], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1d531481) function
        pub fn log_117(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 83, 20, 129], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x57a8d06a) function
        pub fn log_118(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 168, 208, 106], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4ebc3f88) function
        pub fn log_119(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 188, 63, 136], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8d4f60f3) function
        pub fn log_120(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 79, 96, 243], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe793f437) function
        pub fn log_33(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 147, 244, 55], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x12fd3334) function
        pub fn log_121(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 253, 51, 52], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc10bbfc9) function
        pub fn log_34(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 11, 191, 201], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe7413910) function
        pub fn log_122(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 65, 57, 16], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8ee09221) function
        pub fn log_35(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 224, 146, 33], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf9c29ca1) function
        pub fn log_123(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 194, 156, 161], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x87f4f2da) function
        pub fn log_124(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 244, 242, 218], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x73465a65) function
        pub fn log_125(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 70, 90, 101], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2b451fdd) function
        pub fn log_126(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 69, 31, 221], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa2ff2343) function
        pub fn log_127(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 255, 35, 67], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe718bc5d) function
        pub fn log_128(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 24, 188, 93], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6d73aa3f) function
        pub fn log_129(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 115, 170, 63], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x986699a1) function
        pub fn log_36(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 102, 153, 161], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xaa05db7c) function
        pub fn log_130(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 5, 219, 124], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe0d286c9) function
        pub fn log_131(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 210, 134, 201], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x919176f1) function
        pub fn log_132(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 145, 118, 241], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1eae6cc6) function
        pub fn log_7(
            &self,
            p_0: bool,
            p_1: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 174, 108, 198], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf9112a45) function
        pub fn log_133(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 17, 42, 69], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7b1bdc49) function
        pub fn log_134(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 27, 220, 73], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdd71ee01) function
        pub fn log_135(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 113, 238, 1], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x18a1462f) function
        pub fn log_136(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 161, 70, 47], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc371d1a1) function
        pub fn log_1(
            &self,
            p_0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 113, 209, 161], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa4897300) function
        pub fn log_137(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 137, 115, 0], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf8d24eed) function
        pub fn log_37(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 210, 78, 237], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdb884c5c) function
        pub fn log_138(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 136, 76, 92], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9f66a349) function
        pub fn log_139(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 102, 163, 73], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc985d2bd) function
        pub fn log_8(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 133, 210, 189], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x045ceb4e) function
        pub fn log_2(
            &self,
            p_0: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 92, 235, 78], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5f8b0ae8) function
        pub fn log_140(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 139, 10, 232], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x888f32ef) function
        pub fn log_141(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 143, 50, 239], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x46c3e4af) function
        pub fn log_38(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 195, 228, 175], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x98243bb8) function
        pub fn log_142(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 36, 59, 184], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x41f7f930) function
        pub fn log_143(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 247, 249, 48], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x92519541) function
        pub fn log_39(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 81, 149, 65], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x57fac99d) function
        pub fn log_144(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 250, 201, 157], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x408c80e0) function
        pub fn log_40(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 140, 128, 224], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf189c485) function
        pub fn log_145(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 137, 196, 133], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xee31dcd5) function
        pub fn log_146(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 49, 220, 213], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5faff71a) function
        pub fn log_9(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 175, 247, 26], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x67c65264) function
        pub fn log_147(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 198, 82, 100], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x97e2ba46) function
        pub fn log_148(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 226, 186, 70], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfde5f4b6) function
        pub fn log_149(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 229, 244, 182], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa2104f56) function
        pub fn log_150(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 16, 79, 86], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x67508e52) function
        pub fn log_151(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 80, 142, 82], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xedb86a18) function
        pub fn log_152(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 184, 106, 24], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcc9c3971) function
        pub fn log_153(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 156, 57, 113], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5d846457) function
        pub fn log_3(
            &self,
            p_0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 132, 100, 87], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8d6ad054) function
        pub fn log_154(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 106, 208, 84], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x12fd8d7c) function
        pub fn log_155(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 253, 141, 124], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa1ce8de0) function
        pub fn log_156(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 206, 141, 224], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x72de3baa) function
        pub fn log_157(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 222, 59, 170], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x400ab54c) function
        pub fn log_158(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 10, 181, 76], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6d0bcd28) function
        pub fn log_159(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 11, 205, 40], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc0b32706) function
        pub fn log_160(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 179, 39, 6], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x60413141) function
        pub fn log_161(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 65, 49, 65], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x05d12fd3) function
        pub fn log_41(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 209, 47, 211], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8e0922f8) function
        pub fn log_162(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 9, 34, 248], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xee100856) function
        pub fn log_163(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 16, 8, 86], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x331191d2) function
        pub fn log_164(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 17, 145, 210], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xca2b3de9) function
        pub fn log_165(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 43, 61, 233], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc36e01c0) function
        pub fn log_10(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 110, 1, 192], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9821418c) function
        pub fn log_166(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 33, 65, 140], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2d0972be) function
        pub fn log_42(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 9, 114, 190], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe72f81b8) function
        pub fn log_167(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 47, 129, 184], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd2401bda) function
        pub fn log_43(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 64, 27, 218], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x139b5077) function
        pub fn log_168(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 155, 80, 119], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x57219423) function
        pub fn log_169(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 33, 148, 35], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x177e481c) function
        pub fn log_0(&self) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 126, 72, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa0acbbc1) function
        pub fn log_170(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 172, 187, 193], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0161ead2) function
        pub fn log_171(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 97, 234, 210], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4cad4fb8) function
        pub fn log_172(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 173, 79, 184], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4cf45e04) function
        pub fn log_173(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 244, 94, 4], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbf291806) function
        pub fn log_44(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 41, 24, 6], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf62d0941) function
        pub fn log_45(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 45, 9, 65], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf72f2bbb) function
        pub fn log_174(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 47, 43, 187], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4fb261e2) function
        pub fn log_175(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 178, 97, 226], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xae7bdb09) function
        pub fn log_46(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 123, 219, 9], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb6971fc0) function
        pub fn log_176(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 151, 31, 192], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcf73b53e) function
        pub fn log_177(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 115, 181, 62], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1ba32771) function
        pub fn log_178(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 163, 39, 113], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa9c0e9e2) function
        pub fn log_47(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 192, 233, 226], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x794e1917) function
        pub fn log_179(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 78, 25, 23], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe484cd44) function
        pub fn log_180(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 132, 205, 68], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf2ce6a92) function
        pub fn log_181(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 206, 106, 146], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdbc6ed62) function
        pub fn log_182(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 198, 237, 98], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa9ff5e35) function
        pub fn log_183(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 255, 94, 53], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xce31d1cb) function
        pub fn log_184(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 49, 209, 203], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7d08b2b8) function
        pub fn log_185(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 8, 178, 184], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfa7945ae) function
        pub fn log_186(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 121, 69, 174], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa60ab0f6) function
        pub fn log_187(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 10, 176, 246], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x998e5d52) function
        pub fn log_188(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 142, 93, 82], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5fbb263d) function
        pub fn log_48(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 187, 38, 61], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x61a05c7c) function
        pub fn log_189(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 160, 92, 124], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x47735e3a) function
        pub fn log_190(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 115, 94, 58], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3ff220a9) function
        pub fn log_191(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 242, 32, 169], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc3218bed) function
        pub fn log_49(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 33, 139, 237], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcba009c7) function
        pub fn log_192(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 160, 9, 199], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa8682514) function
        pub fn log_11(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([168, 104, 37, 20], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xffcec76f) function
        pub fn log_193(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 206, 199, 111], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x11f010e1) function
        pub fn log_194(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 240, 16, 225], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa113de70) function
        pub fn log_195(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 19, 222, 112], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf9115a1e) function
        pub fn log_196(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 17, 90, 30], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x71fc19a1) function
        pub fn log_50(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 252, 25, 161], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0520d4a3) function
        pub fn log_51(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 32, 212, 163], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa7d947c0) function
        pub fn log_197(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 217, 71, 192], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9b933ca0) function
        pub fn log_198(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 147, 60, 160], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1fe72f17) function
        pub fn log_12(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 231, 47, 23], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6aae1b32) function
        pub fn log_199(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 174, 27, 50], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcca9c27f) function
        pub fn log_200(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 169, 194, 127], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x785b2def) function
        pub fn log_201(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 91, 45, 239], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3e6fe005) function
        pub fn log_202(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 111, 224, 5], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0d1846bc) function
        pub fn log_203(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 24, 70, 188], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x14567320) function
        pub fn log_204(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 86, 115, 32], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe84511a5) function
        pub fn log_205(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 69, 17, 165], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x00bdcf5d) function
        pub fn log_206(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 189, 207, 93], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3b63f413) function
        pub fn log_207(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 99, 244, 19], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x819bd95d) function
        pub fn log_208(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 155, 217, 93], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x46cef1ff) function
        pub fn log_209(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 206, 241, 255], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd43a08d0) function
        pub fn log_210(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 58, 8, 208], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1318bcfc) function
        pub fn log_52(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 24, 188, 252], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x96668786) function
        pub fn log_211(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 102, 135, 134], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1b4e65fe) function
        pub fn log_212(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 78, 101, 254], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x19c75a59) function
        pub fn log_213(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 199, 90, 89], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd9a56a6a) function
        pub fn log_13(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 165, 106, 106], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x228e68c4) function
        pub fn log_14(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 142, 104, 196], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xefa0f887) function
        pub fn log_214(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 160, 248, 135], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xde6942a7) function
        pub fn log_215(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 105, 66, 167], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0b0aa13c) function
        pub fn log_216(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 10, 161, 60], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x19290410) function
        pub fn log_53(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 41, 4, 16], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7b706e61) function
        pub fn log_54(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 112, 110, 97], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbaa5a4a6) function
        pub fn log_217(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 165, 164, 166], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0dd60ea3) function
        pub fn log_218(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 214, 14, 163], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x05fa0283) function
        pub fn log_219(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 250, 2, 131], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf4e3e01e) function
        pub fn log_220(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 227, 224, 30], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3461f38c) function
        pub fn log_221(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 97, 243, 140], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc06ef5a4) function
        pub fn log_222(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 110, 245, 164], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcc294a3e) function
        pub fn log_223(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 41, 74, 62], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x555be40f) function
        pub fn log_224(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 91, 228, 15], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfe8ea117) function
        pub fn log_225(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 142, 161, 23], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x48d3f012) function
        pub fn log_226(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 211, 240, 18], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x345caf24) function
        pub fn log_227(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 92, 175, 36], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe143dc85) function
        pub fn log_15(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 67, 220, 133], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0a9945ef) function
        pub fn log_55(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 153, 69, 239], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc7a347d1) function
        pub fn log_16(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 163, 71, 209], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4ae0e4d1) function
        pub fn log_228(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 224, 228, 209], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5369f3c6) function
        pub fn log_56(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 105, 243, 198], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x83570228) function
        pub fn log_229(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 87, 2, 40], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa4d262ae) function
        pub fn log_230(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 210, 98, 174], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfce03316) function
        pub fn log_231(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 224, 51, 22], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0ffdb6c2) function
        pub fn log_232(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 253, 182, 194], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9ee47742) function
        pub fn log_233(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 228, 119, 66], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7cd96643) function
        pub fn log_234(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 217, 102, 67], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2f839870) function
        pub fn log_235(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 131, 152, 112], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x192b6675) function
        pub fn log_236(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 43, 102, 117], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc7d1ca3b) function
        pub fn log_237(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 209, 202, 59], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8ee3464f) function
        pub fn log_238(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 227, 70, 79], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x46c677be) function
        pub fn log_239(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 198, 119, 190], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa7c72489) function
        pub fn log_240(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 199, 36, 137], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb55b7eb4) function
        pub fn log_241(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 91, 126, 180], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfd615560) function
        pub fn log_17(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 97, 85, 96], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x74a622fc) function
        pub fn log_242(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 166, 34, 252], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2f7aeb41) function
        pub fn log_243(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 122, 235, 65], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x027ce9b6) function
        pub fn log_244(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 124, 233, 182], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd791c92d) function
        pub fn log_245(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 145, 201, 45], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0515a072) function
        pub fn log_246(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 21, 160, 114], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x91566519) function
        pub fn log_57(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 86, 101, 25], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x09d30dba) function
        pub fn log_247(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 211, 13, 186], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf2262b4f) function
        pub fn log_248(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 38, 43, 79], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1efc2344) function
        pub fn log_249(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 252, 35, 68], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x17ca92d7) function
        pub fn log_58(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 202, 146, 215], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc31c8d99) function
        pub fn log_59(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 28, 141, 153], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x72a56348) function
        pub fn log_250(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 165, 99, 72], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc4080d3d) function
        pub fn log_251(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 8, 13, 61], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x44f408cc) function
        pub fn log_252(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 244, 8, 204], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4edf91eb) function
        pub fn log_253(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 223, 145, 235], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa45c64d2) function
        pub fn log_60(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 92, 100, 210], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4f9c1dbf) function
        pub fn log_254(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 156, 29, 191], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1da3581d) function
        pub fn log_61(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 163, 88, 29], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x637d0aa8) function
        pub fn log_255(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 125, 10, 168], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe461800a) function
        pub fn log_256(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 97, 128, 10], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1b73a8f2) function
        pub fn log_257(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 115, 168, 242], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xaab46fd0) function
        pub fn log_258(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 180, 111, 208], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x32d316de) function
        pub fn log_259(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 211, 22, 222], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0d37e7fb) function
        pub fn log_260(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 55, 231, 251], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x36ce73ee) function
        pub fn log_261(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 206, 115, 238], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4be10841) function
        pub fn log_262(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 225, 8, 65], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4483c3d8) function
        pub fn log_62(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 131, 195, 216], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x197e7251) function
        pub fn log_263(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 126, 114, 81], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8534a1e2) function
        pub fn log_264(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 52, 161, 226], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb782f0b7) function
        pub fn log_265(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 130, 240, 183], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe6f9b46a) function
        pub fn log_266(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 249, 180, 106], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf9a831eb) function
        pub fn log_267(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 168, 49, 235], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x79a60112) function
        pub fn log_268(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 166, 1, 18], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x654b5b59) function
        pub fn log_269(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 75, 91, 89], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9a759a29) function
        pub fn log_270(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 117, 154, 41], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc28f764c) function
        pub fn log_271(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 143, 118, 76], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe712ea46) function
        pub fn log_272(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 18, 234, 70], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0b8e46c1) function
        pub fn log_273(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 142, 70, 193], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xeebd800a) function
        pub fn log_274(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 189, 128, 10], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xce359c55) function
        pub fn log_275(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 53, 156, 85], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4e3afd00) function
        pub fn log_276(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 58, 253, 0], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3900cb74) function
        pub fn log_277(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 0, 203, 116], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdf388680) function
        pub fn log_63(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 56, 134, 128], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc6fdd72e) function
        pub fn log_64(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 253, 215, 46], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x929cdd11) function
        pub fn log_65(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 156, 221, 17], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2a3e97fe) function
        pub fn log_278(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 62, 151, 254], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x490f490f) function
        pub fn log_279(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 15, 73, 15], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1acafb75) function
        pub fn log_280(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([26, 202, 251, 117], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf6c3f5a6) function
        pub fn log_18(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 195, 245, 166], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0b3670f0) function
        pub fn log_66(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 54, 112, 240], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x41d0ed12) function
        pub fn log_281(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 208, 237, 18], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3964a2b2) function
        pub fn log_282(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 100, 162, 178], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1e0176ef) function
        pub fn log_283(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 1, 118, 239], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5198713d) function
        pub fn log_284(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 152, 113, 61], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xea06fc05) function
        pub fn log_285(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 6, 252, 5], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x64f1b3b8) function
        pub fn log_67(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 241, 179, 184], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xea62979a) function
        pub fn log_286(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 98, 151, 154], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x09e611d4) function
        pub fn log_287(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 230, 17, 212], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x02885535) function
        pub fn log_288(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 136, 85, 53], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8ab8a86e) function
        pub fn log_289(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 184, 168, 110], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd49e2c8e) function
        pub fn log_290(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 158, 44, 142], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe1ae3ea9) function
        pub fn log_291(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 174, 62, 169], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xceb204f1) function
        pub fn log_292(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 178, 4, 241], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe812ba5d) function
        pub fn log_19(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 18, 186, 93], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2961b9ed) function
        pub fn log_68(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 97, 185, 237], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x27504f03) function
        pub fn log_293(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 80, 79, 3], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb53591e7) function
        pub fn log_294(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 53, 145, 231], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x10dcb6e8) function
        pub fn log_295(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 220, 182, 232], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x84f254ff) function
        pub fn log_296(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 242, 84, 255], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0e21b9f9) function
        pub fn log_297(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 33, 185, 249], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x237e7b06) function
        pub fn log_69(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 126, 123, 6], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf56c4b01) function
        pub fn log_70(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 108, 75, 1], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfebd96ed) function
        pub fn log_71(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 189, 150, 237], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x01f33575) function
        pub fn log_72(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 243, 53, 117], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfb35b7b7) function
        pub fn log_298(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 53, 183, 183], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x71fece64) function
        pub fn log_299(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 254, 206, 100], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf2cf99f2) function
        pub fn log_300(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 207, 153, 242], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9c09a082) function
        pub fn log_301(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 9, 160, 130], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf42a5165) function
        pub fn log_302(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 42, 81, 101], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xec4c6992) function
        pub fn log_73(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 76, 105, 146], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe998243a) function
        pub fn log_303(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 152, 36, 58], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4bac5f20) function
        pub fn log_304(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 172, 95, 32], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7df2e393) function
        pub fn log_74(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 242, 227, 147], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe6b30ea0) function
        pub fn log_75(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 179, 14, 160], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x308f9442) function
        pub fn log_305(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 143, 148, 66], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdb19b057) function
        pub fn log_306(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 25, 176, 87], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb1ba94e3) function
        pub fn log_307(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 186, 148, 227], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5991a1cd) function
        pub fn log_308(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 145, 161, 205], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb843fb29) function
        pub fn log_309(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 67, 251, 41], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x523f6c84) function
        pub fn log_20(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 63, 108, 132], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5e362327) function
        pub fn log_76(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 54, 35, 39], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x31db2eb6) function
        pub fn log_310(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 219, 46, 182], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xee12427e) function
        pub fn log_311(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 18, 66, 126], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6d438eff) function
        pub fn log_312(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 67, 142, 255], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa03ba96b) function
        pub fn log_313(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 59, 169, 107], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfabb3236) function
        pub fn log_314(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 187, 50, 54], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x38219bf3) function
        pub fn log_77(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 33, 155, 243], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x90c95b4c) function
        pub fn log_315(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 201, 91, 76], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x42f2c4eb) function
        pub fn log_316(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 242, 196, 235], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd4bdc000) function
        pub fn log_317(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 189, 192, 0], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0d4c649b) function
        pub fn log_78(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 76, 100, 155], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5ffc1940) function
        pub fn log_318(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 252, 25, 64], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6f8a3f91) function
        pub fn log_79(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 138, 63, 145], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6b037020) function
        pub fn log_319(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 3, 112, 32], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb9b6f231) function
        pub fn log_320(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 182, 242, 49], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc8f7b8a6) function
        pub fn log_321(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 247, 184, 166], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x04a9ded6) function
        pub fn log_322(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 169, 222, 214], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5f86729b) function
        pub fn log_323(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 134, 114, 155], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x163bc6f7) function
        pub fn log_324(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 59, 198, 247], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa8b567d4) function
        pub fn log_80(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([168, 181, 103, 212], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4ef8bf42) function
        pub fn log_325(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 248, 191, 66], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0c17a864) function
        pub fn log_326(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: bool,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 23, 168, 100], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbf027bf0) function
        pub fn log_81(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 2, 123, 240], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x83d2a6eb) function
        pub fn log_327(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 210, 166, 235], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf1a69e41) function
        pub fn log_328(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 166, 158, 65], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xca15e359) function
        pub fn log_329(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 21, 227, 89], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6c1be72e) function
        pub fn log_330(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 27, 231, 46], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9f9d518c) function
        pub fn log_331(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 157, 81, 140], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xee276c63) function
        pub fn log_82(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 39, 108, 99], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbf05e352) function
        pub fn log_83(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 5, 227, 82], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x83ff6319) function
        pub fn log_84(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 255, 99, 25], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x13c57403) function
        pub fn log_332(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 197, 116, 3], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x30e87bfb) function
        pub fn log_333(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::corebc_core::types::Address,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 232, 123, 251], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb42647bb) function
        pub fn log_334(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 38, 71, 187], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0be3e8a9) function
        pub fn log_21(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 227, 232, 169], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x22b985e0) function
        pub fn log_335(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 185, 133, 224], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4812f0fc) function
        pub fn log_336(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 18, 240, 252], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5bb32ecb) function
        pub fn log_4(
            &self,
            p_0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 179, 46, 203], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x16b5a47b) function
        pub fn log_337(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 181, 164, 123], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x18f43d02) function
        pub fn log_338(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 244, 61, 2], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb03f62d4) function
        pub fn log_339(
            &self,
            p_0: ::corebc_core::types::U256,
            p_1: ::corebc_core::types::U256,
            p_2: ::corebc_core::types::U256,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 63, 98, 212], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7567bd05) function
        pub fn log_85(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 103, 189, 5], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc4dc84e5) function
        pub fn log_340(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 220, 132, 229], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9279e6d1) function
        pub fn log_86(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 121, 230, 209], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8c425101) function
        pub fn log_341(
            &self,
            p_0: ::corebc_core::types::Address,
            p_1: ::corebc_core::types::Address,
            p_2: ::corebc_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 66, 81, 1], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfb956bd1) function
        pub fn log_342(
            &self,
            p_0: bool,
            p_1: ::corebc_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 149, 107, 209], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa7dc43c8) function
        pub fn log_5(
            &self,
            p_0: ::corebc_core::types::I256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 220, 67, 200], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbe452625) function
        pub fn log_22(
            &self,
            p_0: ::std::string::String,
            p_1: ::corebc_core::types::I256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 69, 38, 37], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logAddress` (0x5ff27b8b) function
        pub fn log_address(
            &self,
            p_0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 242, 123, 139], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBool` (0xff703bae) function
        pub fn log_bool(
            &self,
            p_0: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 112, 59, 174], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes` (0xc61fe65a) function
        pub fn log_bytes(
            &self,
            p_0: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 31, 230, 90], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes1` (0x63bdde73) function
        pub fn log_bytes_1(
            &self,
            p_0: [u8; 1],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 189, 222, 115], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes10` (0x13182132) function
        pub fn log_bytes_10(
            &self,
            p_0: [u8; 10],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 24, 33, 50], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes11` (0xae8ff377) function
        pub fn log_bytes_11(
            &self,
            p_0: [u8; 11],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 143, 243, 119], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes12` (0x19ffaac6) function
        pub fn log_bytes_12(
            &self,
            p_0: [u8; 12],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 255, 170, 198], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes13` (0xffe04c39) function
        pub fn log_bytes_13(
            &self,
            p_0: [u8; 13],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 224, 76, 57], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes14` (0x84421355) function
        pub fn log_bytes_14(
            &self,
            p_0: [u8; 14],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 66, 19, 85], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes15` (0x35b0b1c1) function
        pub fn log_bytes_15(
            &self,
            p_0: [u8; 15],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 176, 177, 193], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes16` (0x56c1a66f) function
        pub fn log_bytes_16(
            &self,
            p_0: [u8; 16],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 193, 166, 111], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes17` (0xb6d042e2) function
        pub fn log_bytes_17(
            &self,
            p_0: [u8; 17],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 208, 66, 226], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes18` (0xe70c31dd) function
        pub fn log_bytes_18(
            &self,
            p_0: [u8; 18],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 12, 49, 221], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes19` (0x2b6d5e26) function
        pub fn log_bytes_19(
            &self,
            p_0: [u8; 19],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 109, 94, 38], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes2` (0xecfe8dc4) function
        pub fn log_bytes_2(
            &self,
            p_0: [u8; 2],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 254, 141, 196], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes20` (0x2b7c4d2c) function
        pub fn log_bytes_20(
            &self,
            p_0: [u8; 20],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 124, 77, 44], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes21` (0x85f3fa24) function
        pub fn log_bytes_21(
            &self,
            p_0: [u8; 21],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 243, 250, 36], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes22` (0xd44077d6) function
        pub fn log_bytes_22(
            &self,
            p_0: [u8; 22],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 64, 119, 214], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes23` (0x88dc2b5d) function
        pub fn log_bytes_23(
            &self,
            p_0: [u8; 23],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 220, 43, 93], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes24` (0x6fe238b3) function
        pub fn log_bytes_24(
            &self,
            p_0: [u8; 24],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 226, 56, 179], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes25` (0x01b7049a) function
        pub fn log_bytes_25(
            &self,
            p_0: [u8; 25],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 183, 4, 154], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes26` (0x790533dc) function
        pub fn log_bytes_26(
            &self,
            p_0: [u8; 26],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 5, 51, 220], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes27` (0xa273f7c8) function
        pub fn log_bytes_27(
            &self,
            p_0: [u8; 27],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 115, 247, 200], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes28` (0x4b0d7975) function
        pub fn log_bytes_28(
            &self,
            p_0: [u8; 28],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 13, 121, 117], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes29` (0x2a9a5ff5) function
        pub fn log_bytes_29(
            &self,
            p_0: [u8; 29],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 154, 95, 245], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes3` (0x15c25093) function
        pub fn log_bytes_3(
            &self,
            p_0: [u8; 3],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 194, 80, 147], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes30` (0x6011e35c) function
        pub fn log_bytes_30(
            &self,
            p_0: [u8; 30],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 17, 227, 92], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes31` (0x201a17bb) function
        pub fn log_bytes_31(
            &self,
            p_0: [u8; 31],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 26, 23, 187], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes32` (0x427f7397) function
        pub fn log_bytes_32(
            &self,
            p_0: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 127, 115, 151], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes4` (0x3ea34a1e) function
        pub fn log_bytes_4(
            &self,
            p_0: [u8; 4],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 163, 74, 30], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes5` (0x32df44f0) function
        pub fn log_bytes_5(
            &self,
            p_0: [u8; 5],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 223, 68, 240], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes6` (0xa926d8f2) function
        pub fn log_bytes_6(
            &self,
            p_0: [u8; 6],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 38, 216, 242], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes7` (0xf75e1304) function
        pub fn log_bytes_7(
            &self,
            p_0: [u8; 7],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 94, 19, 4], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes8` (0xd575f938) function
        pub fn log_bytes_8(
            &self,
            p_0: [u8; 8],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 117, 249, 56], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes9` (0xbd0e659f) function
        pub fn log_bytes_9(
            &self,
            p_0: [u8; 9],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 14, 101, 159], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logInt` (0x337147c6) function
        pub fn log_int(
            &self,
            p_0: ::corebc_core::types::I256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 113, 71, 198], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logString` (0xcbe47d77) function
        pub fn log_string(
            &self,
            p_0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 228, 125, 119], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logUint` (0x7f6ce492) function
        pub fn log_uint(
            &self,
            p_0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 108, 228, 146], p_0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::corebc_providers::Middleware> From<::corebc_contract::Contract<M>>
    for HardhatConsole<M> {
        fn from(contract: ::corebc_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string)` and selector `0xf5518d02`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string)")]
    pub struct Log23Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256,address)` and selector `0xb7b720f1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256,address)")]
    pub struct Log87Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address)` and selector `0x697ca861`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address)")]
    pub struct Log24Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address,string)` and selector `0xc7b92562`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address,string)")]
    pub struct Log88Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool,string)` and selector `0x4c448553`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool,string)")]
    pub struct Log89Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address,uint256)` and selector `0xaa818e86`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address,uint256)")]
    pub struct Log90Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool,uint256)` and selector `0xcab56040`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool,uint256)")]
    pub struct Log91Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address)` and selector `0x2c4fc75b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address)")]
    pub struct Log25Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address,bool)` and selector `0xbf4ad3f2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address,bool)")]
    pub struct Log92Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256,string)` and selector `0x51632250`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256,string)")]
    pub struct Log93Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256,uint256)` and selector `0x1014f2c1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256,uint256)")]
    pub struct Log94Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address,uint256)` and selector `0x6d5bf117`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address,uint256)")]
    pub struct Log95Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256,uint256)` and selector `0x6b885c5b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256,uint256)")]
    pub struct Log96Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256)` and selector `0xf9d77a15`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256)")]
    pub struct Log26Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address,address)` and selector `0x6407b4d8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address,address)")]
    pub struct Log97Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address,bool)` and selector `0x053bf2f0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address,bool)")]
    pub struct Log98Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address,bool)` and selector `0x19a2eca8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address,bool)")]
    pub struct Log99Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256,bool)` and selector `0x0f44fef8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256,bool)")]
    pub struct Log100Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address,uint256)` and selector `0xbe28811c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address,uint256)")]
    pub struct Log101Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256,address)` and selector `0x25dc7e21`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256,address)")]
    pub struct Log102Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address)` and selector `0x516f1022`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address)")]
    pub struct Log27Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256)` and selector `0xc1d8bdfd`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256)")]
    pub struct Log28Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address,string)` and selector `0xdc74f6fe`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address,string)")]
    pub struct Log103Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256)` and selector `0x8dc489a6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256)")]
    pub struct Log29Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256,address)` and selector `0xe1c37768`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256,address)")]
    pub struct Log104Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address,uint256)` and selector `0x10a511e4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address,uint256)")]
    pub struct Log105Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256,address)` and selector `0x4b0fe5b2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256,address)")]
    pub struct Log106Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string,uint256)` and selector `0x65f7f895`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string,uint256)")]
    pub struct Log107Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256,address)` and selector `0x38490c03`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256,address)")]
    pub struct Log108Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address,bool)` and selector `0x642f1c8f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address,bool)")]
    pub struct Log109Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool,uint256)` and selector `0xd0001512`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool,uint256)")]
    pub struct Log110Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string,string)` and selector `0xc23055b4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string,string)")]
    pub struct Log111Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256)` and selector `0xb54ffdc4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256)")]
    pub struct Log30Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool)` and selector `0xa3fcc8db`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool)")]
    pub struct Log31Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256,uint256)` and selector `0xbca7d815`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256,uint256)")]
    pub struct Log112Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string,address)` and selector `0x93a1d6e4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string,address)")]
    pub struct Log113Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256,string)` and selector `0xead3ed41`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256,string)")]
    pub struct Log114Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address,string)` and selector `0xdd3bab2e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address,string)")]
    pub struct Log115Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool,address)` and selector `0x8c3dc7fd`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool,address)")]
    pub struct Log116Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address)` and selector `0x69bd1047`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address)")]
    pub struct Log32Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool)` and selector `0xbec0d1ab`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool)")]
    pub struct Log6Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address,address)` and selector `0x1d531481`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address,address)")]
    pub struct Log117Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address,string)` and selector `0x57a8d06a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address,string)")]
    pub struct Log118Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256,uint256)` and selector `0x4ebc3f88`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256,uint256)")]
    pub struct Log119Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string,bool)` and selector `0x8d4f60f3`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string,bool)")]
    pub struct Log120Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256)` and selector `0xe793f437`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256)")]
    pub struct Log33Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool,address)` and selector `0x12fd3334`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool,address)")]
    pub struct Log121Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool)` and selector `0xc10bbfc9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool)")]
    pub struct Log34Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256,address)` and selector `0xe7413910`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256,address)")]
    pub struct Log122Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string)` and selector `0x8ee09221`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string)")]
    pub struct Log35Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string,string)` and selector `0xf9c29ca1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string,string)")]
    pub struct Log123Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string,string)` and selector `0x87f4f2da`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string,string)")]
    pub struct Log124Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool,address)` and selector `0x73465a65`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool,address)")]
    pub struct Log125Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool,uint256)` and selector `0x2b451fdd`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool,uint256)")]
    pub struct Log126Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string,string)` and selector `0xa2ff2343`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string,string)")]
    pub struct Log127Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address,address)` and selector `0xe718bc5d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address,address)")]
    pub struct Log128Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string,uint256)` and selector `0x6d73aa3f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string,uint256)")]
    pub struct Log129Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string)` and selector `0x986699a1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string)")]
    pub struct Log36Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address,address)` and selector `0xaa05db7c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address,address)")]
    pub struct Log130Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string,string)` and selector `0xe0d286c9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string,string)")]
    pub struct Log131Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256,uint256)` and selector `0x919176f1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256,uint256)")]
    pub struct Log132Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool)` and selector `0x1eae6cc6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool)")]
    pub struct Log7Call {
        pub p_0: bool,
        pub p_1: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool,string)` and selector `0xf9112a45`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool,string)")]
    pub struct Log133Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address,address)` and selector `0x7b1bdc49`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address,address)")]
    pub struct Log134Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string,bool)` and selector `0xdd71ee01`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string,bool)")]
    pub struct Log135Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string,uint256)` and selector `0x18a1462f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string,uint256)")]
    pub struct Log136Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address)` and selector `0xc371d1a1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address)")]
    pub struct Log1Call {
        pub p_0: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool,bool)` and selector `0xa4897300`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool,bool)")]
    pub struct Log137Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string)` and selector `0xf8d24eed`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string)")]
    pub struct Log37Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address,string)` and selector `0xdb884c5c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address,string)")]
    pub struct Log138Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address,string)` and selector `0x9f66a349`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address,string)")]
    pub struct Log139Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address)` and selector `0xc985d2bd`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address)")]
    pub struct Log8Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool)` and selector `0x045ceb4e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool)")]
    pub struct Log2Call {
        pub p_0: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address,address)` and selector `0x5f8b0ae8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address,address)")]
    pub struct Log140Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256,uint256)` and selector `0x888f32ef`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256,uint256)")]
    pub struct Log141Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address)` and selector `0x46c3e4af`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address)")]
    pub struct Log38Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool,bool)` and selector `0x98243bb8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool,bool)")]
    pub struct Log142Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string,bool)` and selector `0x41f7f930`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string,bool)")]
    pub struct Log143Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256)` and selector `0x92519541`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256)")]
    pub struct Log39Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256,uint256)` and selector `0x57fac99d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256,uint256)")]
    pub struct Log144Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256)` and selector `0x408c80e0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256)")]
    pub struct Log40Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256,uint256)` and selector `0xf189c485`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256,uint256)")]
    pub struct Log145Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool,uint256)` and selector `0xee31dcd5`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool,uint256)")]
    pub struct Log146Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256)` and selector `0x5faff71a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256)")]
    pub struct Log9Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256,address)` and selector `0x67c65264`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256,address)")]
    pub struct Log147Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool,bool)` and selector `0x97e2ba46`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool,bool)")]
    pub struct Log148Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool,bool)` and selector `0xfde5f4b6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool,bool)")]
    pub struct Log149Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string,string)` and selector `0xa2104f56`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string,string)")]
    pub struct Log150Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool,uint256)` and selector `0x67508e52`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool,uint256)")]
    pub struct Log151Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string,bool)` and selector `0xedb86a18`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string,bool)")]
    pub struct Log152Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool,bool)` and selector `0xcc9c3971`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool,bool)")]
    pub struct Log153Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string)` and selector `0x5d846457`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string)")]
    pub struct Log3Call {
        pub p_0: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string,address)` and selector `0x8d6ad054`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string,address)")]
    pub struct Log154Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address,address)` and selector `0x12fd8d7c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address,address)")]
    pub struct Log155Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256,string)` and selector `0xa1ce8de0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256,string)")]
    pub struct Log156Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address,bool)` and selector `0x72de3baa`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address,bool)")]
    pub struct Log157Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address,uint256)` and selector `0x400ab54c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address,uint256)")]
    pub struct Log158Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address,bool)` and selector `0x6d0bcd28`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address,bool)")]
    pub struct Log159Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string,uint256)` and selector `0xc0b32706`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string,uint256)")]
    pub struct Log160Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string,string)` and selector `0x60413141`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string,string)")]
    pub struct Log161Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool)` and selector `0x05d12fd3`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool)")]
    pub struct Log41Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address,address)` and selector `0x8e0922f8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address,address)")]
    pub struct Log162Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool,string)` and selector `0xee100856`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool,string)")]
    pub struct Log163Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256,string)` and selector `0x331191d2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256,string)")]
    pub struct Log164Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool,string)` and selector `0xca2b3de9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool,string)")]
    pub struct Log165Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string)` and selector `0xc36e01c0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string)")]
    pub struct Log10Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address,uint256)` and selector `0x9821418c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address,uint256)")]
    pub struct Log166Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool)` and selector `0x2d0972be`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool)")]
    pub struct Log42Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address,uint256)` and selector `0xe72f81b8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address,uint256)")]
    pub struct Log167Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool)` and selector `0xd2401bda`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool)")]
    pub struct Log43Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string,bool)` and selector `0x139b5077`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string,bool)")]
    pub struct Log168Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool,uint256)` and selector `0x57219423`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool,uint256)")]
    pub struct Log169Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log()` and selector `0x177e481c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log()")]
    pub struct Log0Call;
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256,string)` and selector `0xa0acbbc1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256,string)")]
    pub struct Log170Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool,address)` and selector `0x0161ead2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool,address)")]
    pub struct Log171Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256,address)` and selector `0x4cad4fb8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256,address)")]
    pub struct Log172Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address,address)` and selector `0x4cf45e04`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address,address)")]
    pub struct Log173Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256)` and selector `0xbf291806`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256)")]
    pub struct Log44Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string)` and selector `0xf62d0941`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string)")]
    pub struct Log45Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256,string)` and selector `0xf72f2bbb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256,string)")]
    pub struct Log174Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256,string)` and selector `0x4fb261e2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256,string)")]
    pub struct Log175Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256)` and selector `0xae7bdb09`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256)")]
    pub struct Log46Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string,string)` and selector `0xb6971fc0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string,string)")]
    pub struct Log176Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool,uint256)` and selector `0xcf73b53e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool,uint256)")]
    pub struct Log177Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string,address)` and selector `0x1ba32771`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string,address)")]
    pub struct Log178Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address)` and selector `0xa9c0e9e2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address)")]
    pub struct Log47Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address,bool)` and selector `0x794e1917`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address,bool)")]
    pub struct Log179Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string,string)` and selector `0xe484cd44`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string,string)")]
    pub struct Log180Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address,uint256)` and selector `0xf2ce6a92`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address,uint256)")]
    pub struct Log181Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256,string)` and selector `0xdbc6ed62`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256,string)")]
    pub struct Log182Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string,uint256)` and selector `0xa9ff5e35`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string,uint256)")]
    pub struct Log183Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool,string)` and selector `0xce31d1cb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool,string)")]
    pub struct Log184Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address,address)` and selector `0x7d08b2b8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address,address)")]
    pub struct Log185Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string,bool)` and selector `0xfa7945ae`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string,bool)")]
    pub struct Log186Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool,bool)` and selector `0xa60ab0f6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool,bool)")]
    pub struct Log187Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256,bool)` and selector `0x998e5d52`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256,bool)")]
    pub struct Log188Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256)` and selector `0x5fbb263d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256)")]
    pub struct Log48Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address,address)` and selector `0x61a05c7c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address,address)")]
    pub struct Log189Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256,bool)` and selector `0x47735e3a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256,bool)")]
    pub struct Log190Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256,address)` and selector `0x3ff220a9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256,address)")]
    pub struct Log191Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string)` and selector `0xc3218bed`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string)")]
    pub struct Log49Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256,address)` and selector `0xcba009c7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256,address)")]
    pub struct Log192Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string)` and selector `0xa8682514`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string)")]
    pub struct Log11Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256,uint256)` and selector `0xffcec76f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256,uint256)")]
    pub struct Log193Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address,address)` and selector `0x11f010e1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address,address)")]
    pub struct Log194Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address,address)` and selector `0xa113de70`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address,address)")]
    pub struct Log195Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256,bool)` and selector `0xf9115a1e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256,bool)")]
    pub struct Log196Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool)` and selector `0x71fc19a1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool)")]
    pub struct Log50Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256)` and selector `0x0520d4a3`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256)")]
    pub struct Log51Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string,string)` and selector `0xa7d947c0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string,string)")]
    pub struct Log197Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256,bool)` and selector `0x9b933ca0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256,bool)")]
    pub struct Log198Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address)` and selector `0x1fe72f17`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address)")]
    pub struct Log12Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool,address)` and selector `0x6aae1b32`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool,address)")]
    pub struct Log199Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string,uint256)` and selector `0xcca9c27f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string,uint256)")]
    pub struct Log200Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool,bool)` and selector `0x785b2def`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool,bool)")]
    pub struct Log201Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256,bool)` and selector `0x3e6fe005`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256,bool)")]
    pub struct Log202Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address,string)` and selector `0x0d1846bc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address,string)")]
    pub struct Log203Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string,string)` and selector `0x14567320`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string,string)")]
    pub struct Log204Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string,address)` and selector `0xe84511a5`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string,address)")]
    pub struct Log205Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool,uint256)` and selector `0x00bdcf5d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool,uint256)")]
    pub struct Log206Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address,bool)` and selector `0x3b63f413`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address,bool)")]
    pub struct Log207Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string,bool)` and selector `0x819bd95d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string,bool)")]
    pub struct Log208Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string,address)` and selector `0x46cef1ff`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string,address)")]
    pub struct Log209Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool,address)` and selector `0xd43a08d0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool,address)")]
    pub struct Log210Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string)` and selector `0x1318bcfc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string)")]
    pub struct Log52Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address,uint256)` and selector `0x96668786`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address,uint256)")]
    pub struct Log211Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256,string)` and selector `0x1b4e65fe`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256,string)")]
    pub struct Log212Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool,uint256)` and selector `0x19c75a59`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool,uint256)")]
    pub struct Log213Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string)` and selector `0xd9a56a6a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string)")]
    pub struct Log13Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool)` and selector `0x228e68c4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool)")]
    pub struct Log14Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256,bool)` and selector `0xefa0f887`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256,bool)")]
    pub struct Log214Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool,bool)` and selector `0xde6942a7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool,bool)")]
    pub struct Log215Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string,bool)` and selector `0x0b0aa13c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string,bool)")]
    pub struct Log216Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address)` and selector `0x19290410`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address)")]
    pub struct Log53Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address)` and selector `0x7b706e61`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address)")]
    pub struct Log54Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string,uint256)` and selector `0xbaa5a4a6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string,uint256)")]
    pub struct Log217Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256,uint256)` and selector `0x0dd60ea3`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256,uint256)")]
    pub struct Log218Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string,address)` and selector `0x05fa0283`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string,address)")]
    pub struct Log219Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address,uint256)` and selector `0xf4e3e01e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address,uint256)")]
    pub struct Log220Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string,bool)` and selector `0x3461f38c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string,bool)")]
    pub struct Log221Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256,string)` and selector `0xc06ef5a4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256,string)")]
    pub struct Log222Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool,uint256)` and selector `0xcc294a3e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool,uint256)")]
    pub struct Log223Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address,string)` and selector `0x555be40f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address,string)")]
    pub struct Log224Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string,uint256)` and selector `0xfe8ea117`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string,uint256)")]
    pub struct Log225Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address,bool)` and selector `0x48d3f012`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address,bool)")]
    pub struct Log226Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256,uint256)` and selector `0x345caf24`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256,uint256)")]
    pub struct Log227Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256)` and selector `0xe143dc85`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256)")]
    pub struct Log15Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool)` and selector `0x0a9945ef`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool)")]
    pub struct Log55Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address)` and selector `0xc7a347d1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address)")]
    pub struct Log16Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256,string)` and selector `0x4ae0e4d1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256,string)")]
    pub struct Log228Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string)` and selector `0x5369f3c6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string)")]
    pub struct Log56Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string,string)` and selector `0x83570228`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string,string)")]
    pub struct Log229Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256,address)` and selector `0xa4d262ae`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256,address)")]
    pub struct Log230Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address,uint256)` and selector `0xfce03316`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address,uint256)")]
    pub struct Log231Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool,bool)` and selector `0x0ffdb6c2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool,bool)")]
    pub struct Log232Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256,bool)` and selector `0x9ee47742`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256,bool)")]
    pub struct Log233Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool,address)` and selector `0x7cd96643`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool,address)")]
    pub struct Log234Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool,uint256)` and selector `0x2f839870`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool,uint256)")]
    pub struct Log235Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256,address)` and selector `0x192b6675`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256,address)")]
    pub struct Log236Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool,uint256)` and selector `0xc7d1ca3b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool,uint256)")]
    pub struct Log237Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256,string)` and selector `0x8ee3464f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256,string)")]
    pub struct Log238Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string,uint256)` and selector `0x46c677be`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string,uint256)")]
    pub struct Log239Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address,uint256)` and selector `0xa7c72489`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address,uint256)")]
    pub struct Log240Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string,address)` and selector `0xb55b7eb4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string,address)")]
    pub struct Log241Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string)` and selector `0xfd615560`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string)")]
    pub struct Log17Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address,bool)` and selector `0x74a622fc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address,bool)")]
    pub struct Log242Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool,string)` and selector `0x2f7aeb41`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool,string)")]
    pub struct Log243Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool,string)` and selector `0x027ce9b6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool,string)")]
    pub struct Log244Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256,bool)` and selector `0xd791c92d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256,bool)")]
    pub struct Log245Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string,uint256)` and selector `0x0515a072`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string,uint256)")]
    pub struct Log246Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address)` and selector `0x91566519`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address)")]
    pub struct Log57Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256,address)` and selector `0x09d30dba`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256,address)")]
    pub struct Log247Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address,uint256)` and selector `0xf2262b4f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address,uint256)")]
    pub struct Log248Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address,bool)` and selector `0x1efc2344`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address,bool)")]
    pub struct Log249Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address)` and selector `0x17ca92d7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address)")]
    pub struct Log58Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address)` and selector `0xc31c8d99`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address)")]
    pub struct Log59Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string,address)` and selector `0x72a56348`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string,address)")]
    pub struct Log250Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool,address)` and selector `0xc4080d3d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool,address)")]
    pub struct Log251Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool,address)` and selector `0x44f408cc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool,address)")]
    pub struct Log252Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256,bool)` and selector `0x4edf91eb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256,bool)")]
    pub struct Log253Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool)` and selector `0xa45c64d2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool)")]
    pub struct Log60Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address,string)` and selector `0x4f9c1dbf`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address,string)")]
    pub struct Log254Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256)` and selector `0x1da3581d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256)")]
    pub struct Log61Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string,address)` and selector `0x637d0aa8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string,address)")]
    pub struct Log255Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool,string)` and selector `0xe461800a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool,string)")]
    pub struct Log256Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool,address)` and selector `0x1b73a8f2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool,address)")]
    pub struct Log257Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address,string)` and selector `0xaab46fd0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address,string)")]
    pub struct Log258Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string,address)` and selector `0x32d316de`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string,address)")]
    pub struct Log259Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address,string)` and selector `0x0d37e7fb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address,string)")]
    pub struct Log260Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address,bool)` and selector `0x36ce73ee`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address,bool)")]
    pub struct Log261Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address,address)` and selector `0x4be10841`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address,address)")]
    pub struct Log262Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string)` and selector `0x4483c3d8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string)")]
    pub struct Log62Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool,address)` and selector `0x197e7251`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool,address)")]
    pub struct Log263Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool,string)` and selector `0x8534a1e2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool,string)")]
    pub struct Log264Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address,uint256)` and selector `0xb782f0b7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address,uint256)")]
    pub struct Log265Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address,bool)` and selector `0xe6f9b46a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address,bool)")]
    pub struct Log266Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string,string)` and selector `0xf9a831eb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string,string)")]
    pub struct Log267Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address,uint256)` and selector `0x79a60112`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address,uint256)")]
    pub struct Log268Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256,uint256)` and selector `0x654b5b59`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256,uint256)")]
    pub struct Log269Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string,string)` and selector `0x9a759a29`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string,string)")]
    pub struct Log270Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool,string)` and selector `0xc28f764c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool,string)")]
    pub struct Log271Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string,address)` and selector `0xe712ea46`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string,address)")]
    pub struct Log272Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool,bool)` and selector `0x0b8e46c1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool,bool)")]
    pub struct Log273Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool,string)` and selector `0xeebd800a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool,string)")]
    pub struct Log274Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address,string)` and selector `0xce359c55`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address,string)")]
    pub struct Log275Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool,address)` and selector `0x4e3afd00`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool,address)")]
    pub struct Log276Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string,uint256)` and selector `0x3900cb74`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string,uint256)")]
    pub struct Log277Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string)` and selector `0xdf388680`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string)")]
    pub struct Log63Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool)` and selector `0xc6fdd72e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool)")]
    pub struct Log64Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string)` and selector `0x929cdd11`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string)")]
    pub struct Log65Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string,bool)` and selector `0x2a3e97fe`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string,bool)")]
    pub struct Log278Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address,bool)` and selector `0x490f490f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address,bool)")]
    pub struct Log279Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address,bool)` and selector `0x1acafb75`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address,bool)")]
    pub struct Log280Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256)` and selector `0xf6c3f5a6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256)")]
    pub struct Log18Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256)` and selector `0x0b3670f0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256)")]
    pub struct Log66Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool,bool)` and selector `0x41d0ed12`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool,bool)")]
    pub struct Log281Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256,string)` and selector `0x3964a2b2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256,string)")]
    pub struct Log282Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string,bool)` and selector `0x1e0176ef`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string,bool)")]
    pub struct Log283Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool,bool)` and selector `0x5198713d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool,bool)")]
    pub struct Log284Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool,string)` and selector `0xea06fc05`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool,string)")]
    pub struct Log285Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address)` and selector `0x64f1b3b8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address)")]
    pub struct Log67Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256,uint256)` and selector `0xea62979a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256,uint256)")]
    pub struct Log286Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256,bool)` and selector `0x09e611d4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256,bool)")]
    pub struct Log287Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string,uint256)` and selector `0x02885535`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string,uint256)")]
    pub struct Log288Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address,bool)` and selector `0x8ab8a86e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address,bool)")]
    pub struct Log289Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string,uint256)` and selector `0xd49e2c8e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string,uint256)")]
    pub struct Log290Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool,address)` and selector `0xe1ae3ea9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool,address)")]
    pub struct Log291Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256,bool)` and selector `0xceb204f1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256,bool)")]
    pub struct Log292Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool)` and selector `0xe812ba5d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool)")]
    pub struct Log19Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string)` and selector `0x2961b9ed`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string)")]
    pub struct Log68Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256,bool)` and selector `0x27504f03`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256,bool)")]
    pub struct Log293Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256,bool)` and selector `0xb53591e7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256,bool)")]
    pub struct Log294Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool,string)` and selector `0x10dcb6e8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool,string)")]
    pub struct Log295Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string,uint256)` and selector `0x84f254ff`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string,uint256)")]
    pub struct Log296Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256,uint256)` and selector `0x0e21b9f9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256,uint256)")]
    pub struct Log297Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool)` and selector `0x237e7b06`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool)")]
    pub struct Log69Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256)` and selector `0xf56c4b01`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256)")]
    pub struct Log70Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256)` and selector `0xfebd96ed`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256)")]
    pub struct Log71Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool)` and selector `0x01f33575`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool)")]
    pub struct Log72Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool,bool)` and selector `0xfb35b7b7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool,bool)")]
    pub struct Log298Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string,bool)` and selector `0x71fece64`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string,bool)")]
    pub struct Log299Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256,address)` and selector `0xf2cf99f2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256,address)")]
    pub struct Log300Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool,bool)` and selector `0x9c09a082`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool,bool)")]
    pub struct Log301Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool,uint256)` and selector `0xf42a5165`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool,uint256)")]
    pub struct Log302Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool)` and selector `0xec4c6992`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool)")]
    pub struct Log73Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string,bool)` and selector `0xe998243a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string,bool)")]
    pub struct Log303Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool,address)` and selector `0x4bac5f20`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool,address)")]
    pub struct Log304Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256)` and selector `0x7df2e393`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256)")]
    pub struct Log74Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address)` and selector `0xe6b30ea0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address)")]
    pub struct Log75Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool,string)` and selector `0x308f9442`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool,string)")]
    pub struct Log305Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string,address)` and selector `0xdb19b057`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string,address)")]
    pub struct Log306Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256,bool)` and selector `0xb1ba94e3`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256,bool)")]
    pub struct Log307Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool,uint256)` and selector `0x5991a1cd`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool,uint256)")]
    pub struct Log308Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address,string)` and selector `0xb843fb29`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address,string)")]
    pub struct Log309Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address)` and selector `0x523f6c84`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address)")]
    pub struct Log20Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool)` and selector `0x5e362327`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool)")]
    pub struct Log76Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool,bool)` and selector `0x31db2eb6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool,bool)")]
    pub struct Log310Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256,string)` and selector `0xee12427e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256,string)")]
    pub struct Log311Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool,string)` and selector `0x6d438eff`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool,string)")]
    pub struct Log312Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256,string)` and selector `0xa03ba96b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256,string)")]
    pub struct Log313Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string,string)` and selector `0xfabb3236`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string,string)")]
    pub struct Log314Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string)` and selector `0x38219bf3`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string)")]
    pub struct Log77Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool,string)` and selector `0x90c95b4c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool,string)")]
    pub struct Log315Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string,address)` and selector `0x42f2c4eb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string,address)")]
    pub struct Log316Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool,address)` and selector `0xd4bdc000`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool,address)")]
    pub struct Log317Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string)` and selector `0x0d4c649b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string)")]
    pub struct Log78Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256,address)` and selector `0x5ffc1940`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256,address)")]
    pub struct Log318Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string)` and selector `0x6f8a3f91`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string)")]
    pub struct Log79Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string,bool)` and selector `0x6b037020`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string,bool)")]
    pub struct Log319Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool,bool)` and selector `0xb9b6f231`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool,bool)")]
    pub struct Log320Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string,uint256)` and selector `0xc8f7b8a6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string,uint256)")]
    pub struct Log321Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool,uint256)` and selector `0x04a9ded6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool,uint256)")]
    pub struct Log322Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string,bool)` and selector `0x5f86729b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string,bool)")]
    pub struct Log323Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address,uint256)` and selector `0x163bc6f7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address,uint256)")]
    pub struct Log324Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool)` and selector `0xa8b567d4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool)")]
    pub struct Log80Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address,string)` and selector `0x4ef8bf42`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address,string)")]
    pub struct Log325Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool,uint256)` and selector `0x0c17a864`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool,uint256)")]
    pub struct Log326Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool)` and selector `0xbf027bf0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool)")]
    pub struct Log81Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string,bool)` and selector `0x83d2a6eb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string,bool)")]
    pub struct Log327Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address,address)` and selector `0xf1a69e41`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address,address)")]
    pub struct Log328Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string,uint256)` and selector `0xca15e359`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string,uint256)")]
    pub struct Log329Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string,address)` and selector `0x6c1be72e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string,address)")]
    pub struct Log330Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool,address)` and selector `0x9f9d518c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool,address)")]
    pub struct Log331Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address)` and selector `0xee276c63`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address)")]
    pub struct Log82Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address)` and selector `0xbf05e352`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address)")]
    pub struct Log83Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool)` and selector `0x83ff6319`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool)")]
    pub struct Log84Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256,uint256)` and selector `0x13c57403`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256,uint256)")]
    pub struct Log332Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address,address)` and selector `0x30e87bfb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address,address)")]
    pub struct Log333Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string,string)` and selector `0xb42647bb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string,string)")]
    pub struct Log334Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256)` and selector `0x0be3e8a9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256)")]
    pub struct Log21Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address,string)` and selector `0x22b985e0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address,string)")]
    pub struct Log335Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address,string)` and selector `0x4812f0fc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address,string)")]
    pub struct Log336Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256)` and selector `0x5bb32ecb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256)")]
    pub struct Log4Call {
        pub p_0: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256,uint256)` and selector `0x16b5a47b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256,uint256)")]
    pub struct Log337Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string,address)` and selector `0x18f43d02`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string,address)")]
    pub struct Log338Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256,address)` and selector `0xb03f62d4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256,address)")]
    pub struct Log339Call {
        pub p_0: ::corebc_core::types::U256,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string)` and selector `0x7567bd05`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string)")]
    pub struct Log85Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256,bool)` and selector `0xc4dc84e5`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256,bool)")]
    pub struct Log340Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address)` and selector `0x9279e6d1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address)")]
    pub struct Log86Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256,string)` and selector `0x8c425101`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256,string)")]
    pub struct Log341Call {
        pub p_0: ::corebc_core::types::Address,
        pub p_1: ::corebc_core::types::Address,
        pub p_2: ::corebc_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string,address)` and selector `0xfb956bd1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string,address)")]
    pub struct Log342Call {
        pub p_0: bool,
        pub p_1: ::corebc_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(int256)` and selector `0xa7dc43c8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(int256)")]
    pub struct Log5Call {
        pub p_0: ::corebc_core::types::I256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,int256)` and selector `0xbe452625`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,int256)")]
    pub struct Log22Call {
        pub p_0: ::std::string::String,
        pub p_1: ::corebc_core::types::I256,
    }
    ///Container type for all input parameters for the `logAddress` function with signature `logAddress(address)` and selector `0x5ff27b8b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logAddress", abi = "logAddress(address)")]
    pub struct LogAddressCall {
        pub p_0: ::corebc_core::types::Address,
    }
    ///Container type for all input parameters for the `logBool` function with signature `logBool(bool)` and selector `0xff703bae`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBool", abi = "logBool(bool)")]
    pub struct LogBoolCall {
        pub p_0: bool,
    }
    ///Container type for all input parameters for the `logBytes` function with signature `logBytes(bytes)` and selector `0xc61fe65a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes", abi = "logBytes(bytes)")]
    pub struct LogBytesCall {
        pub p_0: ::corebc_core::types::Bytes,
    }
    ///Container type for all input parameters for the `logBytes1` function with signature `logBytes1(bytes1)` and selector `0x63bdde73`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes1", abi = "logBytes1(bytes1)")]
    pub struct LogBytes1Call {
        pub p_0: [u8; 1],
    }
    ///Container type for all input parameters for the `logBytes10` function with signature `logBytes10(bytes10)` and selector `0x13182132`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes10", abi = "logBytes10(bytes10)")]
    pub struct LogBytes10Call {
        pub p_0: [u8; 10],
    }
    ///Container type for all input parameters for the `logBytes11` function with signature `logBytes11(bytes11)` and selector `0xae8ff377`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes11", abi = "logBytes11(bytes11)")]
    pub struct LogBytes11Call {
        pub p_0: [u8; 11],
    }
    ///Container type for all input parameters for the `logBytes12` function with signature `logBytes12(bytes12)` and selector `0x19ffaac6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes12", abi = "logBytes12(bytes12)")]
    pub struct LogBytes12Call {
        pub p_0: [u8; 12],
    }
    ///Container type for all input parameters for the `logBytes13` function with signature `logBytes13(bytes13)` and selector `0xffe04c39`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes13", abi = "logBytes13(bytes13)")]
    pub struct LogBytes13Call {
        pub p_0: [u8; 13],
    }
    ///Container type for all input parameters for the `logBytes14` function with signature `logBytes14(bytes14)` and selector `0x84421355`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes14", abi = "logBytes14(bytes14)")]
    pub struct LogBytes14Call {
        pub p_0: [u8; 14],
    }
    ///Container type for all input parameters for the `logBytes15` function with signature `logBytes15(bytes15)` and selector `0x35b0b1c1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes15", abi = "logBytes15(bytes15)")]
    pub struct LogBytes15Call {
        pub p_0: [u8; 15],
    }
    ///Container type for all input parameters for the `logBytes16` function with signature `logBytes16(bytes16)` and selector `0x56c1a66f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes16", abi = "logBytes16(bytes16)")]
    pub struct LogBytes16Call {
        pub p_0: [u8; 16],
    }
    ///Container type for all input parameters for the `logBytes17` function with signature `logBytes17(bytes17)` and selector `0xb6d042e2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes17", abi = "logBytes17(bytes17)")]
    pub struct LogBytes17Call {
        pub p_0: [u8; 17],
    }
    ///Container type for all input parameters for the `logBytes18` function with signature `logBytes18(bytes18)` and selector `0xe70c31dd`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes18", abi = "logBytes18(bytes18)")]
    pub struct LogBytes18Call {
        pub p_0: [u8; 18],
    }
    ///Container type for all input parameters for the `logBytes19` function with signature `logBytes19(bytes19)` and selector `0x2b6d5e26`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes19", abi = "logBytes19(bytes19)")]
    pub struct LogBytes19Call {
        pub p_0: [u8; 19],
    }
    ///Container type for all input parameters for the `logBytes2` function with signature `logBytes2(bytes2)` and selector `0xecfe8dc4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes2", abi = "logBytes2(bytes2)")]
    pub struct LogBytes2Call {
        pub p_0: [u8; 2],
    }
    ///Container type for all input parameters for the `logBytes20` function with signature `logBytes20(bytes20)` and selector `0x2b7c4d2c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes20", abi = "logBytes20(bytes20)")]
    pub struct LogBytes20Call {
        pub p_0: [u8; 20],
    }
    ///Container type for all input parameters for the `logBytes21` function with signature `logBytes21(bytes21)` and selector `0x85f3fa24`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes21", abi = "logBytes21(bytes21)")]
    pub struct LogBytes21Call {
        pub p_0: [u8; 21],
    }
    ///Container type for all input parameters for the `logBytes22` function with signature `logBytes22(bytes22)` and selector `0xd44077d6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes22", abi = "logBytes22(bytes22)")]
    pub struct LogBytes22Call {
        pub p_0: [u8; 22],
    }
    ///Container type for all input parameters for the `logBytes23` function with signature `logBytes23(bytes23)` and selector `0x88dc2b5d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes23", abi = "logBytes23(bytes23)")]
    pub struct LogBytes23Call {
        pub p_0: [u8; 23],
    }
    ///Container type for all input parameters for the `logBytes24` function with signature `logBytes24(bytes24)` and selector `0x6fe238b3`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes24", abi = "logBytes24(bytes24)")]
    pub struct LogBytes24Call {
        pub p_0: [u8; 24],
    }
    ///Container type for all input parameters for the `logBytes25` function with signature `logBytes25(bytes25)` and selector `0x01b7049a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes25", abi = "logBytes25(bytes25)")]
    pub struct LogBytes25Call {
        pub p_0: [u8; 25],
    }
    ///Container type for all input parameters for the `logBytes26` function with signature `logBytes26(bytes26)` and selector `0x790533dc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes26", abi = "logBytes26(bytes26)")]
    pub struct LogBytes26Call {
        pub p_0: [u8; 26],
    }
    ///Container type for all input parameters for the `logBytes27` function with signature `logBytes27(bytes27)` and selector `0xa273f7c8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes27", abi = "logBytes27(bytes27)")]
    pub struct LogBytes27Call {
        pub p_0: [u8; 27],
    }
    ///Container type for all input parameters for the `logBytes28` function with signature `logBytes28(bytes28)` and selector `0x4b0d7975`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes28", abi = "logBytes28(bytes28)")]
    pub struct LogBytes28Call {
        pub p_0: [u8; 28],
    }
    ///Container type for all input parameters for the `logBytes29` function with signature `logBytes29(bytes29)` and selector `0x2a9a5ff5`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes29", abi = "logBytes29(bytes29)")]
    pub struct LogBytes29Call {
        pub p_0: [u8; 29],
    }
    ///Container type for all input parameters for the `logBytes3` function with signature `logBytes3(bytes3)` and selector `0x15c25093`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes3", abi = "logBytes3(bytes3)")]
    pub struct LogBytes3Call {
        pub p_0: [u8; 3],
    }
    ///Container type for all input parameters for the `logBytes30` function with signature `logBytes30(bytes30)` and selector `0x6011e35c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes30", abi = "logBytes30(bytes30)")]
    pub struct LogBytes30Call {
        pub p_0: [u8; 30],
    }
    ///Container type for all input parameters for the `logBytes31` function with signature `logBytes31(bytes31)` and selector `0x201a17bb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes31", abi = "logBytes31(bytes31)")]
    pub struct LogBytes31Call {
        pub p_0: [u8; 31],
    }
    ///Container type for all input parameters for the `logBytes32` function with signature `logBytes32(bytes32)` and selector `0x427f7397`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes32", abi = "logBytes32(bytes32)")]
    pub struct LogBytes32Call {
        pub p_0: [u8; 32],
    }
    ///Container type for all input parameters for the `logBytes4` function with signature `logBytes4(bytes4)` and selector `0x3ea34a1e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes4", abi = "logBytes4(bytes4)")]
    pub struct LogBytes4Call {
        pub p_0: [u8; 4],
    }
    ///Container type for all input parameters for the `logBytes5` function with signature `logBytes5(bytes5)` and selector `0x32df44f0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes5", abi = "logBytes5(bytes5)")]
    pub struct LogBytes5Call {
        pub p_0: [u8; 5],
    }
    ///Container type for all input parameters for the `logBytes6` function with signature `logBytes6(bytes6)` and selector `0xa926d8f2`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes6", abi = "logBytes6(bytes6)")]
    pub struct LogBytes6Call {
        pub p_0: [u8; 6],
    }
    ///Container type for all input parameters for the `logBytes7` function with signature `logBytes7(bytes7)` and selector `0xf75e1304`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes7", abi = "logBytes7(bytes7)")]
    pub struct LogBytes7Call {
        pub p_0: [u8; 7],
    }
    ///Container type for all input parameters for the `logBytes8` function with signature `logBytes8(bytes8)` and selector `0xd575f938`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes8", abi = "logBytes8(bytes8)")]
    pub struct LogBytes8Call {
        pub p_0: [u8; 8],
    }
    ///Container type for all input parameters for the `logBytes9` function with signature `logBytes9(bytes9)` and selector `0xbd0e659f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes9", abi = "logBytes9(bytes9)")]
    pub struct LogBytes9Call {
        pub p_0: [u8; 9],
    }
    ///Container type for all input parameters for the `logInt` function with signature `logInt(int256)` and selector `0x337147c6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logInt", abi = "logInt(int256)")]
    pub struct LogIntCall {
        pub p_0: ::corebc_core::types::I256,
    }
    ///Container type for all input parameters for the `logString` function with signature `logString(string)` and selector `0xcbe47d77`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logString", abi = "logString(string)")]
    pub struct LogStringCall {
        pub p_0: ::std::string::String,
    }
    ///Container type for all input parameters for the `logUint` function with signature `logUint(uint256)` and selector `0x7f6ce492`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        foxar_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logUint", abi = "logUint(uint256)")]
    pub struct LogUintCall {
        pub p_0: ::corebc_core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        foxar_macros::ConsoleFmt,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum HardhatConsoleCalls {
        Log23(Log23Call),
        Log87(Log87Call),
        Log24(Log24Call),
        Log88(Log88Call),
        Log89(Log89Call),
        Log90(Log90Call),
        Log91(Log91Call),
        Log25(Log25Call),
        Log92(Log92Call),
        Log93(Log93Call),
        Log94(Log94Call),
        Log95(Log95Call),
        Log96(Log96Call),
        Log26(Log26Call),
        Log97(Log97Call),
        Log98(Log98Call),
        Log99(Log99Call),
        Log100(Log100Call),
        Log101(Log101Call),
        Log102(Log102Call),
        Log27(Log27Call),
        Log28(Log28Call),
        Log103(Log103Call),
        Log29(Log29Call),
        Log104(Log104Call),
        Log105(Log105Call),
        Log106(Log106Call),
        Log107(Log107Call),
        Log108(Log108Call),
        Log109(Log109Call),
        Log110(Log110Call),
        Log111(Log111Call),
        Log30(Log30Call),
        Log31(Log31Call),
        Log112(Log112Call),
        Log113(Log113Call),
        Log114(Log114Call),
        Log115(Log115Call),
        Log116(Log116Call),
        Log32(Log32Call),
        Log6(Log6Call),
        Log117(Log117Call),
        Log118(Log118Call),
        Log119(Log119Call),
        Log120(Log120Call),
        Log33(Log33Call),
        Log121(Log121Call),
        Log34(Log34Call),
        Log122(Log122Call),
        Log35(Log35Call),
        Log123(Log123Call),
        Log124(Log124Call),
        Log125(Log125Call),
        Log126(Log126Call),
        Log127(Log127Call),
        Log128(Log128Call),
        Log129(Log129Call),
        Log36(Log36Call),
        Log130(Log130Call),
        Log131(Log131Call),
        Log132(Log132Call),
        Log7(Log7Call),
        Log133(Log133Call),
        Log134(Log134Call),
        Log135(Log135Call),
        Log136(Log136Call),
        Log1(Log1Call),
        Log137(Log137Call),
        Log37(Log37Call),
        Log138(Log138Call),
        Log139(Log139Call),
        Log8(Log8Call),
        Log2(Log2Call),
        Log140(Log140Call),
        Log141(Log141Call),
        Log38(Log38Call),
        Log142(Log142Call),
        Log143(Log143Call),
        Log39(Log39Call),
        Log144(Log144Call),
        Log40(Log40Call),
        Log145(Log145Call),
        Log146(Log146Call),
        Log9(Log9Call),
        Log147(Log147Call),
        Log148(Log148Call),
        Log149(Log149Call),
        Log150(Log150Call),
        Log151(Log151Call),
        Log152(Log152Call),
        Log153(Log153Call),
        Log3(Log3Call),
        Log154(Log154Call),
        Log155(Log155Call),
        Log156(Log156Call),
        Log157(Log157Call),
        Log158(Log158Call),
        Log159(Log159Call),
        Log160(Log160Call),
        Log161(Log161Call),
        Log41(Log41Call),
        Log162(Log162Call),
        Log163(Log163Call),
        Log164(Log164Call),
        Log165(Log165Call),
        Log10(Log10Call),
        Log166(Log166Call),
        Log42(Log42Call),
        Log167(Log167Call),
        Log43(Log43Call),
        Log168(Log168Call),
        Log169(Log169Call),
        Log0(Log0Call),
        Log170(Log170Call),
        Log171(Log171Call),
        Log172(Log172Call),
        Log173(Log173Call),
        Log44(Log44Call),
        Log45(Log45Call),
        Log174(Log174Call),
        Log175(Log175Call),
        Log46(Log46Call),
        Log176(Log176Call),
        Log177(Log177Call),
        Log178(Log178Call),
        Log47(Log47Call),
        Log179(Log179Call),
        Log180(Log180Call),
        Log181(Log181Call),
        Log182(Log182Call),
        Log183(Log183Call),
        Log184(Log184Call),
        Log185(Log185Call),
        Log186(Log186Call),
        Log187(Log187Call),
        Log188(Log188Call),
        Log48(Log48Call),
        Log189(Log189Call),
        Log190(Log190Call),
        Log191(Log191Call),
        Log49(Log49Call),
        Log192(Log192Call),
        Log11(Log11Call),
        Log193(Log193Call),
        Log194(Log194Call),
        Log195(Log195Call),
        Log196(Log196Call),
        Log50(Log50Call),
        Log51(Log51Call),
        Log197(Log197Call),
        Log198(Log198Call),
        Log12(Log12Call),
        Log199(Log199Call),
        Log200(Log200Call),
        Log201(Log201Call),
        Log202(Log202Call),
        Log203(Log203Call),
        Log204(Log204Call),
        Log205(Log205Call),
        Log206(Log206Call),
        Log207(Log207Call),
        Log208(Log208Call),
        Log209(Log209Call),
        Log210(Log210Call),
        Log52(Log52Call),
        Log211(Log211Call),
        Log212(Log212Call),
        Log213(Log213Call),
        Log13(Log13Call),
        Log14(Log14Call),
        Log214(Log214Call),
        Log215(Log215Call),
        Log216(Log216Call),
        Log53(Log53Call),
        Log54(Log54Call),
        Log217(Log217Call),
        Log218(Log218Call),
        Log219(Log219Call),
        Log220(Log220Call),
        Log221(Log221Call),
        Log222(Log222Call),
        Log223(Log223Call),
        Log224(Log224Call),
        Log225(Log225Call),
        Log226(Log226Call),
        Log227(Log227Call),
        Log15(Log15Call),
        Log55(Log55Call),
        Log16(Log16Call),
        Log228(Log228Call),
        Log56(Log56Call),
        Log229(Log229Call),
        Log230(Log230Call),
        Log231(Log231Call),
        Log232(Log232Call),
        Log233(Log233Call),
        Log234(Log234Call),
        Log235(Log235Call),
        Log236(Log236Call),
        Log237(Log237Call),
        Log238(Log238Call),
        Log239(Log239Call),
        Log240(Log240Call),
        Log241(Log241Call),
        Log17(Log17Call),
        Log242(Log242Call),
        Log243(Log243Call),
        Log244(Log244Call),
        Log245(Log245Call),
        Log246(Log246Call),
        Log57(Log57Call),
        Log247(Log247Call),
        Log248(Log248Call),
        Log249(Log249Call),
        Log58(Log58Call),
        Log59(Log59Call),
        Log250(Log250Call),
        Log251(Log251Call),
        Log252(Log252Call),
        Log253(Log253Call),
        Log60(Log60Call),
        Log254(Log254Call),
        Log61(Log61Call),
        Log255(Log255Call),
        Log256(Log256Call),
        Log257(Log257Call),
        Log258(Log258Call),
        Log259(Log259Call),
        Log260(Log260Call),
        Log261(Log261Call),
        Log262(Log262Call),
        Log62(Log62Call),
        Log263(Log263Call),
        Log264(Log264Call),
        Log265(Log265Call),
        Log266(Log266Call),
        Log267(Log267Call),
        Log268(Log268Call),
        Log269(Log269Call),
        Log270(Log270Call),
        Log271(Log271Call),
        Log272(Log272Call),
        Log273(Log273Call),
        Log274(Log274Call),
        Log275(Log275Call),
        Log276(Log276Call),
        Log277(Log277Call),
        Log63(Log63Call),
        Log64(Log64Call),
        Log65(Log65Call),
        Log278(Log278Call),
        Log279(Log279Call),
        Log280(Log280Call),
        Log18(Log18Call),
        Log66(Log66Call),
        Log281(Log281Call),
        Log282(Log282Call),
        Log283(Log283Call),
        Log284(Log284Call),
        Log285(Log285Call),
        Log67(Log67Call),
        Log286(Log286Call),
        Log287(Log287Call),
        Log288(Log288Call),
        Log289(Log289Call),
        Log290(Log290Call),
        Log291(Log291Call),
        Log292(Log292Call),
        Log19(Log19Call),
        Log68(Log68Call),
        Log293(Log293Call),
        Log294(Log294Call),
        Log295(Log295Call),
        Log296(Log296Call),
        Log297(Log297Call),
        Log69(Log69Call),
        Log70(Log70Call),
        Log71(Log71Call),
        Log72(Log72Call),
        Log298(Log298Call),
        Log299(Log299Call),
        Log300(Log300Call),
        Log301(Log301Call),
        Log302(Log302Call),
        Log73(Log73Call),
        Log303(Log303Call),
        Log304(Log304Call),
        Log74(Log74Call),
        Log75(Log75Call),
        Log305(Log305Call),
        Log306(Log306Call),
        Log307(Log307Call),
        Log308(Log308Call),
        Log309(Log309Call),
        Log20(Log20Call),
        Log76(Log76Call),
        Log310(Log310Call),
        Log311(Log311Call),
        Log312(Log312Call),
        Log313(Log313Call),
        Log314(Log314Call),
        Log77(Log77Call),
        Log315(Log315Call),
        Log316(Log316Call),
        Log317(Log317Call),
        Log78(Log78Call),
        Log318(Log318Call),
        Log79(Log79Call),
        Log319(Log319Call),
        Log320(Log320Call),
        Log321(Log321Call),
        Log322(Log322Call),
        Log323(Log323Call),
        Log324(Log324Call),
        Log80(Log80Call),
        Log325(Log325Call),
        Log326(Log326Call),
        Log81(Log81Call),
        Log327(Log327Call),
        Log328(Log328Call),
        Log329(Log329Call),
        Log330(Log330Call),
        Log331(Log331Call),
        Log82(Log82Call),
        Log83(Log83Call),
        Log84(Log84Call),
        Log332(Log332Call),
        Log333(Log333Call),
        Log334(Log334Call),
        Log21(Log21Call),
        Log335(Log335Call),
        Log336(Log336Call),
        Log4(Log4Call),
        Log337(Log337Call),
        Log338(Log338Call),
        Log339(Log339Call),
        Log85(Log85Call),
        Log340(Log340Call),
        Log86(Log86Call),
        Log341(Log341Call),
        Log342(Log342Call),
        Log5(Log5Call),
        Log22(Log22Call),
        LogAddress(LogAddressCall),
        LogBool(LogBoolCall),
        LogBytes(LogBytesCall),
        LogBytes1(LogBytes1Call),
        LogBytes10(LogBytes10Call),
        LogBytes11(LogBytes11Call),
        LogBytes12(LogBytes12Call),
        LogBytes13(LogBytes13Call),
        LogBytes14(LogBytes14Call),
        LogBytes15(LogBytes15Call),
        LogBytes16(LogBytes16Call),
        LogBytes17(LogBytes17Call),
        LogBytes18(LogBytes18Call),
        LogBytes19(LogBytes19Call),
        LogBytes2(LogBytes2Call),
        LogBytes20(LogBytes20Call),
        LogBytes21(LogBytes21Call),
        LogBytes22(LogBytes22Call),
        LogBytes23(LogBytes23Call),
        LogBytes24(LogBytes24Call),
        LogBytes25(LogBytes25Call),
        LogBytes26(LogBytes26Call),
        LogBytes27(LogBytes27Call),
        LogBytes28(LogBytes28Call),
        LogBytes29(LogBytes29Call),
        LogBytes3(LogBytes3Call),
        LogBytes30(LogBytes30Call),
        LogBytes31(LogBytes31Call),
        LogBytes32(LogBytes32Call),
        LogBytes4(LogBytes4Call),
        LogBytes5(LogBytes5Call),
        LogBytes6(LogBytes6Call),
        LogBytes7(LogBytes7Call),
        LogBytes8(LogBytes8Call),
        LogBytes9(LogBytes9Call),
        LogInt(LogIntCall),
        LogString(LogStringCall),
        LogUint(LogUintCall),
    }
    impl ::corebc_core::abi::AbiDecode for HardhatConsoleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::corebc_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Log23Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log23(decoded));
            }
            if let Ok(decoded) = <Log87Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log87(decoded));
            }
            if let Ok(decoded) = <Log24Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log24(decoded));
            }
            if let Ok(decoded) = <Log88Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log88(decoded));
            }
            if let Ok(decoded) = <Log89Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log89(decoded));
            }
            if let Ok(decoded) = <Log90Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log90(decoded));
            }
            if let Ok(decoded) = <Log91Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log91(decoded));
            }
            if let Ok(decoded) = <Log25Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log25(decoded));
            }
            if let Ok(decoded) = <Log92Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log92(decoded));
            }
            if let Ok(decoded) = <Log93Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log93(decoded));
            }
            if let Ok(decoded) = <Log94Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log94(decoded));
            }
            if let Ok(decoded) = <Log95Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log95(decoded));
            }
            if let Ok(decoded) = <Log96Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log96(decoded));
            }
            if let Ok(decoded) = <Log26Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log26(decoded));
            }
            if let Ok(decoded) = <Log97Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log97(decoded));
            }
            if let Ok(decoded) = <Log98Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log98(decoded));
            }
            if let Ok(decoded) = <Log99Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log99(decoded));
            }
            if let Ok(decoded) = <Log100Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log100(decoded));
            }
            if let Ok(decoded) = <Log101Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log101(decoded));
            }
            if let Ok(decoded) = <Log102Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log102(decoded));
            }
            if let Ok(decoded) = <Log27Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log27(decoded));
            }
            if let Ok(decoded) = <Log28Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log28(decoded));
            }
            if let Ok(decoded) = <Log103Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log103(decoded));
            }
            if let Ok(decoded) = <Log29Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log29(decoded));
            }
            if let Ok(decoded) = <Log104Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log104(decoded));
            }
            if let Ok(decoded) = <Log105Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log105(decoded));
            }
            if let Ok(decoded) = <Log106Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log106(decoded));
            }
            if let Ok(decoded) = <Log107Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log107(decoded));
            }
            if let Ok(decoded) = <Log108Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log108(decoded));
            }
            if let Ok(decoded) = <Log109Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log109(decoded));
            }
            if let Ok(decoded) = <Log110Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log110(decoded));
            }
            if let Ok(decoded) = <Log111Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log111(decoded));
            }
            if let Ok(decoded) = <Log30Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log30(decoded));
            }
            if let Ok(decoded) = <Log31Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log31(decoded));
            }
            if let Ok(decoded) = <Log112Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log112(decoded));
            }
            if let Ok(decoded) = <Log113Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log113(decoded));
            }
            if let Ok(decoded) = <Log114Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log114(decoded));
            }
            if let Ok(decoded) = <Log115Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log115(decoded));
            }
            if let Ok(decoded) = <Log116Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log116(decoded));
            }
            if let Ok(decoded) = <Log32Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log32(decoded));
            }
            if let Ok(decoded) = <Log6Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log6(decoded));
            }
            if let Ok(decoded) = <Log117Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log117(decoded));
            }
            if let Ok(decoded) = <Log118Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log118(decoded));
            }
            if let Ok(decoded) = <Log119Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log119(decoded));
            }
            if let Ok(decoded) = <Log120Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log120(decoded));
            }
            if let Ok(decoded) = <Log33Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log33(decoded));
            }
            if let Ok(decoded) = <Log121Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log121(decoded));
            }
            if let Ok(decoded) = <Log34Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log34(decoded));
            }
            if let Ok(decoded) = <Log122Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log122(decoded));
            }
            if let Ok(decoded) = <Log35Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log35(decoded));
            }
            if let Ok(decoded) = <Log123Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log123(decoded));
            }
            if let Ok(decoded) = <Log124Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log124(decoded));
            }
            if let Ok(decoded) = <Log125Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log125(decoded));
            }
            if let Ok(decoded) = <Log126Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log126(decoded));
            }
            if let Ok(decoded) = <Log127Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log127(decoded));
            }
            if let Ok(decoded) = <Log128Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log128(decoded));
            }
            if let Ok(decoded) = <Log129Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log129(decoded));
            }
            if let Ok(decoded) = <Log36Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log36(decoded));
            }
            if let Ok(decoded) = <Log130Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log130(decoded));
            }
            if let Ok(decoded) = <Log131Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log131(decoded));
            }
            if let Ok(decoded) = <Log132Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log132(decoded));
            }
            if let Ok(decoded) = <Log7Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log7(decoded));
            }
            if let Ok(decoded) = <Log133Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log133(decoded));
            }
            if let Ok(decoded) = <Log134Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log134(decoded));
            }
            if let Ok(decoded) = <Log135Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log135(decoded));
            }
            if let Ok(decoded) = <Log136Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log136(decoded));
            }
            if let Ok(decoded) = <Log1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log1(decoded));
            }
            if let Ok(decoded) = <Log137Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log137(decoded));
            }
            if let Ok(decoded) = <Log37Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log37(decoded));
            }
            if let Ok(decoded) = <Log138Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log138(decoded));
            }
            if let Ok(decoded) = <Log139Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log139(decoded));
            }
            if let Ok(decoded) = <Log8Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log8(decoded));
            }
            if let Ok(decoded) = <Log2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log2(decoded));
            }
            if let Ok(decoded) = <Log140Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log140(decoded));
            }
            if let Ok(decoded) = <Log141Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log141(decoded));
            }
            if let Ok(decoded) = <Log38Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log38(decoded));
            }
            if let Ok(decoded) = <Log142Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log142(decoded));
            }
            if let Ok(decoded) = <Log143Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log143(decoded));
            }
            if let Ok(decoded) = <Log39Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log39(decoded));
            }
            if let Ok(decoded) = <Log144Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log144(decoded));
            }
            if let Ok(decoded) = <Log40Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log40(decoded));
            }
            if let Ok(decoded) = <Log145Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log145(decoded));
            }
            if let Ok(decoded) = <Log146Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log146(decoded));
            }
            if let Ok(decoded) = <Log9Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log9(decoded));
            }
            if let Ok(decoded) = <Log147Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log147(decoded));
            }
            if let Ok(decoded) = <Log148Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log148(decoded));
            }
            if let Ok(decoded) = <Log149Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log149(decoded));
            }
            if let Ok(decoded) = <Log150Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log150(decoded));
            }
            if let Ok(decoded) = <Log151Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log151(decoded));
            }
            if let Ok(decoded) = <Log152Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log152(decoded));
            }
            if let Ok(decoded) = <Log153Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log153(decoded));
            }
            if let Ok(decoded) = <Log3Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log3(decoded));
            }
            if let Ok(decoded) = <Log154Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log154(decoded));
            }
            if let Ok(decoded) = <Log155Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log155(decoded));
            }
            if let Ok(decoded) = <Log156Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log156(decoded));
            }
            if let Ok(decoded) = <Log157Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log157(decoded));
            }
            if let Ok(decoded) = <Log158Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log158(decoded));
            }
            if let Ok(decoded) = <Log159Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log159(decoded));
            }
            if let Ok(decoded) = <Log160Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log160(decoded));
            }
            if let Ok(decoded) = <Log161Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log161(decoded));
            }
            if let Ok(decoded) = <Log41Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log41(decoded));
            }
            if let Ok(decoded) = <Log162Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log162(decoded));
            }
            if let Ok(decoded) = <Log163Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log163(decoded));
            }
            if let Ok(decoded) = <Log164Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log164(decoded));
            }
            if let Ok(decoded) = <Log165Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log165(decoded));
            }
            if let Ok(decoded) = <Log10Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log10(decoded));
            }
            if let Ok(decoded) = <Log166Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log166(decoded));
            }
            if let Ok(decoded) = <Log42Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log42(decoded));
            }
            if let Ok(decoded) = <Log167Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log167(decoded));
            }
            if let Ok(decoded) = <Log43Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log43(decoded));
            }
            if let Ok(decoded) = <Log168Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log168(decoded));
            }
            if let Ok(decoded) = <Log169Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log169(decoded));
            }
            if let Ok(decoded) = <Log0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log0(decoded));
            }
            if let Ok(decoded) = <Log170Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log170(decoded));
            }
            if let Ok(decoded) = <Log171Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log171(decoded));
            }
            if let Ok(decoded) = <Log172Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log172(decoded));
            }
            if let Ok(decoded) = <Log173Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log173(decoded));
            }
            if let Ok(decoded) = <Log44Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log44(decoded));
            }
            if let Ok(decoded) = <Log45Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log45(decoded));
            }
            if let Ok(decoded) = <Log174Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log174(decoded));
            }
            if let Ok(decoded) = <Log175Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log175(decoded));
            }
            if let Ok(decoded) = <Log46Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log46(decoded));
            }
            if let Ok(decoded) = <Log176Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log176(decoded));
            }
            if let Ok(decoded) = <Log177Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log177(decoded));
            }
            if let Ok(decoded) = <Log178Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log178(decoded));
            }
            if let Ok(decoded) = <Log47Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log47(decoded));
            }
            if let Ok(decoded) = <Log179Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log179(decoded));
            }
            if let Ok(decoded) = <Log180Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log180(decoded));
            }
            if let Ok(decoded) = <Log181Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log181(decoded));
            }
            if let Ok(decoded) = <Log182Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log182(decoded));
            }
            if let Ok(decoded) = <Log183Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log183(decoded));
            }
            if let Ok(decoded) = <Log184Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log184(decoded));
            }
            if let Ok(decoded) = <Log185Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log185(decoded));
            }
            if let Ok(decoded) = <Log186Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log186(decoded));
            }
            if let Ok(decoded) = <Log187Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log187(decoded));
            }
            if let Ok(decoded) = <Log188Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log188(decoded));
            }
            if let Ok(decoded) = <Log48Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log48(decoded));
            }
            if let Ok(decoded) = <Log189Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log189(decoded));
            }
            if let Ok(decoded) = <Log190Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log190(decoded));
            }
            if let Ok(decoded) = <Log191Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log191(decoded));
            }
            if let Ok(decoded) = <Log49Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log49(decoded));
            }
            if let Ok(decoded) = <Log192Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log192(decoded));
            }
            if let Ok(decoded) = <Log11Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log11(decoded));
            }
            if let Ok(decoded) = <Log193Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log193(decoded));
            }
            if let Ok(decoded) = <Log194Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log194(decoded));
            }
            if let Ok(decoded) = <Log195Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log195(decoded));
            }
            if let Ok(decoded) = <Log196Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log196(decoded));
            }
            if let Ok(decoded) = <Log50Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log50(decoded));
            }
            if let Ok(decoded) = <Log51Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log51(decoded));
            }
            if let Ok(decoded) = <Log197Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log197(decoded));
            }
            if let Ok(decoded) = <Log198Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log198(decoded));
            }
            if let Ok(decoded) = <Log12Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log12(decoded));
            }
            if let Ok(decoded) = <Log199Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log199(decoded));
            }
            if let Ok(decoded) = <Log200Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log200(decoded));
            }
            if let Ok(decoded) = <Log201Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log201(decoded));
            }
            if let Ok(decoded) = <Log202Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log202(decoded));
            }
            if let Ok(decoded) = <Log203Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log203(decoded));
            }
            if let Ok(decoded) = <Log204Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log204(decoded));
            }
            if let Ok(decoded) = <Log205Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log205(decoded));
            }
            if let Ok(decoded) = <Log206Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log206(decoded));
            }
            if let Ok(decoded) = <Log207Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log207(decoded));
            }
            if let Ok(decoded) = <Log208Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log208(decoded));
            }
            if let Ok(decoded) = <Log209Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log209(decoded));
            }
            if let Ok(decoded) = <Log210Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log210(decoded));
            }
            if let Ok(decoded) = <Log52Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log52(decoded));
            }
            if let Ok(decoded) = <Log211Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log211(decoded));
            }
            if let Ok(decoded) = <Log212Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log212(decoded));
            }
            if let Ok(decoded) = <Log213Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log213(decoded));
            }
            if let Ok(decoded) = <Log13Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log13(decoded));
            }
            if let Ok(decoded) = <Log14Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log14(decoded));
            }
            if let Ok(decoded) = <Log214Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log214(decoded));
            }
            if let Ok(decoded) = <Log215Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log215(decoded));
            }
            if let Ok(decoded) = <Log216Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log216(decoded));
            }
            if let Ok(decoded) = <Log53Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log53(decoded));
            }
            if let Ok(decoded) = <Log54Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log54(decoded));
            }
            if let Ok(decoded) = <Log217Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log217(decoded));
            }
            if let Ok(decoded) = <Log218Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log218(decoded));
            }
            if let Ok(decoded) = <Log219Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log219(decoded));
            }
            if let Ok(decoded) = <Log220Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log220(decoded));
            }
            if let Ok(decoded) = <Log221Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log221(decoded));
            }
            if let Ok(decoded) = <Log222Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log222(decoded));
            }
            if let Ok(decoded) = <Log223Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log223(decoded));
            }
            if let Ok(decoded) = <Log224Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log224(decoded));
            }
            if let Ok(decoded) = <Log225Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log225(decoded));
            }
            if let Ok(decoded) = <Log226Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log226(decoded));
            }
            if let Ok(decoded) = <Log227Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log227(decoded));
            }
            if let Ok(decoded) = <Log15Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log15(decoded));
            }
            if let Ok(decoded) = <Log55Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log55(decoded));
            }
            if let Ok(decoded) = <Log16Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log16(decoded));
            }
            if let Ok(decoded) = <Log228Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log228(decoded));
            }
            if let Ok(decoded) = <Log56Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log56(decoded));
            }
            if let Ok(decoded) = <Log229Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log229(decoded));
            }
            if let Ok(decoded) = <Log230Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log230(decoded));
            }
            if let Ok(decoded) = <Log231Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log231(decoded));
            }
            if let Ok(decoded) = <Log232Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log232(decoded));
            }
            if let Ok(decoded) = <Log233Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log233(decoded));
            }
            if let Ok(decoded) = <Log234Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log234(decoded));
            }
            if let Ok(decoded) = <Log235Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log235(decoded));
            }
            if let Ok(decoded) = <Log236Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log236(decoded));
            }
            if let Ok(decoded) = <Log237Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log237(decoded));
            }
            if let Ok(decoded) = <Log238Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log238(decoded));
            }
            if let Ok(decoded) = <Log239Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log239(decoded));
            }
            if let Ok(decoded) = <Log240Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log240(decoded));
            }
            if let Ok(decoded) = <Log241Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log241(decoded));
            }
            if let Ok(decoded) = <Log17Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log17(decoded));
            }
            if let Ok(decoded) = <Log242Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log242(decoded));
            }
            if let Ok(decoded) = <Log243Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log243(decoded));
            }
            if let Ok(decoded) = <Log244Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log244(decoded));
            }
            if let Ok(decoded) = <Log245Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log245(decoded));
            }
            if let Ok(decoded) = <Log246Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log246(decoded));
            }
            if let Ok(decoded) = <Log57Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log57(decoded));
            }
            if let Ok(decoded) = <Log247Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log247(decoded));
            }
            if let Ok(decoded) = <Log248Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log248(decoded));
            }
            if let Ok(decoded) = <Log249Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log249(decoded));
            }
            if let Ok(decoded) = <Log58Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log58(decoded));
            }
            if let Ok(decoded) = <Log59Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log59(decoded));
            }
            if let Ok(decoded) = <Log250Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log250(decoded));
            }
            if let Ok(decoded) = <Log251Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log251(decoded));
            }
            if let Ok(decoded) = <Log252Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log252(decoded));
            }
            if let Ok(decoded) = <Log253Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log253(decoded));
            }
            if let Ok(decoded) = <Log60Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log60(decoded));
            }
            if let Ok(decoded) = <Log254Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log254(decoded));
            }
            if let Ok(decoded) = <Log61Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log61(decoded));
            }
            if let Ok(decoded) = <Log255Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log255(decoded));
            }
            if let Ok(decoded) = <Log256Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log256(decoded));
            }
            if let Ok(decoded) = <Log257Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log257(decoded));
            }
            if let Ok(decoded) = <Log258Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log258(decoded));
            }
            if let Ok(decoded) = <Log259Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log259(decoded));
            }
            if let Ok(decoded) = <Log260Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log260(decoded));
            }
            if let Ok(decoded) = <Log261Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log261(decoded));
            }
            if let Ok(decoded) = <Log262Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log262(decoded));
            }
            if let Ok(decoded) = <Log62Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log62(decoded));
            }
            if let Ok(decoded) = <Log263Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log263(decoded));
            }
            if let Ok(decoded) = <Log264Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log264(decoded));
            }
            if let Ok(decoded) = <Log265Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log265(decoded));
            }
            if let Ok(decoded) = <Log266Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log266(decoded));
            }
            if let Ok(decoded) = <Log267Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log267(decoded));
            }
            if let Ok(decoded) = <Log268Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log268(decoded));
            }
            if let Ok(decoded) = <Log269Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log269(decoded));
            }
            if let Ok(decoded) = <Log270Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log270(decoded));
            }
            if let Ok(decoded) = <Log271Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log271(decoded));
            }
            if let Ok(decoded) = <Log272Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log272(decoded));
            }
            if let Ok(decoded) = <Log273Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log273(decoded));
            }
            if let Ok(decoded) = <Log274Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log274(decoded));
            }
            if let Ok(decoded) = <Log275Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log275(decoded));
            }
            if let Ok(decoded) = <Log276Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log276(decoded));
            }
            if let Ok(decoded) = <Log277Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log277(decoded));
            }
            if let Ok(decoded) = <Log63Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log63(decoded));
            }
            if let Ok(decoded) = <Log64Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log64(decoded));
            }
            if let Ok(decoded) = <Log65Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log65(decoded));
            }
            if let Ok(decoded) = <Log278Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log278(decoded));
            }
            if let Ok(decoded) = <Log279Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log279(decoded));
            }
            if let Ok(decoded) = <Log280Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log280(decoded));
            }
            if let Ok(decoded) = <Log18Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log18(decoded));
            }
            if let Ok(decoded) = <Log66Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log66(decoded));
            }
            if let Ok(decoded) = <Log281Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log281(decoded));
            }
            if let Ok(decoded) = <Log282Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log282(decoded));
            }
            if let Ok(decoded) = <Log283Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log283(decoded));
            }
            if let Ok(decoded) = <Log284Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log284(decoded));
            }
            if let Ok(decoded) = <Log285Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log285(decoded));
            }
            if let Ok(decoded) = <Log67Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log67(decoded));
            }
            if let Ok(decoded) = <Log286Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log286(decoded));
            }
            if let Ok(decoded) = <Log287Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log287(decoded));
            }
            if let Ok(decoded) = <Log288Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log288(decoded));
            }
            if let Ok(decoded) = <Log289Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log289(decoded));
            }
            if let Ok(decoded) = <Log290Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log290(decoded));
            }
            if let Ok(decoded) = <Log291Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log291(decoded));
            }
            if let Ok(decoded) = <Log292Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log292(decoded));
            }
            if let Ok(decoded) = <Log19Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log19(decoded));
            }
            if let Ok(decoded) = <Log68Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log68(decoded));
            }
            if let Ok(decoded) = <Log293Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log293(decoded));
            }
            if let Ok(decoded) = <Log294Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log294(decoded));
            }
            if let Ok(decoded) = <Log295Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log295(decoded));
            }
            if let Ok(decoded) = <Log296Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log296(decoded));
            }
            if let Ok(decoded) = <Log297Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log297(decoded));
            }
            if let Ok(decoded) = <Log69Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log69(decoded));
            }
            if let Ok(decoded) = <Log70Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log70(decoded));
            }
            if let Ok(decoded) = <Log71Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log71(decoded));
            }
            if let Ok(decoded) = <Log72Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log72(decoded));
            }
            if let Ok(decoded) = <Log298Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log298(decoded));
            }
            if let Ok(decoded) = <Log299Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log299(decoded));
            }
            if let Ok(decoded) = <Log300Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log300(decoded));
            }
            if let Ok(decoded) = <Log301Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log301(decoded));
            }
            if let Ok(decoded) = <Log302Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log302(decoded));
            }
            if let Ok(decoded) = <Log73Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log73(decoded));
            }
            if let Ok(decoded) = <Log303Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log303(decoded));
            }
            if let Ok(decoded) = <Log304Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log304(decoded));
            }
            if let Ok(decoded) = <Log74Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log74(decoded));
            }
            if let Ok(decoded) = <Log75Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log75(decoded));
            }
            if let Ok(decoded) = <Log305Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log305(decoded));
            }
            if let Ok(decoded) = <Log306Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log306(decoded));
            }
            if let Ok(decoded) = <Log307Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log307(decoded));
            }
            if let Ok(decoded) = <Log308Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log308(decoded));
            }
            if let Ok(decoded) = <Log309Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log309(decoded));
            }
            if let Ok(decoded) = <Log20Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log20(decoded));
            }
            if let Ok(decoded) = <Log76Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log76(decoded));
            }
            if let Ok(decoded) = <Log310Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log310(decoded));
            }
            if let Ok(decoded) = <Log311Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log311(decoded));
            }
            if let Ok(decoded) = <Log312Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log312(decoded));
            }
            if let Ok(decoded) = <Log313Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log313(decoded));
            }
            if let Ok(decoded) = <Log314Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log314(decoded));
            }
            if let Ok(decoded) = <Log77Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log77(decoded));
            }
            if let Ok(decoded) = <Log315Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log315(decoded));
            }
            if let Ok(decoded) = <Log316Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log316(decoded));
            }
            if let Ok(decoded) = <Log317Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log317(decoded));
            }
            if let Ok(decoded) = <Log78Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log78(decoded));
            }
            if let Ok(decoded) = <Log318Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log318(decoded));
            }
            if let Ok(decoded) = <Log79Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log79(decoded));
            }
            if let Ok(decoded) = <Log319Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log319(decoded));
            }
            if let Ok(decoded) = <Log320Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log320(decoded));
            }
            if let Ok(decoded) = <Log321Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log321(decoded));
            }
            if let Ok(decoded) = <Log322Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log322(decoded));
            }
            if let Ok(decoded) = <Log323Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log323(decoded));
            }
            if let Ok(decoded) = <Log324Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log324(decoded));
            }
            if let Ok(decoded) = <Log80Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log80(decoded));
            }
            if let Ok(decoded) = <Log325Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log325(decoded));
            }
            if let Ok(decoded) = <Log326Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log326(decoded));
            }
            if let Ok(decoded) = <Log81Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log81(decoded));
            }
            if let Ok(decoded) = <Log327Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log327(decoded));
            }
            if let Ok(decoded) = <Log328Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log328(decoded));
            }
            if let Ok(decoded) = <Log329Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log329(decoded));
            }
            if let Ok(decoded) = <Log330Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log330(decoded));
            }
            if let Ok(decoded) = <Log331Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log331(decoded));
            }
            if let Ok(decoded) = <Log82Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log82(decoded));
            }
            if let Ok(decoded) = <Log83Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log83(decoded));
            }
            if let Ok(decoded) = <Log84Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log84(decoded));
            }
            if let Ok(decoded) = <Log332Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log332(decoded));
            }
            if let Ok(decoded) = <Log333Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log333(decoded));
            }
            if let Ok(decoded) = <Log334Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log334(decoded));
            }
            if let Ok(decoded) = <Log21Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log21(decoded));
            }
            if let Ok(decoded) = <Log335Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log335(decoded));
            }
            if let Ok(decoded) = <Log336Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log336(decoded));
            }
            if let Ok(decoded) = <Log4Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log4(decoded));
            }
            if let Ok(decoded) = <Log337Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log337(decoded));
            }
            if let Ok(decoded) = <Log338Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log338(decoded));
            }
            if let Ok(decoded) = <Log339Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log339(decoded));
            }
            if let Ok(decoded) = <Log85Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log85(decoded));
            }
            if let Ok(decoded) = <Log340Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log340(decoded));
            }
            if let Ok(decoded) = <Log86Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log86(decoded));
            }
            if let Ok(decoded) = <Log341Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log341(decoded));
            }
            if let Ok(decoded) = <Log342Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log342(decoded));
            }
            if let Ok(decoded) = <Log5Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log5(decoded));
            }
            if let Ok(decoded) = <Log22Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Log22(decoded));
            }
            if let Ok(decoded) = <LogAddressCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogAddress(decoded));
            }
            if let Ok(decoded) = <LogBoolCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBool(decoded));
            }
            if let Ok(decoded) = <LogBytesCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes(decoded));
            }
            if let Ok(decoded) = <LogBytes1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes1(decoded));
            }
            if let Ok(decoded) = <LogBytes10Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes10(decoded));
            }
            if let Ok(decoded) = <LogBytes11Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes11(decoded));
            }
            if let Ok(decoded) = <LogBytes12Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes12(decoded));
            }
            if let Ok(decoded) = <LogBytes13Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes13(decoded));
            }
            if let Ok(decoded) = <LogBytes14Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes14(decoded));
            }
            if let Ok(decoded) = <LogBytes15Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes15(decoded));
            }
            if let Ok(decoded) = <LogBytes16Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes16(decoded));
            }
            if let Ok(decoded) = <LogBytes17Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes17(decoded));
            }
            if let Ok(decoded) = <LogBytes18Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes18(decoded));
            }
            if let Ok(decoded) = <LogBytes19Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes19(decoded));
            }
            if let Ok(decoded) = <LogBytes2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes2(decoded));
            }
            if let Ok(decoded) = <LogBytes20Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes20(decoded));
            }
            if let Ok(decoded) = <LogBytes21Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes21(decoded));
            }
            if let Ok(decoded) = <LogBytes22Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes22(decoded));
            }
            if let Ok(decoded) = <LogBytes23Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes23(decoded));
            }
            if let Ok(decoded) = <LogBytes24Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes24(decoded));
            }
            if let Ok(decoded) = <LogBytes25Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes25(decoded));
            }
            if let Ok(decoded) = <LogBytes26Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes26(decoded));
            }
            if let Ok(decoded) = <LogBytes27Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes27(decoded));
            }
            if let Ok(decoded) = <LogBytes28Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes28(decoded));
            }
            if let Ok(decoded) = <LogBytes29Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes29(decoded));
            }
            if let Ok(decoded) = <LogBytes3Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes3(decoded));
            }
            if let Ok(decoded) = <LogBytes30Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes30(decoded));
            }
            if let Ok(decoded) = <LogBytes31Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes31(decoded));
            }
            if let Ok(decoded) = <LogBytes32Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes32(decoded));
            }
            if let Ok(decoded) = <LogBytes4Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes4(decoded));
            }
            if let Ok(decoded) = <LogBytes5Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes5(decoded));
            }
            if let Ok(decoded) = <LogBytes6Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes6(decoded));
            }
            if let Ok(decoded) = <LogBytes7Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes7(decoded));
            }
            if let Ok(decoded) = <LogBytes8Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes8(decoded));
            }
            if let Ok(decoded) = <LogBytes9Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogBytes9(decoded));
            }
            if let Ok(decoded) = <LogIntCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogInt(decoded));
            }
            if let Ok(decoded) = <LogStringCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogString(decoded));
            }
            if let Ok(decoded) = <LogUintCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LogUint(decoded));
            }
            Err(::corebc_core::abi::Error::InvalidData.into())
        }
    }
    impl ::corebc_core::abi::AbiEncode for HardhatConsoleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Log23(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log87(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log24(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log88(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log89(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log90(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log91(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log25(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log92(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log93(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log94(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log95(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log96(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log26(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log97(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log98(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log99(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log100(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log101(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log102(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log27(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log28(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log103(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log29(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log104(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log105(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log106(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log107(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log108(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log109(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log110(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log111(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log30(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log31(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log112(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log113(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log114(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log115(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log116(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log32(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log6(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log117(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log118(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log119(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log120(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log33(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log121(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log34(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log122(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log35(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log123(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log124(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log125(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log126(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log127(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log128(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log129(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log36(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log130(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log131(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log132(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log7(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log133(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log134(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log135(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log136(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log1(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log137(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log37(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log138(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log139(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log8(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log2(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log140(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log141(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log38(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log142(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log143(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log39(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log144(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log40(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log145(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log146(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log9(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log147(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log148(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log149(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log150(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log151(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log152(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log153(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log3(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log154(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log155(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log156(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log157(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log158(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log159(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log160(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log161(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log41(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log162(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log163(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log164(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log165(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log10(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log166(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log42(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log167(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log43(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log168(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log169(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log0(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log170(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log171(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log172(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log173(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log44(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log45(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log174(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log175(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log46(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log176(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log177(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log178(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log47(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log179(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log180(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log181(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log182(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log183(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log184(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log185(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log186(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log187(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log188(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log48(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log189(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log190(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log191(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log49(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log192(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log11(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log193(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log194(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log195(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log196(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log50(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log51(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log197(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log198(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log12(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log199(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log200(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log201(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log202(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log203(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log204(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log205(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log206(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log207(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log208(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log209(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log210(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log52(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log211(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log212(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log213(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log13(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log14(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log214(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log215(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log216(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log53(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log54(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log217(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log218(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log219(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log220(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log221(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log222(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log223(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log224(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log225(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log226(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log227(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log15(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log55(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log16(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log228(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log56(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log229(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log230(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log231(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log232(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log233(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log234(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log235(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log236(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log237(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log238(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log239(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log240(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log241(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log17(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log242(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log243(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log244(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log245(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log246(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log57(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log247(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log248(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log249(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log58(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log59(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log250(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log251(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log252(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log253(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log60(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log254(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log61(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log255(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log256(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log257(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log258(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log259(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log260(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log261(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log262(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log62(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log263(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log264(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log265(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log266(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log267(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log268(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log269(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log270(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log271(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log272(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log273(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log274(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log275(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log276(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log277(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log63(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log64(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log65(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log278(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log279(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log280(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log18(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log66(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log281(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log282(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log283(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log284(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log285(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log67(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log286(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log287(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log288(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log289(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log290(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log291(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log292(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log19(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log68(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log293(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log294(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log295(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log296(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log297(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log69(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log70(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log71(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log72(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log298(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log299(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log300(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log301(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log302(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log73(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log303(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log304(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log74(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log75(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log305(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log306(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log307(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log308(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log309(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log20(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log76(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log310(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log311(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log312(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log313(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log314(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log77(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log315(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log316(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log317(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log78(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log318(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log79(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log319(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log320(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log321(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log322(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log323(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log324(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log80(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log325(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log326(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log81(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log327(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log328(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log329(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log330(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log331(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log82(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log83(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log84(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log332(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log333(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log334(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log21(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log335(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log336(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log4(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log337(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log338(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log339(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log85(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log340(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log86(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log341(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log342(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log5(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Log22(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::LogAddress(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBool(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::LogBytes(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::LogBytes1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes10(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes11(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes12(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes13(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes14(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes15(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes16(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes17(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes18(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes19(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes20(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes21(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes22(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes23(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes24(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes25(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes26(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes27(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes28(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes29(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes3(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes30(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes31(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes32(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes4(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes5(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes6(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes7(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes8(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes9(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogInt(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::LogString(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::LogUint(element) => ::corebc_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for HardhatConsoleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Log23(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log87(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log24(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log88(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log89(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log90(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log91(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log25(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log92(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log93(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log94(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log95(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log96(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log26(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log97(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log98(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log99(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log100(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log101(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log102(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log27(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log28(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log103(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log29(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log104(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log105(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log106(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log107(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log108(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log109(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log110(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log111(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log30(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log31(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log112(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log113(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log114(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log115(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log116(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log32(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log6(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log117(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log118(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log119(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log120(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log33(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log121(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log34(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log122(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log35(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log123(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log124(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log125(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log126(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log127(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log128(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log129(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log36(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log130(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log131(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log132(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log7(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log133(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log134(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log135(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log136(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log137(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log37(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log138(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log139(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log8(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log140(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log141(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log38(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log142(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log143(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log39(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log144(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log40(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log145(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log146(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log9(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log147(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log148(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log149(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log150(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log151(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log152(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log153(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log154(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log155(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log156(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log157(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log158(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log159(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log160(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log161(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log41(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log162(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log163(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log164(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log165(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log10(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log166(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log42(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log167(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log43(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log168(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log169(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log170(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log171(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log172(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log173(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log44(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log45(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log174(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log175(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log46(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log176(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log177(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log178(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log47(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log179(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log180(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log181(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log182(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log183(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log184(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log185(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log186(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log187(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log188(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log48(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log189(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log190(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log191(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log49(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log192(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log11(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log193(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log194(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log195(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log196(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log50(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log51(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log197(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log198(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log12(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log199(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log200(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log201(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log202(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log203(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log204(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log205(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log206(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log207(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log208(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log209(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log210(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log52(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log211(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log212(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log213(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log13(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log14(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log214(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log215(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log216(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log53(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log54(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log217(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log218(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log219(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log220(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log221(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log222(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log223(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log224(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log225(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log226(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log227(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log15(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log55(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log16(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log228(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log56(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log229(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log230(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log231(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log232(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log233(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log234(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log235(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log236(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log237(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log238(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log239(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log240(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log241(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log17(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log242(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log243(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log244(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log245(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log246(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log57(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log247(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log248(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log249(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log58(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log59(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log250(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log251(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log252(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log253(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log60(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log254(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log61(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log255(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log256(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log257(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log258(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log259(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log260(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log261(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log262(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log62(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log263(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log264(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log265(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log266(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log267(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log268(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log269(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log270(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log271(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log272(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log273(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log274(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log275(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log276(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log277(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log63(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log64(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log65(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log278(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log279(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log280(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log18(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log66(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log281(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log282(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log283(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log284(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log285(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log67(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log286(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log287(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log288(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log289(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log290(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log291(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log292(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log19(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log68(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log293(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log294(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log295(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log296(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log297(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log69(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log70(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log71(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log72(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log298(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log299(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log300(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log301(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log302(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log73(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log303(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log304(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log74(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log75(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log305(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log306(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log307(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log308(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log309(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log20(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log76(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log310(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log311(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log312(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log313(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log314(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log77(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log315(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log316(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log317(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log78(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log318(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log79(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log319(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log320(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log321(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log322(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log323(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log324(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log80(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log325(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log326(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log81(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log327(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log328(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log329(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log330(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log331(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log82(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log83(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log84(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log332(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log333(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log334(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log21(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log335(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log336(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log4(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log337(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log338(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log339(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log85(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log340(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log86(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log341(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log342(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log5(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log22(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes1(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes10(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes11(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes12(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes13(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes14(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes15(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes16(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes17(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes18(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes19(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes2(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes20(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes21(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes22(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes23(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes24(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes25(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes26(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes27(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes28(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes29(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes3(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes30(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes31(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes4(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes5(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes6(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes7(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes8(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes9(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogString(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUint(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Log23Call> for HardhatConsoleCalls {
        fn from(value: Log23Call) -> Self {
            Self::Log23(value)
        }
    }
    impl ::core::convert::From<Log87Call> for HardhatConsoleCalls {
        fn from(value: Log87Call) -> Self {
            Self::Log87(value)
        }
    }
    impl ::core::convert::From<Log24Call> for HardhatConsoleCalls {
        fn from(value: Log24Call) -> Self {
            Self::Log24(value)
        }
    }
    impl ::core::convert::From<Log88Call> for HardhatConsoleCalls {
        fn from(value: Log88Call) -> Self {
            Self::Log88(value)
        }
    }
    impl ::core::convert::From<Log89Call> for HardhatConsoleCalls {
        fn from(value: Log89Call) -> Self {
            Self::Log89(value)
        }
    }
    impl ::core::convert::From<Log90Call> for HardhatConsoleCalls {
        fn from(value: Log90Call) -> Self {
            Self::Log90(value)
        }
    }
    impl ::core::convert::From<Log91Call> for HardhatConsoleCalls {
        fn from(value: Log91Call) -> Self {
            Self::Log91(value)
        }
    }
    impl ::core::convert::From<Log25Call> for HardhatConsoleCalls {
        fn from(value: Log25Call) -> Self {
            Self::Log25(value)
        }
    }
    impl ::core::convert::From<Log92Call> for HardhatConsoleCalls {
        fn from(value: Log92Call) -> Self {
            Self::Log92(value)
        }
    }
    impl ::core::convert::From<Log93Call> for HardhatConsoleCalls {
        fn from(value: Log93Call) -> Self {
            Self::Log93(value)
        }
    }
    impl ::core::convert::From<Log94Call> for HardhatConsoleCalls {
        fn from(value: Log94Call) -> Self {
            Self::Log94(value)
        }
    }
    impl ::core::convert::From<Log95Call> for HardhatConsoleCalls {
        fn from(value: Log95Call) -> Self {
            Self::Log95(value)
        }
    }
    impl ::core::convert::From<Log96Call> for HardhatConsoleCalls {
        fn from(value: Log96Call) -> Self {
            Self::Log96(value)
        }
    }
    impl ::core::convert::From<Log26Call> for HardhatConsoleCalls {
        fn from(value: Log26Call) -> Self {
            Self::Log26(value)
        }
    }
    impl ::core::convert::From<Log97Call> for HardhatConsoleCalls {
        fn from(value: Log97Call) -> Self {
            Self::Log97(value)
        }
    }
    impl ::core::convert::From<Log98Call> for HardhatConsoleCalls {
        fn from(value: Log98Call) -> Self {
            Self::Log98(value)
        }
    }
    impl ::core::convert::From<Log99Call> for HardhatConsoleCalls {
        fn from(value: Log99Call) -> Self {
            Self::Log99(value)
        }
    }
    impl ::core::convert::From<Log100Call> for HardhatConsoleCalls {
        fn from(value: Log100Call) -> Self {
            Self::Log100(value)
        }
    }
    impl ::core::convert::From<Log101Call> for HardhatConsoleCalls {
        fn from(value: Log101Call) -> Self {
            Self::Log101(value)
        }
    }
    impl ::core::convert::From<Log102Call> for HardhatConsoleCalls {
        fn from(value: Log102Call) -> Self {
            Self::Log102(value)
        }
    }
    impl ::core::convert::From<Log27Call> for HardhatConsoleCalls {
        fn from(value: Log27Call) -> Self {
            Self::Log27(value)
        }
    }
    impl ::core::convert::From<Log28Call> for HardhatConsoleCalls {
        fn from(value: Log28Call) -> Self {
            Self::Log28(value)
        }
    }
    impl ::core::convert::From<Log103Call> for HardhatConsoleCalls {
        fn from(value: Log103Call) -> Self {
            Self::Log103(value)
        }
    }
    impl ::core::convert::From<Log29Call> for HardhatConsoleCalls {
        fn from(value: Log29Call) -> Self {
            Self::Log29(value)
        }
    }
    impl ::core::convert::From<Log104Call> for HardhatConsoleCalls {
        fn from(value: Log104Call) -> Self {
            Self::Log104(value)
        }
    }
    impl ::core::convert::From<Log105Call> for HardhatConsoleCalls {
        fn from(value: Log105Call) -> Self {
            Self::Log105(value)
        }
    }
    impl ::core::convert::From<Log106Call> for HardhatConsoleCalls {
        fn from(value: Log106Call) -> Self {
            Self::Log106(value)
        }
    }
    impl ::core::convert::From<Log107Call> for HardhatConsoleCalls {
        fn from(value: Log107Call) -> Self {
            Self::Log107(value)
        }
    }
    impl ::core::convert::From<Log108Call> for HardhatConsoleCalls {
        fn from(value: Log108Call) -> Self {
            Self::Log108(value)
        }
    }
    impl ::core::convert::From<Log109Call> for HardhatConsoleCalls {
        fn from(value: Log109Call) -> Self {
            Self::Log109(value)
        }
    }
    impl ::core::convert::From<Log110Call> for HardhatConsoleCalls {
        fn from(value: Log110Call) -> Self {
            Self::Log110(value)
        }
    }
    impl ::core::convert::From<Log111Call> for HardhatConsoleCalls {
        fn from(value: Log111Call) -> Self {
            Self::Log111(value)
        }
    }
    impl ::core::convert::From<Log30Call> for HardhatConsoleCalls {
        fn from(value: Log30Call) -> Self {
            Self::Log30(value)
        }
    }
    impl ::core::convert::From<Log31Call> for HardhatConsoleCalls {
        fn from(value: Log31Call) -> Self {
            Self::Log31(value)
        }
    }
    impl ::core::convert::From<Log112Call> for HardhatConsoleCalls {
        fn from(value: Log112Call) -> Self {
            Self::Log112(value)
        }
    }
    impl ::core::convert::From<Log113Call> for HardhatConsoleCalls {
        fn from(value: Log113Call) -> Self {
            Self::Log113(value)
        }
    }
    impl ::core::convert::From<Log114Call> for HardhatConsoleCalls {
        fn from(value: Log114Call) -> Self {
            Self::Log114(value)
        }
    }
    impl ::core::convert::From<Log115Call> for HardhatConsoleCalls {
        fn from(value: Log115Call) -> Self {
            Self::Log115(value)
        }
    }
    impl ::core::convert::From<Log116Call> for HardhatConsoleCalls {
        fn from(value: Log116Call) -> Self {
            Self::Log116(value)
        }
    }
    impl ::core::convert::From<Log32Call> for HardhatConsoleCalls {
        fn from(value: Log32Call) -> Self {
            Self::Log32(value)
        }
    }
    impl ::core::convert::From<Log6Call> for HardhatConsoleCalls {
        fn from(value: Log6Call) -> Self {
            Self::Log6(value)
        }
    }
    impl ::core::convert::From<Log117Call> for HardhatConsoleCalls {
        fn from(value: Log117Call) -> Self {
            Self::Log117(value)
        }
    }
    impl ::core::convert::From<Log118Call> for HardhatConsoleCalls {
        fn from(value: Log118Call) -> Self {
            Self::Log118(value)
        }
    }
    impl ::core::convert::From<Log119Call> for HardhatConsoleCalls {
        fn from(value: Log119Call) -> Self {
            Self::Log119(value)
        }
    }
    impl ::core::convert::From<Log120Call> for HardhatConsoleCalls {
        fn from(value: Log120Call) -> Self {
            Self::Log120(value)
        }
    }
    impl ::core::convert::From<Log33Call> for HardhatConsoleCalls {
        fn from(value: Log33Call) -> Self {
            Self::Log33(value)
        }
    }
    impl ::core::convert::From<Log121Call> for HardhatConsoleCalls {
        fn from(value: Log121Call) -> Self {
            Self::Log121(value)
        }
    }
    impl ::core::convert::From<Log34Call> for HardhatConsoleCalls {
        fn from(value: Log34Call) -> Self {
            Self::Log34(value)
        }
    }
    impl ::core::convert::From<Log122Call> for HardhatConsoleCalls {
        fn from(value: Log122Call) -> Self {
            Self::Log122(value)
        }
    }
    impl ::core::convert::From<Log35Call> for HardhatConsoleCalls {
        fn from(value: Log35Call) -> Self {
            Self::Log35(value)
        }
    }
    impl ::core::convert::From<Log123Call> for HardhatConsoleCalls {
        fn from(value: Log123Call) -> Self {
            Self::Log123(value)
        }
    }
    impl ::core::convert::From<Log124Call> for HardhatConsoleCalls {
        fn from(value: Log124Call) -> Self {
            Self::Log124(value)
        }
    }
    impl ::core::convert::From<Log125Call> for HardhatConsoleCalls {
        fn from(value: Log125Call) -> Self {
            Self::Log125(value)
        }
    }
    impl ::core::convert::From<Log126Call> for HardhatConsoleCalls {
        fn from(value: Log126Call) -> Self {
            Self::Log126(value)
        }
    }
    impl ::core::convert::From<Log127Call> for HardhatConsoleCalls {
        fn from(value: Log127Call) -> Self {
            Self::Log127(value)
        }
    }
    impl ::core::convert::From<Log128Call> for HardhatConsoleCalls {
        fn from(value: Log128Call) -> Self {
            Self::Log128(value)
        }
    }
    impl ::core::convert::From<Log129Call> for HardhatConsoleCalls {
        fn from(value: Log129Call) -> Self {
            Self::Log129(value)
        }
    }
    impl ::core::convert::From<Log36Call> for HardhatConsoleCalls {
        fn from(value: Log36Call) -> Self {
            Self::Log36(value)
        }
    }
    impl ::core::convert::From<Log130Call> for HardhatConsoleCalls {
        fn from(value: Log130Call) -> Self {
            Self::Log130(value)
        }
    }
    impl ::core::convert::From<Log131Call> for HardhatConsoleCalls {
        fn from(value: Log131Call) -> Self {
            Self::Log131(value)
        }
    }
    impl ::core::convert::From<Log132Call> for HardhatConsoleCalls {
        fn from(value: Log132Call) -> Self {
            Self::Log132(value)
        }
    }
    impl ::core::convert::From<Log7Call> for HardhatConsoleCalls {
        fn from(value: Log7Call) -> Self {
            Self::Log7(value)
        }
    }
    impl ::core::convert::From<Log133Call> for HardhatConsoleCalls {
        fn from(value: Log133Call) -> Self {
            Self::Log133(value)
        }
    }
    impl ::core::convert::From<Log134Call> for HardhatConsoleCalls {
        fn from(value: Log134Call) -> Self {
            Self::Log134(value)
        }
    }
    impl ::core::convert::From<Log135Call> for HardhatConsoleCalls {
        fn from(value: Log135Call) -> Self {
            Self::Log135(value)
        }
    }
    impl ::core::convert::From<Log136Call> for HardhatConsoleCalls {
        fn from(value: Log136Call) -> Self {
            Self::Log136(value)
        }
    }
    impl ::core::convert::From<Log1Call> for HardhatConsoleCalls {
        fn from(value: Log1Call) -> Self {
            Self::Log1(value)
        }
    }
    impl ::core::convert::From<Log137Call> for HardhatConsoleCalls {
        fn from(value: Log137Call) -> Self {
            Self::Log137(value)
        }
    }
    impl ::core::convert::From<Log37Call> for HardhatConsoleCalls {
        fn from(value: Log37Call) -> Self {
            Self::Log37(value)
        }
    }
    impl ::core::convert::From<Log138Call> for HardhatConsoleCalls {
        fn from(value: Log138Call) -> Self {
            Self::Log138(value)
        }
    }
    impl ::core::convert::From<Log139Call> for HardhatConsoleCalls {
        fn from(value: Log139Call) -> Self {
            Self::Log139(value)
        }
    }
    impl ::core::convert::From<Log8Call> for HardhatConsoleCalls {
        fn from(value: Log8Call) -> Self {
            Self::Log8(value)
        }
    }
    impl ::core::convert::From<Log2Call> for HardhatConsoleCalls {
        fn from(value: Log2Call) -> Self {
            Self::Log2(value)
        }
    }
    impl ::core::convert::From<Log140Call> for HardhatConsoleCalls {
        fn from(value: Log140Call) -> Self {
            Self::Log140(value)
        }
    }
    impl ::core::convert::From<Log141Call> for HardhatConsoleCalls {
        fn from(value: Log141Call) -> Self {
            Self::Log141(value)
        }
    }
    impl ::core::convert::From<Log38Call> for HardhatConsoleCalls {
        fn from(value: Log38Call) -> Self {
            Self::Log38(value)
        }
    }
    impl ::core::convert::From<Log142Call> for HardhatConsoleCalls {
        fn from(value: Log142Call) -> Self {
            Self::Log142(value)
        }
    }
    impl ::core::convert::From<Log143Call> for HardhatConsoleCalls {
        fn from(value: Log143Call) -> Self {
            Self::Log143(value)
        }
    }
    impl ::core::convert::From<Log39Call> for HardhatConsoleCalls {
        fn from(value: Log39Call) -> Self {
            Self::Log39(value)
        }
    }
    impl ::core::convert::From<Log144Call> for HardhatConsoleCalls {
        fn from(value: Log144Call) -> Self {
            Self::Log144(value)
        }
    }
    impl ::core::convert::From<Log40Call> for HardhatConsoleCalls {
        fn from(value: Log40Call) -> Self {
            Self::Log40(value)
        }
    }
    impl ::core::convert::From<Log145Call> for HardhatConsoleCalls {
        fn from(value: Log145Call) -> Self {
            Self::Log145(value)
        }
    }
    impl ::core::convert::From<Log146Call> for HardhatConsoleCalls {
        fn from(value: Log146Call) -> Self {
            Self::Log146(value)
        }
    }
    impl ::core::convert::From<Log9Call> for HardhatConsoleCalls {
        fn from(value: Log9Call) -> Self {
            Self::Log9(value)
        }
    }
    impl ::core::convert::From<Log147Call> for HardhatConsoleCalls {
        fn from(value: Log147Call) -> Self {
            Self::Log147(value)
        }
    }
    impl ::core::convert::From<Log148Call> for HardhatConsoleCalls {
        fn from(value: Log148Call) -> Self {
            Self::Log148(value)
        }
    }
    impl ::core::convert::From<Log149Call> for HardhatConsoleCalls {
        fn from(value: Log149Call) -> Self {
            Self::Log149(value)
        }
    }
    impl ::core::convert::From<Log150Call> for HardhatConsoleCalls {
        fn from(value: Log150Call) -> Self {
            Self::Log150(value)
        }
    }
    impl ::core::convert::From<Log151Call> for HardhatConsoleCalls {
        fn from(value: Log151Call) -> Self {
            Self::Log151(value)
        }
    }
    impl ::core::convert::From<Log152Call> for HardhatConsoleCalls {
        fn from(value: Log152Call) -> Self {
            Self::Log152(value)
        }
    }
    impl ::core::convert::From<Log153Call> for HardhatConsoleCalls {
        fn from(value: Log153Call) -> Self {
            Self::Log153(value)
        }
    }
    impl ::core::convert::From<Log3Call> for HardhatConsoleCalls {
        fn from(value: Log3Call) -> Self {
            Self::Log3(value)
        }
    }
    impl ::core::convert::From<Log154Call> for HardhatConsoleCalls {
        fn from(value: Log154Call) -> Self {
            Self::Log154(value)
        }
    }
    impl ::core::convert::From<Log155Call> for HardhatConsoleCalls {
        fn from(value: Log155Call) -> Self {
            Self::Log155(value)
        }
    }
    impl ::core::convert::From<Log156Call> for HardhatConsoleCalls {
        fn from(value: Log156Call) -> Self {
            Self::Log156(value)
        }
    }
    impl ::core::convert::From<Log157Call> for HardhatConsoleCalls {
        fn from(value: Log157Call) -> Self {
            Self::Log157(value)
        }
    }
    impl ::core::convert::From<Log158Call> for HardhatConsoleCalls {
        fn from(value: Log158Call) -> Self {
            Self::Log158(value)
        }
    }
    impl ::core::convert::From<Log159Call> for HardhatConsoleCalls {
        fn from(value: Log159Call) -> Self {
            Self::Log159(value)
        }
    }
    impl ::core::convert::From<Log160Call> for HardhatConsoleCalls {
        fn from(value: Log160Call) -> Self {
            Self::Log160(value)
        }
    }
    impl ::core::convert::From<Log161Call> for HardhatConsoleCalls {
        fn from(value: Log161Call) -> Self {
            Self::Log161(value)
        }
    }
    impl ::core::convert::From<Log41Call> for HardhatConsoleCalls {
        fn from(value: Log41Call) -> Self {
            Self::Log41(value)
        }
    }
    impl ::core::convert::From<Log162Call> for HardhatConsoleCalls {
        fn from(value: Log162Call) -> Self {
            Self::Log162(value)
        }
    }
    impl ::core::convert::From<Log163Call> for HardhatConsoleCalls {
        fn from(value: Log163Call) -> Self {
            Self::Log163(value)
        }
    }
    impl ::core::convert::From<Log164Call> for HardhatConsoleCalls {
        fn from(value: Log164Call) -> Self {
            Self::Log164(value)
        }
    }
    impl ::core::convert::From<Log165Call> for HardhatConsoleCalls {
        fn from(value: Log165Call) -> Self {
            Self::Log165(value)
        }
    }
    impl ::core::convert::From<Log10Call> for HardhatConsoleCalls {
        fn from(value: Log10Call) -> Self {
            Self::Log10(value)
        }
    }
    impl ::core::convert::From<Log166Call> for HardhatConsoleCalls {
        fn from(value: Log166Call) -> Self {
            Self::Log166(value)
        }
    }
    impl ::core::convert::From<Log42Call> for HardhatConsoleCalls {
        fn from(value: Log42Call) -> Self {
            Self::Log42(value)
        }
    }
    impl ::core::convert::From<Log167Call> for HardhatConsoleCalls {
        fn from(value: Log167Call) -> Self {
            Self::Log167(value)
        }
    }
    impl ::core::convert::From<Log43Call> for HardhatConsoleCalls {
        fn from(value: Log43Call) -> Self {
            Self::Log43(value)
        }
    }
    impl ::core::convert::From<Log168Call> for HardhatConsoleCalls {
        fn from(value: Log168Call) -> Self {
            Self::Log168(value)
        }
    }
    impl ::core::convert::From<Log169Call> for HardhatConsoleCalls {
        fn from(value: Log169Call) -> Self {
            Self::Log169(value)
        }
    }
    impl ::core::convert::From<Log0Call> for HardhatConsoleCalls {
        fn from(value: Log0Call) -> Self {
            Self::Log0(value)
        }
    }
    impl ::core::convert::From<Log170Call> for HardhatConsoleCalls {
        fn from(value: Log170Call) -> Self {
            Self::Log170(value)
        }
    }
    impl ::core::convert::From<Log171Call> for HardhatConsoleCalls {
        fn from(value: Log171Call) -> Self {
            Self::Log171(value)
        }
    }
    impl ::core::convert::From<Log172Call> for HardhatConsoleCalls {
        fn from(value: Log172Call) -> Self {
            Self::Log172(value)
        }
    }
    impl ::core::convert::From<Log173Call> for HardhatConsoleCalls {
        fn from(value: Log173Call) -> Self {
            Self::Log173(value)
        }
    }
    impl ::core::convert::From<Log44Call> for HardhatConsoleCalls {
        fn from(value: Log44Call) -> Self {
            Self::Log44(value)
        }
    }
    impl ::core::convert::From<Log45Call> for HardhatConsoleCalls {
        fn from(value: Log45Call) -> Self {
            Self::Log45(value)
        }
    }
    impl ::core::convert::From<Log174Call> for HardhatConsoleCalls {
        fn from(value: Log174Call) -> Self {
            Self::Log174(value)
        }
    }
    impl ::core::convert::From<Log175Call> for HardhatConsoleCalls {
        fn from(value: Log175Call) -> Self {
            Self::Log175(value)
        }
    }
    impl ::core::convert::From<Log46Call> for HardhatConsoleCalls {
        fn from(value: Log46Call) -> Self {
            Self::Log46(value)
        }
    }
    impl ::core::convert::From<Log176Call> for HardhatConsoleCalls {
        fn from(value: Log176Call) -> Self {
            Self::Log176(value)
        }
    }
    impl ::core::convert::From<Log177Call> for HardhatConsoleCalls {
        fn from(value: Log177Call) -> Self {
            Self::Log177(value)
        }
    }
    impl ::core::convert::From<Log178Call> for HardhatConsoleCalls {
        fn from(value: Log178Call) -> Self {
            Self::Log178(value)
        }
    }
    impl ::core::convert::From<Log47Call> for HardhatConsoleCalls {
        fn from(value: Log47Call) -> Self {
            Self::Log47(value)
        }
    }
    impl ::core::convert::From<Log179Call> for HardhatConsoleCalls {
        fn from(value: Log179Call) -> Self {
            Self::Log179(value)
        }
    }
    impl ::core::convert::From<Log180Call> for HardhatConsoleCalls {
        fn from(value: Log180Call) -> Self {
            Self::Log180(value)
        }
    }
    impl ::core::convert::From<Log181Call> for HardhatConsoleCalls {
        fn from(value: Log181Call) -> Self {
            Self::Log181(value)
        }
    }
    impl ::core::convert::From<Log182Call> for HardhatConsoleCalls {
        fn from(value: Log182Call) -> Self {
            Self::Log182(value)
        }
    }
    impl ::core::convert::From<Log183Call> for HardhatConsoleCalls {
        fn from(value: Log183Call) -> Self {
            Self::Log183(value)
        }
    }
    impl ::core::convert::From<Log184Call> for HardhatConsoleCalls {
        fn from(value: Log184Call) -> Self {
            Self::Log184(value)
        }
    }
    impl ::core::convert::From<Log185Call> for HardhatConsoleCalls {
        fn from(value: Log185Call) -> Self {
            Self::Log185(value)
        }
    }
    impl ::core::convert::From<Log186Call> for HardhatConsoleCalls {
        fn from(value: Log186Call) -> Self {
            Self::Log186(value)
        }
    }
    impl ::core::convert::From<Log187Call> for HardhatConsoleCalls {
        fn from(value: Log187Call) -> Self {
            Self::Log187(value)
        }
    }
    impl ::core::convert::From<Log188Call> for HardhatConsoleCalls {
        fn from(value: Log188Call) -> Self {
            Self::Log188(value)
        }
    }
    impl ::core::convert::From<Log48Call> for HardhatConsoleCalls {
        fn from(value: Log48Call) -> Self {
            Self::Log48(value)
        }
    }
    impl ::core::convert::From<Log189Call> for HardhatConsoleCalls {
        fn from(value: Log189Call) -> Self {
            Self::Log189(value)
        }
    }
    impl ::core::convert::From<Log190Call> for HardhatConsoleCalls {
        fn from(value: Log190Call) -> Self {
            Self::Log190(value)
        }
    }
    impl ::core::convert::From<Log191Call> for HardhatConsoleCalls {
        fn from(value: Log191Call) -> Self {
            Self::Log191(value)
        }
    }
    impl ::core::convert::From<Log49Call> for HardhatConsoleCalls {
        fn from(value: Log49Call) -> Self {
            Self::Log49(value)
        }
    }
    impl ::core::convert::From<Log192Call> for HardhatConsoleCalls {
        fn from(value: Log192Call) -> Self {
            Self::Log192(value)
        }
    }
    impl ::core::convert::From<Log11Call> for HardhatConsoleCalls {
        fn from(value: Log11Call) -> Self {
            Self::Log11(value)
        }
    }
    impl ::core::convert::From<Log193Call> for HardhatConsoleCalls {
        fn from(value: Log193Call) -> Self {
            Self::Log193(value)
        }
    }
    impl ::core::convert::From<Log194Call> for HardhatConsoleCalls {
        fn from(value: Log194Call) -> Self {
            Self::Log194(value)
        }
    }
    impl ::core::convert::From<Log195Call> for HardhatConsoleCalls {
        fn from(value: Log195Call) -> Self {
            Self::Log195(value)
        }
    }
    impl ::core::convert::From<Log196Call> for HardhatConsoleCalls {
        fn from(value: Log196Call) -> Self {
            Self::Log196(value)
        }
    }
    impl ::core::convert::From<Log50Call> for HardhatConsoleCalls {
        fn from(value: Log50Call) -> Self {
            Self::Log50(value)
        }
    }
    impl ::core::convert::From<Log51Call> for HardhatConsoleCalls {
        fn from(value: Log51Call) -> Self {
            Self::Log51(value)
        }
    }
    impl ::core::convert::From<Log197Call> for HardhatConsoleCalls {
        fn from(value: Log197Call) -> Self {
            Self::Log197(value)
        }
    }
    impl ::core::convert::From<Log198Call> for HardhatConsoleCalls {
        fn from(value: Log198Call) -> Self {
            Self::Log198(value)
        }
    }
    impl ::core::convert::From<Log12Call> for HardhatConsoleCalls {
        fn from(value: Log12Call) -> Self {
            Self::Log12(value)
        }
    }
    impl ::core::convert::From<Log199Call> for HardhatConsoleCalls {
        fn from(value: Log199Call) -> Self {
            Self::Log199(value)
        }
    }
    impl ::core::convert::From<Log200Call> for HardhatConsoleCalls {
        fn from(value: Log200Call) -> Self {
            Self::Log200(value)
        }
    }
    impl ::core::convert::From<Log201Call> for HardhatConsoleCalls {
        fn from(value: Log201Call) -> Self {
            Self::Log201(value)
        }
    }
    impl ::core::convert::From<Log202Call> for HardhatConsoleCalls {
        fn from(value: Log202Call) -> Self {
            Self::Log202(value)
        }
    }
    impl ::core::convert::From<Log203Call> for HardhatConsoleCalls {
        fn from(value: Log203Call) -> Self {
            Self::Log203(value)
        }
    }
    impl ::core::convert::From<Log204Call> for HardhatConsoleCalls {
        fn from(value: Log204Call) -> Self {
            Self::Log204(value)
        }
    }
    impl ::core::convert::From<Log205Call> for HardhatConsoleCalls {
        fn from(value: Log205Call) -> Self {
            Self::Log205(value)
        }
    }
    impl ::core::convert::From<Log206Call> for HardhatConsoleCalls {
        fn from(value: Log206Call) -> Self {
            Self::Log206(value)
        }
    }
    impl ::core::convert::From<Log207Call> for HardhatConsoleCalls {
        fn from(value: Log207Call) -> Self {
            Self::Log207(value)
        }
    }
    impl ::core::convert::From<Log208Call> for HardhatConsoleCalls {
        fn from(value: Log208Call) -> Self {
            Self::Log208(value)
        }
    }
    impl ::core::convert::From<Log209Call> for HardhatConsoleCalls {
        fn from(value: Log209Call) -> Self {
            Self::Log209(value)
        }
    }
    impl ::core::convert::From<Log210Call> for HardhatConsoleCalls {
        fn from(value: Log210Call) -> Self {
            Self::Log210(value)
        }
    }
    impl ::core::convert::From<Log52Call> for HardhatConsoleCalls {
        fn from(value: Log52Call) -> Self {
            Self::Log52(value)
        }
    }
    impl ::core::convert::From<Log211Call> for HardhatConsoleCalls {
        fn from(value: Log211Call) -> Self {
            Self::Log211(value)
        }
    }
    impl ::core::convert::From<Log212Call> for HardhatConsoleCalls {
        fn from(value: Log212Call) -> Self {
            Self::Log212(value)
        }
    }
    impl ::core::convert::From<Log213Call> for HardhatConsoleCalls {
        fn from(value: Log213Call) -> Self {
            Self::Log213(value)
        }
    }
    impl ::core::convert::From<Log13Call> for HardhatConsoleCalls {
        fn from(value: Log13Call) -> Self {
            Self::Log13(value)
        }
    }
    impl ::core::convert::From<Log14Call> for HardhatConsoleCalls {
        fn from(value: Log14Call) -> Self {
            Self::Log14(value)
        }
    }
    impl ::core::convert::From<Log214Call> for HardhatConsoleCalls {
        fn from(value: Log214Call) -> Self {
            Self::Log214(value)
        }
    }
    impl ::core::convert::From<Log215Call> for HardhatConsoleCalls {
        fn from(value: Log215Call) -> Self {
            Self::Log215(value)
        }
    }
    impl ::core::convert::From<Log216Call> for HardhatConsoleCalls {
        fn from(value: Log216Call) -> Self {
            Self::Log216(value)
        }
    }
    impl ::core::convert::From<Log53Call> for HardhatConsoleCalls {
        fn from(value: Log53Call) -> Self {
            Self::Log53(value)
        }
    }
    impl ::core::convert::From<Log54Call> for HardhatConsoleCalls {
        fn from(value: Log54Call) -> Self {
            Self::Log54(value)
        }
    }
    impl ::core::convert::From<Log217Call> for HardhatConsoleCalls {
        fn from(value: Log217Call) -> Self {
            Self::Log217(value)
        }
    }
    impl ::core::convert::From<Log218Call> for HardhatConsoleCalls {
        fn from(value: Log218Call) -> Self {
            Self::Log218(value)
        }
    }
    impl ::core::convert::From<Log219Call> for HardhatConsoleCalls {
        fn from(value: Log219Call) -> Self {
            Self::Log219(value)
        }
    }
    impl ::core::convert::From<Log220Call> for HardhatConsoleCalls {
        fn from(value: Log220Call) -> Self {
            Self::Log220(value)
        }
    }
    impl ::core::convert::From<Log221Call> for HardhatConsoleCalls {
        fn from(value: Log221Call) -> Self {
            Self::Log221(value)
        }
    }
    impl ::core::convert::From<Log222Call> for HardhatConsoleCalls {
        fn from(value: Log222Call) -> Self {
            Self::Log222(value)
        }
    }
    impl ::core::convert::From<Log223Call> for HardhatConsoleCalls {
        fn from(value: Log223Call) -> Self {
            Self::Log223(value)
        }
    }
    impl ::core::convert::From<Log224Call> for HardhatConsoleCalls {
        fn from(value: Log224Call) -> Self {
            Self::Log224(value)
        }
    }
    impl ::core::convert::From<Log225Call> for HardhatConsoleCalls {
        fn from(value: Log225Call) -> Self {
            Self::Log225(value)
        }
    }
    impl ::core::convert::From<Log226Call> for HardhatConsoleCalls {
        fn from(value: Log226Call) -> Self {
            Self::Log226(value)
        }
    }
    impl ::core::convert::From<Log227Call> for HardhatConsoleCalls {
        fn from(value: Log227Call) -> Self {
            Self::Log227(value)
        }
    }
    impl ::core::convert::From<Log15Call> for HardhatConsoleCalls {
        fn from(value: Log15Call) -> Self {
            Self::Log15(value)
        }
    }
    impl ::core::convert::From<Log55Call> for HardhatConsoleCalls {
        fn from(value: Log55Call) -> Self {
            Self::Log55(value)
        }
    }
    impl ::core::convert::From<Log16Call> for HardhatConsoleCalls {
        fn from(value: Log16Call) -> Self {
            Self::Log16(value)
        }
    }
    impl ::core::convert::From<Log228Call> for HardhatConsoleCalls {
        fn from(value: Log228Call) -> Self {
            Self::Log228(value)
        }
    }
    impl ::core::convert::From<Log56Call> for HardhatConsoleCalls {
        fn from(value: Log56Call) -> Self {
            Self::Log56(value)
        }
    }
    impl ::core::convert::From<Log229Call> for HardhatConsoleCalls {
        fn from(value: Log229Call) -> Self {
            Self::Log229(value)
        }
    }
    impl ::core::convert::From<Log230Call> for HardhatConsoleCalls {
        fn from(value: Log230Call) -> Self {
            Self::Log230(value)
        }
    }
    impl ::core::convert::From<Log231Call> for HardhatConsoleCalls {
        fn from(value: Log231Call) -> Self {
            Self::Log231(value)
        }
    }
    impl ::core::convert::From<Log232Call> for HardhatConsoleCalls {
        fn from(value: Log232Call) -> Self {
            Self::Log232(value)
        }
    }
    impl ::core::convert::From<Log233Call> for HardhatConsoleCalls {
        fn from(value: Log233Call) -> Self {
            Self::Log233(value)
        }
    }
    impl ::core::convert::From<Log234Call> for HardhatConsoleCalls {
        fn from(value: Log234Call) -> Self {
            Self::Log234(value)
        }
    }
    impl ::core::convert::From<Log235Call> for HardhatConsoleCalls {
        fn from(value: Log235Call) -> Self {
            Self::Log235(value)
        }
    }
    impl ::core::convert::From<Log236Call> for HardhatConsoleCalls {
        fn from(value: Log236Call) -> Self {
            Self::Log236(value)
        }
    }
    impl ::core::convert::From<Log237Call> for HardhatConsoleCalls {
        fn from(value: Log237Call) -> Self {
            Self::Log237(value)
        }
    }
    impl ::core::convert::From<Log238Call> for HardhatConsoleCalls {
        fn from(value: Log238Call) -> Self {
            Self::Log238(value)
        }
    }
    impl ::core::convert::From<Log239Call> for HardhatConsoleCalls {
        fn from(value: Log239Call) -> Self {
            Self::Log239(value)
        }
    }
    impl ::core::convert::From<Log240Call> for HardhatConsoleCalls {
        fn from(value: Log240Call) -> Self {
            Self::Log240(value)
        }
    }
    impl ::core::convert::From<Log241Call> for HardhatConsoleCalls {
        fn from(value: Log241Call) -> Self {
            Self::Log241(value)
        }
    }
    impl ::core::convert::From<Log17Call> for HardhatConsoleCalls {
        fn from(value: Log17Call) -> Self {
            Self::Log17(value)
        }
    }
    impl ::core::convert::From<Log242Call> for HardhatConsoleCalls {
        fn from(value: Log242Call) -> Self {
            Self::Log242(value)
        }
    }
    impl ::core::convert::From<Log243Call> for HardhatConsoleCalls {
        fn from(value: Log243Call) -> Self {
            Self::Log243(value)
        }
    }
    impl ::core::convert::From<Log244Call> for HardhatConsoleCalls {
        fn from(value: Log244Call) -> Self {
            Self::Log244(value)
        }
    }
    impl ::core::convert::From<Log245Call> for HardhatConsoleCalls {
        fn from(value: Log245Call) -> Self {
            Self::Log245(value)
        }
    }
    impl ::core::convert::From<Log246Call> for HardhatConsoleCalls {
        fn from(value: Log246Call) -> Self {
            Self::Log246(value)
        }
    }
    impl ::core::convert::From<Log57Call> for HardhatConsoleCalls {
        fn from(value: Log57Call) -> Self {
            Self::Log57(value)
        }
    }
    impl ::core::convert::From<Log247Call> for HardhatConsoleCalls {
        fn from(value: Log247Call) -> Self {
            Self::Log247(value)
        }
    }
    impl ::core::convert::From<Log248Call> for HardhatConsoleCalls {
        fn from(value: Log248Call) -> Self {
            Self::Log248(value)
        }
    }
    impl ::core::convert::From<Log249Call> for HardhatConsoleCalls {
        fn from(value: Log249Call) -> Self {
            Self::Log249(value)
        }
    }
    impl ::core::convert::From<Log58Call> for HardhatConsoleCalls {
        fn from(value: Log58Call) -> Self {
            Self::Log58(value)
        }
    }
    impl ::core::convert::From<Log59Call> for HardhatConsoleCalls {
        fn from(value: Log59Call) -> Self {
            Self::Log59(value)
        }
    }
    impl ::core::convert::From<Log250Call> for HardhatConsoleCalls {
        fn from(value: Log250Call) -> Self {
            Self::Log250(value)
        }
    }
    impl ::core::convert::From<Log251Call> for HardhatConsoleCalls {
        fn from(value: Log251Call) -> Self {
            Self::Log251(value)
        }
    }
    impl ::core::convert::From<Log252Call> for HardhatConsoleCalls {
        fn from(value: Log252Call) -> Self {
            Self::Log252(value)
        }
    }
    impl ::core::convert::From<Log253Call> for HardhatConsoleCalls {
        fn from(value: Log253Call) -> Self {
            Self::Log253(value)
        }
    }
    impl ::core::convert::From<Log60Call> for HardhatConsoleCalls {
        fn from(value: Log60Call) -> Self {
            Self::Log60(value)
        }
    }
    impl ::core::convert::From<Log254Call> for HardhatConsoleCalls {
        fn from(value: Log254Call) -> Self {
            Self::Log254(value)
        }
    }
    impl ::core::convert::From<Log61Call> for HardhatConsoleCalls {
        fn from(value: Log61Call) -> Self {
            Self::Log61(value)
        }
    }
    impl ::core::convert::From<Log255Call> for HardhatConsoleCalls {
        fn from(value: Log255Call) -> Self {
            Self::Log255(value)
        }
    }
    impl ::core::convert::From<Log256Call> for HardhatConsoleCalls {
        fn from(value: Log256Call) -> Self {
            Self::Log256(value)
        }
    }
    impl ::core::convert::From<Log257Call> for HardhatConsoleCalls {
        fn from(value: Log257Call) -> Self {
            Self::Log257(value)
        }
    }
    impl ::core::convert::From<Log258Call> for HardhatConsoleCalls {
        fn from(value: Log258Call) -> Self {
            Self::Log258(value)
        }
    }
    impl ::core::convert::From<Log259Call> for HardhatConsoleCalls {
        fn from(value: Log259Call) -> Self {
            Self::Log259(value)
        }
    }
    impl ::core::convert::From<Log260Call> for HardhatConsoleCalls {
        fn from(value: Log260Call) -> Self {
            Self::Log260(value)
        }
    }
    impl ::core::convert::From<Log261Call> for HardhatConsoleCalls {
        fn from(value: Log261Call) -> Self {
            Self::Log261(value)
        }
    }
    impl ::core::convert::From<Log262Call> for HardhatConsoleCalls {
        fn from(value: Log262Call) -> Self {
            Self::Log262(value)
        }
    }
    impl ::core::convert::From<Log62Call> for HardhatConsoleCalls {
        fn from(value: Log62Call) -> Self {
            Self::Log62(value)
        }
    }
    impl ::core::convert::From<Log263Call> for HardhatConsoleCalls {
        fn from(value: Log263Call) -> Self {
            Self::Log263(value)
        }
    }
    impl ::core::convert::From<Log264Call> for HardhatConsoleCalls {
        fn from(value: Log264Call) -> Self {
            Self::Log264(value)
        }
    }
    impl ::core::convert::From<Log265Call> for HardhatConsoleCalls {
        fn from(value: Log265Call) -> Self {
            Self::Log265(value)
        }
    }
    impl ::core::convert::From<Log266Call> for HardhatConsoleCalls {
        fn from(value: Log266Call) -> Self {
            Self::Log266(value)
        }
    }
    impl ::core::convert::From<Log267Call> for HardhatConsoleCalls {
        fn from(value: Log267Call) -> Self {
            Self::Log267(value)
        }
    }
    impl ::core::convert::From<Log268Call> for HardhatConsoleCalls {
        fn from(value: Log268Call) -> Self {
            Self::Log268(value)
        }
    }
    impl ::core::convert::From<Log269Call> for HardhatConsoleCalls {
        fn from(value: Log269Call) -> Self {
            Self::Log269(value)
        }
    }
    impl ::core::convert::From<Log270Call> for HardhatConsoleCalls {
        fn from(value: Log270Call) -> Self {
            Self::Log270(value)
        }
    }
    impl ::core::convert::From<Log271Call> for HardhatConsoleCalls {
        fn from(value: Log271Call) -> Self {
            Self::Log271(value)
        }
    }
    impl ::core::convert::From<Log272Call> for HardhatConsoleCalls {
        fn from(value: Log272Call) -> Self {
            Self::Log272(value)
        }
    }
    impl ::core::convert::From<Log273Call> for HardhatConsoleCalls {
        fn from(value: Log273Call) -> Self {
            Self::Log273(value)
        }
    }
    impl ::core::convert::From<Log274Call> for HardhatConsoleCalls {
        fn from(value: Log274Call) -> Self {
            Self::Log274(value)
        }
    }
    impl ::core::convert::From<Log275Call> for HardhatConsoleCalls {
        fn from(value: Log275Call) -> Self {
            Self::Log275(value)
        }
    }
    impl ::core::convert::From<Log276Call> for HardhatConsoleCalls {
        fn from(value: Log276Call) -> Self {
            Self::Log276(value)
        }
    }
    impl ::core::convert::From<Log277Call> for HardhatConsoleCalls {
        fn from(value: Log277Call) -> Self {
            Self::Log277(value)
        }
    }
    impl ::core::convert::From<Log63Call> for HardhatConsoleCalls {
        fn from(value: Log63Call) -> Self {
            Self::Log63(value)
        }
    }
    impl ::core::convert::From<Log64Call> for HardhatConsoleCalls {
        fn from(value: Log64Call) -> Self {
            Self::Log64(value)
        }
    }
    impl ::core::convert::From<Log65Call> for HardhatConsoleCalls {
        fn from(value: Log65Call) -> Self {
            Self::Log65(value)
        }
    }
    impl ::core::convert::From<Log278Call> for HardhatConsoleCalls {
        fn from(value: Log278Call) -> Self {
            Self::Log278(value)
        }
    }
    impl ::core::convert::From<Log279Call> for HardhatConsoleCalls {
        fn from(value: Log279Call) -> Self {
            Self::Log279(value)
        }
    }
    impl ::core::convert::From<Log280Call> for HardhatConsoleCalls {
        fn from(value: Log280Call) -> Self {
            Self::Log280(value)
        }
    }
    impl ::core::convert::From<Log18Call> for HardhatConsoleCalls {
        fn from(value: Log18Call) -> Self {
            Self::Log18(value)
        }
    }
    impl ::core::convert::From<Log66Call> for HardhatConsoleCalls {
        fn from(value: Log66Call) -> Self {
            Self::Log66(value)
        }
    }
    impl ::core::convert::From<Log281Call> for HardhatConsoleCalls {
        fn from(value: Log281Call) -> Self {
            Self::Log281(value)
        }
    }
    impl ::core::convert::From<Log282Call> for HardhatConsoleCalls {
        fn from(value: Log282Call) -> Self {
            Self::Log282(value)
        }
    }
    impl ::core::convert::From<Log283Call> for HardhatConsoleCalls {
        fn from(value: Log283Call) -> Self {
            Self::Log283(value)
        }
    }
    impl ::core::convert::From<Log284Call> for HardhatConsoleCalls {
        fn from(value: Log284Call) -> Self {
            Self::Log284(value)
        }
    }
    impl ::core::convert::From<Log285Call> for HardhatConsoleCalls {
        fn from(value: Log285Call) -> Self {
            Self::Log285(value)
        }
    }
    impl ::core::convert::From<Log67Call> for HardhatConsoleCalls {
        fn from(value: Log67Call) -> Self {
            Self::Log67(value)
        }
    }
    impl ::core::convert::From<Log286Call> for HardhatConsoleCalls {
        fn from(value: Log286Call) -> Self {
            Self::Log286(value)
        }
    }
    impl ::core::convert::From<Log287Call> for HardhatConsoleCalls {
        fn from(value: Log287Call) -> Self {
            Self::Log287(value)
        }
    }
    impl ::core::convert::From<Log288Call> for HardhatConsoleCalls {
        fn from(value: Log288Call) -> Self {
            Self::Log288(value)
        }
    }
    impl ::core::convert::From<Log289Call> for HardhatConsoleCalls {
        fn from(value: Log289Call) -> Self {
            Self::Log289(value)
        }
    }
    impl ::core::convert::From<Log290Call> for HardhatConsoleCalls {
        fn from(value: Log290Call) -> Self {
            Self::Log290(value)
        }
    }
    impl ::core::convert::From<Log291Call> for HardhatConsoleCalls {
        fn from(value: Log291Call) -> Self {
            Self::Log291(value)
        }
    }
    impl ::core::convert::From<Log292Call> for HardhatConsoleCalls {
        fn from(value: Log292Call) -> Self {
            Self::Log292(value)
        }
    }
    impl ::core::convert::From<Log19Call> for HardhatConsoleCalls {
        fn from(value: Log19Call) -> Self {
            Self::Log19(value)
        }
    }
    impl ::core::convert::From<Log68Call> for HardhatConsoleCalls {
        fn from(value: Log68Call) -> Self {
            Self::Log68(value)
        }
    }
    impl ::core::convert::From<Log293Call> for HardhatConsoleCalls {
        fn from(value: Log293Call) -> Self {
            Self::Log293(value)
        }
    }
    impl ::core::convert::From<Log294Call> for HardhatConsoleCalls {
        fn from(value: Log294Call) -> Self {
            Self::Log294(value)
        }
    }
    impl ::core::convert::From<Log295Call> for HardhatConsoleCalls {
        fn from(value: Log295Call) -> Self {
            Self::Log295(value)
        }
    }
    impl ::core::convert::From<Log296Call> for HardhatConsoleCalls {
        fn from(value: Log296Call) -> Self {
            Self::Log296(value)
        }
    }
    impl ::core::convert::From<Log297Call> for HardhatConsoleCalls {
        fn from(value: Log297Call) -> Self {
            Self::Log297(value)
        }
    }
    impl ::core::convert::From<Log69Call> for HardhatConsoleCalls {
        fn from(value: Log69Call) -> Self {
            Self::Log69(value)
        }
    }
    impl ::core::convert::From<Log70Call> for HardhatConsoleCalls {
        fn from(value: Log70Call) -> Self {
            Self::Log70(value)
        }
    }
    impl ::core::convert::From<Log71Call> for HardhatConsoleCalls {
        fn from(value: Log71Call) -> Self {
            Self::Log71(value)
        }
    }
    impl ::core::convert::From<Log72Call> for HardhatConsoleCalls {
        fn from(value: Log72Call) -> Self {
            Self::Log72(value)
        }
    }
    impl ::core::convert::From<Log298Call> for HardhatConsoleCalls {
        fn from(value: Log298Call) -> Self {
            Self::Log298(value)
        }
    }
    impl ::core::convert::From<Log299Call> for HardhatConsoleCalls {
        fn from(value: Log299Call) -> Self {
            Self::Log299(value)
        }
    }
    impl ::core::convert::From<Log300Call> for HardhatConsoleCalls {
        fn from(value: Log300Call) -> Self {
            Self::Log300(value)
        }
    }
    impl ::core::convert::From<Log301Call> for HardhatConsoleCalls {
        fn from(value: Log301Call) -> Self {
            Self::Log301(value)
        }
    }
    impl ::core::convert::From<Log302Call> for HardhatConsoleCalls {
        fn from(value: Log302Call) -> Self {
            Self::Log302(value)
        }
    }
    impl ::core::convert::From<Log73Call> for HardhatConsoleCalls {
        fn from(value: Log73Call) -> Self {
            Self::Log73(value)
        }
    }
    impl ::core::convert::From<Log303Call> for HardhatConsoleCalls {
        fn from(value: Log303Call) -> Self {
            Self::Log303(value)
        }
    }
    impl ::core::convert::From<Log304Call> for HardhatConsoleCalls {
        fn from(value: Log304Call) -> Self {
            Self::Log304(value)
        }
    }
    impl ::core::convert::From<Log74Call> for HardhatConsoleCalls {
        fn from(value: Log74Call) -> Self {
            Self::Log74(value)
        }
    }
    impl ::core::convert::From<Log75Call> for HardhatConsoleCalls {
        fn from(value: Log75Call) -> Self {
            Self::Log75(value)
        }
    }
    impl ::core::convert::From<Log305Call> for HardhatConsoleCalls {
        fn from(value: Log305Call) -> Self {
            Self::Log305(value)
        }
    }
    impl ::core::convert::From<Log306Call> for HardhatConsoleCalls {
        fn from(value: Log306Call) -> Self {
            Self::Log306(value)
        }
    }
    impl ::core::convert::From<Log307Call> for HardhatConsoleCalls {
        fn from(value: Log307Call) -> Self {
            Self::Log307(value)
        }
    }
    impl ::core::convert::From<Log308Call> for HardhatConsoleCalls {
        fn from(value: Log308Call) -> Self {
            Self::Log308(value)
        }
    }
    impl ::core::convert::From<Log309Call> for HardhatConsoleCalls {
        fn from(value: Log309Call) -> Self {
            Self::Log309(value)
        }
    }
    impl ::core::convert::From<Log20Call> for HardhatConsoleCalls {
        fn from(value: Log20Call) -> Self {
            Self::Log20(value)
        }
    }
    impl ::core::convert::From<Log76Call> for HardhatConsoleCalls {
        fn from(value: Log76Call) -> Self {
            Self::Log76(value)
        }
    }
    impl ::core::convert::From<Log310Call> for HardhatConsoleCalls {
        fn from(value: Log310Call) -> Self {
            Self::Log310(value)
        }
    }
    impl ::core::convert::From<Log311Call> for HardhatConsoleCalls {
        fn from(value: Log311Call) -> Self {
            Self::Log311(value)
        }
    }
    impl ::core::convert::From<Log312Call> for HardhatConsoleCalls {
        fn from(value: Log312Call) -> Self {
            Self::Log312(value)
        }
    }
    impl ::core::convert::From<Log313Call> for HardhatConsoleCalls {
        fn from(value: Log313Call) -> Self {
            Self::Log313(value)
        }
    }
    impl ::core::convert::From<Log314Call> for HardhatConsoleCalls {
        fn from(value: Log314Call) -> Self {
            Self::Log314(value)
        }
    }
    impl ::core::convert::From<Log77Call> for HardhatConsoleCalls {
        fn from(value: Log77Call) -> Self {
            Self::Log77(value)
        }
    }
    impl ::core::convert::From<Log315Call> for HardhatConsoleCalls {
        fn from(value: Log315Call) -> Self {
            Self::Log315(value)
        }
    }
    impl ::core::convert::From<Log316Call> for HardhatConsoleCalls {
        fn from(value: Log316Call) -> Self {
            Self::Log316(value)
        }
    }
    impl ::core::convert::From<Log317Call> for HardhatConsoleCalls {
        fn from(value: Log317Call) -> Self {
            Self::Log317(value)
        }
    }
    impl ::core::convert::From<Log78Call> for HardhatConsoleCalls {
        fn from(value: Log78Call) -> Self {
            Self::Log78(value)
        }
    }
    impl ::core::convert::From<Log318Call> for HardhatConsoleCalls {
        fn from(value: Log318Call) -> Self {
            Self::Log318(value)
        }
    }
    impl ::core::convert::From<Log79Call> for HardhatConsoleCalls {
        fn from(value: Log79Call) -> Self {
            Self::Log79(value)
        }
    }
    impl ::core::convert::From<Log319Call> for HardhatConsoleCalls {
        fn from(value: Log319Call) -> Self {
            Self::Log319(value)
        }
    }
    impl ::core::convert::From<Log320Call> for HardhatConsoleCalls {
        fn from(value: Log320Call) -> Self {
            Self::Log320(value)
        }
    }
    impl ::core::convert::From<Log321Call> for HardhatConsoleCalls {
        fn from(value: Log321Call) -> Self {
            Self::Log321(value)
        }
    }
    impl ::core::convert::From<Log322Call> for HardhatConsoleCalls {
        fn from(value: Log322Call) -> Self {
            Self::Log322(value)
        }
    }
    impl ::core::convert::From<Log323Call> for HardhatConsoleCalls {
        fn from(value: Log323Call) -> Self {
            Self::Log323(value)
        }
    }
    impl ::core::convert::From<Log324Call> for HardhatConsoleCalls {
        fn from(value: Log324Call) -> Self {
            Self::Log324(value)
        }
    }
    impl ::core::convert::From<Log80Call> for HardhatConsoleCalls {
        fn from(value: Log80Call) -> Self {
            Self::Log80(value)
        }
    }
    impl ::core::convert::From<Log325Call> for HardhatConsoleCalls {
        fn from(value: Log325Call) -> Self {
            Self::Log325(value)
        }
    }
    impl ::core::convert::From<Log326Call> for HardhatConsoleCalls {
        fn from(value: Log326Call) -> Self {
            Self::Log326(value)
        }
    }
    impl ::core::convert::From<Log81Call> for HardhatConsoleCalls {
        fn from(value: Log81Call) -> Self {
            Self::Log81(value)
        }
    }
    impl ::core::convert::From<Log327Call> for HardhatConsoleCalls {
        fn from(value: Log327Call) -> Self {
            Self::Log327(value)
        }
    }
    impl ::core::convert::From<Log328Call> for HardhatConsoleCalls {
        fn from(value: Log328Call) -> Self {
            Self::Log328(value)
        }
    }
    impl ::core::convert::From<Log329Call> for HardhatConsoleCalls {
        fn from(value: Log329Call) -> Self {
            Self::Log329(value)
        }
    }
    impl ::core::convert::From<Log330Call> for HardhatConsoleCalls {
        fn from(value: Log330Call) -> Self {
            Self::Log330(value)
        }
    }
    impl ::core::convert::From<Log331Call> for HardhatConsoleCalls {
        fn from(value: Log331Call) -> Self {
            Self::Log331(value)
        }
    }
    impl ::core::convert::From<Log82Call> for HardhatConsoleCalls {
        fn from(value: Log82Call) -> Self {
            Self::Log82(value)
        }
    }
    impl ::core::convert::From<Log83Call> for HardhatConsoleCalls {
        fn from(value: Log83Call) -> Self {
            Self::Log83(value)
        }
    }
    impl ::core::convert::From<Log84Call> for HardhatConsoleCalls {
        fn from(value: Log84Call) -> Self {
            Self::Log84(value)
        }
    }
    impl ::core::convert::From<Log332Call> for HardhatConsoleCalls {
        fn from(value: Log332Call) -> Self {
            Self::Log332(value)
        }
    }
    impl ::core::convert::From<Log333Call> for HardhatConsoleCalls {
        fn from(value: Log333Call) -> Self {
            Self::Log333(value)
        }
    }
    impl ::core::convert::From<Log334Call> for HardhatConsoleCalls {
        fn from(value: Log334Call) -> Self {
            Self::Log334(value)
        }
    }
    impl ::core::convert::From<Log21Call> for HardhatConsoleCalls {
        fn from(value: Log21Call) -> Self {
            Self::Log21(value)
        }
    }
    impl ::core::convert::From<Log335Call> for HardhatConsoleCalls {
        fn from(value: Log335Call) -> Self {
            Self::Log335(value)
        }
    }
    impl ::core::convert::From<Log336Call> for HardhatConsoleCalls {
        fn from(value: Log336Call) -> Self {
            Self::Log336(value)
        }
    }
    impl ::core::convert::From<Log4Call> for HardhatConsoleCalls {
        fn from(value: Log4Call) -> Self {
            Self::Log4(value)
        }
    }
    impl ::core::convert::From<Log337Call> for HardhatConsoleCalls {
        fn from(value: Log337Call) -> Self {
            Self::Log337(value)
        }
    }
    impl ::core::convert::From<Log338Call> for HardhatConsoleCalls {
        fn from(value: Log338Call) -> Self {
            Self::Log338(value)
        }
    }
    impl ::core::convert::From<Log339Call> for HardhatConsoleCalls {
        fn from(value: Log339Call) -> Self {
            Self::Log339(value)
        }
    }
    impl ::core::convert::From<Log85Call> for HardhatConsoleCalls {
        fn from(value: Log85Call) -> Self {
            Self::Log85(value)
        }
    }
    impl ::core::convert::From<Log340Call> for HardhatConsoleCalls {
        fn from(value: Log340Call) -> Self {
            Self::Log340(value)
        }
    }
    impl ::core::convert::From<Log86Call> for HardhatConsoleCalls {
        fn from(value: Log86Call) -> Self {
            Self::Log86(value)
        }
    }
    impl ::core::convert::From<Log341Call> for HardhatConsoleCalls {
        fn from(value: Log341Call) -> Self {
            Self::Log341(value)
        }
    }
    impl ::core::convert::From<Log342Call> for HardhatConsoleCalls {
        fn from(value: Log342Call) -> Self {
            Self::Log342(value)
        }
    }
    impl ::core::convert::From<Log5Call> for HardhatConsoleCalls {
        fn from(value: Log5Call) -> Self {
            Self::Log5(value)
        }
    }
    impl ::core::convert::From<Log22Call> for HardhatConsoleCalls {
        fn from(value: Log22Call) -> Self {
            Self::Log22(value)
        }
    }
    impl ::core::convert::From<LogAddressCall> for HardhatConsoleCalls {
        fn from(value: LogAddressCall) -> Self {
            Self::LogAddress(value)
        }
    }
    impl ::core::convert::From<LogBoolCall> for HardhatConsoleCalls {
        fn from(value: LogBoolCall) -> Self {
            Self::LogBool(value)
        }
    }
    impl ::core::convert::From<LogBytesCall> for HardhatConsoleCalls {
        fn from(value: LogBytesCall) -> Self {
            Self::LogBytes(value)
        }
    }
    impl ::core::convert::From<LogBytes1Call> for HardhatConsoleCalls {
        fn from(value: LogBytes1Call) -> Self {
            Self::LogBytes1(value)
        }
    }
    impl ::core::convert::From<LogBytes10Call> for HardhatConsoleCalls {
        fn from(value: LogBytes10Call) -> Self {
            Self::LogBytes10(value)
        }
    }
    impl ::core::convert::From<LogBytes11Call> for HardhatConsoleCalls {
        fn from(value: LogBytes11Call) -> Self {
            Self::LogBytes11(value)
        }
    }
    impl ::core::convert::From<LogBytes12Call> for HardhatConsoleCalls {
        fn from(value: LogBytes12Call) -> Self {
            Self::LogBytes12(value)
        }
    }
    impl ::core::convert::From<LogBytes13Call> for HardhatConsoleCalls {
        fn from(value: LogBytes13Call) -> Self {
            Self::LogBytes13(value)
        }
    }
    impl ::core::convert::From<LogBytes14Call> for HardhatConsoleCalls {
        fn from(value: LogBytes14Call) -> Self {
            Self::LogBytes14(value)
        }
    }
    impl ::core::convert::From<LogBytes15Call> for HardhatConsoleCalls {
        fn from(value: LogBytes15Call) -> Self {
            Self::LogBytes15(value)
        }
    }
    impl ::core::convert::From<LogBytes16Call> for HardhatConsoleCalls {
        fn from(value: LogBytes16Call) -> Self {
            Self::LogBytes16(value)
        }
    }
    impl ::core::convert::From<LogBytes17Call> for HardhatConsoleCalls {
        fn from(value: LogBytes17Call) -> Self {
            Self::LogBytes17(value)
        }
    }
    impl ::core::convert::From<LogBytes18Call> for HardhatConsoleCalls {
        fn from(value: LogBytes18Call) -> Self {
            Self::LogBytes18(value)
        }
    }
    impl ::core::convert::From<LogBytes19Call> for HardhatConsoleCalls {
        fn from(value: LogBytes19Call) -> Self {
            Self::LogBytes19(value)
        }
    }
    impl ::core::convert::From<LogBytes2Call> for HardhatConsoleCalls {
        fn from(value: LogBytes2Call) -> Self {
            Self::LogBytes2(value)
        }
    }
    impl ::core::convert::From<LogBytes20Call> for HardhatConsoleCalls {
        fn from(value: LogBytes20Call) -> Self {
            Self::LogBytes20(value)
        }
    }
    impl ::core::convert::From<LogBytes21Call> for HardhatConsoleCalls {
        fn from(value: LogBytes21Call) -> Self {
            Self::LogBytes21(value)
        }
    }
    impl ::core::convert::From<LogBytes22Call> for HardhatConsoleCalls {
        fn from(value: LogBytes22Call) -> Self {
            Self::LogBytes22(value)
        }
    }
    impl ::core::convert::From<LogBytes23Call> for HardhatConsoleCalls {
        fn from(value: LogBytes23Call) -> Self {
            Self::LogBytes23(value)
        }
    }
    impl ::core::convert::From<LogBytes24Call> for HardhatConsoleCalls {
        fn from(value: LogBytes24Call) -> Self {
            Self::LogBytes24(value)
        }
    }
    impl ::core::convert::From<LogBytes25Call> for HardhatConsoleCalls {
        fn from(value: LogBytes25Call) -> Self {
            Self::LogBytes25(value)
        }
    }
    impl ::core::convert::From<LogBytes26Call> for HardhatConsoleCalls {
        fn from(value: LogBytes26Call) -> Self {
            Self::LogBytes26(value)
        }
    }
    impl ::core::convert::From<LogBytes27Call> for HardhatConsoleCalls {
        fn from(value: LogBytes27Call) -> Self {
            Self::LogBytes27(value)
        }
    }
    impl ::core::convert::From<LogBytes28Call> for HardhatConsoleCalls {
        fn from(value: LogBytes28Call) -> Self {
            Self::LogBytes28(value)
        }
    }
    impl ::core::convert::From<LogBytes29Call> for HardhatConsoleCalls {
        fn from(value: LogBytes29Call) -> Self {
            Self::LogBytes29(value)
        }
    }
    impl ::core::convert::From<LogBytes3Call> for HardhatConsoleCalls {
        fn from(value: LogBytes3Call) -> Self {
            Self::LogBytes3(value)
        }
    }
    impl ::core::convert::From<LogBytes30Call> for HardhatConsoleCalls {
        fn from(value: LogBytes30Call) -> Self {
            Self::LogBytes30(value)
        }
    }
    impl ::core::convert::From<LogBytes31Call> for HardhatConsoleCalls {
        fn from(value: LogBytes31Call) -> Self {
            Self::LogBytes31(value)
        }
    }
    impl ::core::convert::From<LogBytes32Call> for HardhatConsoleCalls {
        fn from(value: LogBytes32Call) -> Self {
            Self::LogBytes32(value)
        }
    }
    impl ::core::convert::From<LogBytes4Call> for HardhatConsoleCalls {
        fn from(value: LogBytes4Call) -> Self {
            Self::LogBytes4(value)
        }
    }
    impl ::core::convert::From<LogBytes5Call> for HardhatConsoleCalls {
        fn from(value: LogBytes5Call) -> Self {
            Self::LogBytes5(value)
        }
    }
    impl ::core::convert::From<LogBytes6Call> for HardhatConsoleCalls {
        fn from(value: LogBytes6Call) -> Self {
            Self::LogBytes6(value)
        }
    }
    impl ::core::convert::From<LogBytes7Call> for HardhatConsoleCalls {
        fn from(value: LogBytes7Call) -> Self {
            Self::LogBytes7(value)
        }
    }
    impl ::core::convert::From<LogBytes8Call> for HardhatConsoleCalls {
        fn from(value: LogBytes8Call) -> Self {
            Self::LogBytes8(value)
        }
    }
    impl ::core::convert::From<LogBytes9Call> for HardhatConsoleCalls {
        fn from(value: LogBytes9Call) -> Self {
            Self::LogBytes9(value)
        }
    }
    impl ::core::convert::From<LogIntCall> for HardhatConsoleCalls {
        fn from(value: LogIntCall) -> Self {
            Self::LogInt(value)
        }
    }
    impl ::core::convert::From<LogStringCall> for HardhatConsoleCalls {
        fn from(value: LogStringCall) -> Self {
            Self::LogString(value)
        }
    }
    impl ::core::convert::From<LogUintCall> for HardhatConsoleCalls {
        fn from(value: LogUintCall) -> Self {
            Self::LogUint(value)
        }
    }
}
