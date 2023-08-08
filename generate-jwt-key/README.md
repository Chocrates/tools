# Generate jwt Key
This tool uses an app's private key to generate a jwt token.
See [the docs](https://docs.github.com/en/developers/apps/authenticating-with-github-apps#authenticating-as-a-github-app) for full context

## Usage 
```
npm install
node generate-jwt-key.js --private-key /path/to/key
curl -i -H "Authorization: Bearer $(node generate-jwt-key.js -i <app id> --private-key /path/to/key)" -H "Accept: application/vnd.github.v3+json" https://api.github.com/app
```

## Get Installation token flow
### Get Installations
```
curl -i -H "Authorization: Bearer $(node generate-jwt-key.js -i <app id> --private-key /path/to/key)" -H "Accept: application/vnd.github.v3+json" https://api.github.com/app/installations
```

### Get Token
```
curl -L \
  -X POST \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer $(node generate-jwt-key.js -i <app id> -p /path/to/key)" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/app/installations/<installation_id>/access_tokens \
  -d '{"repository":"<repo-name>","permissions":{"contents":"read"}}'
```
*Note:* Permissions JSON object must match [these properties](https://docs.github.com/en/rest/apps/apps?apiVersion=2022-11-28#create-an-installation-access-token-for-an-app-permissions-object) and your App must be granted the scopes you requested


### Use Token like a PAT
```
curl -i -H "Authorization: Bearer <installation_token>" -H "Accept: application/vnd.github.v3+json"   https://api.github.com/repos/OWNER/REPO
```
