import React, { useState } from 'react';
import { Button, Container, Typography, TextField, } from '@material-ui/core';
import { makeStyles } from '@material-ui/core';
import { useDispatch, useSelector } from 'react-redux';
import { useMoralisFile, } from 'react-moralis';

const useStyles = makeStyles({
    inputContainer: {
        width: '100%',
        textAlign: 'center',
    },
    input: {
        display: 'none',
    },
    imgContainer: {
        borderWidth: '5px',
        borderStyle: 'double',
        borderColor: 'arkslategray',
        height: '35rem',
    },
    img: {
        width: '100%',
        height: '100%',
        padding: '24px 0px',
    },
    buttonContainer: {
        textAlign: 'center',
    },
})

const UploadArt = () => {
    const dispatch = useDispatch();
    const classes = useStyles();
    const [uploadImage, setUploadImage] = useState();
    const [imageUrl, setImageUrl] = useState();
    const [name, setName] = useState("");
    const { accounts } = useSelector((state) => state.loadBlockchain);
    const { saveFile, } = useMoralisFile();

    const createNftHandler = async () => {
        dispatch({
            type: "SHOW_LOADER"
        });
        const metadata = {
            name: name,
            accounts: accounts[0]
        };

        await saveFile(name, uploadImage, { metadata, saveIPFS: true })
            .then((result) => {
                dispatch({
                    type: "IPFS_UPLOAD",
                    ipfsHash: result._hash,
                    ipfsUrl: result._ipfs,
                    name: name,
                });
                dispatch({
                    type: "CLOSE_LOADER"
                });
            })
            .catch((error) => {
                alert(error);
            });
    };

    return (
        <>
            <Container className={classes.inputContainer}>
                <input
                    accept="image/*"
                    className={classes.input}
                    id="contained-button-file"
                    multiple
                    type="file"
                    onChange={(event) => {
                        let img = event.target.files[0];
                        setUploadImage(img);
                        setImageUrl(URL.createObjectURL(img))
                    }}
                />
                <label htmlFor="contained-button-file">
                    <Button variant="contained" color="primary" component="span">
                        Upload Your Work
                    </Button>
                </label>
            </Container>
            <Container className={classes.imgContainer}>
                <img src={imageUrl} alt="" className={classes.img} />
            </Container>
            <form noValidate>
                <TextField
                    variant="outlined"
                    margin="normal"
                    fullWidth
                    id="name"
                    label="Name"
                    autoComplete="name"
                    autoFocus
                    value={name}
                    onChange={(event) => setName(event.currentTarget.value)}
                />
                <TextField
                    variant="outlined"
                    margin="normal"
                    fullWidth
                    id="accounts"
                    label="Accounts"
                    autoComplete="accounts"
                    autoFocus
                    aria-readonly="true"
                    disabled
                    value={accounts}
                />
            </form>
            <Container className={classes.buttonContainer}>
                <Typography component="p">
                    We will upload the your work to IPFS to create your own NFT

                </Typography>
                <Button variant="contained" color="primary" onClick={createNftHandler}>
                    UPLOAD TO IPFS
                </Button>
            </Container>
        </>
    );
}

export default UploadArt;
