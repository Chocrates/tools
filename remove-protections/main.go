package main

import (
	"context"
	"github.com/google/go-github/v33/github"
	"github.com/thatisuday/commando"
	"golang.org/x/oauth2"
	"log"
	"strings"
)

func main() {
	token, org, repoPrefix := commandoInit()

	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: token},
	)
	tc := oauth2.NewClient(ctx, ts)

	client := github.NewClient(tc)

	// Get repos to work on
	var allRepos []*github.Repository
	opt := &github.RepositoryListByOrgOptions{}

	for {
		repos, resp, err := client.Repositories.ListByOrg(ctx, org, opt)
		if err != nil {
			log.Printf(github.Stringify(resp))
			log.Panicf("Error getting repositories: ", err)

		}
		allRepos = append(allRepos, repos...)
		if resp.NextPage == 0 {
			break
		}
		opt.Page = resp.NextPage
	}

	for _, repository := range allRepos {
		if strings.HasPrefix(*repository.Name, repoPrefix) {
			log.Printf("Looking at Repo %s", *repository.Name)
			protection, resp, err := client.Repositories.GetBranchProtection(ctx, org, *repository.Name, *repository.DefaultBranch)
			if resp.StatusCode != 404 {
				if err != nil {
					log.Panicf("Error getting protections: ", err)
				}

				if protection != nil {
					log.Printf("Branch protection found, removing")
					_, err := client.Repositories.RemoveBranchProtection(ctx, org, *repository.Name, *repository.DefaultBranch)
					if err != nil {
						log.Panicf("Error removing protections: ", err)
					}
				}
			}
		}
	}
}

func commandoInit() (token, org, repoPrefix string) {
	// configure commando
	commando.
		SetExecutableName("main").
		SetVersion("0.0.1").
		SetDescription("This is a basic Go script sample module for intracting with GitHub")

	// configure the root command
	commando.
		Register(nil).
		AddFlag("token,t", "GitHub Personal Access Token", commando.String, nil).
		AddFlag("organization,o", "Organization in which to create the repo's", commando.String, nil).
		AddFlag("repo-prefix,p", "New repo prefix to prepend to user name", commando.String, nil).
		SetAction(func(args map[string]commando.ArgValue, flags map[string]commando.FlagValue) {
			token = flags["token"].Value.(string)
			org = flags["organization"].Value.(string)
			repoPrefix = flags["repo-prefix"].Value.(string)
		})

	// parse command-line arguments
	commando.Parse(nil)
	return
}
