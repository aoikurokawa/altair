import React, { useState } from 'react';
import { Grid, Button, Container, Typography, TextField, } from '@material-ui/core';
import { makeStyles } from '@material-ui/core';
import { useDispatch, useSelector } from 'react-redux';
import { useMoralis, useMoralisFile, useMoralisQuery } from 'react-moralis';

const useStyles = makeStyles({
    root: {
        height: '60.4rem',
        flexGrow: 1,
    },
    uploadTy: {
        padding: '2rem',
    },
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
    circular: {
        height: '61rem',
        textAlign: 'center',
        paddingTop: '50%',
    },
})

const UploadArt = () => {
    const classes = useStyles();
    const [uploadImage, setUploadImage] = useState();
    const [imageUrl, setImageUrl] = useState();
    const [name, setName] = useState("");
    const { accounts } = useSelector((state) => state.loadBlockchain);
    const { contractInstance } = useSelector((state) => state.artToken);
    const { saveFile, } = useMoralisFile();

    const createNftHandler = async () => {
        const metadata = {
            name: name,
            accounts: accounts
        };

        await saveFile(name, uploadImage, { metadata, saveIPFS: true })
            .then((result) => {
                mint(result._ipfs);
            })
            .catch((error) => {
                alert(error);
            });
    };

    const mint = (link) => {
        contractInstance.methods
            .mint(link)
            .send()
            .on(("transactionHash"), (hash) => {
                console.log(hash);
            })
            .on(("receipt"), (receipt) => {
                console.log(receipt);
            })
    }

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
                    Create a NFT with your account and NFT's name
                </Typography>
                <Button variant="contained" color="primary" onClick={() => mint("hello")}>
                    Create NFT
                </Button>
            </Container>
        </>
    );
}

export default UploadArt;
