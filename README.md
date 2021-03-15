## Another Word Count

This is a simple Linux/BSD cli or Windows console executable which mimics (somehow) the well known *wc* GNU
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

Usage:

```
Another word count v 0.3
Alain Viguier dandyvica@gmail.com
A word counters inspired by the GNU wc command.

            Project home page: https://github.com/dandyvica/awc
            
            

USAGE:
    awc [FLAGS] [FILES]

FLAGS:
    -a, --all
            same as -bclLMw       

    -b, --bytes
            print the byte counts

    -c, --chars
            print the character counts

    -h, --help
            Prints help information

    -l, --lines
            print the newline counts (UNIX) or LF/CR counts (Windows)

    -L, --max-line-length
            print the maximum display width

    -M, --min-line-length
            print the minimum display width

    -w, --words
            print the word counts

    -z, --zip
            means the input file is gzipped
```

# Caveat
This is mainly used to count UTF-8 files, without a BOM (Bye Order Mark). it doesn't handle UCS/2, UTF-16 or UTF-32 encoded files.
