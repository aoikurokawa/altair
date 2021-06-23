// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract Auction {
    address payable public beneficiary;
    uint256 public auctionTimeEnd;

    address public highestBidder;
    uint256 public highestBid;

    mapping(address => uint256) pendingReturns;

    bool ended;

    event HighestBidIncreased(address bidder, uint256 amount);
    event AuctionEnded(address winner, uint256 amount);
    event WithdrawPendingReturns(
        address pender,
        uint256 amount,
        bool isSuccess
    );

    constructor(uint256 _biddingTime, address payable _beneficiary) public {
        beneficiary = _beneficiary;
        auctionTimeEnd = _biddingTime;
    }

    function bid() public payable {
        require(now <= auctionTimeEnd, "Auction already ended");
        require(msg.value > highestBid, "Value is less than highest value");

        if (highestBid != 0) {
            pendingReturns[highestBidder] += highestBid;
        }

        highestBidder = msg.sender;
        highestBid = msg.value;
    }

    function withdraw() public returns (bool) {
        bool isSuccess = false;
        uint256 amount = pendingReturns[msg.sender];
        if (amount > 0) {
            pendingReturns[msg.sender] = 0;

            if (!msg.sender.send(amount)) {
                pendingReturns[msg.sender] = amount;
                emit WithdrawPendingReturns(msg.sender, amount, isSuccess);
                return isSuccess;
            } else {
                isSuccess = true;
                emit WithdrawPendingReturns(msg.sender, amount, isSuccess);
                return isSuccess;
            }
        } else {
            emit WithdrawPendingReturns(msg.sender, amount, isSuccess);
        }

    }

    function auctionEnd() public {
        require(now >= auctionTimeEnd, "auction is still processing");
        require(!ended, "Auction has not finished yet");

        ended = true;
        emit AuctionEnded(highestBidder, highestBid);

        (bool success, ) = beneficiary.call.value(highestBid)("");
        require(success, "Transfer failed");
        // beneficiary.transfer(highestBid);
    }
}
