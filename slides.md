%title: Rust: From Zero to Crate in Sixty Minutes
%author: Christopher R. Miller
%date: 2018-10-06

-> Rust: From Zero to Crate in 60 Minutes <-
============================================

^
-> Rust is a systems language pursuing the trifecta: <-

^
-> safety <-
^
-> concurrency <-
^
-> speed <-
^

Used in several projects where speed is required without
sacrificing safety:

* Firefox
* Dropbox
* Redox
* OpenDNS

-------------------------------------------------

-> # Syllabus <-

1. Rust - speed, safety, and concurrency features
2. Toolchain - cargo, crates, dependencies
3. Example - querying a webservice with an api that wouldn't
   look out of place in a language like Ruby

-------------------------------------------------

-> # Speed <-

^
Compiled language like Java or C#; native code, like C or C++
^

Uses LLVM/MSVC to emit optimized machine code - so things like loop
unrolling and autovectorization come for free
^

Zero-cost abstractions: fluff that is expensive in other languages
is free in Rust (things like streaming over a collection with a
fluent syntax)

-------------------------------------------------

-> # Speed: Zero-cost Abstractions <-

Which one is faster?


    let mut acc = 0;              let sum_of_squared_odd_numbers: u32 =
                                    (0..).map(|n| n * n)
    for n in 0.. {                       .take_while(|&n| n < upper)
      let n_squared = n * n;             .filter(|n| (*n).is_odd())
                                         .fold(0, |sum, i| sum + i)
      if n_squared >= upper {
        break;
      } else if n_squared.is_odd() {
        acc += n_squared;
      }
    }
    acc
^

      0.787768ms                              0.740197ms
      111 page faults                         110 page faults
      1,702,824 cycles                        1,604,922 cycles

-------------------------------------------------

-> # Speed: Zero-cost Abstractions <-

The Rust compiler (rustc) is an aggressive optimizer

Elides common overhead, such as the iterator and state structures
from the preceding example

Automatic lambda inlining where possible

Rust lets you write what you want to say and handles the optimization
under the hood - mostly. :)

-------------------------------------------------

-> # Safety <-

Rust statically proves the memory-correctness of programs using a
borrow checker

All memory in Rust is "owned" by something, and can be borrowed

These checks are static - they fall away after compile and they
aren't in the built artifact

No segfaults or data races.

-------------------------------------------------

-> # Safety <-

    xpm@vegas$ cat borrowck1.rs
    fn main() {
        let s1 = String::from("hello");
        let s2 = s1; // move s1 to s2

        println!("{}, world!", s1);
    }
^

    xpm@vegas$ rustc borrowck1.rs
    error[E0382]: use of moved value: `s1`
     --> borrowck.rs:5:28
      |
    3 |     let s2 = s1;
      |         -- value moved here
    4 |
    5 |     println!("{}, world!", s1);
      |                            ^^ value used here after move
      |
      = note: move occurs because `s1` has type `std::string::String`,
        which does not implement the `Copy` trait

-------------------------------------------------

-> # Safety <-

This can be fixed by cloning (or copying) the string.

    xpm@vegas$ cat borrowck2.rs
    fn main() {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("{}, world!", s1);
    }
    xpm@vegas$ rustc borrowck2.rs
    xpm@vegas$ ./borrowck2
    hello, world!

-------------------------------------------------

-> # Safety <-

Ownership: you can send data to other parts of your program,
then that data is freed when its no longer used

    xpm@vegas$ cat borrowck3.rs
    fn main() {
        let s = String::from("hello");
        take_ownership(s); // s is now owned by take_ownership
        // and is freed as soon as the method returns

        println!("{}, world!", s);
    }
    fn take_ownership(s: String) { }

-------------------------------------------------

-> # Safety <-

    xpm@vegas$ rustc borrowck3.rs
    error[E0382]: use of moved value: `s`
      --> borrowck.rs:13:32
       |
    11 |         take_ownership(s);
       |                        - value moved here
    12 |
    13 |         println!("{}, world!", s);
       |                                ^ value used here after move
       |
       = note: move occurs because `s` has type `std::string::String`,
         which does not implement the `Copy` trait

-------------------------------------------------

-> # Safety <-

Borrowing: you can loan data to other parts of your program, when
it's done you get the data back

    xpm@vegas$ cat borrowck4.rs
    fn main() {
        let s = String::from("hello");
        borrow(&s);

        println!("{}, world!", s);
    }
    fn borrow(s: &String) { }
    xpm@vegas$ rustc borrowck4.rs
    xpm@vegas$ ./borrowck4
    hello, world!

