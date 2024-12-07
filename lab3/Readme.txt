CSE 542 Fall 2024 Lab 3

## Basic Information about Project

* Group member:
Qinzhou Song qinzhounick@wustl.edu
Sicheng Yang sicheng@wustl.edu

* Program Design
This program includes a server and a client. The server starts and waits for connection from clients and receive messages from clients.
We first modified the client from lab2 so that it uses multithreading to read from config files and print out the lines in the scene.
Then, we desgined the server so that it starts a TcpListener and listen to connection requests. When the server receives a connection request 
    and the file name from a client, it reads all the content in the file and writes back to the client so that the client can print it out.
Lastly, we designed a test client to check if the connection requests are sent and received correctly and the tokens are printed out by the server.
    If the token says quit, the server quits properly.

* Observations:
This assignment focused refactoring the client to use multithreading in the beginning and there was a learning curve to it. 
We had problems with variable ownerships and both starting and ending a thread properly as the scope of lab3 is bigger than the studio practices.
However, we implemented error checking carefully throughout the entire lab and was able to see error messages when we encountered failures.
The debugging part for Tcp connection was surprisingly easy as we learned lessons from piazza posts and Dr. Gill. Our test client worked for the first try.
Our client, however, was behaving incorrectly as we added a loop to read lines and sending the server, but we never ended the loop.
We think that the lab was fun and intuitive.




## Detailed Instruction

* How to run our program
1. Unzip the lab3 folder
2. Open a terminal in lab3 folder and cd into the lab3server folder. Run with "cargo run 127.0.0.1:7777"
3. Open another terminal in lab3 folder and cd into the lab3testclient folder. Run with "cargo run 127.0.0.1:7777 filename.txt"
4. Open aanother terminal in lab3 folder and cd into the lab3client folder. Run with "cargo run net:127.0.0.1:7777:filename.txt"





## Testing

* Description: 
We first solved the infinite loop bug from lab2 before starting lab3. Then, we implemented thread-safe outputs and data sharing(Step 1-4) in client.
    We tested with "cargo run normal.txt", which is the same as partial_hamlet_act_ii_script.txt, and the program outputs the scene correctly. 
    We also tested without filename and with non-existing file, the program correctly outputs error message, indicating functional thread-safe outputs
Then we added the multithreading file operations(Step 5-6) by changing play.rs, player.rs, and scenefragment.rs following the intructions. 
    We tested with "cargo run normal.txt" for correct behavior.
    We tested with "cargo run file_io.txt" which includes a play file that does not exist. The warning message says that thread panics 
        and main returns the right error code.
We moved on to implement the server(step 7-12), we implemented the TcpListener and TcpStream connection in server.rs. Then, we designed the test client
that connects to the server and send tokens to server.
    We tested with "cargo run 127.0.0.1:7777" to start the server and "cargo run 127.0.0.1:7777 1234". The server receives 1234 and quits after one second wait.
    We also tested with "cargo run wrong" to test the server with bad network addresses.
    We tested with "cargo run wrong 1234" to test the testclient with bad network addresses.
Lastly, we modified the client(Step 13-14) to implement remote file IO, adding get_buffered_reader to create a reader for network connection and calling it in grab_trimmed_file_lines.
    The testing of this part is in the Testing section.

* Testing:

1. Server and client correct behavior

a) Remote file:
Server: cargo run 127.0.0.1:7777 
----Server output------------------------------------------------------------------------------------------------------------------------------
Received: normal.txt from 127.0.0.1:59168

Client: cargo run net:127.0.0.1:7777:normal.txt 
----Client output------------------------------------------------------------------------------------------------------------------------------
Hamlet Prince of Denmark ACT II Scene I A room in Polonius house by William Shakespeare 
[Enter Polonius.]
[Enter Reynaldo.]

Polonius
Give him this money and these notes, Reynaldo.

Reynaldo
I will, my lord.
...
[Exit Ophelia.]
[Exit Polonius.]

Hamlet Prince of Denmark ACT II Scene II A room in the Castle by William Shakespeare
[Enter King.]
[Enter Queen.]
[Enter Rosencrantz.]
[Enter Guildenstern.]

King
Welcome, dear Rosencrantz and Guildenstern!
Moreover that we much did long to see you,
...
[Exit Guildenstern.]
[Exit Rosencrantz.]
[Exit Queen.]
[Exit King.]
----END----------------------------------------------------------------------------------------------------------------------------------------

b) Local file IO:
Client: cargo run normal.txt
----Client output------------------------------------------------------------------------------------------------------------------------------
same as client remote test result

