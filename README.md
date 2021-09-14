# Lemmy-Terminal-Viewer

A WIP  Terminal User Interface for lemmy for Linux Terminals (should work in MacOs but i can't test)

## Install and Usage (for now)

You will need the rust toolchain to run the project until it's relatively stable to produce releases
for now you'll have to clone the repo and build locally to use it.

### Loggin In

You can log in to as many accounts as you want in any number of instances

To add an account run

```
cargo run login
```
You will be prompted to provide login details, if you successfully authenticate you will be redirected to your Subscribed Feed :);

To log in to an already added accouunt run
```
cargo run instance.url username_or_email
```

Or you can always browse without logging in by running
```
cargo run instance.url
```

### Navigation

- Navigation is based on Arrow keys (for now)

- use Up and Down keys to traverse lists of posts and comments

- when viewing a post press Down arrow to see it's comments

- you can view only one level of deep nested comments (for now)

### Browsing communities

In the default view press "i" to enter edit-mode to select community, enter the name of community and press Right arrow to submit. use Left arrow to exit edit mode.

## Planned Features
-  Configs for several aspects of the app including but not limited to theming
-  view N-level Deep nested comments
-  parse and render markdown of post and comments
