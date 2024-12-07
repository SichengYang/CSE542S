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
We first solved the 
