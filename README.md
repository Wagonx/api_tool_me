# What is this?

This is an interactive command line tool to query the swDeployment endpoint for the Manage Engine API. 

I created this, so I can quickly grab templateIDs when I am building tools for the platform

For a full reference of the API endpoint [see this GIST](https://gist.github.com/Wagonx/613f47d1c356e339d437d05a085b9bbd)

## Requirements
- On-prem manage engine with an API version >= 1.3

### Steps to compile from source

1. `git clone https://github.com/Wagonx/api_tool_me.git`
2. `cp .env.example .env`
3. `vim .env` - Update environment variables to match your environment
4. Leave vim with `ESC`, `SHIFT + :`, `wq`
5. `cargo build --release`
6. Your .exe will be in `target\release`