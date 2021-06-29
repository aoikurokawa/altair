import React, { useEffect, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { useMoralisQuery } from 'react-moralis';
import { Card, CardHeader, Avatar, IconButton, CardMedia, CardContent, Typography, CardActions, Collapse, Link, makeStyles, } from '@material-ui/core';
import MoreVertIcon from '@material-ui/icons/MoreVert';
import GavelIcon from '@material-ui/icons/Gavel';
import { red } from '@material-ui/core/colors';
import Skeleton from '@material-ui/lab/Skeleton';
import clsx from 'clsx';
import { Carousel } from 'react-bootstrap';
import { getToken } from '../actions/artTokenAction';
import { CircularProgress } from '@material-ui/core';
import { PulseLoader } from 'react-spinners';

const useStyles = makeStyles((theme) => ({
    root: {
        height: '60.4rem',
        paddingTop: '1rem',
        padding: '0rem 7rem',
    },
    loader: {
        textAlign: 'center',
        marginTop: '50%',
    },
    cardRoot: {

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
}));

const MyPage = () => {
    const classes = useStyles();
    const [arrayData, setArrayData] = useState([]);
    const dispatch = useDispatch();
    const { accounts } = useSelector((state) => state.artToken);
    const { data, error, isFetching } = useMoralisQuery("Nft", query => query.limit(3));

    useEffect(() => {
        const array = [];
        data.map((d) => {
            if (d.attributes["Account"] === accounts[0]) {
                if (d.attributes["IpfsUrl"] !== undefined) {
                    array.push(d);
                }
            }
        });
        setArrayData(array);
    }, [data]);

    console.log(data);

    const handleModal = () => {
        dispatch({
            type: "SHOW_MODAL",
            functionType: "MyPage",
            title: "Do you want to put up your work for auction?",
        })
    }

    return (
        <div className={classes.root}>
            {
                isFetching ?
                    (
                        <div className={classes.loader}>
                            <PulseLoader size="30" margin="2" color="#62DEC2" />
                        </div>
                    ) :
                    (
                        <>
                            <Typography align="center" component="h1" variant="h3">
                                My Page
                            </Typography>
                            <Carousel fade interval={null}>
                                {arrayData.map((d) => {
                                    return (
                                        <Carousel.Item className={classes.carouselItem} key={d.id}>
                                            <Card className={classes.cardRoot}>
                                                <CardHeader
                                                    avatar={
                                                        isFetching ? (
                                                            <Skeleton animation="wave" variant="circle" width={40} height={40} />
                                                        ) : (
                                                            <>
                                                                <Avatar aria-label="recipe" className={classes.avatar}>
                                                                    {d.attributes["Account"]}
                                                                </Avatar>
                                                            </>
                                                        )

                                                    }
                                                    action={
                                                        <IconButton aria-label="settings" onClick={handleModal}>
                                                            <GavelIcon />
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
                                                        IPFS Hash: {d.attributes["IpfsHash"]}
                                                    </Typography>
                                                    <Typography variant="body2" color="textSecondary" component="p">
                                                        IPFS URL:
                                                        <Link href={d.attributes["IpfsUrl"]} target="_blank">
                                                            {d.attributes["IpfsUrl"]}
                                                        </Link>
                                                    </Typography>
                                                    <Typography variant="body2" color="textSecondary" component="p">
                                                        Token ID: {d.attributes["TokenId"]}
                                                    </Typography>
                                                </CardContent>
                                            </Card>
                                        </Carousel.Item>
                                    )
                                })}
                            </Carousel>
                        </>
                    )
            }
        </div >

    );
}

export default MyPage;
