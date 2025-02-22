#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("6RMzzoy8iRv9a6ATQbxva3p5GCLFtBukjVN195aBNmQ8");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>, 
                            poll_id: u64,
                            description: String,
                            poll_start: u64,
                            poll_end: u64) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.description = description;
        poll.poll_start = poll_start; 
        poll.poll_end = poll_end;
        poll.candidate_amount = 0;
        poll.voter_count = 0;
        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>,
                                candidate_name: String,
                                _poll_id: u64) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        let poll = &mut ctx.accounts.poll;
        poll.candidate_amount += 1;
        candidate.candidate_name = candidate_name;
        candidate.candidate_votes = 0;
        Ok(())
    }  

    pub fn register_voter(ctx: Context<RegisterVoter>, _poll_id: u64) -> Result<()> {
        let voter = &mut ctx.accounts.voter;
        voter.has_voted = false;
        let poll = &mut ctx.accounts.poll;
        poll.voter_count += 1;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _candidate_name: String, _poll_id: u64) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        let voter = &mut ctx.accounts.voter;
        let poll = &ctx.accounts.poll;

        require!(poll.poll_start <= Clock::get()?.unix_timestamp as u64, CustomError::PollNotStarted);
        require!(poll.poll_end >= Clock::get()?.unix_timestamp as u64, CustomError::PollEnded);
        require!(!voter.has_voted, CustomError::AlreadyVoted);

        candidate.candidate_votes += 1;
        voter.has_voted = true;

        msg!("Voted for candidate: {}", candidate.candidate_name);
        msg!("Votes: {}", candidate.candidate_votes);
        Ok(())
    }

    pub fn get_winner(ctx: Context<GetWinner>, _poll_id: u64) -> Result<String> {
        let poll = &ctx.accounts.poll;
        let mut highest_votes = 0;
        let mut winner_name = "".to_string();
        
        for candidate in &ctx.remaining_accounts {
            let candidate_account: Account<Candidate> = Account::try_from(candidate)?;
            if candidate_account.candidate_votes > highest_votes {
                highest_votes = candidate_account.candidate_votes;
                winner_name = candidate_account.candidate_name.clone();
            }
        }
        Ok(winner_name)
    }
}

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct Vote<'info> {
    pub signer: Signer<'info>,

    #[account(
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub voter: Account<'info, Voter>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct RegisterVoter<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Voter::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub voter: Account<'info, Voter>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Voter {
    pub has_voted: bool,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct GetWinner<'info> {
    #[account(
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
    pub voter_count: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Poll has not started yet.")]
    PollNotStarted,
    #[msg("Poll has already ended.")]
    PollEnded,
    #[msg("Voter has already voted.")]
    AlreadyVoted,
}
