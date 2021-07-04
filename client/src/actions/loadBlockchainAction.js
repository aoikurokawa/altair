import Web3 from 'web3';
import { AUCTIONADDRESS, AUCTIONABI } from '../auctionConfig';

const loadBlockchain = async () => {
    const web3 = new Web3(Web3.givenProvider || "http:localhost:8545");
    const accounts = await web3.eth.getAccounts();
    const contractInstance = new web3.eth.Contract(AUCTIONABI, AUCTIONADDRESS, { from: accounts[0] });
    const auctionContract = {
        contractInstance: contractInstance,
        accounts: accounts,
    }
    return auctionContract;
}


export const auctionContractHandler = (_tokenId) => async (dispatch) => {
    const { contractInstance, accounts } = await loadBlockchain();
    dispatch({
        type: "GET_CONTRACTDATA",
        contractInstance: contractInstance,
        accounts: accounts,
    });
    
    contractInstance.methods
        .getHighestBid(_tokenId)
        .call()
        .then((result) => {
            dispatch({
                type: "GET_HIGHESTBID",
                highestBid: Web3.utils.fromWei(result, "ether"),
            });
        });

    contractInstance.methods
        .getAuctionTimeEnd(_tokenId)
        .call()
        .then((res) => {
            dispatch({
                type: "GET_AUCTIONTIMEEND",
                auctionTimeEnd: res,
            });
        });
};

export const bidHandlerAction = (_tokenId, _price) => async (dispatch) => {
    const { contractInstance, accounts } = await loadBlockchain();

    if (_price <= 0) {
        alert("Please put more than 0");
    } else {
        let config = {
            value: Web3.utils.toWei(_price, "ether"),
            from: accounts[0],
        };
        contractInstance.methods
            .bid(_tokenId)
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

export const widthdrawHandler = () => async () => {
    const { contractInstance } = await loadBlockchain();
    contractInstance.methods
        .withdraw()
        .send()
        .on("transactionHash", (hash) => {
            console.log(hash);
        })
        .on("receipt", (receipt) => {
            console.log(receipt);
            if (receipt.events.WithdrawPendingReturns.returnValues["isSuccess"]) {
                let amount = receipt.events.WithdrawPendingReturns.returnValues["amount"];
                console.log(amount);
                alert(`You received ${Web3.utils.fromWei(amount, "ether")} eth! Check your Metamask wallet`);
            } else {
                alert("Nothing to withdraw");
            }

        });
}

