---
title: "Working with Multiple Files in Vim"
date: "2019-01-02"
tags: ["vim"]
excerpt: "I was recently asked by another member of udellug about my \"top three tips for working with multiple files/large projects in vim\". Three quickly turned into six."
---

I was recently asked by another member of [udellug](https://www.lug.udel.edu/)
about my "top three tips for working with multiple files/large projects in vim".
Three quickly turned into six as I felt these six topics are a great place to
start for transitioning from using vim for odds and ends to living in vim and
taking full advantage of its capabilities. My six tips are about: shell
commands, buffers, windows, tabs, sessions, and using git inside vim.

### 1. Shell commands

I used to spend most of my development time in bash, going back and forth
between entering vim, making an edit, exiting vim, then compiling and running
my code. It was terrible, but I didn't know any better! As it turns out,
there's no need to leave vim if you want to run a shell command or two. In
normal mode, after typing `:` you can add a `!` and everything after it will
be run as a shell command, so you can `rm -rf / --no-preserve-root` without
leaving vim, if that's what you're into.

Try it out. Open vim and type some of these:

```
:!pwd
:!ls
:!cowsay "Vim is so cool"
:!gcc % && ./a.out
```

'%' is a special register in vim that holds the name of the current buffer, so
that last line says "compile the current file then run the resulting executable.

### 2. Buffers

_Buffers are what you edit in vim, **not files**. Files live on disk, and when
you open them in vim, it loads the file's contents into a buffer. When you save
your buffer, it writes it back to the file on the disk._

I didn't know that you could open multiple buffers in vim until I spent a brief
stint using emacs (spacemacs, specifically), but this is key. You'll want to
learn how to open multiple buffers, close buffers, and move between them.
Opening buffers can be done in a few ways, my personal go-tos are using `:edit`
and `:find` for new or existing files, respectively. `:ls` will list open
buffers. Perhaps the must useful command for dealing with buffers is `:buffer`
because it allows you to select which buffer vim should display.

A super useful pro-tip is that Ctrl+Shift+6 or Ctrl-^ (two ways of saying the
same thing) will bring up the last buffer you had showing.

Read the help pages for more about these commands.

```
:h edit
:h find
:h ls
:h buffer
:h new
:h bdelete
```

### 3. Windows

Buffers are your text, and windows are what shows the text on the screen. You
can have any number of buffers open, but if you only have one window, you'll
only be able to see one buffer. On the other hand, if you have more than one
window, you'll be able to see more than one buffer, or alternatively different
places in the same buffer.

You'll want to learn how to make splits (turn the current window into two)
vertically and horizontally, how to move windows, resize windows, close windows,
etc. Check out `:h window` for a primer and `:h ctrl-w` for all the window
commands there are (you don't need to memorize all of them to start, I
certainly don't know every one).

### 4. Tabs

Tabs are collections of windows. By using tabs, you can have one complicated
set of windows in one tab, while having something totally different
in another. Perhaps one tab holds 8 windows that you want to see for reference
while another tab has the two files you're really _actively_ editing.

I use [neovim](https://neovim.io/) which allows me to have a terminal inside of
vim, so in addition to my other tabs, I always have a terminal tab at the ready.

### 5. Sessions

So you've loaded a bunch of buffers into vim and nicely laid out a handful of
windows across a few tabs, and you're done working on your project for now. But
you put all that effort into getting your project open and set up in vim! If you
close vim now it will disappear!

Not to worry. They planned for this, and the answer to your woes is called a
session. By running `:mksession filename`, your session will be stored in
filename and be accessed by starting vim in the style `vim -S filename.vim`

I've just looked into this recently so I'm not an expert but once you've opened
a bunch of buffers, windows, and what-have-you it's annoying to have to reopen
them next time you start vim. Save it to a session.

As always, more information can be found by typing `:h session`.

Additionally, there is a [plugin from tpope that looks quite handy for dealing
with sessions](https://github.com/tpope/vim-obsession)

### 6. Fugitive

If you're working on a project you'll probably be using git. There's really no
question about how you'll want to integrate git with vim. Get [fugitive, another
plugin from tpope.](https://github.com/tpope/vim-fugitive)

