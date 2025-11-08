import { SignerWithAddress } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import { Forwarder, WLIT } from '../../typechain-types';
import { sendMetaTransaction } from '../../utils/contract';

describe('WLIT', function () {
  let signers: SignerWithAddress[];
  let contract: WLIT;
  let forwarder: Forwarder;
  let deployer: SignerWithAddress;
  let user1: SignerWithAddress;
  let user2: SignerWithAddress;
  let user3: SignerWithAddress;
  const INITIAL_NAME = 'Wrapped Lit';
  const INITIAL_SYMBOL = 'WLIT';

  beforeEach(async () => {
    signers = await ethers.getSigners();
    [deployer, user1, user2, user3, ...signers] = signers;
    contract = await ethers.deployContract('WLIT', [
      INITIAL_NAME,
      INITIAL_SYMBOL,
    ]);
    forwarder = await ethers.deployContract('Forwarder');

    await contract.setTrustedForwarder(forwarder.target);
  });

  describe('Basic Properties', function () {
    it('should have correct name and symbol', async function () {
      expect(await contract.name()).to.equal(INITIAL_NAME);
      expect(await contract.symbol()).to.equal(INITIAL_SYMBOL);
      expect(await contract.decimals()).to.equal(18);
    });
  });

  describe('Deposit and Withdraw', function () {
    const depositAmount = ethers.parseEther('1.0');

    it('should allow deposits via fallback and receive functions', async function () {
      // Test fallback function
      await user1.sendTransaction({
        to: contract.target,
        value: depositAmount,
      });
      expect(await contract.balanceOf(user1.address)).to.equal(depositAmount);

      // Test receive function
      await user2.sendTransaction({
        to: contract.target,
        value: depositAmount,
      });
      expect(await contract.balanceOf(user2.address)).to.equal(depositAmount);
    });

    it('should allow deposits via deposit function', async function () {
      await contract.connect(user1).deposit({ value: depositAmount });
      expect(await contract.balanceOf(user1.address)).to.equal(depositAmount);
    });

    it('should allow withdrawals', async function () {
      // First deposit
      await contract.connect(user1).deposit({ value: depositAmount });

      // Then withdraw
      const initialBalance = await ethers.provider.getBalance(user1.address);
      const tx = await contract.connect(user1).withdraw(depositAmount);
      const receipt = await tx.wait();
      const gasUsed = BigInt(receipt!.gasUsed) * receipt!.gasPrice;

      const finalBalance = await ethers.provider.getBalance(user1.address);
      expect(await contract.balanceOf(user1.address)).to.equal(0);
      expect(finalBalance).to.equal(initialBalance + depositAmount - gasUsed);
    });

    it('should fail withdrawal with insufficient balance', async function () {
      await expect(
        contract.connect(user1).withdraw(depositAmount)
      ).to.be.revertedWithCustomError(contract, 'InsufficientBalance');
    });

    it('should allow withdrawals with EIP2771', async function () {
      // First get the balance of user1 before the deposit
      const user3BalanceBefore = await ethers.provider.getBalance(
        user3.address
      );

      // First deposit using 2771
      let txData = await contract.connect(user3).deposit.populateTransaction({
        value: depositAmount,
      });

      await sendMetaTransaction(
        ethers,
        txData,
        user3,
        forwarder,
        contract.target.toString()
      );

      // Assert token balance is depositAmount
      expect(await contract.getBalanceOf(user3.address)).to.equal(
        depositAmount
      );

      // Then withdraw using 2771
      txData = await contract
        .connect(user3)
        .withdraw.populateTransaction(depositAmount);
      await sendMetaTransaction(
        ethers,
        txData,
        user3,
        forwarder,
        contract.target.toString(),
        {
          checkMetaTransactionSignerBalance: false,
        }
      );

      // Assert gas balance of user3 is user3BalanceBefore + depositAmount, and token balance is 0
      expect(await ethers.provider.getBalance(user3.address)).to.equal(
        user3BalanceBefore + depositAmount
      );
      expect(await contract.balanceOf(user3.address)).to.equal(0);
    });
  });

  describe('Transfers and Allowances', function () {
    const transferAmount = ethers.parseEther('1.0');

    beforeEach(async function () {
      // Setup initial balance for user1
      await contract.connect(user1).deposit({ value: transferAmount });
    });

    it('should allow direct transfers', async function () {
      await contract.connect(user1).transfer(user2.address, transferAmount);
      expect(await contract.balanceOf(user1.address)).to.equal(0);
      expect(await contract.balanceOf(user2.address)).to.equal(transferAmount);
    });

    it('should handle approvals and transferFrom', async function () {
      // Approve user2 to spend user1's tokens
      await contract.connect(user1).approve(user2.address, transferAmount);
      expect(await contract.allowance(user1.address, user2.address)).to.equal(
        transferAmount
      );

      // Transfer from user1 to user2 using user2's account
      await contract
        .connect(user2)
        .transferFrom(user1.address, user2.address, transferAmount);
      expect(await contract.balanceOf(user1.address)).to.equal(0);
      expect(await contract.balanceOf(user2.address)).to.equal(transferAmount);
      expect(await contract.allowance(user1.address, user2.address)).to.equal(
        0
      );
    });

    it('should fail transferFrom with insufficient allowance', async function () {
      await expect(
        contract
          .connect(user2)
          .transferFrom(user1.address, user2.address, transferAmount)
      ).to.be.revertedWithCustomError(contract, 'InsufficientAllowance');
    });

    it('should handle infinite allowance', async function () {
      // User 1 deposits twice
      await contract.connect(user1).deposit({ value: transferAmount });
      await contract.connect(user1).deposit({ value: transferAmount });

      // Approve infinite allowance
      await contract.connect(user1).approve(user2.address, ethers.MaxUint256);

      // Should be able to transfer multiple times
      await contract
        .connect(user2)
        .transferFrom(user1.address, user2.address, transferAmount);
      await contract
        .connect(user2)
        .transferFrom(user1.address, user2.address, transferAmount);

      expect(await contract.allowance(user1.address, user2.address)).to.equal(
        ethers.MaxUint256
      );
    });
  });

  describe('Burning', function () {
    const burnAmount = ethers.parseEther('1.0');

    beforeEach(async function () {
      // Setup initial balance for user1
      await contract.connect(user1).deposit({ value: burnAmount });
    });

    it('should allow burning tokens', async function () {
      await contract.connect(user1).burn(burnAmount);
      expect(await contract.balanceOf(user1.address)).to.equal(0);
      expect(await contract.balanceOf(ethers.ZeroAddress)).to.equal(burnAmount);
    });

    it('should allow burning tokens from another address with allowance', async function () {
      await contract.connect(user1).approve(user2.address, burnAmount);
      await contract.connect(user2).burnFrom(user1.address, burnAmount);
      expect(await contract.balanceOf(user1.address)).to.equal(0);
      expect(await contract.balanceOf(ethers.ZeroAddress)).to.equal(burnAmount);
    });

    it('should fail burning with insufficient balance', async function () {
      await expect(
        contract.connect(user2).burn(burnAmount)
      ).to.be.revertedWithCustomError(contract, 'InsufficientBalance');
    });
  });

  describe('Total Supply', function () {
    it('should return correct total supply', async function () {
      const depositAmount = ethers.parseEther('1.0');
      await contract.connect(user1).deposit({ value: depositAmount });
      expect(await contract.totalSupply()).to.equal(depositAmount);
    });
  });
});