c) Remote file IO
Server: cargo run 127.0.0.1:7777 
----Server output------------------------------------------------------------------------------------------------------------------------------
Received: macbeth_i_2b_config_mod.txt from 127.0.0.1:60872
Received: macbeth_i_1_config_mod.txt from 127.0.0.1:60871
Received: macbeth_i_2a_config_mod.txt from 127.0.0.1:60870

Client: cargo run partial_macbeth_act_i_script_mod.txt
This file includes lines like "net:127.0.0.1:7777:macbeth_i_2a_config_mod.txt" and "DUNCAN net:127.0.0.1:7777:DUNCAN_macbeth_i_2a_mod.txt"
----Client output------------------------------------------------------------------------------------------------------------------------------
correctly output Macbeth scene one and scene two


d) multiple client running:
Server: cargo run 127.0.0.1:1234 
----Server output------------------------------------------------------------------------------------------------------------------------------
Received: normal_remote.txt from 127.0.0.1:33354
Received: hamlet_ii_1a_config.txt from 127.0.0.1:33370
Received: hamlet_ii_1b_config.txt from 127.0.0.1:33372
Received: hamlet_ii_2a_config.txt from 127.0.0.1:33378
Received: normal_remote.txt from 127.0.0.1:33392
Received: hamlet_ii_1b_config.txt from 127.0.0.1:33404
Received: hamlet_ii_2a_config.txt from 127.0.0.1:33414
Received: hamlet_ii_1a_config.txt from 127.0.0.1:33426
Received: normal_remote.txt from 127.0.0.1:33442
Received: normal_remote.txt from 127.0.0.1:33446
Received: hamlet_ii_1a_config.txt from 127.0.0.1:33462
Received: hamlet_ii_1b_config.txt from 127.0.0.1:33464
Received: hamlet_ii_2a_config.txt from 127.0.0.1:33478
Received: hamlet_ii_1a_config.txt from 127.0.0.1:33494
Received: normal_remote.txt from 127.0.0.1:33496
Received: hamlet_ii_1b_config.txt from 127.0.0.1:33506
Received: hamlet_ii_2a_config.txt from 127.0.0.1:33516
Received: hamlet_ii_1a_config.txt from 127.0.0.1:33524
Received: hamlet_ii_2a_config.txt from 127.0.0.1:33542
Received: hamlet_ii_1b_config.txt from 127.0.0.1:33540
Received: normal_remote.txt from 127.0.0.1:33544
Received: hamlet_ii_1a_config.txt from 127.0.0.1:33556
Received: hamlet_ii_1b_config.txt from 127.0.0.1:33548
Received: hamlet_ii_2a_config.txt from 127.0.0.1:33560

Client: cargo run net:127.0.0.1:1234:normal_remote.txt & cargo run net:127.0.0.1:1234:normal_remote.txt & cargo run net:127.0.0.1:1234:normal_remote.txt & cargo run net:127.0.0.1:1234:normal_remote.txt & cargo run net:127.0.0.1:1234:normal_remote.txt & cargo run net:127.0.0.1:1234:normal_remote.txt &
----Client output------------------------------------------------------------------------------------------------------------------------------
Multiple copies of normal output generated at a sudden. This server is nice!


2. Test client correct behavior
Server: cargo run 127.0.0.1:7777 
----Server output------------------------------------------------------------------------------------------------------------------------------
Received: 1234 from 127.0.0.1:59246
Received: quit from 127.0.0.1:59247

Client: cargo run 127.0.0.1:7777 1234
----Client output------------------------------------------------------------------------------------------------------------------------------
Write 4 bytes to stream: 1234
Received: HTTP/1.1 400
Received: Server message: File 1234 cannot be read
Write 4 bytes to stream to quit
Wait for one second
Wake the server out of accept call. Server should shutdown now.




3. Server bad command line
Server: cargo run 
----Server output------------------------------------------------------------------------------------------------------------------------------
usage: target\debug\lab3server.exe <address>
         --Warning: Error: 1

Server: cargo run hello world
----Server output------------------------------------------------------------------------------------------------------------------------------
usage: target\debug\lab3server.exe <address>
         --Warning: Error: 1

Server: cargo run 127.0.0.1:wrong
----Server output------------------------------------------------------------------------------------------------------------------------------
         --Warning: Error: 2
PS: We use error code 2 for bad network address

4. Client bad command line

Server: not started
Client: cargo run
----Client output------------------------------------------------------------------------------------------------------------------------------
usage: target\debug\lab3client.exe <script_file_name> [whinge]
         --Warning: Error: 1
