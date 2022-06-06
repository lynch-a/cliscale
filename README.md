`cliscale` turns this:

```
$ wc -l domains
47590 domains

$ time cat domains | httprobe -c 100 -t 3000 | tee alive
..omitted stdout..

time taken: 6 minutes 54 seconds
```

into this:

```
$ time cat domains | cliscale httprobe -c 100 -t 3000 | tee faster-alive
..omitted stdout..

time taken: 30 seconds
```

`cliscale` distributes incoming stdin to processes running on other computers. It works well with command-line tools that take line based input from stdin and provide simple, unordered output to stdout. I mostly tested this with the `httprobe` tool by @tomnomnom, but it should be compatible with any stdin/stdout oriented tool where the order of output doesn't matter.

It works by spawning a simple SSH connection to any number of configured workers (such as spare hosts on your homelab or some large ec2 instances). The command you pass as an argument to `cliscale` will be run in each SSH session. Each incoming stdin line will be distributed (likely unevenly) to the stdin of one SSH session. All SSH session stdout is returned through `cliscale`'s stdout.

![example pic](/example.png "example")

Configuration and usage guide:
```
$ which cliscale
/mybins/cliscale # just make sure the cliscale binary is in your $PATH somewhere

$ which httprobe # we want to "scale" this command
/home/myuser/go/bins/httprobe

$ cat /mybins/cliscale.ini # put your configuration ini next to the cliscale binary
[local_worker]
connection_string=myuser@localhost
binpaths=/usr/bin:/home/myuser/go/bins # for cli scale to work, you have to specify $PATH that is used during the ssh session

[remote_worker_1] # if you can ssh to here and the remote host knows how to run the command passed to cliscale, it should "just work".
connection_string=username@example.com
binpaths=/home/username/go/bins:/usr/bin

$
$
$
$ cat domains | cliscale httprobe -c 100 -t 3000
...<stdout>...

```

todo
* add optional port and sshkey options for ssh connection
* ship with empty ini file and print friendly message if it hasn't been configured yet.
* add -test argument that verifies the configured ssh connections are working
* aggregate any output files created by remote workers
* handle some errors more cleanly / polish
* option to use different config files
* (maybe) figure out how unevenly the work is distributed.