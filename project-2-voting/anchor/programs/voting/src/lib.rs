use anchor_lang::prelude::*;

declare_id!("5s3PtT8kLYCv1WEp6dSh3T7EuF35Z6jSu5Cvx4hWG79H");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        _poll_id: u64,
        start_time: u64,
        end_time: u64,
        name: String,
        description: String,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        poll.poll_name = name;
        poll.poll_description = description;
        poll.poll_voting_start = start_time;
        poll.poll_voting_end = end_time;
        poll.poll_option_index = 0;
        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>, _poll_id: u64, candidate: String) -> Result<()> {
        let candidate_account = &mut ctx.accounts.candidate_account;
        candidate_account.candidate_name = candidate;
        candidate_account.candidate_votes = 0;
        ctx.accounts.poll_account.poll_option_index += 1;
        Ok(())
    }

    pub fn register_voter(ctx: Context<RegisterVoter>, _poll_id: u64) -> Result<()> {
        let voter = &mut ctx.accounts.voter_account;
        voter.has_voted = false;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()> {
        let candidate_account = &mut ctx.accounts.candidate_account;
        let voter = &mut ctx.accounts.voter_account;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time > (ctx.accounts.poll_account.poll_voting_end as i64) {
            return Err(ErrorCode::VotingEnded.into());
        }

        if current_time <= (ctx.accounts.poll_account.poll_voting_start as i64) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        if voter.has_voted {
            return Err(ErrorCode::AlreadyVoted.into());
        }

        candidate_account.candidate_votes += 1;
        voter.has_voted = true;
        Ok(())
    }

    pub fn get_poll_results(ctx: Context<GetPollResults>, _poll_id: u64) -> Result<u64> {
        Ok(ctx.accounts.candidate_account.candidate_votes)
    }
}
#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32 + 280 + 8 + 8 + 8,  // Fixed space allocation
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 8,  // Fixed space allocation
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct RegisterVoter<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 1,  // Fixed space allocation
        seeds = [b"voter".as_ref(), poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub voter_account: Account<'info, VoterAccount>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump)]
    pub candidate_account: Account<'info, CandidateAccount>,

    #[account(
        mut,
        seeds = [b"voter".as_ref(), poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub voter_account: Account<'info, VoterAccount>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct GetPollResults<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump)]
    pub candidate_account: Account<'info, CandidateAccount>,
}

#[account]
pub struct CandidateAccount {
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[account]
pub struct PollAccount{
    pub poll_name: String,
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}

#[account]
pub struct VoterAccount {
    pub has_voted: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
    #[msg("You have already voted")]
    AlreadyVoted,
}
