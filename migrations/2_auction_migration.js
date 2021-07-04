const Auction = artifacts.require("Auction");
const ArtToken = artifacts.require("ArtToken");

const auctionSetting = {
    biddingTime: Date.now(),
}

module.exports = async function (deployer, network, account) {

    deployer.deploy(Auction);

    await deployer.deploy(ArtToken, "Art NFT", "ANFT");
    let tokenInstance = await ArtToken.deployed();
    await tokenInstance.mint("http:ipfs..");
    let art = await tokenInstance.getTokenDetail(0);
    console.log(art);
};
