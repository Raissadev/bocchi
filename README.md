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

Pass Args (in root)
```zsh
$ ./bocchi -r # reload plt
```

```zsh
$ ./bocchi -v <id_video> # post a single video, ex: RM4U148k5jA
```

```zsh
$ ./bocchi -vr # random video of the playlist
```


## Requirements
```zsh
$ mv .env.example .env # and put of data;
# It is necessary to configure the yt-dlp utility;
# You need to have ffmpeg on the machine;
# Paths must have write and read permission;
# If you are going to use systemd change the data to match your machine;
# ...
# and yes... Facebook has the feat of needing different tokens for publishing and media publishing
```

the climb to the server is like the Incas did...

#### That's it, so... Shinzo wo sasageyo