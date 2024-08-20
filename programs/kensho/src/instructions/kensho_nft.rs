use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::accounts::{MasterEdition, Metadata as MetadataAccount};


pub fn create_nft(ctx: Context<CreateNft>) -> Result<()> {

    Ok(())
}

#[derive(Accounts)]
pub struct CreateNft<'info> {
    // User for whom we will be minting the verification(kensho) NFT
    #[account(mut, signer)]
    pub signer: Signer<'info>,

    //Kensho NFT program
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    pub mint: Account<'info, Mint>,

    // ATA for the user to store his NFT
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    /// CHECK: derived from the mint and its PDA is checked
    #[account(
        mut,
        address = MetadataAccount::find_pda(&mint.key()).0,
    )]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: derived from the mint and its PDA is checked
    #[account(
        mut,
        address = MasterEdition::find_pda(&mint.key()).0,
    )]
    pub master_edition_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

}
