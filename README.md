This was the 1st-prize winning entry for the Golem Hackathon at LambdaConf 2024.

That was on May 8th, 2024.

Commits after that were mostly polishing things.

# start golem containers
``` bash
docker compose up
```

# load fish commands
``` bash
source cmds.fish
```

# add component the first time
``` bash
add_component
deploy
```

# redeploy new code
This will build the latest source and update the golem system with the updated component.

``` bash
redeploy
```
