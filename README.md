# Tools
*Note:* This repository is currently in flux, getting rewritten in Rust.  Leaving the historical README below and I will remove it once the rewrite is more mature
This repository holds a number of scripts to assist in administering and migrating GitHub repositories and Organizations
Most deal diredtly with the GitHub API.

## Installation
### From Binaries
You can find the binaries for your system in the releases section.
Download the zip and extract to your file system.  You can then use the binary.

### From Source  
- Install the Rust toolchain.  Follow the [instructions](https://www.rust-lang.org/tools/install) on their website.
- Clone the code down to your machine
- `cargo build`

---

## Scripts
- [create-repos-from-template](https://github.com/Chocrates/tools/tree/master/create-repos-from-template)
  - Creates repos for a list of users based on a template and gives a team admin access to them
- [get-issues](https://github.com/Chocrates/tools/tree/master/get-issues)
  - Gets issues with the label `due-on: todays_date`, for example `due-on: 2021-04-28`
- [get-org-invitations](https://github.com/Chocrates/tools/tree/master/get-org-invitations)
  - Python example of calling the API with requests, gets org invitations
- [pygit-auth-refresh](https://github.com/Chocrates/tools/tree/master/pygit-auth-refresh)
    - PyGit sample that will watch for expired [App Tokens](https://docs.github.com/en/developers/apps/authenticating-with-github-apps#authenticating-as-a-github-app) and refresh them
- [remove-protections](https://github.com/Chocrates/tools/tree/master/remove-protections)
  - Removes branch protections from the default branch for all repos starting with prefix
- [search-org-repos](https://github.com/Chocrates/tools/tree/master/search-org-repos)
  - Takes in a code search query and finds all repos and collaborators with hits
  - Useful if you need to track down a bunch of repos with data and you need to contact their owners
- [transfer-organization](https://github.com/Chocrates/tools/tree/master/transfer-organization)
  - Transfers all repos in the file to the new org and opens a PR changing all references from the old org to the new org
- [delete-repositories](https://github.com/Chocrates/tools/tree/master/delete-repositories)
  - Takes a CSV file of owner/repo's and deletes them
