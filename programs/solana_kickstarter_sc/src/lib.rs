use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("4dm6DpP41wiFUgFy8FHrGACtRTF9mBjfTTx3XS2ap8xt");

#[program]
pub mod solana_kickstarter_sc {

    use super::*;

    //The initial method for this smart contract
    pub fn create(ctx: Context<Create>, name: String, description: String) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;

        // Initialize the values of the campaign account
        campaign.name = name;
        campaign.description = description;
        campaign.amount_donated = 0;
        campaign.admin = *ctx.accounts.user.key;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign; // Get the campaign account from the Withdraw account
        let user = &mut ctx.accounts.user; // Get the user

        if campaign.admin != *user.key {
            // if the user is not the admin they cannot withdraw funds
            return Err(ProgramError::IncorrectProgramId);
        }

        // The balance of the of the PDA of the campaign account
        let balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());

        // Check if the amount is less than the balance after borrowing if not return an Error
        if **campaign.to_account_info().lamports.borrow() - balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        // Get the lamports from the campaign PDA and transfer it to the user's account
        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult {
        // Create a transfer instruction from the user's account to the campaign account with the amount
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.campaign.key(),
            amount,
        );

        // Invoke the transaction from the solana_program
        match anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.campaign.to_account_info(),
            ],
        ) {
            Ok(_) => {
                // If successful add the amount to the amount donated of the campaign account
                (&mut ctx.accounts.campaign).amount_donated += amount;
            }
            Err(_) => return Err(ProgramError::IncorrectProgramId),
        };

        Ok(())
    }
}

#[account]
pub struct Campaign {
    pub admin: Pubkey,       // Admin / User who can withdraw funds
    pub name: String,        // Name of the kickstarter project
    pub description: String, // Description of the project
    pub amount_donated: u64, // Amount in the bank
}

#[derive(Accounts)]
pub struct Donate<'a> {
    #[account(mut)] // Need to mutate the Campaign account
    pub campaign: Account<'a, Campaign>, // Reference to the Campaign account
    #[account(mut)] // Need to mutate the user account
    pub user: Signer<'a>, // User of the contract (The one who is donating)
    pub system_program: Program<'a, System>, // Reference to the system program
}

#[derive(Accounts)]
pub struct Withdraw<'a> {
    #[account(mut)]
    pub campaign: Account<'a, Campaign>, // Reference to the campaign account
    #[account(mut)]
    pub user: Signer<'a>, // Reference to the user who is also the admin
}

#[derive(Accounts)]
pub struct Create<'a> {
    // Initialize the account for Campaign
    // Gave abundant space in case it needs more
    #[account(init, payer=user, space=9000, seeds=[b"CAMPAIGN_DEMO".as_ref(), user.key().as_ref()], bump)]
    pub campaign: Account<'a, Campaign>,
    #[account(mut)]
    pub user: Signer<'a>,
    pub system_program: Program<'a, System>,
}
