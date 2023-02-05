# Romance(a CI pipeline)

As you may well know CI pipelines are the most romantic thing out there but there's something we can do to make them even more romantic. We can make our own in rust that works via git hooks.

## Build

`cargo build`

## Install/running

Romance is not a global install nor is it supposed to be run in an arbitrary location. The executable should be placed in a specific location in the bare directory that is present on remote. Specifically, assuming the root of the bare repo is `./bare`, the location this file should go is `./bare/post-receive`. If you want to learn more about git hooks [this](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks) is an alright place to start. If you want to run it manually without pushing more commits you can cd into the bare directory and run the executable there.
