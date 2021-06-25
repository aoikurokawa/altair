pragma solidity 0.8.0;

import '../client/node_modules/@openzeppelin/contracts/token/ERC721/ERC721.sol';
import '../client/node_modules/@openzeppelin/contracts/access/Ownable.sol';

contract ArtToken is ERC721, Ownable {
    
    struct Art {
        string link;        
    }

    uint256 nextId = 0;

    mapping(uint256 => Art) private _tokenDetails;

    constructor(string memory name, string memory symbol) ERC721(name, symbol){

    }

    function getTokenDetail(uint256 _tokenId) public view returns(Art memory) {
        return _tokenDetails[_tokenId];
    }

    function mint(string memory _link) public onlyOwner {
        _tokenDetails[nextId] = Art(_link);
        _safeMint(msg.sender, nextId);
        nextId++;
    } 




}

