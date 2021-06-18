const Auction = artifacts.require("Auction");

const auctionSetting = {
    biddingTime: Date.now(),
}

module.exports = function (deployer, network, account) {

    let currentDate = new Date(Date.now());
    let auctionTimeEnd = currentDate.setMonth(currentDate.getMonth() + 1);

    deployer.deploy(Auction, auctionTimeEnd, account[0]);
};
