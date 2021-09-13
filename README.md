# Lemmy-Terminal-Viewer

This app is a  WIP and currently supports Redox, Mac OS X, and Linux (or, in general, ANSI terminals).

## Install and Usage (for now)

* clone the repo

```
cargo run "instance.url"
```
if instance url is not provided it will default to lemmy.ml.

### Navigation

- Navigation is based on Arrow keys (for now)

- use Up and Down keys to traverse lists of posts and comments

- when viewing a post press Down arrow to see it's comments

- you can view only one level of deep nested comments (for now)

### Browsing communities

In the default view press "i" to enter edit-mode to select community, enter the name of community and press Right arrow to submit. use Left arrow to exit edit mode.

## Planned Features

-  Auth
-  Configs for several aspects of the app including but not limited to theming
-  view N-level Deep nested comments
-  parse and render markdown of post and comments
