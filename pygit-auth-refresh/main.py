import requests, jwt, cryptography, time
import config
from github import Github
import os
import base64
from github import BadCredentialsException
from github import GithubIntegration


def get_app_token():
    GITHUB_PRIVATE_KEY = open(config.GITHUB_PRIVATE_KEY).read()
    integration = GithubIntegration(
        config.APP_ID, GITHUB_PRIVATE_KEY, config.GITHUB_BASE_URL
    )
    token = integration.get_access_token(config.INSTALLATION_ID)
    print(token)
    return token.token


g = Github(base_url=config.GITHUB_BASE_URL, login_or_token=get_app_token())


while True:
    try:
        git_repo = g.get_repo("chrislee18a/pygithub_githubapp")
    except BadCredentialsException:
        print("BadCredentialsException!")
        g = Github(base_url=config.GITHUB_BASE_URL, login_or_token=get_app_token())
        git_repo = g.get_repo("chrislee18a/pygithub_githubapp")

    print(list(git_repo.get_branches()))
    time.sleep(3 * 60)  # 3 mins
