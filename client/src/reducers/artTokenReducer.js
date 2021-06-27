const initialState = {
    contractInstance: {}, 
    accounts: "",
    ipfsHash: "", 
    ipfsUrl: "", 
}

const artTokenReducer = (state = initialState, action) => {
    switch(action.type) {
        case "GET_CONTRACTDATA": 
            return {
                ...state, 
                contractInstance: action.contractInstance, 
                accounts: action.accounts,
            };

        case "IPFS_HANDLER":
            return {
                ...state, 
                ipfsHash: action.ipfsHash, 
                ipfsUrl: action.ipfsUrl,
            };

        default: 
            return {
                ...state,
            };
    }
} 

export default artTokenReducer;
