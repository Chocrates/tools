#!/bin/bash

# Pre-Reqs
# Need git installed to pull this script
# Ansible and this script should handle the rest

apt-get install ansible

ansible-playbook user_env.yaml

cp .bashrc /home/$USER/.bashrc
cp -r .vim /home/$USER/.vim
cp .vimrc /home/$USER/.vimrc
cp .tmux.conf /home/$USER/.vimrc

cd /home/$USER && rm -rf dotfiles
