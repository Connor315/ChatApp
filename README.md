# Real-Time Chat Application

## Team Members
- Chen Wang (wangc425 | 1006058926 | chennn.wang@mail.utoronto.ca)
- Kangzhi Gao (gaokangz | 1006307827 | kangzhi.gao@mail.utoronto.ca)
- Yalin Tuo (tuoyalin | 1006033196 | yalin.tuo@mail.utoronto.ca)

<!-- Motivation: What motivated your team to spend time on this project? An excellent project idea is satisfying and fun to work on, and fills a gap that may not be easily found in the Rust ecosystem. -->
## Motivation
Our team wanted to build this real-time chat application because we found severe problems with many existing chat platforms. These platforms offer a lot of functionality, but they tend to suffer from performance degradation when the number of users is high, resulting in slower messaging and impacting the user experience. In addition, the interface design of these platforms, while fully functional, is overly complex, making it potentially confusing and costly for new users to learn when using them for the first time. In contrast, our application is designed to be lightweight and easy to use, focusing on core features such as real-time messaging, easy-to-manage chat rooms, and clear online status displays. By keeping the interface simple and straightforward, we wanted to improve the user experience, reduce unnecessary complexity, and make the application easier to use and more efficient.

Our goal was to create an application that would run quickly in highly simultaneously situations, ensuring a smooth flow even when a large number of users are online at the same time. This design approach not only allowed us to address new technical challenges, but also provided the team with opportunities to innovate and improve. The team's extensive experience in user login and interface design gave us the confidence to build an application that was both secure and easy to use. In addition, we chose to develop in Rust, a language popular for its high performance and efficient resource management, but uncommon for real-time chat applications. The use of Rust allowed our project to provide a simple, efficient, and powerful alternative to complex applications. Our goal was to create a clean and robust solution focused on solving the performance and usability issues of current chat applications, ultimately providing users with a communication tool that is both powerful and easy to use.

<!-- Objectives: What are the objectives of this project? -->

## Objective
The main goals of our live chat app are to make it better and solve common problems with online communication tools. We want to work hard on research and development. This will help the app meet modern user needs. The project also aims to create special features and strong performance. Our goal is for the app to stand out in the market.

First of all, a key aim of this project is to offer users quick and smooth communication that feels instantaneous. To achieve this, server load is managed efficiently, and network protocols are optimized. Real-time applications like ours depend on WebSockets for quick communication and asynchronous operations for handling tasks without waiting. The Yew framework helps maintain the app's state effectively, ensuring smooth functionality. By using WebAssembly for rendering on the client side, server pressure is lowered, and response time is faster. Stability is improved through reliable error handling and automatic reconnections, so users stay connected even during temporary issues. These features combine to provide a responsive and reliable chatting experience online.

Our goal is to build a backend that stays fast as more users join. We plan to use SQLite for user login and sled for saving chat messages, with the option to switch to larger cloud databases later if needed. We focus on managing data well by using resources wisely, even though advanced features like load balancing and data partitioning are not included yet. This chat app is made to support many users at the same time. It uses async programming to handle requests quickly and WebSockets to keep real-time chats open with little delay. On the client side, WebAssembly helps by taking some tasks off the server, reducing its workload and making the app faster to respond. The app manages resources carefully to avoid wasting effort, ensuring it works well even with more traffic. Strong error handling and automatic reconnections keep the app stable, so users stay connected during short interruptions. These features support growth, but more upgrades might be needed if the app must handle a much larger user group.

Besides, it boost much users like using our product by making the interface simple to use. It should be neat, without too much going on, and focus on making things easy for users with clear signs for where to click and smooth changes between screens. The live chat app has a user-friendly interface that makes it simple and clear for users to interact. It uses the Yew framework to keep the interface organized and predictable, with every part easy to handle. Navigation is straightforward, featuring clearly marked buttons and obvious cues for interactions. It also includes helpful error messages that inform users about what's happening with their actions. The consistent look of buttons and input fields throughout the app helps users feel comfortable and familiar quickly. Altogether, these design choices create a straightforward user environment that helps all users, no matter their tech skill level, to get used to and use the app effectively.

Use Rust’s speed and memory safety to build a strong and fast chat app. This real-time chat app uses Rust’s best features to make a stable and responsive platform. By following Rust’s ownership and borrowing rules, the app avoids memory problems during concurrent tasks, stopping issues like data races. Async programming in Rust makes it faster, letting the app handle network requests and I/O tasks without slowing down the interface. Serde helps by quickly and safely converting chat messages for network use, which improves communication efficiency. Compiling Rust into WebAssembly allows fast client-side processing, cutting server work and making the app more responsive. On the backend, Rust manages WebSocket connections, keeping chats smooth and fast for frequent message exchanges. Rust’s strong type system and memory control ensure the app stays stable and reliable.

Our Real-Time Chat Application fills a gap in the Rust ecosystem by providing a complete real-time chat solution that’s built entirely with Rust. Currently, there aren’t many complete options in Rust for building chat apps that include secure user authentication, quick messaging, and a user-friendly interface. By using several Rust frameworks and libraries like Rocket for secure backend functions, WebSocket for quick, real-time messaging, and Yew for the user interface, we’re showing that Rust can be used to make responsive and interactive applications.

