const Auction = artifacts.require('./Auction.sol');

contract('Auction', (accounts) => {
    let contract;

    describe('deployment', async () => {
        it('deployee successfully', async () => {
            contract = await Auction.deployed()
            const address = contract.address
            assert.notEqual(address, '');
        });

        it('auction start successfully', async () => {
            contract = await Auction.deployed();
            await contract.methods.auctionStart
                .call()
                .then((res) => {
                    console.log(res);
                });

        })

    })
})
