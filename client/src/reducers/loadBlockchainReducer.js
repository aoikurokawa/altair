const initialState = {
    contractInstance: {},
    accounts: "",
}

const loadBlockchainReducer = (state = initialState, action) => {
    switch (action.type) {
        case "GET_CONTRACTDATA":
            return {
                ...state,
                contractInstance: action.contractInstance,
                accounts: action.accounts,
            };

        default:
            return {
                ...state
            }
    }
}

export default loadBlockchainReducer;


