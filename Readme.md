# No Hello Bot

This bot will reply to a user with instructions to read `www.nohello.com` if they send a single slack message of any variation of a greeting.

## Usage

Create an environment variable `SLACK_BOT_TOKEN` with the token you get after configuring your bot on Slack.

Build the executable with Cargo and use `ngrok` to forward the webhook route so Slack can hit it.

## Is this ready for production?

No.  Probably never will be.