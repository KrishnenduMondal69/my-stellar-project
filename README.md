# my-stellar-project 
# Soroban Review System

A decentralized, transparent review and rating protocol built on Stellar's Soroban smart contract platform.

## Project Description
The **Soroban Review System** is a lightweight smart contract designed to bring trust and immutability to online feedback. By leveraging the Stellar blockchain, this system ensures that once a review is submitted, it cannot be tampered with, deleted, or censored by centralized entities. It serves as a backend foundation for decentralized Yelp-style applications, e-commerce feedback loops, or service provider ratings.

## What it does
The contract acts as a permanent ledger for consumer feedback. 
1. **Submit Reviews**: Users sign transactions with their Stellar identity to post a numerical rating (0–5) and a text-based comment for any unique identifier (Symbol).
2. **Retrieve Feedback**: Anyone can query the contract to view the full history of reviews for a specific subject.
3. **Identity Verification**: It uses Soroban's native authentication to ensure that the reviewer’s address is verified, preventing identity spoofing.

## Features
* **Immutable Storage**: Reviews are stored in Stellar's persistent storage, making them censorship-resistant.
* **Rating Validation**: Built-in logic to ensure ratings remain within a valid range ($0 \leq \text{rating} \leq 5$).
* **Low Latency/Low Cost**: Built on Stellar, making it significantly cheaper and faster than similar systems on Ethereum.
* **Flexible Subject Tagging**: Use unique `Symbols` to categorize reviews for anything—from physical products to individual wallet addresses.

## Technical Math
The average rating for any subject can be calculated by iterating through the stored `Vec<Review>` on the frontend or via a helper function using the formula:

$$\text{Average} = \frac{\sum_{i=1}^{n} \text{rating}_i}{n}$$

Where $n$ is the total number of reviews for that specific subject.

https://stellar.expert/explorer/testnet/contract/CA74OFM47TO5R4L4KE26FQAQGP4PNGBBLX7FMWQXL6QXBNLMA7PYEDBT
<img width="1916" height="1019" alt="image" src="https://github.com/user-attachments/assets/db17027e-25c8-44b7-ad52-9624657d89b1" />