-------------------------------------------------

-> # Safety <-

To allocate something on the heap you use a Box

Boxes also allow you to store things you don't know the
size of at compile time (strings, data structures,
lambdas, buffers, etc)

The reference to your data gets the same borrow
checker treatment as ordinary stack-allocated memory

By extension, heap-allocated memory gets ownership
semantics for free

If you've used C++ unique_ptr, this will be extremely
familiar

-------------------------------------------------

    xpm@vegas$ cat borrowck5.rs
    struct MyStruct {
      pub lambda: Box<Fn()> // store a lambda to call later
    }
    fn main() {
      let s = "hello";
      let speaker = MyStruct {
        // the move keyword transfers ownership of s
        // to the lambda - s is now invalid within main
        lambda: Box::new(move || println!("{}, world", s))
      };
      // call the lambda
      (speaker.lambda)();
    }
    xpm@vegas$ rustc borrowck5.rs
    xpm@vegas$ ./borrowck5
    hello, world

-------------------------------------------------

-> Sharing data <-

Sometimes you don't want to move data, sometimes it needs
to be shared - such as when building graphs or other kinds
of data structures

Rust provides a reference-counted box, called Rc, along with
a weak-pointer box, called Weak

If you remember programming in Objective-C before Arc, this
will be extremely familiar

If you've worked with C++'s shared_ptr and weak_ptr (or their
Boost C++ equivalents) this will be extremely familiar

-------------------------------------------------

    xpm@vegas$ cat rc.rs
    use std::rc::Rc;

    struct Owner { pub name: String }
    struct Node { pub owner: Rc<Owner> }

    fn main() {
      let o = Rc::new(Owner { name: "Owner".to_string() });

      let n1 = Node { owner: Rc::clone(&o) }; // cloning the Rc, not the
      let n2 = Node { owner: Rc::clone(&o) }; // O inside of it

      println!("References to o: {}", Rc::strong_count(&o)); // 3

      drop(o); // dispose of local variable o, releasing a reference

      println!("References to o: {}", Rc::strong_count(&n1.owner)); // 2

      // n1 and n2 are destroyed, dropping the refcount of o to
      // zero, so it too is destroyed
    }

-------------------------------------------------

    xpm@vegas$ rustc rc.rs
    xpm@vegas$ ./rc
    References to o: 3
    References to o: 2

-------------------------------------------------

-> # Concurrency <-

Rust enables "fearless concurrency"

The borrow checker catches use-after-free and unsafe
threading

-------------------------------------------------

    xpm@vegas$ cat thread1.rs
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }

