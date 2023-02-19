# Achievements

This is a small Rust program to show the list of achievements for the game youre currently streaming on Twitch.

## How to use

1. Create a Twitch application [here](https://dev.twitch.tv/console/apps/create)
2. Copy the Client ID and Client Secret
4. Get your Steam API key [here](https://steamcommunity.com/dev/apikey)
3. Copy the `example.env` file to `.env` and fill in the Client ID, Client Secret, Steam API key, your Twitch username and your Steam ID
4. Run the program with `cargo run`
5. Add a browser source to your streaming software and point it to `http://localhost:${PORT}`

## Example
https://user-images.githubusercontent.com/31469876/219956563-4ba367ef-8d7b-45ab-bfb2-f5c8b5f492e1.mov

## Custom CSS

For custom css, for changing the style you can just put css on the `ul` and `li` tags
For other components you can add css to `#icon` for the achievement icon, `#text` for all the text, `#name` for the achievement name and `#description` for the achievement description.
