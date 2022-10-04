# Delete Repositories
This script will delete the list of repositories passed in via a csv file.

## Pre-requisites
- NodeJS needs to be installed on your computer
- Checkout this repository
- `cd` into this directory 
- Run `npm install`
## Config
- `token`
  - This is a [Personal Access Token](https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token) with write access to your repositories
- `file`
  - A csv file with a header.  See [example.csv](example.csv) for an example.  Format of each entry should be `owner/repo`
## Running
- `node delete-repositories.js --token <token> --file path/to/file.csv`

## Getting Help
Please open an issue in this repository with your error message and we will help troubleshoot
