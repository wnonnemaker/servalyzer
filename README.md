### The Vision
It can analyze three parts of how a web server handles requests 
1. internal logic, code run to sort a request before any disk access
2. database or persistent data access (read/write to disk)
3. latency, or time it takes for requests to travel through the internet

Eventually I hope this tool will be smart enough to work on any system

# Status
Currently it is just a proxy, I haven't even added measuring time it takes to receive a response after receiving a request
You can run the tool as a proxy by running the tool
cargo run -- proxyport serverport

Very fun!
