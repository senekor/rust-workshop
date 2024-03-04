This folder contains stuff to integrate with `zellij`, a terminal mutliplexer.
`tabs/` contains zellij layouts which can be loaded into a running zellij session with the command:

```sh
zellij action new-tab --layout <TAB_LAYOUT>
```

The idea is that these tabs contain project-specific stuff, which can be loaded into you existing, personal zellij session.
Say you like to have a tab with `helix` or `neovim` open, along with `btop`.
You probably have such a layout stored in `~/.config/zellij/layouts/`.
You can dynamically load the project tabs into that session with the above command.
Project-specific stuff may include compiling the code, rendering documentation and so on.

If you _don't_ have such a personal layout, you can run `just zellij` to start a session with only the project tabs.

If you _do_ have a personal layout, here's the most seemless way to integrate the two:
in your personal layout, you're gonna include a pane which executes a command and then closes on exit.
That way, you can run any startup-scripts you want in your layout.
To load all project tabs dynamically, that script can be a simple loop over the directory:

```kdl
pane {
	command "bash"
	args "-c" "for tab in $(ls dev/zellij/tabs) ; do zellij action new-tab --layout dev/zellij/tabs/$tab ; done"
	close_on_exit true
}
```

Now, that might seem a little hacky, and it is.
But it works!
It does rely on the convention that every project stores its tabs under `dev/zellij/tabs/`.
If different projects handle it differently, the script would have to become more sophisticated.
