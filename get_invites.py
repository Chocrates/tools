#!/usr/local/bin/python3

import requests
import re
import sys
import csv

def printUsers(file_name, users):
    with open(file_name, 'w') as csv_file:
        wr = csv.writer(csv_file, delimiter=',')
        wr.writerow(['created_at','login','email'])
        for user in users:
            wr.writerow(user.values())

    print(f'Wrote user list to {file_name}')

if len(sys.argv) != 3:
    print(f'Error: This must be called with the organization and your PAT, EG: {sys.argv[0]} orgName TOKEN')
    sys.exit(1)

org = sys.argv[1]
url = f'https://api.github.com/orgs/{org}/invitations'
token = sys.argv[2]

params = { 'per_page': 1,
           'page': 1
}

headers = { 'Content-Type': 'application/json',
            'Accept': 'application/vnd.github.dazzler-preview+json',
            'Authorization': f'Bearer {token}'
}


response = requests.get(url=url, params=params, headers=headers)

page = 2
users = list(map(lambda x: {'created_at': x['created_at'], 'login': x['login'], 'email': x['email']}, response.json()))

if 'Link' not in response.headers:
    printUsers('users.csv',users)
    sys.exit(0)

lastPage = re.search('&page=([0-9]+)>;',response.headers['Link'].split(',')[1]).group(1)


for i in range(2, int(lastPage)+1):
    params['page'] = i
    response = requests.get(url=url, params=params, headers=headers)
    users = users + list(map(lambda x: {'created_at': x['created_at'], 'login': x['login'], 'email': x['email']}, response.json()))

printUsers('users.csv',users)

sys.exit(0)

