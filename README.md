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

## Usage
Use the built in help commands to understand what options are available
```
$ ./github-tools --help                                                                                                   main  ✱
github-tools 0.1.0
Chris McIntosh <chocrates@github.com>
Suite of tools used for GitHub Organizaton/Repository management

USAGE:
    github-tools --token <TOKEN> <SUBCOMMAND>

OPTIONS:
    -h, --help             Print help information
    -t, --token <TOKEN>    GitHub Personal Access Token with access to Organization or Repositories
    -V, --version          Print version information

SUBCOMMANDS:
    consolidate-teams        Consolidates all users in all with access to a repository to the
                                 specified teams in a properties file
    delete-repositories      Delete Repositories contained in csv file
    help                     Print this message or the help of the given subcommand(s)
    transfer-repositories    Transfer repositories contained in csv file to new org, including
                                 teams and members NOTE: Team permissions are not maintained and
                                 will be transfered as read See:
                                 https://docs.github.com/en/rest/repos/repos#transfer-a-repository

```
Currently only the `transfer-repositories` subcommand is complete, usage is as follows:

```
$ ./github-tools transfer-repositories --help                                                                             main  ✱
github-tools-transfer-repositories
Transfer repositories contained in csv file to new org, including teams and members NOTE: Team
permissions are not maintained and will be transfered as read See:
https://docs.github.com/en/rest/repos/repos#transfer-a-repository

USAGE:
    github-tools --token <TOKEN> transfer-repositories [OPTIONS]

OPTIONS:
    -e, --example
            Prints an example csv file to stdio and then exits This data can be saved and edited to
            create your input file

    -f, --file <FILE>
            Path to CSV file with a single column containing repositories to delete in format
            "owner/repository"

    -h, --help
            Print help information

    -o, --organization <ORGANIZATION>
            Organization in which to transfer the repositories
```

*Note*: `--token` is required in all cases other than help, even for `--example`

Example Usage:
- Create a CSV containing your repositories.  You can utilize the `--example` command to create a baseline CSV
```
$ ./github-tools --token $GITHUB_TOKEN transfer-repositories --example > example.csv                                      main  ✱
$ cat example.csv                                                                                                         main  ✱
repository
organization/repository
# Edit the file here to add your repos
```
- Execute the transfer
```
$  ./github-tools --token $GITHUB_TOKEN transfer-repositories --file example.csv --organization <target organization>
```

## Getting Help
Currently creating a new issues is the best way to address errors and questions.

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
- [update-branch-protection](https://github.com/Chocrates/tools/tree/main/update-branch-protection)
  - Adds branch protections to repos specified in the `properties.json` file
- [delete-repositories](https://github.com/Chocrates/tools/tree/master/delete-repositories)
  - Takes a CSV file of owner/repo's and deletes them
