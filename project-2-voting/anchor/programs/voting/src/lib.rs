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
        description: String
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        
        require!(start_time < end_time, ErrorCode::InvalidTimeRange);
        
        poll.poll_name = name;
        poll.poll_description = description;
        poll.poll_voting_start = start_time;
        poll.poll_voting_end = end_time;
        poll.poll_creator = ctx.accounts.signer.key();
        poll.poll_option_index = 0;
        
        msg!("Poll '{}' initialized successfully!", poll.poll_name);

        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>, 
        _poll_id: u64, 
        candidate: String
    ) -> Result<()> {
        let poll = &ctx.accounts.poll_account;
        let candidate_account = &mut ctx.accounts.candidate_account;

        require!(poll.poll_creator == ctx.accounts.signer.key(), ErrorCode::Unauthorized);
        require!(Clock::get()?.unix_timestamp < (poll.poll_voting_end as i64), ErrorCode::PollClosed);

        candidate_account.candidate_name = candidate;
        candidate_account.candidate_votes = 0;
        poll.poll_option_index += 1;

        msg!("Candidate '{}' added successfully!", candidate_account.candidate_name);

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()> {
        let poll = &ctx.accounts.poll_account;
        let candidate_account = &mut ctx.accounts.candidate_account;
        let voter = &mut ctx.accounts.voter_account;
        let current_time = Clock::get()?.unix_timestamp;

        require!(current_time >= (poll.poll_voting_start as i64), ErrorCode::VotingNotStarted);
        require!(current_time <= (poll.poll_voting_end as i64), ErrorCode::VotingEnded);
        require!(!voter.has_voted, ErrorCode::AlreadyVoted);

        candidate_account.candidate_votes += 1;
        voter.has_voted = true;

        msg!("Vote cast successfully for '{}'", candidate_account.candidate_name);

        Ok(())
    }

    pub fn get_total_votes(ctx: Context<GetTotalVotes>, _poll_id: u64) -> Result<u64> {
        let candidate = &ctx.accounts.candidate_account;
        msg!("Total votes for {}: {}", candidate.candidate_name, candidate.candidate_votes);
        Ok(candidate.candidate_votes)
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
        space = 8 + PollAccount::INIT_SPACE,
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

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

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
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + VoterAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub voter_account: Account<'info, VoterAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct GetTotalVotes<'info> {
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_account.candidate_name.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
    pub poll_creator: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct VoterAccount {
    pub has_voted: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
    #[msg("Poll creator only can add candidates")]
    Unauthorized,
    #[msg("User has already voted")]
    AlreadyVoted,
    #[msg("Invalid time range: Start time must be before end time")]
    InvalidTimeRange,
    #[msg("Poll has ended, no further modifications allowed")]
    PollClosed,
}
