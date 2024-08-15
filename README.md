# Bocchi

Code made sporadically, for geek music lovers, <br/>
Download (and compress) videos from a public playlist and then upload to Instagram.<br/>
New videos are published daily, with a curated selection of at least one old video per day.<br/>
```
__     ____                 _     _ 
\ \   | __ )  ___   ___ ___| |__ (_)
 \ \  |  _ \ / _ \ / __/ __| '_ \| |
 / /  | |_) | (_) | (_| (__| | | | |
/_/___|____/ \___/ \___\___|_| |_|_|
 |_____|                            
```

## Usage

```zsh
$ make

Usage: make <target>
  build                      Build application
  run                        Run application
  docker.build               Build docker image
  docker.run                 Run docker image
```

## Run

```zsh
$ docker build -t raissageek/bocchi . && \
    docker run -it --rm raissageek/bocchi bash
# Or
$ make build && make run
```

Running in background (in root)
```zsh
$ ./systemd.sh # executable script ∴ set permission to execute
```

```zsh
$ sqlite3 ./database/database.db < ./database/init.sql
```


## Requirements
```zsh
$ mv .env.example .env # and put of data
# and yes... Facebook has the feat of needing different tokens for publishing and media publishing
```


the climb to the server is like the Incas did...

#### That's it, so... Shinzo wo sasageyo