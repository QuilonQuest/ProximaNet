// SPDX-License-Identifier: Apache License 2.0

pragma solidity >=0.6.0 <0.8.0;
pragma experimental ABIEncoderV2;

import "./Context.sol";
import "./SafeMath.sol";
import "./Address.sol";
import "./ReentrancyGuard.sol";
import "./mimc7.sol";
import "./IERC165.sol";
import "./IERC721.sol";
import "./IERC721TokenReceiver.sol";

contract SiddhiToken is IERC721, ReentrancyGuard{
    string public name = "Siddhi NFT Token";
    string public symbol = "SIDDHI";

    uint256 private _totalTokens = 0;
    mapping(uint256 => address) private tokenToOwner;
    mapping(address => uint256) private tokensByOwner;
    mapping(address => mapping(address => bool)) private authOperators;
    mapping(uint256 => address) private approvedAddresses;
    mapping(uint256 => Ticket) private tickets;

    struct Ticket {
        string name;
        uint256 adn;
        uint256 points;
        bytes data;
    }

    constructor() {
        _totalTokens++;
        tokensByOwner[address(this)]++;
        tickets[_totalTokens] = Ticket(
            "GOLD TOKEN",
            _totalTokens,
            10000,
            "KILLER"
        );
        tokenToOwner[_totalTokens] = address(this);
    }

    function isContract(address addr) private view returns (bool) {
        uint256 size;
        assembly {
            size := extcodesize(addr)
        }
        return size > 0;
    }

    function balanceOf(address _owner)
        external
        view
        override
        returns (uint256)
    {
        require(_owner != address(0));
        return tokensByOwner[_owner];
    }

    function ownerOf(uint256 _tokenId)
        external
        view
        override
        returns (address)
    {
        address _tokenOwner = tokenToOwner[_tokenId];
        require(_tokenOwner != address(0));
        return _tokenOwner;
    }

    function _transfer(
        address _from,
        address _to,
        uint256 _tokenId
    ) private {
        require(
            msg.sender == tokenToOwner[_tokenId] ||
                authOperators[tokenToOwner[_tokenId]][msg.sender] ||
                msg.sender == approvedAddresses[_tokenId]
        );
        approvedAddresses[_tokenId] = address(0);

        require(_from == tokenToOwner[_tokenId]);
        require(_to != address(0));

        // Require NFT to be a valid ID
        require(_tokenId > 0 && _tokenId <= _totalTokens);

        // Transafer (overflow issue to solve with safeMath)
        tokensByOwner[_to]++;
        tokensByOwner[_from]--;
        tokenToOwner[_tokenId] = _to;

        emit Transfer(_from, _to, _tokenId);
    }

    function safeTransferFrom(
        address _from,
        address _to,
        uint256 _tokenId,
        bytes memory data
    ) external payable override {
        _transfer(_from, _to, _tokenId);
        if (isContract(_to)) {
            require(
                IERC721TokenReceiver(_to).onERC721Received(
                    msg.sender,
                    _from,
                    _tokenId,
                    data
                ) == this.onERC721Received(msg.sender, _from, _tokenId, data)
            );
        }
    }

    function safeTransferFrom(
        address _from,
        address _to,
        uint256 _tokenId
    ) external payable override {
        this.safeTransferFrom(_from, _to, _tokenId, "");
    }

    function transferFrom(
        address _from,
        address _to,
        uint256 _tokenId
    ) external payable override {
        _transfer(_from, _to, _tokenId);
    }

    function approve(address _approved, uint256 _tokenId)
        external
        payable
        override
    {
        require(
            msg.sender == tokenToOwner[_tokenId] ||
                authOperators[tokenToOwner[_tokenId]][msg.sender]
        );
        approvedAddresses[_tokenId] = _approved;

        emit Approval(msg.sender, _approved, _tokenId);
    }

    function setApprovalForAll(address _operator, bool _approved)
        external
        override
    {
        authOperators[msg.sender][_operator] = _approved;

        emit ApprovalForAll(msg.sender, _operator, _approved);
    }

    function getApproved(uint256 _tokenId)
        external
        view
        override
        returns (address)
    {
        return approvedAddresses[_tokenId];
    }

    function isApprovedForAll(address _owner, address _operator)
        external
        view
        override
        returns (bool)
    {
        return authOperators[_owner][_operator] || false;
    }

    function onERC721Received(
        address _operator,
        address _from,
        uint256 _tokenId,
        bytes memory _data
    ) external pure returns (bytes4) {
        return
            bytes4(
                keccak256(
                    abi.encodePacked(
                        "onERC721Received(",
                        _operator,
                        _from,
                        _tokenId,
                        _data,
                        ")"
                    )
                )
            );
    }
}
