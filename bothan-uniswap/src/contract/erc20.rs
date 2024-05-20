use alloy::sol;

sol! {
    #[sol(rpc)]
    contract ERC20 {
        #[derive(Debug)]
        function decimals() public view virtual returns (uint8);
    }
}