By emphasizing these key features, we aim to highlight the potential for Rust in developing modern web applications that are not only scalable and performant but also maintain a high standard of security and simplicity.

<!-- Features: What are the main features offered by the final project deliverable? -->
## Features
Our real-time chat application offers a variety of features to enhance user experience, including persistent storage with databases, user authentication, the ability to create and join chat rooms, real-time messaging, presence detection, and a user-friendly interface. Below is a detailed description of these features:

### Persistent Storage
Our application utilizes two lightweight databases for local persistent storage: a relational database (SQLite) using the `sqlx` library and a NoSQL database powered by the `sled` crate. SQLite is used to store sensitive data such as usernames, passwords, chat room names, and owners, as these are accessed less frequently, resulting in lower workload and enhanced data security through the `sqlx` library’s built-in features. In contrast, the sled NoSQL database handles less sensitive but frequently accessed data, such as channel messages and user statuses, providing high-performance read and write operations. These databases were chosen for their lightweight nature and lack of external dependencies, initializing automatically when the application is first run. This combination ensures a balance of performance, scalability, and security for an optimal user experience.

### User Authentication
Our application implements user authentication using password hashing provided by the `pwhash` crate. User credentials, including usernames and hashed passwords, are securely stored in the database to protect sensitive information. The `username` field in the `Users` table is designed to be unique, ensuring no duplicate accounts can exist with the same username. The authentication process supports user registration and login by handling API `POST` requests. During registration, the application hashes the user's password before storing it in the database. For login, the provided credentials are validated by matching the username and verifying the hashed password against the stored data. This approach ensures a secure and straightforward implementation of user authentication while safeguarding user data. Once a user successfully logs in, the application stores the user ID and username in the session. This allows for convenient access to user information for authorization-required operations within the application, eliminating the need to repeatedly send API requests to verify the user for each action.

### Channel Creation and Joining
Our application allows users to create new channels or join existing ones created by others. In the database design, the `channel` table includes a unique `name` field to ensure consistency and enhance user experience, as well as a foreign key linking the channel to the username of its creator. Each channel is designed for private conversations, ensuring that only users who join a specific channel can participate and view its messages. When a user creates a channel, they automatically become the owner of that channel. Users joining an existing channel connect to its corresponding WebSocket channel, enabling real-time communication. Messages sent by users in a channel are broadcast to all participants in the same channel through the server, enabling real-time group chatting within the private conversation.

### Real-Time Messaging
Our application uses WebSocket, powered by `actix-web-actors`, to enable full-duplex communication for real-time messaging functionality. This allows the server and client to send and receive messages simultaneously over a single persistent connection, ensuring low-latency communication. Unlike HTTP, where connections must be re-established for each request, WebSocket connections remain open as long as needed, significantly reducing latency and making it ideal for real-time applications. We use the `chrono` crate to record the timestamp for each message sent, ensuring accurate tracking and ordering of messages. Additionally, all message data is stored in the NoSQL database for later retrieval, allowing users to access chat history as needed. WebSocket also allows servers to push data updates directly to users, eliminating the need for frequent database polling, reducing redundant queries, and optimizing database performance while providing a smooth messaging experience.

### View Channel History
When users join an existing channel, our application automatically fetches the complete channel history from the database, including messages from all users, and displays them in a user-friendly format on the interface to enhance the user experience. The history fetch operation occurs only once upon entering the channel to minimize backend workload. Subsequent chat messages are temporarily stored in memory for real-time display and persistently stored in the database for long-term access. This approach ensures a smooth user experience while reducing the frequency of heavy read operations on the database, optimizing application performance.

### Presence Detection
Our application includes a presence detection feature to track user statuses in real time. Alongside the chat component, each channel has its own user status list that displays active and inactive users specific to that channel. Color indicators are used to represent user statuses: green signifies users currently in the channel, red indicates users who have joined the channel before but are not present at the moment, and users not listed have never joined the channel. This functionality is implemented within WebSocket, with user statuses stored in the NoSQL database. When a user joins a channel, their status is updated to "online," and when they exit, it changes to "offline." To ensure real-time updates, user statuses are refreshed every 5 seconds. The performance impact remains minimal since the number of status entries for a channel is limited to the total number of application users, maintaining efficient resource utilization.

### User Interface
Our application includes a lightweight Rust-based project folder dedicated to a user-friendly interface, developed using the Yew framework. The interface features multiple pages, including a home page, registration, login, chat creation, chat list, and chat window. By leveraging the `gloo` and `gloo-net` crates, the application efficiently handles API requests to integrate the frontend with the backend and facilitates seamless navigation between pages. This integration ensures a smooth and responsive user experience. The design and functionality of the user interface can be viewed in our demo video or explored directly through the application.

<!-- Reproducibility Guide: What are the commands needed to set up the runtime environment, if any, and to build the project, so that its features can be used by a user or a developer? Note: The instructor will follow the steps you have included in this section, step-by-step, with no deviation. The instructor has access to a Ubuntu Linux server and a macOS Sonoma laptop computer. -->
## Reproducibility Guide
Please follow these steps to set up and run the project:

