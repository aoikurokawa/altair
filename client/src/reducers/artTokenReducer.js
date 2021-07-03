const initialState = {
    contractInstance: {}, 
    accounts: "",
    ipfsHash: "", 
    ipfsUrl: "", 
    name: "",
    nftDetail: {},
}

const artTokenReducer = (state = initialState, action) => {
    switch(action.type) {
        case "GET_CONTRACTDATA": 
            return {
                ...state, 
                contractInstance: action.contractInstance, 
                accounts: action.accounts,
            };

        case "IPFS_UPLOAD":
            return {
                ...state, 
                ipfsHash: action.ipfsHash, 
                ipfsUrl: action.ipfsUrl,
                name: action.name,
            };

        case "IPFS_CLEAR": 
            return {
                ...state, 
                ipfsHash: "", 
                ipfsUrl: "", 
                name: "",
            };

        case "MOVE_AUCTIONDETAIL":
            return {
                ...state, 
                nftDetail: action.nftDetail,
            };

        default: 
            return {
                ...state,
            };
    }
} 

export default artTokenReducer;
