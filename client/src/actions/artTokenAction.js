import Web3 from "web3";
import { ARTTOKENADDRESS, ARTTOKENABI } from '../artTokenConfig';

export const artTokenAction = () => async (dispatch) => {
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
