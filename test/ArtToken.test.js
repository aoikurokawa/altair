const ArtToken = artifacts.require('./ArtToken.sol');

contract('ArtToken', (accounts) => {
    let contract 

    describe('deployment', async () => {
        it('deploye successfully', async () => {
            contract = await ArtToken.deployed()
            const address = contract.address
            console.log(address);
            assert.notEqual(address, '');
        })
    })
})
