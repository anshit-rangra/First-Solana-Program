use anchor_lang::prelude::*;

declare_id!("2G9pyxFP3khH6KksLSkxHYhuWte97XNXfNzXjYkAoCj5");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OPERATION {
   add,
   sub,
   mul,
   div

}

#[program]
mod anshit_rangra {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.add_account.payer = ctx.accounts.fee_payer.key();
        msg!("Hello Solana World");
        Ok(())
    }

    pub fn opr(ctx: Context<Operation>, a: u16, b: u16, opr: OPERATION) -> Result<()> {
        
        ctx.accounts.operation.a = a;
        ctx.accounts.operation.b = b;


        match opr {
            OPERATION::add => {
            ctx.accounts.operation.opr = String::from("Addition");
            ctx.accounts.operation.c = a+b;
            },
            OPERATION::sub => {
            ctx.accounts.operation.opr = String::from("Subtraction");
            ctx.accounts.operation.c = a-b;
            },
            OPERATION::mul => {
            ctx.accounts.operation.opr = String::from("Multiplication");
            ctx.accounts.operation.c = a*b;
            },
            OPERATION::div => {
            ctx.accounts.operation.opr = String::from("Division");
            ctx.accounts.operation.c = a/b;
            }
        }

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Operation<'info> {
    #[account(mut)]
    operation: Account<'info, Answer>
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    fee_payer: Signer<'info>,

    // creating account
    #[account(init, space=64, payer = fee_payer)]
    add_account: Account<'info, Answer>,
    
    // giving systemProgram to create account
    system_program : Program<'info, System>

}

#[account]
pub struct Answer {
    a: u16,
    b:u16,
    c: u16,
    opr: String,
    payer: Pubkey
}
