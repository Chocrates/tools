# Generate jwt Key
This tool uses an app's private key to generate a jwt token.
See [the docs](https://docs.github.com/en/developers/apps/authenticating-with-github-apps#authenticating-as-a-github-app) for full context

## Usage 
`npm install`
`node generate-jwt-key.js --private-key /path/to/key`
`curl -i -H "Authorization: Bearer $(node generate-jwt-key.js --private-key /path/to/key)" -H "Accept: application/vnd.github.v3+json" https://api.github.com/app`

## Get Installation token flow
### Get Installations
`curl -i -H "Authorization: Bearer $(node generate-jwt-key.js --private-key /path/to/key)" -H "Accept: application/vnd.github.v3+json" https://api.github.com/app/installations`

### Get Token
`curl -i -H "Authorization: Bearer $(node generate-jwt-key.js --private-key /path/to/key)" -H "Accept: application/vnd.github.v3+json" https://api.github.com/app/installations/<installation id>/access_tokens`

### Use Token like a PAT
`curl -i -H "Authorization: Bearer <token>" -H "Accept: application/vnd.github.v3+json" https://api.github.com/user`
