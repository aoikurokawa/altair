const initialState = {
    contractInstance: {}, 
    accounts: "", 
}

const artTokenReducer = (state = initialState, action) => {
    switch(action.type) {
        case "GET_CONTRACTDATA": 
            return {
                ...state, 
                contractInstance: action.contractInstance, 
                accounts: action.accounts,
            };

        default: 
            return {
                ...state,
            };
    }
} 

export default artTokenReducer;
