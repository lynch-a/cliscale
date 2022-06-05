Takes a raw command as an argument and some input from stdin, then pipes the stdin lines to other hosts running the raw command. Aggregates all stdout output.

It works by spawning an SSH connection to any number of configured workers. The command you pass to cliscale will be run in the SSH session. Each incoming stdin line will be distributed (likely unevenly) to the stdin of one SSH session. All SSH stdout is returned through cliscale's stdout.

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

[remote_worker_1] # if you can ssh to a host and the host knows how to run the command passed to cliscale, it should "just work".
connection_string=username@example.com
binpaths=/home/username/go/bins:/usr/bin

$ wc -l domains
517 domains

$ time cat domains | httprobe -c 100 -t 3000
..omitted stdout..
httprobe -t 2000: 20.708 total

$ time cat domains | cliscale httprobe -c 100 -t 3000
..omitted stdout..
cliscale httprobe -t 2000: 12.936 total

```

todo
* add optional port and sshkey options for ssh connection
* ship with empty ini file and print friendly message if it hasn't been configured yet.
* add -test argument that verifies the configured ssh connections are working
* aggregate any output files created by remote workers
* handle some errors more cleanly / polish