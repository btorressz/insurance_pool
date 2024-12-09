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

# Instructions

## 1. Initialize Pool

**initialize_pool**

- Initializes the insurance reserve pool.
- Sets the governance authority.
- Establishes initial values for reserves and counters.

---

## 2. Purchase Insurance

**purchase_insurance**

- Allows users to purchase a new insurance policy.
- Calculates premiums based on various parameters, including risk level and coverage type.
- Allocates premiums to the respective risk reserve pool.
- Tracks referral bonuses for incentivizing onboarding.

---

## 3. Renew Policy

**renew_policy**

- Enables users to renew an existing policy.
- Applies a discount to the premium during renewal.
- Extends the policy's expiration time by a specified duration.

---

## 4. Rebalance Reserves

**rebalance_reserves**

- Rebalances funds across different risk pools to ensure stability.
- Redistributes excess funds from low-risk reserves to medium and high-risk pools.

---

## Enumerations

### Policy Types

**PolicyType**
- Flight Delay
- Crop Failure
- Property Damage

### Coverage Levels

**CoverageLevel**
- Basic
- Premium
- Platinum

### Risk Levels

**RiskLevel**
- Low
- Medium
- High

---

## Events

**PolicyPurchased**

- Records details of purchased policies.
- Logs the policyholder, premium amount, and policy type.

---

## Error Codes

**MyError**
- InvalidEvent: Oracle data is invalid.
- InsufficientFunds: Reserve pool lacks sufficient funds.
- Unauthorized: User lacks required permissions.
- OperationPaused: Operations are paused.
- PolicyNotRenewable: Policy cannot be renewed at this time.

---

##  Workflows

### Premium Calculation

**calculate_premium_with_coverage**

- Premiums are adjusted based on policy type, coverage level, and total policies.
- Early-bird discounts are applied for pools with fewer than 100 policies.

### Reserve Rebalancing

**rebalance_reserves**

- Ensures balanced allocation of funds to minimize risks.
- Dynamically shifts excess funds to stabilize the pool.

---

## Future Enhancements

- Expanding support for additional policy types.
- Introducing dynamic pricing models through external data oracles.
- Providing staking rewards for reserve pool contributions.



