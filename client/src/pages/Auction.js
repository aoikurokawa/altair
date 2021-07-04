import React from 'react';
import { useHistory } from 'react-router-dom';
import { useDispatch, } from 'react-redux'
import { Typography, Card, CardHeader, Avatar, CardMedia, CardContent, Link, IconButton, makeStyles } from '@material-ui/core';
import { red } from '@material-ui/core/colors';
import MoreVertIcon from '@material-ui/icons/MoreVert';
import { useMoralisQuery } from 'react-moralis';

const useStyles = makeStyles((theme) => ({
    root: {
        height: '60.4rem',
        paddingTop: '1rem',
    },
    loader: {
        textAlign: 'center',
        marginTop: '50%',
    },
    container: {
        marginTop: "5rem",
    },
    cardContainer: {
        display: 'flex',
        padding: '2rem 6rem',
        overflow: 'hidden',
    },
    cardRoot: {
        width: '75%',
    },
    carouselItem: {
        backgroundColor: '#3F50B5',
        padding: '88px 140px',
    },
    media: {
        height: 0,
        paddingTop: '56.25%', // 16:9
    },
    expand: {
        transform: 'rotate(0deg)',
        marginLeft: 'auto',
        transition: theme.transitions.create('transform', {
            duration: theme.transitions.duration.shortest,
        }),
    },
    expandOpen: {
        transform: 'rotate(180deg)',
    },
    avatar: {
        backgroundColor: red[500],
    },
}))

const Auction = () => {
    const classes = useStyles();
    const dispatch = useDispatch();
    const history = useHistory();

    const { data, } = useMoralisQuery(
        "Nft",
        query =>
            query.equalTo("IsSelled", true),
        [],

    );

    const auctionDetailHandler = (auctionDetail) => {
        dispatch({
            type: "MOVE_AUCTIONDETAIL",
            nftDetail: auctionDetail,
        });
        history.push("./auctionDetail");
    };

    return (
        <div style={{ height: '60.4rem' }}>
            <Typography component="h1" variant="h3" gutterBottom style={{ padding: '5px' }}>
                Welcome to Auction
            </Typography>
            <div className={classes.cardContainer}>
                {data.map((d) => {
                    return (
                        <div key={d.attributes["TokenId"]}>
                            <Card className={classes.cardRoot} >
                                <CardHeader
                                    avatar={
                                        <Avatar aria-label="recipe" className={classes.avatar}>
                                            {d.attributes["Account"]}
                                        </Avatar>
                                    }
                                    action={
                                        <IconButton aria-label="settings" onClick={() => auctionDetailHandler(d)}>
                                            <MoreVertIcon />
                                        </IconButton>
                                    }
                                    title={d.attributes["Name"]}
                                />
                                <CardMedia
                                    className={classes.media}
                                    image={d.attributes["IpfsUrl"]}
                                    title="Paella dish"
                                />
                                <CardContent>
                                    <Typography variant="body2" color="textSecondary" component="p">
                                        IPFS Hash: <br />{d.attributes["IpfsHash"]}
                                    </Typography>
                                    <Typography variant="body2" color="textSecondary" component="p">
                                        IPFS URL: <br />
                                        <Link href={d.attributes["IpfsUrl"]} target="_blank">
                                            {d.attributes["IpfsUrl"]}
                                        </Link>
                                    </Typography>
                                    <Typography variant="body2" color="textSecondary" component="p">
                                        Token ID: {d.attributes["TokenId"]}
                                    </Typography>
                                    {
                                        d.attributes["IsSelled"] ?
                                            <Typography variant="body2" color="textSecondary" component="p">
                                                Sale
                                            </Typography> :
                                            <Typography variant="body2" color="textSecondary" component="p">
                                                Not Sale
                                            </Typography>
                                    }
                                </CardContent>
                            </Card>
                        </div>
                    )
                })}
            </div>
        </div>
    );
}

export default Auction;
