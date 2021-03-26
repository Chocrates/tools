# Create Repos From Template
This script will take in a file with a list of GitHub usernames.
For each username it will add them to the group specified, create a repo from the template specified, and add the group as an admin to that repo
## Usage
```
go mod tidy
go run main.go --token <PAT> --organization <Organization> --team <Org Team> --template-repo <Template Repo Name> --repo-prefix <Prefix> --user-list <user list file>
```
