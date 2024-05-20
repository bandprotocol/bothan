use alloy::eips::BlockId;
use alloy::providers::{Network, Provider};
use alloy::sol;
use alloy::transports::Transport;
use alloy_primitives::Address;

use crate::contract::erc20::ERC20;
use crate::contract::error::Error;

const TICK: f64 = 1.0001_f64;

sol! {
    #[sol(rpc)]
    contract UniswapV3Pool {
        address public immutable override token0;
        address public immutable override token1;

        #[derive(Debug)]
        function slot0()
            external
            view
            returns (
                uint160 sqrtPriceX96,
                int24 tick,
                uint16 observationIndex,
                uint16 observationCardinality,
                uint16 observationCardinalityNext,
                uint8 feeProtocol,
                bool unlocked
            );

        #[derive(Debug)]
        function feeGrowthGlobal0X128() external view returns (uint256);

        #[derive(Debug)]
        function feeGrowthGlobal1X128() external view returns (uint256);

        #[derive(Debug)]
        function protocolFees() external view returns (uint128 token0, uint128 token1);

        #[derive(Debug)]
        function liquidity() external view returns (uint128);

        #[derive(Debug)]
        function ticks(int24 tick)
            external
            view
            returns (
                uint128 liquidityGross,
                int128 liquidityNet,
                uint256 feeGrowthOutside0X128,
                uint256 feeGrowthOutside1X128,
                int56 tickCumulativeOutside,
                uint160 secondsPerLiquidityOutsideX128,
                uint32 secondsOutside,
                bool initialized
            );

        #[derive(Debug)]
        function tickBitmap(int16 wordPosition) external view returns (uint256);

        #[derive(Debug)]
        function positions(bytes32 key)
            external
            view
            returns (
                uint128 _liquidity,
                uint256 feeGrowthInside0LastX128,
                uint256 feeGrowthInside1LastX128,
                uint128 tokensOwed0,
                uint128 tokensOwed1
            );

        #[derive(Debug)]
        function observations(uint256 index)
            external
            view
            returns (
                uint32 blockTimestamp,
                int56 tickCumulative,
                uint160 secondsPerLiquidityCumulativeX128,
                bool initialized
            );
    }
}

pub async fn get_spot_price<P: Provider<T, N>, T: Transport + Clone, N: Network>(
    provider: P,
    pool_address: &str,
    block: Option<BlockId>,
) -> Result<f64, Error> {
    let block_id = block.unwrap_or_else(BlockId::latest);

    let address = Address::parse_checksummed(pool_address, None)?;
    let contract = UniswapV3Pool::new(address, provider.root());

    let token0 = contract.token0().block(block_id).call().await?._0;
    let token1 = contract.token1().block(block_id).call().await?._0;
    let slot0 = contract.slot0().block(block_id).call().await?;

    let price = get_price_from_tick(slot0.tick, token0, token1, provider).await?;
    Ok(price)
}

async fn get_price_from_tick<P: Provider<T, N>, T: Transport + Clone, N: Network>(
    tick: i32,
    token0: Address,
    token1: Address,
    provider: P,
) -> Result<f64, Error> {
    let token0 = ERC20::new(token0, provider.root());
    let token1 = ERC20::new(token1, provider.root());

    let token0_decimals = token0.decimals().call().await?._0;
    let token1_decimals = token1.decimals().call().await?._0;

    Ok(TICK.powf(tick as f64) * 10_f64.powf((token0_decimals - token1_decimals) as f64))
}
