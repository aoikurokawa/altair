import Web3 from 'web3';
import { AUCTIONADDRESS, AUCTIONABI } from '../auctionConfig';

export const loadBlockchain = () => async (dispatch) => {
    const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
    const accounts = await web3.eth.getAccounts();
    const selectedAccounts = window.ethereum.selectedAddress;
    const contractInstance = new web3.eth.Contract(AUCTIONABI, AUCTIONADDRESS, { from: accounts[0] });

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

            let endDate = new Date(res * 1);
            let auctionTime = endDate.toDateString();
            dispatch({
                type: "GET_AUCTIONTIMEEND",
                auctionTimeEnd: auctionTime,
            });
        });
};

export const bidHandlerAction = (price) => async (dispatch) => {
    const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
    const accounts = await web3.eth.getAccounts();
    const selectedAccounts = window.ethereum.selectedAddress;
    const contractInstance = new web3.eth.Contract(AUCTIONABI, AUCTIONADDRESS, { from: accounts[0] });

    if (price <= 0) {
        alert("Please put more than 0");
    } else {
        let config = {
            value: Web3.utils.toWei(price, "ether"),
            from: accounts[0],
        };
        contractInstance.methods
            .bid()
            .send(config)
            .on("transactionHash", (hash) => {
                console.log(hash);
            })
            .on("receipt", (receipt) => {
                console.log(receipt);
                alert("Congratulations!");
            });
    }
};

