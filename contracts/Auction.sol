//SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.0;

contract Auction {
    bool ended;

    struct AuctionDetail {
        address payable beneficiary;
        uint256 auctionTimeEnd;
        address highestBidder;
        uint256 highestBid;
    }

    mapping(string => AuctionDetail) tokenBuyerDetails;
    mapping(string => mapping(address => uint256)) pendingReturns;

    event AuctionStarted(address beneficiary, uint256 biddingTime);
    event HighestBidIncreased(address bidder, uint256 amount);
    event AuctionEnded(address winner, uint256 amount);
    event WithdrawPendingReturns(
        address pender,
        uint256 amount,
        bool isSuccess
    );

    function auctionStart(string memory _tokenId, uint256 _biddingTime)
        public
        payable
    {
        // require(_tokenId != "", "Token should not be empty");
        AuctionDetail memory auctionDetail = AuctionDetail({
            beneficiary: payable(msg.sender),
            auctionTimeEnd: _biddingTime,
            highestBidder: msg.sender,
            highestBid: 0
        });
        tokenBuyerDetails[_tokenId] = auctionDetail;

        emit AuctionStarted(msg.sender, _biddingTime);
    }

    function getHighestBid(string memory _tokenId) public view returns (uint256) {
        AuctionDetail memory auctionDetail = tokenBuyerDetails[_tokenId];
        return auctionDetail.highestBid;
    }

    function getAuctionTimeEnd(string memory _tokenId) public view returns (uint256) {
        AuctionDetail memory auctionDetail = tokenBuyerDetails[_tokenId];
        return auctionDetail.auctionTimeEnd;
    }

    function bid(string memory _tokenId) public payable {
        AuctionDetail memory existAuctionDetail = tokenBuyerDetails[_tokenId];

        require(
            msg.value > existAuctionDetail.highestBid,
            "Value is less than highest value"
        );
        require(
            block.timestamp <= existAuctionDetail.auctionTimeEnd,
            "Auction already ended"
        );

        if (existAuctionDetail.highestBid != 0) {
            pendingReturns[_tokenId][
                existAuctionDetail.highestBidder
            ] += existAuctionDetail.highestBid;
        }

        AuctionDetail memory newAuctionDetail = AuctionDetail(
            existAuctionDetail.beneficiary,
            existAuctionDetail.auctionTimeEnd,
            msg.sender,
            msg.value
        );
        tokenBuyerDetails[_tokenId] = newAuctionDetail;
    }

    function withdraw(string memory _tokenId) public returns (bool) {
        bool isSuccess = false;
        uint256 amount = pendingReturns[_tokenId][msg.sender];
        if (amount > 0) {
            pendingReturns[_tokenId][msg.sender] = 0;

            if (!payable(msg.sender).send(amount)) {
                pendingReturns[_tokenId][msg.sender] = amount;
                emit WithdrawPendingReturns(msg.sender, amount, isSuccess);
                return false;
            } else {
                isSuccess = true;
                emit WithdrawPendingReturns(msg.sender, amount, isSuccess);
                return true;
            }
        } else {
            emit WithdrawPendingReturns(msg.sender, amount, isSuccess);
            return false;
        }
    }

    function auctionEnd(string memory _tokenId) public {
        AuctionDetail memory existAuctionDetail = tokenBuyerDetails[_tokenId];

        require(
            block.timestamp >= existAuctionDetail.auctionTimeEnd,
            "auction is still processing"
        );
        require(!ended, "Auction has not finished yet");

        ended = true;
        emit AuctionEnded(
            existAuctionDetail.highestBidder,
            existAuctionDetail.highestBid
        );

        (bool success, ) = existAuctionDetail.beneficiary.call{
            value: existAuctionDetail.highestBid
        }("");
        require(success, "Transfer failed");
        // beneficiary.transfer(highestBid);
    }
}
