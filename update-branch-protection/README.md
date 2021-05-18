# UpdateBranchProtection
This script will update the branch protections for the repos specified in the `properties.json` file

## Pre-requisites
- NodeJS needs to be installed on your computer
- Checkout this repository
- Run `npm install`
## Config
- `token`
  - This is a [Personal Access Token](https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token) with write access to your repositories
- `githubUrl`
  - This is the api url for your GitHub instance
  - Leave unchanged for GitHub.com
- `organization` 
  - Change to the organization in which your repositories live
- `repos`
  - A list of repositories on which you would like to run the automation
- 'branch'
  - The branch you would like the branch protection to be applied to
  - *Note:* The API does not currently support wildcard branch names so `master*` will not work
- The rest of the properties map to the `[updateBranchProtections](https://docs.github.com/en/rest/reference/repos#update-branch-protection)` api
  - *Note:* The teams and users under `restrictions` must already have access to the repository before you run this script, otherwise they will not be added under the "Restrict who can push" section

- 

## Running
- `node updateBranchProtection.js`

## Getting Help
Please open an issue in this repository with your error message and we will help troubleshoot
