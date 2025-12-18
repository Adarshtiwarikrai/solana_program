use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock::Clock;
use anchor_lang::system_program;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{
        initialize_mint2, MintTo, Token2022,
    },
    token_interface::{
        non_transferable_mint_initialize, NonTransferableMintInitialize,
        default_account_state_initialize, DefaultAccountStateInitialize,
        AccountState, TokenInterface,
    },
};


declare_id!("BuPnkiVCYGdbpiFH3aE9H5BMTvD3qFGGziwvssBP8krE");

#[program]
pub mod subscription_nft{
    use super::*;
    pub fn initialize_subscription(ctx:Context<SubscriptionInitialize>,plan: String,duration_days: u64,)->Result<()>{
        let clock = Clock::get()?;
        let start_time = clock.unix_timestamp;
        let expiry_time = start_time + (duration_days as i64 * 86400);
        let extensions = vec![
                    ExtensionType::NonTransferable,
                    ExtensionType::DefaultAccountState,
                    ExtensionType::MetadataPointer,
                            ];


        let mint_len = ExtensionType::get_account_len::<spl_token_2022::state::Mint>(&extensions);
        let rent = Rent::get()?;
        let seeds = &[
        b"mint",
        ctx.accounts.subscription.key().as_ref(),
        &[ctx.bumps.mint],
        ];
        let account_create=system_instruction::create_account(
            ctx.accounts.mint.key(),
            ctx.accounts.signer.key(),
            rent.minimum_balance(mint_len)
            mint_len
            Token2022::id()
        )
        invoke_signed(
            account_create,
            [
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.signer.to_account_info()
            ]
            //why we need seeds is the normal transcation whre user is paying 
            [seeds]
        )
        let account_non_transfer=anchor_spl::token_interface::NonTransferableMintInitialize(
            mint:ctx.accounts.mint.key(),
            authority:ctx.accounts.subscription.key()
        )
        let cpi_non_transfer=CpiContext::new(ctx.accounts.token_program.to_account_info(),account_non_transfer);
        non_transferable_mint_initialize(cpi_non_transfer)?;
        let account_default=anchor_spl::token_interface::DefaultAccountStateInitialize(
            token_program_id:ctx.accounts.token_program.to_account_info(),
            mint:ctx.accounts.mint.key(),
        );
        let cpi_default=CpiContext::new(ctx.accounts.token_program.to_account_info(),account_default);
        default_account_state_initialize(cpi_default,AccountState::Initialized);
        let metadata=metadata_pointer_ix::initialize(
          Token2022::id(),
          ctx.accounts.mint.key(),
          Some(ctx.accounts.subcription.key()),
          Some(ctx.accounts.metadata.key())
        ).
        invoke_signed(
            &[ctx.accounts.mint.to_account_info(),
            ctx.accounts.subcription.to_account_info(),
            ctx.accounts.metadata.to_account_info()
            ]
            &[seeds]
        );
        spl_token_2022::instruction::initialize_mint2(
            Token2022:id()
            ctx.accounts.mint.key(),
            ctx.accounts.subcription.key(),
            None,
            0
        ).invoke_signed([ctx.accounts.token_program.to_account_info()],[seeds]);
        spl_token_2022::instruction::mint_to(
             Token2022::id(),
             ctx.accounts.mint.key(),
             ctx.accounts.useraccount.key(),
             ctx.accounts.subscription.key(),
             [],
              1,
        ).invoke_signed(&[ctx.accounts.mint.to_account_info(),ctx.accounts.subscription.to_account_info(),ctx.accounts.useraccount.to_account_info()],&[seeds]);
        let subscription=&mut ctx.accounts.subcription;
        subcription.plan=plan;
        subcription.start_time=start_time;
        subcription.exp_time=expiry_time;
        Ok(())
    }
    pub fn renew_subscription(ctx:Context<SubscriptionRenew>,plan: String,duration_days: u64)->Result<()>{
        let clock = Clock::get()?;
        let start_time = clock.unix_timestamp;
        let expiry_time = start_time + (duration_days as i64 * 86400);
        let subscription=&mut ctx.accounts.subcription;
        subcription.exp_time=expiry_time;
        let account_default=anchor_spl::token_interface::DefaultAccountStateInitialize(
            token_program_id:ctx.accounts.token_program.to_account_info(),
            mint:ctx.accounts.mint.key(),
        );
        let cpi_default=CpiContext::new(ctx.accounts.token_program.to_account_info(),account_default);
        default_account_state_initialize(cpi_default,AccountState::Initialized);
        Ok(())
    }
    pub fn force_expiry(ctx.Context<SubscriptionExpire>)->Result<()>{
        let clock = Clock::get()?;
        let start_time = clock.unix_timestamp;
        let subscription=&mut ctx.accounts.subcription;
        if subcription.exp_time<start_time {
            let account_default=anchor_spl::token_interface::DefaultAccountStateInitialize(
                token_program_id:ctx.accounts.token_program.to_account_info(),
                mint:ctx.accounts.mint.key(),
            );
            let cpi_default=CpiContext::new(ctx.accounts.token_program.to_account_info(),account_default);
            default_account_state_initialize(cpi_default,AccountState::Frozen);
        }
    }
}

#[derive(Accounts)]
pub struct SubscriptionInitialize<'info>{
    pub mint:AccountInfo<'info>,
    #[account(
        init,
        associated_token::mint=mint,
        associated_token::authority=signer,
        associated_token::token_program=token_program
    )]
    pub useraccount:InterfaceAccount<'info,TokenAccount>,
    #[account(mut)]
    pub metadata:AccountInfo<'info>,
    #[account(
        init,
        space=8+64+8+8,
        payer=signer,
        seeds=[b"subcription",signer.key(),mint.key()]
    )]
    pub subscription:Account<'info,Subscription>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub token_program:Program<'info,Token2022>,
    pub associated_token:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>
}
#[derive(Accounts)]
pub struct SubscriptionRenew<'info>{
    pub mint:AccountInfo<'info>,
    #[account(
       mut
    )]
    pub useraccount:InterfaceAccount<'info,TokenAccount>,
    #[account(
       mut
    )]
    pub subscription:Account<'info,Subscription>,
    #[account(mut)]
    pub token_program:Program<'info,Token2022>,
}
#[derive(Accounts)]
pub struct SubscriptionExpire<'info>{
    pub mint:AccountInfo<'info>,
    #[account(
       mut
    )]
    pub useraccount:InterfaceAccount<'info,TokenAccount>,
    #[account(
       mut
    )]
    pub subscription:Account<'info,Subscription>,
    #[account(mut)]
    pub token_program:Program<'info,Token2022>,
}
#[accounts]
pub struct Subscription{
    pub name:String
    pub exp_time:u64,
    pub start_time:u64
}