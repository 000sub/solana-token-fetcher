# solana-token-fetcher

### Delivers Brief Summary of Solana Mainnet Tokens

<hr/>

# What This Service Provides:

- ### Token Metadata
- ### Token Extensions
- ### Token Information
  - #### Mint Authority
  - #### Total Supply
  - #### etc
- ### Top 10 Holders (descending order based on their percentage of total supply)

<hr/>

# How to Setup

To ensure proper functionality, you need to enter your private RPC URI in `.env` file at root directory. You can get a free RPC URI for Solana mainnet from [helius](https://dev.helius.xyz).


<hr/>

# Important Note

This service does not guarantee proper functionality for all SPL20 tokens.

<hr/>

# API Info

## GET /{mintAddress}

### Read Summary of Token

- HTTP Request
```
http://localhost:3000/EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm
```
- HTTP Response
```
200 OK 5.61s
{
    "mint": "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm",
    "metadata": {
        "update_authority": "wifq4CRwpXCK8NYtKNsQAYoDethT1aR7R1DaKCLFgAd",
        "mint": "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm",
        "name": "dogwifhat",
        "symbol": "$WIF",
        "uri": "https://bafkreihwqhounu3cdwgvk2gc2dqcinpntlccbo3xcy4xuerd24yndldl5q.ipfs.nftstorage.link",
        "seller_fee_basis_points": 0,
        "is_mutable": false
    },
    "extensions": [],
    "info": {
        "mint_authority": "null",
        "supply": 998845775896289,
        "decimals": 6,
        "is_initialized": true,
        "freeze_authority": "null"
    },
    "top_holders": [
        {
            "address": "7XX64f8UKE1nxNCSwncCUj3c2FJYdXHmDV5xd5DRdFky",
            "owner_address": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
            "amount": "191092953821000",
            "ui_amount": 191092953.821,
            "ui_amount_string": "191092953.821",
            "percentage": 19.13137727889249
        },
        {
            "address": "1LLLyMZvAdimbcsC4Xp7GVWiJScztEpfXVR9upyxq5G",
            "owner_address": "AC5RDfQFmDS1deWZos921JfqscXdByf8BKHs5ACWjtW2",
            "amount": "25365340173894",
            "ui_amount": 25365340.173894,
            "ui_amount_string": "25365340.173894",
            "percentage": 2.5394651292521164
        },
        {
            "address": "6JAQy4JniKoFbPLjSVkb72NaJjfxs46tM5LETLrqhTRn",
            "owner_address": "6wbUfPPEAnrnUNud3tssxaSswaPV9qNMUpEy8rERWXZg",
            "amount": "23391805393516",
            "ui_amount": 23391805.393516,
            "ui_amount_string": "23391805.393516",
            "percentage": 2.341883597848322
        },
        {
            "address": "EHDnHqX6PtnSdPfsnnr1oWtmQbenDwqZAKUo5QAq9Ro3",
            "owner_address": "81BgcfZuZf9bESLvw3zDkh7cZmMtDwTPgkCvYu7zx26o",
            "amount": "20447801261728",
            "ui_amount": 20447801.261728,
            "ui_amount_string": "20447801.261728",
            "percentage": 2.047142987953239
        },
        {
            "address": "DsxpGaCZ13aUfDDz2CSjuzCA8XJJYLuGsV65KXdyLWdW",
            "owner_address": "FWznbcNXWQuHTawe9RxvQ2LdCENssh12dsznf4RiouN5",
            "amount": "17199550497840",
            "ui_amount": 17199550.49784,
            "ui_amount_string": "17199550.49784",
            "percentage": 1.721942557388944
        },
        {
            "address": "EdhM4qGK29P2X1GsRYZtF8E5v9rkd52NRXEh1d8BzUc8",
            "owner_address": "5tzFkiKscXHK5ZXCGbXZxdw7gTjjD1mBwuoFbhUvuAi9",
            "amount": "16388037554551",
            "ui_amount": 16388037.554551,
            "ui_amount_string": "16388037.554551",
            "percentage": 1.6406974880426968
        },
        {
            "address": "4kT3EXc5dDVndUU9mV6EH3Jh3CSEvpcCZjuMkwqrtxUy",
            "owner_address": "9gNrvvZ9RuTyRWooiEEypwcXU6kyXW8yWuhXU8tWUH5L",
            "amount": "15906281382399",
            "ui_amount": 15906281.382399,
            "ui_amount_string": "15906281.382399",
            "percentage": 1.5924662011135702
        },
        {
            "address": "Fm7yBTcuUDBdzj2PbP6tr8uiFAJwS53kNQbwEERHP65A",
            "owner_address": "9un5wqE3q4oCjyrDkwsdD48KteCJitQX5978Vh7KKxHo",
            "amount": "12353742792009",
            "ui_amount": 12353742.792009,
            "ui_amount_string": "12353742.792009",
            "percentage": 1.2368018256796132
        },
        {
            "address": "GqzXwoPv8TJtxqDDB1G3Q35MQpor6433fdB18vdNYfqC",
            "owner_address": "ADaENsRth44fqKE9vwXwQjjiNr6ZLd2jVTEkCp8hfRcL",
            "amount": "12257866015003",
            "ui_amount": 12257866.015003,
            "ui_amount_string": "12257866.015003",
            "percentage": 1.2272030688625293
        },
        {
            "address": "APudUTTfi8WfcwNPoLLeJYpSZdCmJKt8pCKH7hEjPzeu",
            "owner_address": "2h8JJq1kAsJvKYVrsEqwhQTcy99p465esHUFcJA94QY2",
            "amount": "10466530245160",
            "ui_amount": 10466530.24516,
            "ui_amount_string": "10466530.24516",
            "percentage": 1.0478624926624058
        }
    ]
}
```

"### Note from contributor\nImproved documentation for new contributors." 
