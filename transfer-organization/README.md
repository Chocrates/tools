# Transfer Organization
This tool will take in a list of repos, separated by new lines, and transfer them to the target repo.  It will also do a find and replace on references to the original organization and open a PR to change them to the new organization

## Usage
`node transfer-organization.js --token <token> --source-organization <org> --destination-organization <org> --file <repo file>`

## Repo file format
The repo file should be in JSON format as follows
```
[ 
  {
    "name": "Name of the repo",
    "teams": ["team_slug", "team_slug"]
  }
]
