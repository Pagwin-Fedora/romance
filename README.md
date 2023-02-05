# Romance(a CI pipeline)

As you may well know CI pipelines are the most romantic thing out there but there's something we can do to make them even more romantic. We can make our own in rust that works via git hooks.

## Build

`cargo build`
 
## Install/running

Romance is not a global install nor is it supposed to be run in an arbitrary location. The executable should be placed in a specific location in the bare directory that is present on remote. Specifically, assuming the root of the bare repo is `./bare`, the location this file should go is `./bare/post-receive`. If you want to learn more about git hooks [this](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks) is an alright place to start. If you want to run it manually without pushing more commits you can cd into the bare directory and run the executable there.

NOTE: please don't use this in prod I made it in less than 24 hours for a hackathon, the code is garbage and even if it wasn't I don't trust myself. Also I only tested this on linux and I suspect it'll only work on unix systems due to it using /tmp to store artifacts as a hard coded location.

## Using

To run CI jobs via Romance after putting the executable in place you add a .romance\_jobs.yml file to the repo
