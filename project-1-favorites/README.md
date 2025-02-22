# Favorites Program on Solana

## 📌 Overview
This Solana program, built using **Anchor**, allows users to store and update their **favorite number, color, and hobbies** on the blockchain. It also includes additional features like **timestamp tracking**, updating favorite numbers, and retrieving stored favorites.

## 🚀 Features
- **Set Favorites**: Users can store their favorite number, color, and hobbies.
- **Get Favorites**: Retrieve stored favorites from the blockchain.
- **Update Favorite Number**: Users can modify only their favorite number without affecting other data.
- **Timestamp Tracking**: The last update time is recorded for each entry.

## 🏗️ Installation & Setup

### Prerequisites
Ensure you have the following installed:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://project-serum.github.io/anchor/getting-started/installation.html)

### Clone the Repository
```sh
git clone https://github.com/your-username/favorites-solana.git
cd favorites-solana
```

### Build & Deploy
```sh
anchor build
solana program deploy target/deploy/favorites.so
```

### Running Tests
```sh
anchor test
```

## 🛠️ Usage

### Set Favorites
Call the **set_favorites** instruction with:
- A number (`u64`)
- A color (`String`)
- A list of hobbies (`Vec<String>`)

### Get Favorites
Retrieve stored favorite details for a user.

### Update Favorite Number
Modify only the favorite number while keeping other data intact.

## 📜 Smart Contract Code (Simplified)
```rust
#[program]
pub mod favorites {
    use super::*;
    
    pub fn set_favorites(ctx: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        let user_key = ctx.accounts.user.key();
        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
            last_updated: Clock::get()?.unix_timestamp
        });
        msg!("Updated Favorites for {}", user_key);
        Ok(())
    }

    pub fn get_favorites(ctx: Context<GetFavorites>) -> Result<Favorites> {
        Ok(ctx.accounts.favorites.clone())
    }
}
```

## 📄 License
This project is licensed under the MIT License.

---
🌟 **Contributions are welcome!** Feel free to fork, modify, and improve the project.

