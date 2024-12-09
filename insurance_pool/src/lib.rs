use anchor_lang::prelude::*;

declare_id!("9rgwCLaTUbT79wiXQ4dgqrKKnPD31tLQHw3oUqMUnovP");

#[program]
pub mod insurance_pool {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        let pool = &mut ctx.accounts.reserve_pool;
        pool.low_risk_reserves = 0;
        pool.medium_risk_reserves = 0;
        pool.high_risk_reserves = 0;
        pool.total_policies = 0;
        pool.policy_counts = [0; 3];
        pool.is_paused = false;
        pool.governance = ctx.accounts.user.key();
        Ok(())
    }

    pub fn purchase_insurance(
        ctx: Context<PurchaseInsurance>, 
        base_rate: u64, 
        policy_type: PolicyType, 
        coverage: CoverageLevel, 
        duration: i64, 
        risk: RiskLevel, 
        referrer: Option<Pubkey>,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.reserve_pool;
        require!(!pool.is_paused, MyError::OperationPaused);

        let premium = apply_early_bird_discount(
            calculate_premium_with_coverage(pool.total_policies, base_rate, coverage),
            pool.total_policies,
        );
        allocate_to_risk_pool(pool, premium, risk);

        pool.policy_counts[policy_type as usize] += 1;
        pool.total_policies += 1;

        // Initialize policy
        let policy = &mut ctx.accounts.policy;
        let current_time = Clock::get()?.unix_timestamp;
        policy.user = ctx.accounts.user.key();
        policy.policy_type = policy_type;
        policy.coverage = coverage;
        policy.purchase_time = current_time;
        policy.expiry_time = current_time + duration;
        policy.referrer = referrer;

        // Transfer premium payment
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.reserve_pool.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        anchor_lang::system_program::transfer(cpi_ctx, premium)?;

        // Reward referrer if applicable
        if let Some(referrer) = referrer {
            msg!("Rewarding referrer {:?}", referrer);
            // Add future reward logic here
        }

        // Emit leaderboard event
        emit!(PolicyPurchased {
            user: ctx.accounts.user.key(),
            premium,
            policy_type,
        });

        Ok(())
    }

    pub fn renew_policy(ctx: Context<RenewPolicy>, duration: i64, renewal_discount: u64) -> Result<()> {
        let policy = &mut ctx.accounts.policy;
        let pool = &mut ctx.accounts.reserve_pool;

        let current_time = Clock::get()?.unix_timestamp;
        require!(policy.expiry_time >= current_time - 7 * 24 * 60 * 60, MyError::PolicyNotRenewable);

        let base_premium = calculate_premium_with_coverage(pool.total_policies, renewal_discount, policy.coverage);

        policy.expiry_time += duration;

        // Transfer renewal payment logic here
        Ok(())
    }

    pub fn rebalance_reserves(ctx: Context<RebalanceReserves>) -> Result<()> {
        let pool = &mut ctx.accounts.reserve_pool;

        if pool.low_risk_reserves > 100_000 {
            let excess = pool.low_risk_reserves - 100_000;
            pool.low_risk_reserves -= excess;
            pool.medium_risk_reserves += excess / 2;
            pool.high_risk_reserves += excess / 2;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = user, space = 8 + 48 + 32 + 24, seeds = [b"reserve_pool"], bump)]
    pub reserve_pool: Account<'info, ReservePool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseInsurance<'info> {
    #[account(mut, seeds = [b"reserve_pool"], bump)]
    pub reserve_pool: Account<'info, ReservePool>,
    #[account(init, payer = user, space = 8 + 72, seeds = [b"policy", user.key().as_ref()], bump)]
    pub policy: Account<'info, Policy>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RenewPolicy<'info> {
    #[account(mut, has_one = user, seeds = [b"policy", user.key().as_ref()], bump)]
    pub policy: Account<'info, Policy>,
    #[account(mut, seeds = [b"reserve_pool"], bump)]
    pub reserve_pool: Account<'info, ReservePool>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RebalanceReserves<'info> {
    #[account(mut, seeds = [b"reserve_pool"], bump)]
    pub reserve_pool: Account<'info, ReservePool>,
    pub governance: Signer<'info>,
}

#[account]
pub struct ReservePool {
    pub low_risk_reserves: u64,
    pub medium_risk_reserves: u64,
    pub high_risk_reserves: u64,
    pub total_policies: u64,
    pub policy_counts: [u64; 3],
    pub governance: Pubkey,
    pub is_paused: bool,
}

#[account]
pub struct Policy {
    pub user: Pubkey,
    pub policy_type: PolicyType,
    pub coverage: CoverageLevel,
    pub purchase_time: i64,
    pub expiry_time: i64,
    pub referrer: Option<Pubkey>,
}

#[event]
pub struct PolicyPurchased {
    pub user: Pubkey,
    pub premium: u64,
    pub policy_type: PolicyType,
}

#[error_code]
pub enum MyError {
    #[msg("Invalid event data from oracle")]
    InvalidEvent,
    #[msg("Insufficient funds in reserve pool")]
    InsufficientFunds,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Operation is currently paused")]
    OperationPaused,
    #[msg("Policy not renewable at this time")]
    PolicyNotRenewable,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum PolicyType {
    FlightDelay,
    CropFailure,
    PropertyDamage,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum CoverageLevel {
    Basic,
    Premium,
    Platinum,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

fn calculate_premium_with_coverage(total_policies: u64, base_rate: u64, coverage: CoverageLevel) -> u64 {
    let multiplier = match coverage {
        CoverageLevel::Basic => 1,
        CoverageLevel::Premium => 2,
        CoverageLevel::Platinum => 3,
    };
    // Integer-based approximation of square root
    let curve_factor = 1 + (total_policies as f64).sqrt().floor() as u64;
    base_rate * multiplier * curve_factor
}

fn apply_early_bird_discount(premium: u64, total_policies: u64) -> u64 {
    if total_policies < 100 {
        premium - (premium / 10)
    } else {
        premium
    }
}

fn allocate_to_risk_pool(pool: &mut ReservePool, amount: u64, risk: RiskLevel) {
    match risk {
        RiskLevel::Low => pool.low_risk_reserves += amount,
        RiskLevel::Medium => pool.medium_risk_reserves += amount,
        RiskLevel::High => pool.high_risk_reserves += amount,
    }
}