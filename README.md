# Tity3

Tity3 is a terminal multiplexer with i3-like keybinding and the functioning of a tilling window manager. With a really simple learning curve.

## Key Bindings (In progress)

| Key(s) | Description
|-------:|:------------
|<kbd>Meta+Enter</kbd> | Create a new pane
|<kbd>Meta+t</kbd> | Change to tab mode
|<kbd>Meta+s</kbd> | Change to split mode
|<kbd>Meta+Shift+F</kbd> | Make the selected pane fullscreen.
|<kbd>Meta+&larr;/&darr;/&uarr;/&rarr;</kbd><br><kbd>Meta+h/j/k/l</kbd> | Navigate through the panes
|<kbd>Meta+Shift+&larr;/&darr;/&uarr;/&rarr;</kbd><br><kbd>Meta+Shift+h/j/k/l</kbd> | Move the selected pane
|<kbd>Meta+R</kbd> | Enter resize mode. Resize selected pane with arrow keys or <kbd>h/j/k/l</kbd>. Exit using any other key(s)

## Installation

You can clone this repository and run `cargo run` to launch the program.

## Progress
- [ ] Implementation of the stdout
- [x] Implementation of the stdin
- [x] pane creation
- [x] pseudo-terminal management
- [x] pane drawing
- [x] having a better modular architecture
- [ ] split and tabbed mode
- [ ] moving pane
- [ ] multiple workspace
