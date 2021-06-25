const Auction = artifacts.require("Auction");
const ArtToken = artifacts.require("ArtToken");

const auctionSetting = {
    biddingTime: Date.now(),
}

module.exports = async function (deployer, network, account) {

    let currentDate = new Date(Date.now());
    let auctionTimeEnd = currentDate.setMonth(currentDate.getMonth() + 1);

    deployer.deploy(Auction, auctionTimeEnd, account[0]);

    await deployer.deploy(ArtToken, "Art NFT", "ANFT");
    let tokenInstance = await ArtToken.deployed();
    await tokenInstance.mint("http:ipfs..");
    let art = await tokenInstance.getTokenDetail(0);
    console.log(art);
};
