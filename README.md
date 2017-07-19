# optimizelyd-maildir 

This is another approach to implementing optimizelyd as a maildir queue implementation.  Right now, it is a very generic maildir queue consumer that takes files from new moves them to cur, sends them via json if a request body was provided.  If there is no request body it is sent as a GET.  The response is read but not consumed at this point. The queued items are json human readable files that contain a url and a request body.

* since it uses the filesystem, it is very transparent and also easy to understand from an ops perspective.  
* it is very easy to scale via nfs mount.
* it is easy to scale with process as well since there can be several queue writers and several queue consumers at this time. 
* it uses os calls.
For these reasons, the [Rust](https://www.rust-lang.org/en-US/) programming language was chosen for implementation.

## Background
The queue is based on the lockless maildir queue.  The maildir queue file structure is shown below:

`
basedir/
  tmp/
  new/
  cur/
`

https://en.wikipedia.org/wiki/Maildir

The idea is that the client creates a unique filename (the one caveat) in tmp and then moves the file to new. File move is an atomic operation on all operating systems.  That is a `push` onto the queue.  
The optimizelyd-maildir picks up a file from new and moves it to cur, then the file is read and passed to the jsonsender to send.  That is the `pop`.

## Project status

* `src/` contains implementation of the maildirqueue, the jsonsender, and the main module to drive the consuming.
* `examples/` contains a sample maildirqueue directory.  You can test by running `../optimizelyd-maildir client` for client test and leave off the client `..\optimizelyd-maildir` if you want the server side. 

## Building

cargo build

### Prerequisites

```sh
# Install [rustup](https://github.com/rust-lang-nursery/rustup.rs), the Rust toolchain manager,
# with the instructions found at https://rustup.rs/. Current instructions as of May 2017:
curl https://sh.rustup.rs -sSf | sh

# Bash users, you can persist the just-installed CLI tools to your $PATH with
echo 'export PATH=$HOME/.cargo/bin:$PATH' >> ~/.bash_profile`
```

### Compiling, running tests, etc.

```sh
# From the repo root,
cargo build

