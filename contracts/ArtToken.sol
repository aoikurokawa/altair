pragma solidity 0.8.0;

import '../client/node_modules/@openzeppelin/contracts/token/ERC721/ERC721.sol';
import '../client/node_modules/@openzeppelin/contracts/access/Ownable.sol';

contract ArtToken is ERC721, Ownable {
    
    struct Art {
        string link;        
    }

    constructor(string memory name, string memory symbol) ERC721(name, symbol){

    }

    function mint(string memory _link) public onlyOwner {
        
    } 
}

