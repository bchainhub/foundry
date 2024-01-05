// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3703
contract Issue3703Test is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    function setUp() public {
        uint256 fork = vm.createSelectFork(
            "https://polygon-mainnet.g.alchemy.com/v2/bVjX9v-FpmUhf5R_oHIgwJx2kXvYPRbx",
            bytes32(
                0xbed0c8c1b9ff8bf0452979d170c52893bb8954f18a904aa5bcbd0f709be050b9
            )
        );
    }

    function poolState(
        address poolAddr,
        uint256 expectedSqrtPriceX96,
        uint256 expectedLiquidity
    ) private {
        IUniswapV3Pool pool = IUniswapV3Pool(poolAddr);

        (uint256 actualSqrtPriceX96, , , , , , ) = pool.slot0();
        uint256 actualLiquidity = pool.liquidity();

        assertEq(expectedSqrtPriceX96, actualSqrtPriceX96);
        assertEq(expectedLiquidity, actualLiquidity);
    }

    function testStatePool1() public {
        poolState(
            0xcb48847b64f9d3a95e977d157866447a5c0a5dfa0ee5,
            1076133273204200901840477866344,
            1221531661829
        );
    }
}

interface IUniswapV3Pool {
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

    function liquidity() external view returns (uint128);
}
