import React, { useEffect, useState } from 'react';
import { Button, Container } from '@material-ui/core';
import { useDispatch, useSelector } from 'react-redux';
import { makeStyles } from '@material-ui/core';

const useStyles = makeStyles({
    root: {
        height: '60.4rem',
    },
    inputContainer: {
        
    },
    input: {
        display: 'none',
    }
})

const Upload = () => {
    const { contractInstance } = useSelector((state) => state.artToken);
    const [uploadFile, setUploadFile] = useState()

    useEffect(() => {

    }, [uploadFile]);

    console.log(uploadFile);

    const classes = useStyles();
    return (
        <div className={classes.root}>
            <Container className={classes}>
                <input
                    accept="image/*"
                    className={classes.input}
                    id="contained-button-file"
                    multiple
                    type="file"
                    onChange={(event) => {
                        let img = event.target.files[0];
                        setUploadFile(URL.createObjectURL(img))
                    }}
                />
                <label htmlFor="contained-button-file">
                    <Button variant="contained" color="primary" component="span">
                        Upload
                    </Button>
                </label>
                <img src={uploadFile} alt="" />
            </Container>
            <Container>
                <Button variant="contained" color="primary">
                    Create NFT
                </Button>
            </Container>
        </div>
    );
}

export default Upload;
