const initialState = {
    isModalVisible: false,
    functionType: "",
    title: "",
    objectId: "",
}

const modalReducer = (state = initialState, action) => {
    switch(action.type) {
        case 'SHOW_MODAL': 
            return {
                isModalVisible: true,
                title: action.title,
                functionType: action.functionType,
                objectId: action.objectId,
            }

        case 'CLOSE_MODAL':
            return {
                isModalVisible: false, 
                title: "", 
                functionType: "",
                objectId: "",
            }

        default: 
            return {
                ...state
            }    
    }
}

export default modalReducer;
