import Web3 from "web3";
import { ARTTOKENADDRESS, ARTTOKENABI } from '../artTokenConfig';
import { useNewMoralisObject } from 'react-moralis';

const Moralis = require('moralis');

export const getArtTokenContract = () => async (dispatch) => {
    const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
    const accounts = await web3.eth.getAccounts();
    const selectedAccounts = window.ethereum.selectedAddress;
    const contractInstance = new web3.eth.Contract(ARTTOKENABI, ARTTOKENADDRESS, { from: accounts[0] });

    dispatch({
        type: "GET_CONTRACTDATA",
        contractInstance: contractInstance,
        accounts: accounts
    });
}

export const mint = async (link) => async (dispatch) => {
    const tokenId = 0;
    const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
    const accounts = await web3.eth.getAccounts();
    const selectedAccounts = window.ethereum.selectedAddress;
    const contractInstance = new web3.eth.Contract(ARTTOKENABI, ARTTOKENADDRESS, { from: accounts[0] });

    console.log(link);

    contractInstance.methods
        .mint("https://ipfs.moralis.io:2053/ipfs/QmboUXweMtpApzYMZMdwTSDtWD7Euos6NMfJHbuutcFrS3")
        .send()
        .on(("transactionHash"), (hash) => {
            console.log(hash);
        })
        .on(("receipt"), (receipt) => {
            tokenId = parseInt(receipt.events.Transfer.returnValues[2]);
            const nft = new Moralis.Object('Nft');
            nft.set('TokenId', tokenId);
            nft.set('Account', accounts[0]);
            nft.save();
        });

}