PS: Error code 1 is used for bad commandline

Server: not started
Client: cargo run
----Client output------------------------------------------------------------------------------------------------------------------------------

PS: Error code 2 is used for failing to generate script

Server: not started
Client: cargo run file_io.txt
----Client output------------------------------------------------------------------------------------------------------------------------------
thread '<unnamed>' panicked at src\lab3\player.rs:71:17:
Thread panics in Player prepare
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at src\lab3\scene_fragment.rs:90:31:
Thread panics in SceneFragment prepare
         --Warning: Error: 3
PS: Error code 3 is used for concurrency hazard


Server: not started
Client: cargo run net:127.0.0.1:7777:normal.txt
----Client output------------------------------------------------------------------------------------------------------------------------------
         --Warning: Invalid file name: net:127.0.0.1:7777:normal.txt
         --Warning: Error: 4
PS: Error code 4 is used for connection failures

Server: not started
Client: cargo run no_existing.txt
----Client output------------------------------------------------------------------------------------------------------------------------------
         --Warning: Invalid file name: no_existing.txt
         --Warning: Error: 5
PS: Error code 5 is used for fail open local file

Server: cargo run 127.0.0.1:7777
----Server output------------------------------------------------------------------------------------------------------------------------------
Received: no_existing.txt from 127.0.0.1:59374
Client: cargo run net:127.0.0.1:7777:no_existing.txt
----Client output------------------------------------------------------------------------------------------------------------------------------
Server response with status HTTP/1.1 400
Server message: File no_existing.txt cannot be read
         --Warning: Invalid file name: net:127.0.0.1:7777:no_existing.txt
         --Warning: Error: 7
PS: Error code 7 is used for fail open remote file


Server: cargo run 127.0.0.1:7777
----Server output------------------------------------------------------------------------------------------------------------------------------
Received: ./partial_macbeth_act_i_script_mod.txt from 127.0.0.1:61070
Client: cargo run net:127.0.0.1:7777:./partial_macbeth_act_i_script_mod.txt
----Client output------------------------------------------------------------------------------------------------------------------------------
Server response with status HTTP/1.1 400
Server message: $, /, \, and .. is not permited
         --Warning: Invalid file name: net:127.0.0.1:7777:./partial_macbeth_act_i_script_mod.txt
         --Warning: Error: 7
PS: Error code 7 is used for fail open remote file



Server: cargo run 127.0.0.1:7777
----Server output------------------------------------------------------------------------------------------------------------------------------
Received: file_io.txt from 127.0.0.1:59484
Client: cargo run net:127.0.0.1:7777:file_io.txt
----Client output------------------------------------------------------------------------------------------------------------------------------
thread '<unnamed>' panicked at src\lab3\player.rs:71:17:
Thread panics in Player prepare
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at src\lab3\scene_fragment.rs:90:31:
Thread panics in SceneFragment prepare
         --Warning: Error: 3
PS: file_io.txt include non-existing player


Remote file IO with whinge
Server: cargo run 127.0.0.1:7777 
----Server output------------------------------------------------------------------------------------------------------------------------------
Received: macbeth_i_2b_config_mod.txt from 127.0.0.1:61045
Received: macbeth_i_1_config_mod.txt from 127.0.0.1:61043
Received: macbeth_i_2a_config_mod.txt from 127.0.0.1:61044

Client: cargo run partial_macbeth_act_i_script_mod.txt whinge
This file includes whinge triggers.
----Client output------------------------------------------------------------------------------------------------------------------------------
         --Warning: Config file line "net:127.0.0.1:7777:macbeth_i_1_config_mod.txt  STILL MORE EXTRA TOKENS" has extra tokens
         --Warning: Config file line "" length not equal to 2
         --Warning: Config file line "ALL ALL_macbeth_i_1_mod.txt  THESE ARE EXTRA TOKENS" length not equal to 2
         --Warning: Config file line "ROSS ROSS_macbeth_i_2b_mod.txt  THESE ARE MORE EXTRA TOKENS" length not equal to 2
         --Warning: Token NAN does not represent a value in usize
Macbeth ACT I SCENE I An open Place Thunder and Lightning by William Shakespeare
[Enter FIRST_WITCH.]
[Enter SECOND_WITCH.]
[Enter THIRD_WITCH.]
[Enter ALL.]
         --Warning: Character line "0" missing

FIRST_WITCH
When shall we three meet again?
In thunder, lightning, or in rain?
...