---
title: "Making FZF Completion Automatic in ZSH"
date: "2025-07-25"
tags: ["zsh", "fzf"]
excerpt: "I'm forcing myself to use FZF by triggering it on spacebar with commands that can benefit from it."
---

**[BLUF](https://en.wikipedia.org/wiki/BLUF_(communication))**: I modified ZSH to auto-trigger FZF for certain commands with the set of inputs I'm about to select from, and it is SO handy!

### First Problem

I have a problem: I have habits. I had been using `cd` for years when I discovered [`z`](https://github.com/rupa/z). `z` seemed super neat and useful so I installed it. Fast-forward a few years, and I had forgotten it existed.

I have this same problem using [FZF](https://github.com/junegunn/fzf). It is an awesome tool, but it isn't ingrained in me to trigger it with keyboard shortcuts when it could be useful. So I decided to force it upon myself for certain commands.

### Second Problem

I have a second problem, with `z` specifically, which is that I would like to be as terse as possible when specifying a directory, but it amounts to guessing what one or two letters will get me, and then finding out after hitting return if I was right or wrong. What if I could know before I hit return where I was going?

### Triggers (then)

A while back I aliased `gco` to run `git branch | fzf | xargs git checkout` so I could use FZF to checkout an existing branch. But this requires typing `gco` and then having the presence of mind to hit return. I wanted to improve this workflow so that EVERY time I type `gco ` (with a trailing space) it would automatically open FZF with my existing branches.

It occurred to me that if I could make that happen, I could use the same process to show the `z` predictions.

### ZLE Widgets

Actions (such as what happens when you press return) in ZSH are handled by ZLE "Widgets". Users can define widgets, register them with ZSH, and then bind them to a key... aka we can execute arbitrary code when we press the spacebar 😈.

Putting this in our `.zshrc` will register a new widget and bind it to the spacebar:
```zsh
zle -N auto-fzf-enhanced-widget
bindkey ' ' auto-fzf-enhanced-widget
```

Now all that's left is defining our widget by defining that function, `auto-fzf-enhanced-widget`!

### Generic FZF Invocation

I want to make this useful for `z` and various git commands. I'm sure I'll find more uses in the future as well. For each of these, I need to define our trigger case, the command to find the options to FZF, and the command to actually run FZF with.

#### Triggers (now)

My triggers are simply going to be certain commands that I already use (this is key - if I don't already use them this is useless!), followed by a space. If my widget is overzealous and triggers when I truly don't want it, I can either prevent it with forethought by preceding my command with a space, or realize after the fact and just hit escape.

So my triggers will be `z`, `gco`, `ga`, `gd`.

#### Options to FZF


This is the most complicated bit, [bear with me](https://knowyourmeme.com/photos/1358234-pun), I promise it gets simpler after this.

We need to determine what the contents of the FZF search will be. This varies with each command but I'll give two examples.

For `z`, I want to get just the final directory name of each path the `z` will consider. Running `z` with no arguments gives us a number ranking how much `z` likes this directory, along with a FULL path to it:

```zsh
> z
1939949    /Users/ryan/src/path1
3395609    /Users/ryan/src/path2
7760859    /Users/ryan/src/path3
```

We need to sort these so that they are in numberically descending order, so that we can match based on the [frecency](https://github.com/rupa/z/blob/master/README#L80) order instead of the strength of the fuzzy match. Then we only pass path1, path2, path3 to FZF, otherwise we will have a bunch of unnecessary junk messing with our fuzzy matching.

We can accomplish this with `"z 2>/dev/null | sort -rn | choose -f / -1"`. A complete code snippet is provided further down.

For `gd` (git diff), I want to get the list of modified files AND all the subdirectory paths that eventually lead to the file. For example, if we have a file we have editted at `foo/bar/baz.txt`, I need my output to include `foo`, `foo/bar`, and `foo/bar/baz.txt`, because I may want to see the diff for a particular file, or for a whole directory.

This can be accomplished with this absolute monstrosity of an `awk` command: `git status --short | choose -1 | awk -F'/' '{s=\"\"; for(i=1;i<=NF;i++) {s=s \$i; print s; if(i<NF) s=s \"/\"}}' | sed '1s/^/.\'$'\n/'`.

The `sed` bit at the end adds `.` (the current directory) as an option.

#### The FZF Invocation

Finally, we need to invoke FZF with information about how we want to select our output. This is fairly easy.

For `z`, we have a specific order where we want the first option that a fuzzy search matches, so we will use the `--no-sort` option: `fzf --height=40% --no-sort`

For `gd`, we don't have any special requirements, so we'll just use `fzf --height=40%`.

### Putting it All Together

This is a complete working script you can throw in your `.zshrc` and try it out. You will need [`choose`](https://github.com/theryangeary/choose?tab=readme-ov-file#installing-from-package-managers) or you can replace that usage with `cut`.

```
declare -A z_auto_fzf=(
    [buffer]="z"
    [opts_cmd]="z 2>/dev/null | sort -rn | choose -1 | choose -f / -1"
    [fzf_cmd]="fzf --height=40% --no-sort"
)
declare -A gco_auto_fzf=(
    [buffer]="gco"
    [opts_cmd]="git branch | grep -vE \"^\\*\" | choose -1"
    [fzf_cmd]="fzf --height=40%"
)
declare -A gd_auto_fzf=(
    [buffer]="gd"
    [opts_cmd]="git status --short | choose -1 | awk -F'/' '{s=\"\"; for(i=1;i<=NF;i++) {s=s \$i; print s; if(i<NF) s=s \"/\"}}' | sed '1s/^/.\'$'\n/'"
    [fzf_cmd]="fzf --height=40%"
)
declare -A ga_auto_fzf=(
    [buffer]="ga"
    [opts_cmd]="git status --short | choose -1 | awk -F'/' '{s=\"\"; for(i=1;i<=NF;i++) {s=s \$i; print s; if(i<NF) s=s \"/\"}}' | sed '1s/^/.\'$'\n/'"
    [fzf_cmd]="fzf --height=40%"
)

declare -a auto_fzf=(
    z_auto_fzf
    gco_auto_fzf
    gd_auto_fzf
    ga_auto_fzf
)

auto-fzf-enhanced-widget() {
    local current_buffer="$BUFFER"
    local cursor_pos="$CURSOR"

    for af in $auto_fzf; do
        local -A entry=("${(Pkv@)af}")
        if [[ "$current_buffer" == "${entry[buffer]}" && "$cursor_pos" -eq ${#entry[buffer]} ]]; then
            # Get directories, handling different possible z outputs
            local opts
            # N.B. this assumes the prefix is a valid command in its own right
            if command -v ${entry[buffer]} >/dev/null 2>&1; then
                opts=$(eval "${entry[opts_cmd]}")
            else
                echo "opt command not found"
                zle beep
                zle reset-prompt
                break
            fi

            # Check if we got any inputs
            if [[ -z "$opts" ]]; then
                echo "no inputs found"
                zle beep
                zle reset-prompt
                break
            fi

            # Use fzf with better options
            local fzf_selection=$(echo "$opts" | eval "${entry[fzf_cmd]}")

            if [[ -n "$fzf_selection" ]]; then
                BUFFER="${BUFFER} $fzf_selection"
                CURSOR=${#BUFFER}
                zle accept-line
            fi

            zle reset-prompt
        fi
    done

    # failure case - treat space as normal input. this means our success case MUST return early
    # Insert regular space
    BUFFER="${BUFFER:0:$CURSOR} ${BUFFER:$CURSOR}"
    CURSOR=$((CURSOR + 1))
}

zle -N auto-fzf-enhanced-widget
bindkey ' ' auto-fzf-enhanced-widget
```

You can also find my full `.zshrc` [here](https://github.com/theryangeary/dotfiles/blob/master/zsh/.zshrc) for reference.

[Let me know](https://github.com/theryangeary/dotfiles/issues) if you come up with any neat/cool/exciting adaptions of this setup!

Until next time, I will be trying to stop myself from using `cd` when `z` will work better.
