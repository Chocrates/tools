mv ~/.config/i3/config ~/.config/i3/config.bak
ln -s ~/.config/i3/config ~/dotfiles/.i3config 
ln -s ~/dotfiles/.i3config ~/.config/i3/config
mv ~/.vimrc ~/.vimrc.bak
ln -s ~/dotfiles/.vimrc ~/.vimrc
#ln -s ~/dotfiles/.vim ~/.vim
ln -s ~/dotfiles/.tmux.conf ~/.tmux.conf
mv ~/.bashrc ~/.bashrc.bak
ln -s ~/dotfiles/.bashrc ~/.bashrc
ln -s ~/dotfiles/.compton.conf ~/.config/compton.conf
