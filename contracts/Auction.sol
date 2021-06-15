// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract Auction {

    struct Buyer {
        uint256 balance;
        uint256 declareAmount;
    }

    address[] buyersList;

    mapping(address => Buyer) buyers;

    function declareAmount() public payable {
        require(msg.value > 0, "Amount should be more than 0");

        buyers[msg.sender].declareAmount = msg.value;
        buyersList.push(msg.sender);
    }
    
    function chooseHighestBuyer() view internal returns (address)  {

        address highestBuyerAddress;
        
        for (uint i = 0; i < buyersList.length; i++) {
            
            if (buyers[buyersList[i]].declareAmount > buyers[buyersList[i + 1]].declareAmount) {
                highestBuyerAddress = buyersList[i];
            }
        }
        return highestBuyerAddress;
    }

    function buy() public returns (bool) {

    }

}
