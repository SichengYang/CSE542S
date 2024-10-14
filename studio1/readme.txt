Studio 1
1. 
Member: Nick Song qinzhounick@wustl.edu

2.
iht32-1502.engr.wustl.edu

3. 
[qinzhounick@iht32-1502.sif ~]$ rustc --version
rustc 1.71.0 (8ede3aae2 2023-07-12)
[qinzhounick@iht32-1502.sif ~]$ cargo --version
cargo 1.71.0 (cfd3bbd8f 2023-06-08)
[qinzhounick@iht32-1502.sif ~]$ rustdoc --version
rustdoc 1.71.0 (8ede3aae2 2023-07-12)

4. 
[qinzhounick@iht32-1501.sif cse542]$ cat hello/Cargo.toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
[qinzhounick@iht32-1501.sif cse542]$ cat hello/src/main.rs
fn main() {
    println!("Hello, world!");
}

5.
[qinzhounick@iht32-1501.sif hello]$ cargo run
   Compiling hello v0.1.0 (/home/warehouse/qinzhounick/cse542/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/hello`
Hello, world!

6.
[qinzhounick@iht32-1501.sif hello]$ ./target/debug/hello
Hello, world!

7.
The guideline mainly told us to read the instructions carefully, write down the procedures and thoughts in the readme file, and test thoroughly, meaning think about the edge cases where your program could fail. It also mentioned about coding standard, where we should stay away from magic numbers, hard coding, and unreadable coding styles.

8.
[qinzhounick@iht32-1501.sif cmdargs]$ ./target/debug/cmdargs foo bar arg1 arg2
arguments passed to ./cmdargs were ["foo", "bar", "arg1", "arg2"]
[qinzhounick@iht32-1501.sif cmdargs]$ ./target/debug/cmdargs
usage: ./cmdargs <arg1> [<arg2> ...]

Code without arguments:
if args.len()==0{
        println!("usage: ./cmdargs <arg1> [<arg2> ...]");

Code with arguments:
}else{
        println!("arguments passed to ./cmdargs were {:?}", args);
}

9.
[qinzhounick@iht32-1502 cmdargs]$ ./target/debug/cmdargs
usage: ./cmdargs <arg1> [<arg2> ...]
[qinzhounick@iht32-1502 cmdargs]$ echo $?
1
[qinzhounick@iht32-1502 cmdargs]$ ./target/debug/cmdargs foo bar
arguments passed to ./cmdargs were ["foo", "bar"]
[qinzhounick@iht32-1502 cmdargs]$ echo $?
0

10.
[qinzhounick@iht32-1502 cmdargs]$ ./target/debug/cmdargs 
usage: ./cmdargs <arg1> [<arg2> ...]
Error: 1
[qinzhounick@iht32-1502 cmdargs]$ echo $?
1
[qinzhounick@iht32-1502 cmdargs]$ ./target/debug/cmdargs foo bar arg1 arg2
arguments passed to ./cmdargs were ["foo", "bar", "arg1", "arg2"]
[qinzhounick@iht32-1502 cmdargs]$ echo $?
0

In the case without arguments, it differs from the previous step since it also prints out the message "Error 1". The output with arguments is the same.

11.
[qinzhounick@iht32-1502 cmdargs]$ cargo run
   Compiling cmdargs v0.1.0 (/home/warehouse/qinzhounick/cse542/studio1/cmdargs)
error: negative integers cannot be used to index on a `Vec<String>`
  --> src/main.rs:12:10
   |
12 |     args[-1] = 0;
   |          ^^ cannot use a negative integer for indexing on `Vec<String>`
   |
help: to access an element starting from the end of the `Vec<String>`, compute the index
   |
12 |     args[args.len() -1] = 0;
   |          ++++++++++

error: could not compile `cmdargs` (bin "cmdargs") due to previous error

12.
[qinzhounick@iht32-1502 cmdargs]$ ./target/debug/cmdargs foo bar arg1 arg2
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5', src/main.rs:12:26
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
