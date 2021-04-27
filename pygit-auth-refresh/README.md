# pygit-auth-refresh
Sample python code that will auto refresh app tokens

It does this by subclassing GitHub and adding decorators to all functions.
This has only been tested with `get_repos` so presumably there is more work to be done to ensure that all of the class functions are properly decorated.
