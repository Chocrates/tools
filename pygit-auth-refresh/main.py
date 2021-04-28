import requests, jwt, cryptography, time
import config
import os
import base64
from datetime import datetime
from github import (
    Github,
    BadCredentialsException,
    GithubIntegration,
)
from github.Requester import Requester
import inspect

DEFAULT_BASE_URL = "https://api.github.com"
DEFAULT_STATUS_URL = "https://status.github.com"
# As of 2018-05-17, Github imposes a 10s limit for completion of API requests.
# Thus, the timeout should be slightly > 10s to account for network/front-end
# latency.
DEFAULT_TIMEOUT = 15
DEFAULT_PER_PAGE = 30


class MyGH(Github):
    def error_handling(self, fn):
        def wrapper(*args, **kwargs):
            print(datetime.utcnow())
            print(self._github_integration.expires_at)
            if datetime.utcnow() > self._github_integration.expires_at:
                print("Refreshing the token")
                self._Github__requester = Requester(
                    login_or_token=MyGH.get_app_token().token,
                    password=self._password,
                    jwt=self._jwt,
                    base_url=self._base_url,
                    timeout=self._timeout,
                    user_agent=self._user_agent,
                    per_page=self._per_page,
                    verify=self._verify,
                    retry=self._retry,
                    pool_size=self._pool_size,
                )

            return fn(self, *args, **kwargs)

        return wrapper

    def __init__(
        self,
        github_integration,
        password=None,
        jwt=None,
        base_url=DEFAULT_BASE_URL,
        timeout=DEFAULT_TIMEOUT,
        user_agent="PyGithub/Python",
        per_page=DEFAULT_PER_PAGE,
        verify=True,
        retry=None,
        pool_size=None,
    ):
        self._github_integration = github_integration
        self._password = password
        self._jwt = jwt
        self._base_url = base_url
        self._timeout = timeout
        self._user_agent = user_agent
        self._per_page = per_page
        self._verify = verify
        self._retry = retry
        self._pool_size = pool_size

        super().__init__(
            login_or_token=github_integration.token,
            password=password,
            jwt=jwt,
            base_url=base_url,
            timeout=timeout,
            user_agent=user_agent,
            per_page=per_page,
            verify=verify,
            retry=retry,
            pool_size=pool_size,
        )

        for name, fn in inspect.getmembers(Github, inspect.isfunction):
            setattr(self, name, self.error_handling(fn))

    @classmethod
    def get_app_token(
        self,
    ):
        GITHUB_PRIVATE_KEY = open(config.GITHUB_PRIVATE_KEY).read()
        integration = GithubIntegration(
            config.APP_ID, GITHUB_PRIVATE_KEY, config.GITHUB_BASE_URL
        )
        token = integration.get_access_token(config.INSTALLATION_ID)
        print(token)
        return token


g = MyGH(base_url=config.GITHUB_BASE_URL, github_integration=MyGH.get_app_token())


while True:
    git_repo = g.get_repo("chocrates/tools")
    print(list(git_repo.get_branches()))
    time.sleep(5 * 60)  # 5 mins
