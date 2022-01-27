use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::{associated_token, token};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const WRAPPER_SEED: &[u8] = b"wrapper";
const AUTHORITY_SEED: &[u8] = b"authority";

#[program]
pub mod slickmint {
    use super::*;
    pub fn make_mint(ctx: Context<MakeMint>, mint_wrapper_bump: u8) -> ProgramResult {
        //config new mint authority
        let (slick_authority, slick_authority_bump) = Pubkey::find_program_address(
            &[AUTHORITY_SEED, ctx.accounts.mint.key().as_ref()],
            ctx.program_id,
        );
        validate_slick_authority(ctx.accounts.slick_authority.key(), slick_authority)?;
        ctx.accounts.mint_wrapper.mint_authority = Some(Pda {
            address: slick_authority,
            bump: slick_authority_bump,
        });

        //transfer mint authority

        //set data
        ctx.accounts.mint_wrapper.creator = ctx.accounts.creator.key();
        ctx.accounts.mint_wrapper.mint = ctx.accounts.mint.key();
        ctx.accounts.mint_wrapper.bump = mint_wrapper_bump;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(mint_wrapper_bump: u8)]
pub struct MakeMint<'info> {
    creator: Signer<'info>,
    #[account(
        constraint = mint.supply == 0,
    )]
    mint: Account<'info, token::Mint>,
    #[account(
        constraint = mint.mint_authority.unwrap() == existing_authority.key(),
        constraint = has_valid_freeze_authority(mint.freeze_authority, existing_authority.key())
    )]
    existing_authority: Signer<'info>, //biggest reason im passing auth in is so u can create metadata outside of the program
    slick_authority: AccountInfo<'info>,
    #[account(
        init,
        seeds = [WRAPPER_SEED, mint.key().as_ref()],
        bump = mint_wrapper_bump,
        payer = creator
    )]
    mint_wrapper: Account<'info, MintWrapper>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, token::Token>,
    system_program: Program<'info, System>,
}

pub fn has_valid_freeze_authority(
    freeze_authority: COption<Pubkey>,
    mint_authority: Pubkey,
) -> bool {
    if let COption::Some(freeze_auth) = freeze_authority {
        freeze_auth == mint_authority
    } else {
        true
    }
}

pub fn validate_slick_authority(passed: Pubkey, expected: Pubkey) -> ProgramResult {
    if passed == expected {
        Ok(())
    } else {
        Err(ErrorCode::InvalidSlickAuthority.into())
    }
}

#[account]
#[derive(Default)]
pub struct MintWrapper {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub mint_authority: Option<Pda>,
    pub max_supply: u64,
    pub mint_price: u64,
    pub bump: u8,
}

#[derive(Default, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Pda {
    pub address: Pubkey,
    pub bump: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("slick authority must have seeds 'authority', mintkey w/ canonical bump")]
    InvalidSlickAuthority,
}
