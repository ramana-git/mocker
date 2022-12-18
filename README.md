# mocker
Simple HTTP Server to simulate transactions


Usage:
http://<dns/ip>[:port]/sleep/:duration

Server binds:
  Host: 0.0.0.0
  Port: 8080 (default)
  
PORT can be modified using environment variables.

Example (to simulate 1 second duration):
http://localhost:8080/sleep/1000

duration in milliseconds
