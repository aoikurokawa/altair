// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract Auction {

    address payable public beneficiary;
    uint public auctionTimeEnd;

    address public highestBidder;
    uint public highestBid;

    mapping(address => uint) pendingReturns;

    bool ended;

    event HighestBidIncreased(address bidder, uint amount);
    event AuctionEnded(address winner, uint amount);

    constructor(uint _biddingTime, address payable _beneficiary) {
        beneficiary = _beneficiary;
        auctionTimeEnd = _biddingTime;
    }

    function bid() payable public  {
        require(now <= auctionTimeEnd, "Auction already ended");
        require(msg.value > highestBid, "Value is less than highest value");

        if (highestBid != 0) {
            pendingReturns[highestBidder] += highestBid; 
        }

        highestBidder = msg.sender;
        highestBid = msg.value;
    }

    function withdraw() public returns (bool) {
        uint amount = pendingReturns[msg.sender];
        if (amount > 0) {
            pendingReturns[msg.sender] = 0;

            if (!msg.sender.send(amount)) {
                pendingReturns[msg.sender] = amount;
                return false;
            }
        }
        return true;
    }

    function auctionEnd() public {
        require(now >= auctionTimeEnd, "auction is still processing");
        require(!ended, "Auction has not finished yet");

        ended = true;
        emit AuctionEnded(highestBidder, highestBid);

        beneficiary.transfer(highestBid);
    }

    // function() public payable { }

    
}
