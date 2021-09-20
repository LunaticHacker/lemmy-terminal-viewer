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
You will be prompted to provide login details, if you successfully authenticate you will be redirected to your Feed;

To log in to an already added account, run
```
cargo run instance.url username_or_email
```

Or you can always browse without logging in by running
```
cargo run instance.url
```

### Setting up configs (Optional)
Path for storing configs

On Linux: ``` /home/alice/.config/ltv/```

On Mac: 
```/Users/Alice/Library/Application Support/dev.ltv.ltv/```

copy the [sample config](ltv.sample.toml) rename it to ltv.toml and save to the path given above
and finally make the changes you desire. All configs are explained in the sample configs

### Navigation

- Navigation is based on Arrow keys (for now)

- use Up and Down keys to traverse lists of posts and comments

- when viewing a post press Down arrow to see it's comments

- press right arrow to see a comment's replies and left arrow to go back

### Browsing communities

In the default view press "i" to enter edit-mode to select community, enter the name of community and press Right arrow to submit. use Left arrow to exit edit mode.

## Planned Features
-  parse and render markdown of post and comments
