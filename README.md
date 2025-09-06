# Chat Application in Rust
it is an simple cli based chat application that uses TCP sockets to commiunticate , written in rust. it implements topics like multithreading, socket programming, mutex and concurrency.

## Client-Side
The way client works is that firstly we connect to server then there are two seperate threads running one for reading from server and other is writing from server

## Server-Side
The way server works is that it start listening for data once an client connects to server.its start an new thread for that client and store that client in an mutex vector that stores all the clients when server recieves dat from any client it sends to all the clients connected by iterating over that vector.

When **New Client** connected to server its pushed into the mutex vector and given an new thread.
When an client **Disconnected** is deletes its entry from mutex vector and that thread is ended automatically.
