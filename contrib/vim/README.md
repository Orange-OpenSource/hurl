# Support for Hurl Syntax Highlighting

This enables basic syntax coloring for Hurl files in Vim/Neovim.

## Installation

### Vim

Install Vim syntax highlighting for Hurl

```bash
mkdir -p ~/.vim/{ftdetect,syntax}
cp ftdetect/hurl.vim ~/.vim/ftdetect
cp syntax/hurl.vim ~/.vim/syntax
```

### Neovim

Install Neovim syntax highlighting for Hurl

```bash
mkdir -p ~/.config/nvim/{ftdetect,syntax}
cp ftdetect/hurl.vim ~/.config/nvim/ftdetect
cp syntax/hurl.vim ~/.config/nvim/syntax
```

## Configuration

Activate syntax highlighting in your `~/.vimrc` or `~/.config/nvim/init.vim`

```vim
syntax on
```
