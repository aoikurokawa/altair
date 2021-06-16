const Auction = artifacts.require("./Auction");

const auctionSetting = {
    biddingTime: 0,
}

module.exports = function (deployer, network, account) {

    deployer.deploy(Auction, auctionSetting.biddingTime, account[0]);


};
