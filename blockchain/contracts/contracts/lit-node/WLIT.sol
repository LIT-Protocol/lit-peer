//SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.17;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

contract WLIT is Ownable {
    string public name; // eg. "Wrapped Lit";
    string public symbol; // eg. "WLIT";
    uint8 public decimals = 18;
    address public trustedForwarder;

    event Approval(address indexed src, address indexed guy, uint wad);
    event Transfer(address indexed src, address indexed dst, uint wad);
    event Deposit(address indexed dst, uint wad);
    event Withdrawal(address indexed src, uint wad);

    mapping(address => uint) public balanceOf;
    mapping(address => mapping(address => uint)) public allowance;

    error InsufficientBalance();
    error InsufficientAllowance();

    constructor(string memory name_, string memory symbol_) {
        name = name_;
        symbol = symbol_;
    }

    fallback() external payable {
        deposit();
    }

    receive() external payable {
        deposit();
    }

    function getBalanceOf(address account) public view returns (uint) {
        return balanceOf[account];
    }

    function deposit() public payable {
        balanceOf[_erc2771MsgSender()] += msg.value;
        emit Deposit(_erc2771MsgSender(), msg.value);
    }

    function withdraw(uint wad) public {
        if (balanceOf[_erc2771MsgSender()] < wad) {
            revert InsufficientBalance();
        }
        balanceOf[_erc2771MsgSender()] -= wad;
        payable(_erc2771MsgSender()).transfer(wad);
        emit Withdrawal(_erc2771MsgSender(), wad);
    }

    function totalSupply() public view returns (uint) {
        return address(this).balance;
    }

    function approve(address guy, uint wad) public returns (bool) {
        allowance[_erc2771MsgSender()][guy] = wad;
        emit Approval(_erc2771MsgSender(), guy, wad);
        return true;
    }

    function transfer(address dst, uint wad) public returns (bool) {
        return transferFrom(_erc2771MsgSender(), dst, wad);
    }

    function transferFrom(
        address src,
        address dst,
        uint wad
    ) public returns (bool) {
        if (balanceOf[src] < wad) {
            revert InsufficientBalance();
        }

        if (
            src != _erc2771MsgSender() &&
            allowance[src][_erc2771MsgSender()] != type(uint256).max
        ) {
            if (allowance[src][_erc2771MsgSender()] < wad) {
                revert InsufficientAllowance();
            }
            allowance[src][_erc2771MsgSender()] -= wad;
        }

        balanceOf[src] -= wad;
        balanceOf[dst] += wad;

        emit Transfer(src, dst, wad);

        return true;
    }

    function burn(uint256 amount) public virtual {
        transferFrom(_erc2771MsgSender(), address(0), amount);
    }

    function burnFrom(address account, uint256 amount) public virtual {
        transferFrom(account, address(0), amount);
    }

    // EIP2771 functions

    function setTrustedForwarder(address forwarder) public onlyOwner {
        trustedForwarder = forwarder;
    }

    function getTrustedForwarder() public view returns (address) {
        return trustedForwarder;
    }

    function _erc2771MsgSender() internal view returns (address sender) {
        if (msg.sender == trustedForwarder && msg.data.length >= 20) {
            assembly {
                sender := shr(96, calldataload(sub(calldatasize(), 20)))
            }
        } else {
            sender = msg.sender;
        }
    }
}
