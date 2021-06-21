import Web3 from 'web3';
import { ADDRESS, ABI } from '../config';

export const loadBlockchain = () => async (dispatch) => {
    const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
    const accounts = await web3.eth.getAccounts();
    const selectedAccounts = window.ethereum.selectedAddress;
    const contractInstance = new web3.eth.Contract(ABI, ADDRESS, {from: accounts[0]});

    dispatch({
        type: "GET_CONTRACTDATA", 
        contractInstance: contractInstance, 
        accounts: accounts,
    });
    contractInstance.methods
        .highestBid()
        .call()
        .then((result) => {
            dispatch({
                type: "GET_HIGHESTBID", 
                highestBid: Web3.utils.fromWei(result, "ether"),
            });
        });

    contractInstance.methods
        .auctionTimeEnd()
        .call()
        .then((res) => {
            let endDate = new Date(res * 1000);
            let auctionTime = endDate.toGMTString();
            dispatch({
                type: "GET_AUCTIONTIMEEND", 
                auctionTimeEnd: auctionTime,
            });
        });
};