1. Install [Rust](https://www.rust-lang.org/) and `wasm-pack`:
     ```bash
     cargo install wasm-pack
     ```
2. Clone the project and navigate into it:
     ```bash
     git clone <repository-url>
     cd chat
     ```
3. Navigate to the `frontend` folder and compile it, outputting to the `../static` directory:
     ```bash
     cd frontend
     wasm-pack build --target web --out-dir ../static --release
     ```
4. From the project root directory, build and run the backend:
     ```bash
     cd ..
     cargo build --release
     ./target/release/chat
     ```
5. After seeing successful printouts in the terminal, access the app via browser at [http://localhost:8080/](http://localhost:8080/).

### Notes
- The first time building the project may take longer as Rust downloads and compiles all dependencies.

## User’s Guide
Our real-time chat application allows users to communicate with each other in chat rooms similar to other online chat platforms. Follow the instructions below to start chatting:

### 1. Home Page
Upon launching the application, you will be directed to the home page. Here, you will see options to either log in or register. Simply click the corresponding button to proceed.

### 2. Log in or Register
Users must log in or register to use the application. Registration requires you to create a username and password. There are no specific restrictions on the format or length of these credentials, as the primary focus of the application is on the chat functionality. After registering, you can use these credentials to log in and access the chat features.

### 3. Create or Enter a Channel
In the application, a "channel" represents a chat room where users can communicate with each other. To create a new channel, click the "create a channel" button and provide a unique name for your channel. The application ensures that channel names are unique, and you can consult the "available channel list" to avoid duplication. Once created, the channel becomes accessible, and you will automatically be designated as the owner. If you want to join an existing channel, you can select it from the channel list and click "enter channel" to join the conversation.

### 4. Chat with Other Users
After entering a chat room, you will be able to view all previous messages sent in the channel, along with the sender’s username and the time each message was sent. When you join, a broadcast message such as "*<Username>: <Username> joined the chat*" will appear in the chat for all users to see. To send a message, simply type it into the input field and press "send." Your message, along with your username and the time, will immediately be added to the chat history and displayed for all users in the channel.

### 5. User Status Indicators
The chat interface includes a user status panel on the right-hand side, which provides real-time information about user activity. Users marked in green are currently online, while those marked in red are offline but have been in the channel before. If a user’s name is not listed in the status panel, it means they have never joined the channel.

### 6. Multiple Users Chat
The application supports simultaneous communication between multiple users in the same channel. If you are running the application locally, you can simulate multiple users by using different browsers or browser sessions. Log in with separate credentials and join the same channel to enjoy our real-time chat service.

<!-- Contributions by each team member: What were the individual contributions by each member in the team? -->
## Contributions
**Chen Wang:**
- Implemented real-time communication using websocket.
- Implemented frontend-backend integration of real-time communication.
- Implemented channel enter.
- Implemented channel history.

**Kangzhi Gao:**
- Implemented database and user authentication.
- Implemented frontend-backend integration 
- Implemented channel creation.
- Implemented frontend-backend integration.

**Yalin Tuo:**
- Implemented frontend login and registration.
- Implemented channel list and chat room page design.
- Implemented channel selection.
- Implemented user logout.

<!-- Lessons learned and concluding remarks: Write about any lessons the team has learned throughout the project and concluding remarks, if any. -->
## Learnings
While building our real-time chat app, we gained valuable insights and practical experience in using modern technologies and frameworks to address complex challenges. This project provided a comprehensive understanding of managing high workloads, such as handling API requests, database operations, and real-time communication with the server, while maintaining high performance and reliability. By combining SQL and NoSQL databases, we learned to balance fast data processing with consistent and reliable data management.

A significant learning experience came from working with the `actix-web` framework, particularly its integration with WebSocket functionality using `actix-web-actors`. Implementing WebSocket for real-time messaging helped us understand the fundamentals of full-duplex communication and how to handle persistent connections efficiently. We learned to manage client-server interactions, handle concurrent connections, and broadcast messages to specific channels in real-time. This process deepened our knowledge of how WebSocket works and how frameworks like `actix-web-actors` can simplify implementing such functionality in scalable systems.

On the frontend, we explored the `yew` framework, which introduced us to building dynamic web applications using Rust. A key takeaway was using the `spawn_local` function in WebAssembly for managing asynchronous tasks, ensuring real-time updates and responsive user interactions in the browser. This experience helped us connect frontend functionality with backend WebSocket communication, enabling a real-time chat system.

Data exchange consistency was another point where we gained valuable experience. Using `serde` for serializing and deserializing data helped us maintain a unified data structure between the frontend and backend. Rust’s strong static typing system further reinforced this consistency, allowing us to catch errors during compilation rather than at runtime. This approach reduced bugs and ensured a more reliable application.

Overall, this project was a rich learning journey that enhanced our expertise in Rust’s ecosystem. It provided practical knowledge in using tools like `actix-web`, `yew`, and `serde` while demonstrating the importance of concurrency, and efficiency in building real-time systems. This experience has significantly improved our ability to design and implement high-performance, reliable applications.