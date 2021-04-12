package main

import (
	"context"
	"fmt"
	"github.com/google/go-github/v33/github"
	"github.com/thatisuday/commando"
	"golang.org/x/oauth2"
	"io/ioutil"
	"log"
	"strings"
	"time"
)

func main() {
	token, org, team, template, repoPrefix, userFile := commandoInit()

	ctx := context.Background()
	ts := oauth2.StaticTokenSource(
		&oauth2.Token{AccessToken: token},
	)
	tc := oauth2.NewClient(ctx, ts)

	client := github.NewClient(tc)

	// read in user list
	userList, err := readUsers(userFile)
	if err != nil {
		return
	}

	// for each user
	for _, user := range userList {
		if user != "" {
			opt := &github.TeamAddTeamMembershipOptions{
				Role: "member",
			}
			_, _, err := client.Teams.AddTeamMembershipBySlug(ctx, org, team, user, opt)
			if err != nil {
				log.Panicf("Error adding user to team: ", err)
			}

			repoName := fmt.Sprintf("%s-%s", repoPrefix, user)
			// create repo from template
			repo := &github.TemplateRepoRequest{
				Name:    &repoName,
				Owner:   &org,
				Private: github.Bool(true),
			}
			_, _, err = client.Repositories.CreateFromTemplate(ctx, org, template, repo)
			if err != nil {
				if err.(*github.ErrorResponse).Errors[0].Message == "Could not clone: Name already exists on this account" {
					fmt.Printf("%s exists, skipping\n", repoName)
				} else {
					log.Panic("Error Creating repo: ", err)
				}
			}

			// Wait for repo to exist
			time.Sleep(500 * time.Millisecond)

			// add user to repo as admin
			optAddTeamRepo := &github.TeamAddTeamRepoOptions{
				Permission: "admin",
			}
			_, err = client.Teams.AddTeamRepoBySlug(ctx, org, team, org, repoName, optAddTeamRepo)
			if err != nil {
				log.Panic("Error adding admin: ", err)
			}
		}
	}
}

func commandoInit() (token, org, team, template, repoPrefix, userFile string) {
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
		AddFlag("team,e", "Team to add users", commando.String, nil).
		AddFlag("template-repo,r", "Template repo to use for new user repositories", commando.String, nil).
		AddFlag("repo-prefix,p", "New repo prefix to prepend to user name", commando.String, nil).
		AddFlag("user-list,u", "Template repo to use for new user repositories", commando.String, nil).
		SetAction(func(args map[string]commando.ArgValue, flags map[string]commando.FlagValue) {
			token = flags["token"].Value.(string)
			org = flags["organization"].Value.(string)
			team = flags["team"].Value.(string)
			template = flags["template-repo"].Value.(string)
			repoPrefix = flags["repo-prefix"].Value.(string)
			userFile = flags["user-list"].Value.(string)
		})

	// parse command-line arguments
	commando.Parse(nil)
	return
}
func readUsers(userFile string) ([]string, error) {
	data, err := ioutil.ReadFile(userFile)
	if err != nil {
		fmt.Println("Error reading file: ", userFile, err)
		return nil, err
	}

	return strings.Split(strings.Trim(string(data), " "), "\n"), nil
}
