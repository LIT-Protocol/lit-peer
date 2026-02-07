// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// import enumerableSet from openzeppelin
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

// Because we're not storing funds, do we need to use ReentrancyGuard?
// import { ReentrancyGuard } from "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/**
 * Storage for GetQuoteRequest data (mirrors GetQuoteRequest in swaps/models.rs).
 * Optional fields use empty string or 0 when not set.
 */
contract QuoteStorage {
    
    using EnumerableSet for EnumerableSet.AddressSet;
    using EnumerableSet for EnumerableSet.UintSet;

    enum QuotePricingType {
        Origin,      // Destination amount fixed; origin amount increases
        Destination  // Origin fixed; destination amount reduced
    }

    struct QuoteRequest {
        address from;                      // Message sender (origin of the swap)
        string originSymbol;
        string originChain;
        uint256 originAmount;
        string destinationSymbol;
        string destinationChain;
        uint256 destinationAmount;
        uint256 slippage;                  // Acceptable pricing slippage
        QuotePricingType pricingType;     // Where fees are taken from
        uint256 quoteDeadlineSeconds;       // 0–60 seconds
        address originAddress;             
        address refundAddress;             
        uint256 transactionDeadlineSeconds; // Optional; 0 if not set
        string message;                   
    }

    struct Quote {
        address pkpAddress;
        mapping(uint256 => QuoteRequest) request;
        address providerRefundAddress;
        uint256 quoteExpiry;
        uint256 createdAt;
        uint256 feesTotal;        
    }

    EnumerableSet.UintSet openQuotes;
    EnumerableSet.UintSet openQuoteRequests;
    mapping(uint256 => Quote) quotes;
    mapping(uint256 => QuoteRequest) quoteRequests;
    mapping(address => Quote) quoteByPkpAddress;
    mapping(address => QuoteRequest) lastQuoteRequestFromAddress;

    uint256 public quoteRequestCounter;
    uint256 public quoteCounter;

    constructor() {
        quoteRequestCounter = 0;
        quoteCounter = 0;
    }

    function newQuoteRequest(QuoteRequest calldata data) external returns (uint256) {
        quoteRequestCounter++;
        quoteRequests[quoteRequestCounter] = data;
        openQuoteRequests.add(quoteRequestCounter);
        lastQuoteRequestFromAddress[data.from] = quoteRequests[quoteRequestCounter];
        emit NewQuoteRequest(quoteRequestCounter);
        return quoteRequestCounter;
    }

    function newQuote(uint256 quoteRequestId, address providerRefundAddress) external returns (uint256) {
        quoteCounter++;
        quotes[quoteCounter].pkpAddress = quoteRequests[quoteRequestId].from;
        quotes[quoteCounter].providerRefundAddress = providerRefundAddress;
        quotes[quoteCounter].quoteExpiry = block.timestamp + quoteRequests[quoteRequestId].quoteDeadlineSeconds;
        quotes[quoteCounter].createdAt = block.timestamp;
        quotes[quoteCounter].feesTotal = 0;
    
        openQuotes.add(quoteCounter);
        return quoteCounter;
    }

    event NewQuoteRequest(
        uint256 quoteRequestId
    );




    function getQuoteRequest(uint256 quoteRequestId) external view returns (QuoteRequest memory) {
        return quoteRequests[quoteRequestId];
    }
}