-------------------------------------------------

    xpm@vegas$ rustc thread1.rs
    error[E0373]: closure may outlive the current function, but it borrows `v`,
    which is owned by the current function
     --> src/main.rs:6:32
      |
    6 |     let handle = thread::spawn(|| {
      |                                ^^ may outlive borrowed value `v`
    7 |         println!("Here's a vector: {:?}", v);
      |                                           - `v` is borrowed here
      |
    help: to force the closure to take ownership of `v` (and any other referenced
    variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^

-------------------------------------------------

-> # Message Passing <-

Rust supports message passing out of the box.

    xpm@vegas$ cat mp1.rs
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
    xpm@vegas$ rustc mp1.rs
    xpm@vegas$ ./mp1
    Got: hi

-------------------------------------------------

    xpm@vegas$ cat mp2.rs
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            println!("val is {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

-------------------------------------------------

Again, the borrow checker prevents us from accidentally
doing something unsafe when passing data between threads

    xpm@vegas$ rustc mp2.rs
    error[E0382]: use of moved value: `val`
      --> mp2.rs:10:31
       |
    9  |         tx.send(val).unwrap();
       |                 --- value moved here
    10 |         println!("val is {}", val);
       |                               ^^^ value used here after move
       |
       = note: move occurs because `val` has type `std::string::String`,
       which does not implement the `Copy` trait

-------------------------------------------------

What if I absolutely must share data across threads?

Like all languages, use a Mutex

    xpm@vegas$ cat mutex1.rs
    use std::sync::Mutex;

    fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }
    xpm@vegas$ rustc mutex1.rs
    xpm@vegas$ ./mutex1
    m = Mutex { data: 6 }

-------------------------------------------------

Sharing data between threads requires another step, however.

    xpm@vegas$ cat mutex2.rs
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Mutex::new(0);
        let mut handles = vec![];

        for _ in 0..10 {
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

-------------------------------------------------

    xpm@vegas$ rustc mutex2.rs
    error[E0382]: capture of moved value: `counter`
      --> mutex2.rs:10:27
       |
    9  |         let handle = thread::spawn(move || {
       |                                    ------- value moved (into closure) here
    10 |             let mut num = counter.lock().unwrap();
       |                           ^^^^^^^ value captured here after move
       |
       = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
         which does not implement the `Copy` trait

    error[E0382]: use of moved value: `counter`
      --> mutex2.rs:21:29
       |
    9  |         let handle = thread::spawn(move || {
       |                                    ------- value moved (into closure) here
    ...
    21 |     println!("Result: {}", *counter.lock().unwrap());
       |                             ^^^^^^^ value used here after move
       |
       = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
         which does not implement the `Copy` trait

-------------------------------------------------

You can put a Mutex on the heap using Arc, which is
just like Rc, only atomic

This allows the Mutex to be shared by multiple threads

-------------------------------------------------

    xpm@vegas$ cat arc1.rs
    use std::sync::{Mutex, Arc};
    use std::thread;

    fn main() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

-------------------------------------------------

    xpm@vegas$ rustc arc1.rs
    xpm@vegas$ ./arc1
    xpm@vegas$ Result: 10

-------------------------------------------------

Rust's concurrency safety is extensible

All data is marked as having membership or nonmembership
with the Send and Sync traits

Data that is Send can transferred across thread boundaries
(the ownership can be transferred)

Data that is Sync can be shared between threads (the data
can be borrowed)

-------------------------------------------------

-> # From Zero to Crate <-

1. Get rust
2. Write crate
3. ???
4. Profit!

-------------------------------------------------

-> # Rustup <-

https://rustup.rs

Manages installations of the Rust toolchain

Similar to  
  rvm  
  rbenv  
  nvm  
  asdf  

    unix:  curl https://sh.rustup.rs -sSf | sh
           add $HOME/.cargo/bin to $PATH
    win32: run https://win.rustup.rs/x86_64
           follow instructions

-------------------------------------------------

-> # Intro to Cargo <-

Cargo is a build system and dependency manager for Rust

Dependencies (Crates) are stored on crates.io, similar
to rubygems.org or npmjs.com

Making a new crate (library or program) is easy

-------------------------------------------------

-> # Intro to Cargo <-

Make a new binary (executable) crate called webget

    xpm@vegas$ cargo new webget --bin --name webget
         Created binary (application) `webget` project

Configure the crate using `Cargo.toml` file

    xpm@vegas$ cat Cargo.toml
    [package] # this is some basic information about your crate
    name = "webget" # the name of your crate, what others will call it
    version = "0.1.0" # the version of your crate
    authors = ["Christopher R. Miller <xpm@mysteriouspants.com>"]

-------------------------------------------------

-> # Intro to Cargo <-

Adding dependencies to your crate is easy - about as easy as it
is in Ruby, Maven, or Node

    xpm@vegas$ cat Cargo.toml # continued
    [dependencies] # your crate's dependencies
    clap = "2.32.0" # a command-line arg parsing library
    reqwest = "0.9.2" # a fluent http library built on async io

Find new libraries on crates.io

An excellent starting point is github.com/rust-unofficial/awesome-rust

The simplicity of adding new dependencies makes it easy to
try new ones and experiment

-------------------------------------------------

-> webget/src/main.rs <-

    extern crate clap; // these are going away in
    extern crate reqwest; // a future realease

    use clap::{Arg, App};
^

    fn main() {
^

      let args = App::new("webget") // parse args with Clap
        .version("1.0")
        .author("Desert Code Camp 2018")
        .about("Runs get on a url and prints the response")
        .arg(Arg::with_name("URL") // takes a single argument
          .required(true) // that is required
          .index(1)) // that isn't named
        .get_matches();
^

      let url = args.value_of("URL").unwrap(); // panic if no url given
^      

      let client = reqwest::Client::new();
      let mut respon = client.get(url).send().unwrap(); // GET the url
^

      println!("response = {:?}", respon); // print some high-level info
      println!("body = {:?}", respon.text()) // print the actual contents
    }

-------------------------------------------------

# Confession
^

The above code is entirely cross-platform
^

The following example program outputs come from my Windows PC
^

You can easily recompile (most) Rust programs for other targets

-------------------------------------------------

    target\debug\webget.exe --help
    webget 1.0
    Desert Code Camp 2018
    Runs get on a url and prints the response

    USAGE:
        webget.exe <URL>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <URL>

-------------------------------------------------

    target\debug\webget.exe https://www.mysteriouspants.com/
    response = Response { url: "https://www.mysteriouspants.com/"
                        , status: 200
                        , ... }
    body = Ok("<!DOCTYPE html>
               <html lang=\"en\">
               <head>
                  <!--
                    A city made of wood is built in a forest,
                    a city made of stone is built in the mountains,
                    but a city made of dreams is built in heaven.
                      Jovian proverb
                  -->
                ...")

-------------------------------------------------

What happens if we call a website that doesn't exist?

    target\debug\webget.exe https://www.mysterioustrousers.com/
    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
    Inner { kind: Hyper(Error { kind: Connect, cause: Some(Os { code: 10060, kind: TimedOut,
      ... }) }), url: Some("https://www.mysterioustrousers.com/") }', libcore\result.rs:945:5
    note: Run with `RUST_BACKTRACE=1` for a backtrace.
^

Again, with the backtrace:

    target\debug\webget.exe https://www.mysterioustrousers.com/
    thread 'main' panicked at ...
    stack backtrace:
    ...
      11: webget::main
                 at ./src/main.rs:18
    ...

-------------------------------------------------

-> What if webget was parallel? <-
^

In addition to system threads (pthreads or winthreads), Rust
supports async
^

The library from the previous example, reqwest, is a layer on
Hyper, an HTTP implementation on Tokio, an async i/o library,
which is built on mio, a native cross-platform async io library
^

Until we called .text() the response was "unused" and hadn't
waited to consume all its async message yet

-------------------------------------------------

-> webget/src/main.rs <-

    extern crate clap;
    extern crate reqwest;

    use std::iter::FromIterator;
    use std::thread;
    use clap::{Arg, App};

-------------------------------------------------

-> webget/src/main.rs (continued) <-

    fn main() {
      let args = App::new("webget")
        .version("1.0")
        .author("Desert Code Camp 2018")
        .about("Runs get on a url and prints the response")
        .arg(Arg::with_name("URL")
          .required(true)
          .index(1)
          .multiple(true))
        .get_matches();

-------------------------------------------------

-> webget/src/main.rs (continued) <-

    let handles = Vec::from_iter(
      args.values_of("URL").unwrap().map(|u| {
        let url = String::from(u);
        thread::spawn(move || {
          reqwest::get(&url)
        })
      })
    );
  
-------------------------------------------------

-> webget/src/main.rs (continued) <-

      for handle in handles {
        let mut result = handle.join().unwrap();
        match result {
          Ok(mut r) => {
            println!("response = {:?}", r);
            println!("body = {:?}", r.text());
          },
          Err(e) => {
            println!("Failed to get {:?}", e);
          }
        }
      }
    }

-------------------------------------------------

    target\debug\webget.exe \
      https://www.mysterioustrousers.com/ \
      https://www.mysteriouspants.com/ \
      https://www.rust-lang.org/

-------------------------------------------------

    Failed to get Inner { kind: Hyper(Error { kind: Connect, ... })
                        , url: Some("https://www.mysterioustrousers.com/") }
    response = Response { url: "https://www.mysteriouspants.com/"
                        , status: 200
                        ... }
    body = Ok("<!DOCTYPE html>
               <html lang=\"en\">
               <head>
                <!--
                    A city made of wood is built in a forest,
                    a city made of stone is built in the mountains,
                    but a city made of dreams is built in heaven.
                      Jovian proverb
                -->
                ...")
    response = Response { url: "https://www.rust-lang.org/"
                        , status: 200
                        ... }
    body = Ok("<!DOCTYPE html>
               <html>
               <head>
                 <title>The Rust Programming Language</title>
               ...")

-------------------------------------------------

-> Vending Code <-

Vending code as a Rust crate is easy

Cargo provides a built-in test runner and documentation
generator (similar to doxygen or javadoc)

Cargo also runs and tests the code snippets in your documentation,
so it's easy to catch incorrect documentation as the code itself
changes

-------------------------------------------------

    #[cfg(test)]
    mod tests {
      use std::time::Instant;
      use Throttle;

      #[test]
      fn it_works() {
        // simple throttle configured for 10 TPS
        let mut throttle = Throttle::new_tps_throttle(10.0);

        // the first one is free
        throttle.acquire(());

        let iteration_start = Instant::now();

        for _i in 0..10 {
            throttle.acquire(());
        }

        assert_eq!(iteration_start.elapsed().as_secs() == 1, true);
      }
    }

-------------------------------------------------

    /// Creates a new `Throttle` with a constant delay of `tps`<sup>-1</sup> &middot; 1000 ms, or
    /// `tps`-transactions per second.
    ///
    /// ```rust
    /// # extern crate mysteriouspants_throttle;
    /// # use std::time::{Duration, Instant};
    /// # use mysteriouspants_throttle::Throttle;
    /// let mut throttle = Throttle::new_tps_throttle(0.9);
    ///
    /// // the first one is free!
    /// throttle.acquire(());
    ///
    /// let start = Instant::now();
    /// throttle.acquire(());
    /// assert_eq!(start.elapsed().as_secs() == 1, true);
    /// ```
    pub fn new_tps_throttle(tps: f32) -> Throttle<TArg> {
      ...
    }

-------------------------------------------------

    xpm@vegas$ cargo test
       Compiling mysteriouspants-throttle v0.2.5
        Finished dev [unoptimized + debuginfo] target(s) in 1.04s
    running 7 tests
    test tests::it_works_with_no_delay_at_all_variable ... ok
    test tests::it_works_with_no_delay_at_all_tps ... ok
    test tests::enforce_sync ... ok
    test tests::enforce_send ... ok
    test tests::it_works_with_duration_smaller_than_already_elapsed_time ... ok
    test tests::it_works_more_complicated ... ok
    test tests::it_works ... ok
    test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
       Doc-tests mysteriouspants-throttle
    running 4 tests
    test src/lib.rs - Throttle<TArg>::new_variable_throttle (line 113) ... ok
    test src/lib.rs -  (line 35) ... ok
    test src/lib.rs -  (line 11) ... ok
    test src/lib.rs - Throttle<TArg>::new_tps_throttle (line 148) ... ok
    test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

-------------------------------------------------

-> # Takeaway Slide <-

Rust is a safe, fast, modern language that empowers you to
do complicated things with confidence

Mature tools make it easy to build and vend software

Easily composable abstractions make things easy that were
previously the domain of higher-level languages

Even if you don't use Rust, the principles of ownership
will help you write correct code in other languages

rustup.rs - try it today

-------------------------------------------------

Christopher R. Miller
https://www.mysteriouspants.com/
https://github.com/mysteriouspants/
xpm@mysteriouspants.com
mysteriouspants on irc.freenode.net

Resources

rust-lang.org
users.rust-lang.org
crates.io
rustup.rs
github.com/rust-unofficial/awesome-rust

irc.mozilla.net/#rust
irc.mozilla.net/#rust-beginners
this-week-in-rust.org

-------------------------------------------------
                                                                                              
       `-////-     `--. `-::.   -::-       `:///:.     `---------`    `-:::.     `--. `-::.
     .yNNNNNNNm+   oNNN+mNNNNy/mNNNNm:   :hNNNNNNNd:   oNNNNNNNNN/  .yNNNNNNd/   oNNm+mNNNNh`
     dNNy. .yNNN-  oNNNd//hNNNmo/oNNNh  .mNNo` .dNNm`  .---:dNNNy` -mNNy.`/NNNo  oNNNd+/hNNN+
       `-://yNNN:  oNNN:  -NNNs   dNNd     .-://dNNN`     -dNNm/   hNNN`   hNNN. oNNN-  .NNNs
     :hNNmyshNNN:  oNNN-  -NNNo   dNNd   /dNNdysmNNN`    +NNNh.    mNNm`   sNNN: oNNN.  `NNNs
    :NNNs   oNNN:  oNNN-  -NNNo   dNNd  oNNN+   hNNN`  .hNNNy.`    hNNN.   yNNN. oNNN.  `NNNs
    +NNNy``:mNNNy` oNNN-  -NNNo   dNNd  yNNN+``+NNNNo `mNNNNNNNmh/ :NNNy``/NNNs  oNNN.  `NNNs
    `yNNNNNmsyNNy. oNNN-  -NNNo   dNNd  .dNNNNNmohNms``Ndyo++osyms  -hNNNNNNd/   oNNN.  `NNNs
      .:/:-   -.   `---    ---`   ---.    .:/:.   -`     `....`       `://:.     .---    ---.
                 .-.`                                  .-....:/-
                   .::-.``                              `.-` -/`
                     `.-:/::-..```              ``..-::/:.` `/.
                         `.-:///////::::::::://////:-.`     -`
                              ``..---::::::---..``

-> Work Hard. Have Fun. Make History. <-

-> Come visit us in the Expo Area <-

-> Event Dinner tonight <-
-> Hanger 9 <-
-> 980 E. Pecos Road <-
-> Chandler, AZ 85225 <-
-> 6pm – 9pm <-
