# minigrep
This is the simple `minigrep` command line tool from the book "The Rust Programming Language,"
chapter 12.  

Usage:
```
$ ls
poem.txt minigrep

$ cat poem.txt
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

$ ./minigrep to poem.txt
Are you nobody, too?
How dreary to be somebody!

$ IGNORE_CASE=1 ./minigrep to poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
