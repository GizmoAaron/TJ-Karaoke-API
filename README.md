# TJ-Karaoke-API
This is a small rust app that pulls Karaoke Song Data for the TJ Noraebang systems.
The front end just has a search box and the user enters in their keyword to search for songs, could be song name, artist, w.e. 
Then it makes an api call to API Gateway in AWS. API Gateway routes that call to a Lambda Function which runs a python script. 
That python script goes to S3 and grabs and filters out the songs which match that keyword, then creates an html table with all the corresponding song data. 
It finally serves that html back to api gateway, which routes it back to the user. 
The frontend then just uses javascript to append that html it got from the response back to the dom, and boom, we have our application.
[Link here](https://searchpage.gizmoaaron.repl.co/)
## Architecture Diagram
![SongAPI](https://user-images.githubusercontent.com/20212190/220777105-f38dbbd2-f771-4cbe-8105-33c02b638faa.svg)
## Song Search
![alt text](https://user-images.githubusercontent.com/20212190/220775360-b324a0b1-5adb-4534-a4bb-7871d7511643.png)
![alt text](https://user-images.githubusercontent.com/20212190/220775408-9e9a2524-5b10-4dbb-91e0-528046efa304.png)
