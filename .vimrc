set nocompatible

" set the runtime path to include Vundle and initialize
set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()
" alternatively, pass a path where Vundle should install plugins
"call vundle#begin('~/some/path/here')

" let Vundle manage Vundle, required
Plugin 'VundleVim/Vundle.vim'

Plugin 'https://github.com/fholgado/minibufexpl.vim.git'
Plugin 'https://github.com/vim-scripts/AfterColors.vim.git'
Plugin 'https://github.com/tpope/vim-sleuth.git'

call vundle#end()
filetype plugin indent on


colorscheme desert
highlight Normal guibg=black guifg=lightblue 
set background=dark

scriptencoding utf-8
set encoding=utf-8
" filetype on
" filetype plugin on


" The default for 'backspace' is very confusing to new users, so change it to a
" more sensible value.  Add "set backspace&" to your ~/.vimrc to reset it. 
set backspace=indent,eol,start

" Disable localized menus for now since only some items are translated (e.g.
" the entire MacVim menu is set up in a nib file which currently only is
" translated to English).
set langmenu=none
syntax on
se nu
se ruler
se tabstop=4
se shiftwidth=4
se expandtab
se foldmethod=syntax
let g:jsx_ext_required = 0

set listchars=eol:~,tab:>.,trail:~,extends:>,precedes:<,space:_
set list
set hlsearch

set backupdir=~/.vim/backup//
set directory=~/.vim/swap//
set undodir=~/.vim/undo//

nnoremap <C-j> :%!python3 -m json.tool<cr>
nnoremap <C-x> :%!xmllint --format -<cr>
nnoremap <C-f> :set foldenable<cr>
nnoremap <C-r> :set nofoldenable<cr>

autocmd VimResized * wincmd =

let g:ctrlp_map = '<c-p>'
let g:ctrlp_cmd = 'CtrlP'
