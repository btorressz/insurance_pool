# insurance_pool


## Overview

 **Insurance Pool Program** is a decentralized insurance management system built using Solana's Anchor framework. It provides functionality for creating and managing an insurance reserve pool, purchasing insurance policies, renewing policies, and rebalancing reserves between risk levels. The program is designed to manage multiple insurance types, policy durations, and risk levels while maintaining efficient reserve allocation.
 
**NOTE: This project is a prototype foe fhe future possiblites of decentralized insurance** 

---

## Features

### Reserve Pool Initialization
- Sets up the insurance reserve pool with risk categories and initial values.
- Tracks governance, policy counts, and pause state for operations.

### Policy Management
- Purchase insurance with customizable premiums based on risk level, coverage type, and policy duration.
- Reward referral bonuses for introducing new policyholders.
- Support for renewing policies with a discount.

### Reserve Management
- Allocate premiums to different risk pools (low, medium, high).
- Rebalance reserves dynamically based on pool utilization thresholds.

### Event-Driven Architecture
- Emits events such as `PolicyPurchased` for integrations and analytics.

---

## Accounts

### **ReservePool**
Stores the state of the insurance reserve pool:
- **low_risk_reserves, medium_risk_reserves, high_risk_reserves**: Balances for each risk pool.
- **total_policies**: Total number of policies issued.
- **policy_counts**: Breakdown of policy counts by type.
- **governance**: Public key of the governance authority.
- **is_paused**: Indicates whether operations are paused.

### **Policy**
Tracks the details of an insurance policy:
- **user**: Public key of the policyholder.
- **policy_type**: Type of insurance policy (e.g., flight delay, crop failure).
- **coverage**: Coverage level (Basic, Premium, Platinum).
- **purchase_time**: Timestamp of policy purchase.
- **expiry_time**: Timestamp when the policy expires.
- **referrer**: Optional public key of the referring user.

---



