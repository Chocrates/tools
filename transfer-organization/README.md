# Transfer Organization
This tool will take in a list of repos, separated by new lines, and transfer them to the target repo.  It will also do a find and replace on references to the original organization and open a PR to change them to the new organization

## Usage
`node transfer-organization.js --token <token> --source-organization <org> --destination-organization <org> --file <repo file>`
