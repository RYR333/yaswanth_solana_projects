🗳️ Solana Voting DApp
A decentralized voting system built on Solana using Anchor. This smart contract enables users to create polls, add candidates, and cast votes securely.

🚀 Features
Create Polls: Users can initialize a new poll with a name, description, and voting timeline.
Add Candidates: Only the poll creator can add candidates.
Cast Votes: Users can vote for their preferred candidates.
Prevent Double Voting: Ensures each voter can only vote once per poll.
Check Poll Status: Voting is allowed only within the specified time frame.
Security & Access Control: Admin-only candidate addition, prevents post-deadline modifications.
Retrieve Vote Count: Query the total votes for a candidate.
📌 Prerequisites
Ensure you have the following installed before setting up the project:

Rust & Cargo
Solana CLI
Anchor Framework
⚙️ Installation
1️⃣ Clone the repository

Copy
Edit
git clone https://github.com/your-username/solana-voting-dapp.git
cd solana-voting-dapp
2️⃣ Install Dependencies

Copy
Edit
anchor build
3️⃣ Start a Local Solana Node

Copy
Edit
solana-test-validator
4️⃣ Deploy Anchor Program

Copy
Edit
anchor deploy
🚀 Deploying to Devnet
If you want to deploy to Solana Devnet, first configure your wallet:


Copy
Edit
solana config set --url https://api.devnet.solana.com
solana airdrop 2
Then, update Anchor.toml with:

toml
Copy
Edit
[provider]
cluster = "devnet"
wallet = "/path/to/your/wallet.json"
Finally, deploy:


Copy
Edit
anchor deploy
📜 Usage
📌 Create a Poll

Copy
Edit
anchor test --group create_poll
📌 Add a Candidate

Copy
Edit
anchor test --group add_candidate
📌 Cast a Vote

Copy
Edit
anchor test --group vote
📌 Check Vote Count

Copy
Edit
anchor test --group get_votes
🎯 Future Improvements
✅ Implement Multi-Signature Admin Control
✅ Allow Users to Create Multiple Polls
✅ Introduce Rewards for Voters
✅ Add Staking & Governance
🤝 Contributing
Feel free to submit PRs, report bugs, or suggest features! 🚀

📜 License