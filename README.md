## Another Word Count

This is a simple Linux/OSX cli or Windows console executable which mimics (somehow) the well known *wc* GNU
command line utility. It reads any UTF-8 file as *wc* does and prints out data to the standard output.

This was meant to give a blueprint for: 

* managing command line arguments (but without *clap*)
* reading and managing file data
* reading compressed gzip files transparently using the *flate2* crate

Usage is the same as the *wc* GNU command, but with more consistant flags (e.g.: *-c* is for couting chars while it's meant for bytes in *wc*). Combining flags is possible (e.g.: *-bcw*).
In addition, the *-z* flag indicates the input file is gzipped.

Examples:

```bash
$ awc -bc /var/log/*.log
```

```cmd.exe
C:\> awc -bc c:\windows\system32\*.xml
```


