call plug#begin('~/.vim/plugged')

Plug 'fatih/vim-go'

call plug#end()

set nocompatible

filetype plugin indent on


colorscheme slate
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
set cursorline " highlights the current line
hi CursorLine term=bold cterm=bold

set backupdir=~/.vim/backup//
set directory=~/.vim/swap//
set undodir=~/.vim/undo//

set laststatus=2

" Status Line
set statusline=%-10.3n\                        " buffer number
set statusline+=%t                               "tail of the filename
set statusline+=[%{strlen(&fenc)?&fenc:'none'}, "file encoding
set statusline+=%{&ff}]                         "file format
set statusline+=%h                              "help file flag
set statusline+=%m                              "modified flag
set statusline+=%r                              "read only flag
set statusline+=%y                              "filetype
set statusline+=%=                              "left/right separator
set statusline+=%c,                             "cursor column
set statusline+=%l/%L                           "cursor line/total lines
set statusline+=\ %P                            "percent through file

set path+=/mnt/c/workspace/**,~
set wildignore+=**/node_modules/**

nnoremap <C-j> :%!python3 -m json.tool<cr>
nnoremap <C-x> :%!xmllint --format -<cr>
nnoremap <C-o> :%!~/workspace/pyorder/order.py<cr>
nnoremap <C-y> :%!clip.exe && powershell.exe -command "get-clipboard" \| sed 's/\r/\n/g'<cr><cr>
nnoremap <C-p> :%!powershell.exe -command "get-clipboard" \| sed 's/\r/\n/g'<cr><cr>
nmap <F8> :TagbarToggle<CR>
nnoremap <C-f> :set foldenable<cr>
nnoremap <C-r> :set nofoldenable<cr>

autocmd VimResized * wincmd =

let g:ctrlp_map = '<c-p>'
let g:ctrlp_cmd = 'CtrlP'

if has("autocmd")
    autocmd FileType go set ts=2 sw=2 sts=2 noet autowrite
endif
