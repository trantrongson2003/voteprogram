use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("B3AaCvdvEC3R5mf4UBfymfawSTrWGSCbgYkASQSZFsnX");

#[program]
mod vote_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vote_account.candidates = Vec::new();
        Ok(())
    }

    pub fn add_candidate(ctx: Context<AddCandidate>, nickname: String) -> Result<u32> {
        let candidate = Candidate {
            nickname,
            votes: 0,
        };
        ctx.accounts.vote_account.candidates.push(candidate);
        Ok((ctx.accounts.vote_account.candidates.len() - 1) as u32)
    }

    pub fn vote(ctx: Context<Vote>, candidate_index: u32) -> Result<()> {
        let candidate = &mut ctx.accounts.vote_account.candidates[candidate_index as usize];
        candidate.votes += 1;
        Ok(())
    }

    pub fn view_vote(ctx: Context<ViewVote>, candidate_index: u32) -> Result<u32> {
        let candidate = &ctx.accounts.vote_account.candidates[candidate_index as usize];
        Ok(candidate.votes)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1000)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddCandidate<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

#[derive(Accounts)]
pub struct ViewVote<'info> {
    pub vote_account: Account<'info, VoteAccount>,
}

#[account]
pub struct VoteAccount {
    pub candidates: Vec<Candidate>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Candidate {
    pub nickname: String,
    pub votes: u32,
}
