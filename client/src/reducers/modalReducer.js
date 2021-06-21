const initialState = {
    isModalVisible: false,
    functionType: "",
}

const modalReducer = (state = initialState, action) => {
    switch(action.type) {
        case 'SHOW_MODAL': 
            return {
                isModalVisible: true,
                functionType: action.functionType,
            }

        case 'CLOSE_MODAL':
            return {
                isModalVisible: false, 
                functionType: action.functionType,
            }

        default: 
            return {
                ...state
            }    
    }
}

export default modalReducer;
