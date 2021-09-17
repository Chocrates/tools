# Consolidate Teams
This script will consolidate all teams (IGNORING sub-teams) into the specified teams in the properties file
When run, the teams specified in properties will be created and everyone with existing access will be migrated into the 
proper team.  
For instance if you have 7 teams with read access, all users in those teams will be added to the 1 new read access team and all 
the existing teams will be removed from the repository.

## Pre-requisites
- NodeJS needs to be installed on your computer
- Checkout this repository
- `cd` into this directory 
- Run `npm install`
## Config
- `token`
  - This is a [Personal Access Token](https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token) with write access to your repositories
- `githubUrl`
  - API Url for your github instance
- `organization`
  - The organization on which to work
- `repos`
  - An array of repositories the teams of which you would like to consolidate
- `teams`
  - The default team names that will be created.  Use `<repo-name>` anywhere in the team name and it will be replaced with the current repo name
## Running
- `node consolidate-teams.js`

## Getting Help
Please open an issue in this repository with your error message and we will help troubleshoot
