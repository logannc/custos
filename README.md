# custos
 Custodian (Custos) of record keeping for Mage Marches West


### Getting a Development Environment Set Up

1. Visit [rustup.rs](https://rustup.rs/) to install Rust. Follow those instructions to completion.
2. Install the code editor of your choice. I recommend VS Code
    * I recommend the following extensions: Better TOML, crates, GitLens, Rust Test Lens, rust-analyzer
3. Go to the [Discord Developer Portal](https://discord.com/developers/applications) to create your own application.
4. Go to the `Bot` section to enable the bot functionality.
5. Click `Reset Token` and copy the value.
6. Place that token in `.cargo/config.toml` under `DISCORD_TOKEN` like
```toml
[env]
DISCORD_TOKEN = "<your token here>"
```
7. Configure your bot.
    * You probably want to disable `Public Bot` so it doesn't get invited to rando servers.
    * Enable `Message Content Intent`
8. Invite your bot to your test server.
    * Go to `General Information` and copy the `Application ID`
    * Replacing the placholder with your copied value, go to `https://discord.com/api/oauth2/authorize?client_id=<application_id>&permissions=8&scope=bot`
        * This will allow you to invite the bot to your test server (or any server you have permission for).
        * We are being lazy and requesting `Administrator` permissions.
9. `cargo run` should compile and run the application with your `DISCORD_TOKEN` set in the environment.
10. If all went well, you should see `Starting client...`. This means the bot is running! Test out the bot!