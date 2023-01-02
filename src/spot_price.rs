use std::ops::Mul;

use ethers::{
    abi::{Int, Uint},
    contract::Contract,
    providers::Middleware,
};

pub async fn get_spot_price<M: Middleware>(
    uniswap_pool: &Contract<M>,
    token0_decimals: Uint,
) -> Uint {
    let (sqrt_ratio_x96, _, _, _, _, _, _) = uniswap_pool
        .method::<_, (Uint, Int, Uint, Uint, Uint, Uint, bool)>("slot0", ())
        .expect("Contract call failed")
        .call()
        .await
        .expect("Retrieving sqrt_ratio_x96 failed");

    return (sqrt_ratio_x96.pow(Uint::from(2)) / Uint::from(2).pow(Uint::from(192)))
        .mul(Uint::from(10).pow(token0_decimals));
}
