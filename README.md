## Another Word Count

This is a simple Linux/OSX cli or Windows console executable which mimics (somehow) the well known *wc* GNU
command line utility. It reads any UTF-8 file as *wc* does and prints out data to the standard output.

This was meant to give a blueprint for: 

* managing command line arguments
* reading and managing file data
* reading compressed gzip files transparently using the *flate2* crate