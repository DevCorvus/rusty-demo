# Rusty demo (Actix + Tauri + Yew)

A series of demo projects developed entirely in Rust with the purpose of learning, from the backend in Actix web to the frontend as a desktop application using Tauri and Yew as the UI framework.

The app as a whole is a basic CRUD with users and JWT authentication, nothing fancy but it includes almost everything you may expect from it like: diesel as ORM, password hashing with argon2, form validation in both backend and frontend, human-readable errors, yew router and yewdux in the frontend (routing and state management respectively), etc.

You are free to explore and use the code at your convenience. I hope you find it useful and thanks for reading. ❤️

## Development and thoughts on Rust stack (TL;DR)

I was really excited about making some stuff with Rust which is one of my favorite programming languages out there. There's nothing I can say about it to do justice, it's a gigachad, blazingly fast language IMO (or maybe it's not my opinion, maybe it's just what everyone else says and I'm a NPC, idk).

Looking to deepen my knowledge in Rust and therefore systems programming in general I decided to make something I'm familiar with... web development (I know, this is not exactly related to systems programming but let me live in my lie). That's how I ended up working in a demo project for Actix web and Yew as the frontend web app in wasm (Tauri wasn't in the picture yet).

Everything was going smoothly _-crying behind mask-_ until I decided to try serving the whole wasm frontend from Actix which went really bad. Maybe I'm incompetent but I couldn't find any solution at that moment. From what I guessed (I'm not sure), the issue was because Actix is unable to detect and set the right mime Content-Type for wasm files which is `application/wasm`, instead they were served as `text/plain`. The wasm mime type is not even supported in the rust `Mime` crate. I tried to set manually the Content-Type for wasm files but I found myself in a dead end with no documentation or clear support at all. I only found a few other poor masochists with the same problem and no answer. It was literally the first time where I felt like I was following leads that went nowhere and I don't know if I was too dumb to pull it off that way or there was just no support for serving a wasm frontend in Actix at the time.

I was about to publish the project as it was since serving the frontend from Actix or not didn't make much difference. However, I saw the opportunity to turn the obstacle into a new path. I was thinking of creating a desktop app in Tauri right after this project but thought why not integrate the existing Yew UI in Tauri and that's exactly what I did. It required little to no modification!

Filling two needs with one deed, I finished the development of this project from which I learned a lot of things but there are a couple of things that has to be said about Rust, at least for web development. Unless you're masochist or a gigachad developer (dunno distinguish the difference), I don't recommend using Rust over Javascript for frontend development. The shared type safety and error handling is great but at its current state, the Javascript ecosystem is far more mature, productive and easy. Rust just doesn't feel like the right tool for the job in this case (time might change that perspective). Aditionally, compilation times in Rust are a considerable burden that slow down development and therefore, iterations.

As for the backend and desktop apps, Rust fits as a better option. Backend frameworks in Rust like Actix are pretty decent but Golang is still my preferred choice there. Go is easier, and achieves almost the same results when it comes to performance and compilation times are way more fast. Tauri on the other hand has a huge potential to overcome Electron in desktop applications with web technologies because in comparison, it's just blazingly faster and more resource efficient.

Making this project was a lot of fun and headaches at the same time. I learned so much about Rust, its ecosystem and its use-cases which was the main goal and I'm truly happy about the result and the whole process. I think Rust is the perfect fit when you're looking (over anything else) for memory safety, resource efficiency, security and robustness, even more than performance. Anyway, you can still use a little of rusty code here and there when you need it, nothing stops you from including it in your stack and get its clear benefits. I still have to learn many things about Rust, I love what the language offers and what I can learn from it!

<img src="https://preview.redd.it/7vx28m7qso291.jpg?width=720&format=pjpg&auto=webp&v=enabled&s=1f285b49c2c8dbb0d8483e493d6704d675e59e9c" alt="Rust meme" />

_\- DevCorvus_
