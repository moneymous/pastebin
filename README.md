# pastebin
A simple pastebin clone.

This is a simple project where we wrote a simple Pastebin clone in Rust using Axum, Next.JS and MySQL.

# Development notes:-
1. You should have Rust, Node.JS and MySQL installed.

2. The backend folder contains backend code, run `cargo run` to  start it, it will start listening at 127.0.0.1:3000.

3. The frontemd folder contains frontend code, run `npm run dev` to start it, it will start listening at 127.0.0.1:8000.

4. You should have MySQL database on.

5. Now according to your database creds edit `pastebin/backend/src/db.rs`.

6. Create a database for the project
   Commands:-
   ```sql
   CREATE DATABASE pastebin;

   USE pastebin;

   CREATE TABLE pastes (
       id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
       paste TEXT NOT NULL,
       lang varchar(5) NOT NULL
   );
   ```


   # How to use the API
   After running the backend app

   Send a post request to `http://127.0.0.1:3000/make-paste` to make a paste. There should be a paste body and lang.
   Example request:-
   ```json
   {
       "paste": "<h1>Hello, world!</h1>",
       "lang": "html"
   }
   ```


   Send a get request to `https://127.0.0.1:8000/get-paste/id/<paste-id>`, where `<paste-id>` is your paste ID, to get a paste. You will get a JSON response, containing `lang` and `paste`, where `lang` is the language in which the code is written and `paste` is the code.
