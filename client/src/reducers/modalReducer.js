const initialState = {
    isModalVisible: false,
    functionType: "",
    title: "",
    objectId: "",
    tokenId: "",
}

const modalReducer = (state = initialState, action) => {
    switch(action.type) {
        case 'SHOW_MODAL': 
            return {
                isModalVisible: true,
                title: action.title,
                functionType: action.functionType,
                objectId: action.objectId,
                tokenId: action.tokenId,
            }

        case 'CLOSE_MODAL':
            return {
                isModalVisible: false, 
                title: "", 
                functionType: "",
                objectId: "",
                tokenId: "",
            }

        default: 
            return {
                ...state
            }    
    }
}

export default modalReducer;
