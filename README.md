# Lemmy-Terminal-Viewer

Terminal User Interface for lemmy for Linux Terminals (should work in MacOs but i can't test)

## Install and Usage 

### Linux 

* Download ```ltv-linux.tar.gz``` from [releases](https://github.com/LunaticHacker/lemmy-terminal-viewer/releases)
* Navigate to download location
* Extract it with ```tar -xf ltv-linux.tar.gz ```
* You can now execute the file using ```./ltv```
* To Install globally move the executable to /usr/bin with ```sudo mv ./ltv /usr/bin```

### Mac
Build the project yourself with cargo (no offical support)

### Loggin In

You can log in to as many accounts as you want in any number of instances

To add an account run

```
ltv login
```
You will be prompted to provide login details, if you successfully authenticate you will be redirected to your Feed;

To log in to an already added account, run
```
ltv instance.url username_or_email
```

Or you can always browse without logging in by running
```
ltv instance.url
```

### Setting up configs (Optional)
Path for storing configs

On Linux: ``` /home/alice/.config/ltv/```

On Mac: 
```/Users/Alice/Library/Application Support/dev.ltv.ltv/```

copy the [sample config](ltv.sample.toml) rename it to ltv.toml and save to the path given above
and finally make the changes you desire. All configs are explained in the sample config

### Navigation

- Navigation is based on Arrow keys (for now)

- use ⬆️ and ⬇️ keys to traverse lists of posts and comments

- when viewing a post press ⬇️ to see it's comments

- press ➡️ to see a comment's replies and ⬅️ to go back

### Browsing communities

In the default view press " i "  to enter edit-mode to select community, enter the name of community and press ➡️  to submit. use ⬅️ to exit editing mode.
