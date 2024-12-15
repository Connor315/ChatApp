## Real-Time Chat Application Final Report

#### Team Members
- Chen Wang (wangc425 | 1006058926 | chennn.wang@mail.utoronto.ca)
- Kangzhi Gao (gaokangz | 1006307827 | kangzhi.gao@mail.utoronto.ca)
- Yalin Tuo (tuoyalin | 1006033196 | yalin.tuo@mail.utoronto.ca)

<!-- Motivation: What motivated your team to spend time on this project? An excellent project idea is satisfying and fun to work on, and fills a gap that may not be easily found in the Rust ecosystem. -->

### Motivation

Our team wanted to build this real-time chat application because we found severe problems with many existing chat platforms. These platforms offer a lot of functionality, but they tend to suffer from performance degradation when the number of users is high, resulting in slower messaging and impacting the user experience. In addition, the interface design of these platforms, while fully functional, is overly complex, making it potentially confusing and costly for new users to learn when using them for the first time. In contrast, our application is designed to be lightweight and easy to use, focusing on core features such as real-time messaging, easy-to-manage chat rooms, and clear online status displays. By keeping the interface simple and straightforward, we wanted to improve the user experience, reduce unnecessary complexity, and make the application easier to use and more efficient.

Our goal was to create an application that would run quickly in highly simultaneously situations, ensuring a smooth flow even when a large number of users are online at the same time. This design approach not only allowed us to address new technical challenges, but also provided the team with opportunities to innovate and improve. The team's extensive experience in user login and interface design gave us the confidence to build an application that was both secure and easy to use. In addition, we chose to develop in Rust, a language popular for its high performance and efficient resource management, but uncommon for real-time chat applications. The use of Rust allowed our project to provide a simple, efficient, and powerful alternative to complex applications. Our goal was to create a clean and robust solution focused on solving the performance and usability issues of current chat applications, ultimately providing users with a communication tool that is both powerful and easy to use.

<!-- Objectives: What are the objectives of this project? -->

### Objective


<!-- Features: What are the main features offered by the final project deliverable? -->
### Features
For easy-to-use and user-friendly, we choose to use Sqlite by using sqlx and one nosql database using sled crate rather than using Postgresql which requires external dependencies, as you will see in the user's guide and Reproducibility guide, our setup procedures is super simple and easy to use and understand.
User authentications such as ... is considered lower volumn compared to storing chat messages persistently in real life and existing productions, since ...so that we can use sqlite database with relational sql for data integrity and minimize errors, and use nosql database for chat messages to optimize read and write performance since messaging is priorities in our application with high volumn requests where we want to store the chat messages persistently as well.

<!-- User’s (or Developer’s) Guide: How does a user — or developer, if the project is a crate — use each of the main features in the project deliverable? -->
### User’s Guide

Since it is a real-time chat application, a user is able to talk to other users by joining a chat room just like every other normal online chat application following the instructions below:

1. Log in or register:

   > Users need to either log in or register using their self-created credentials (username and password). We don't have any limitations on the format or length of them since our main goal is on the chatting portion.

2. Create or enter a channel:

   > The "channel" here represents a chat room where you can see other users' status and chat with them. 
   >
   > Clicking on the "create a channel" button and inputting a unique "channel name" helps you successfully create a channel owned by yourself. You can also check the "available channel list" to avoid duplicate channel names.
   >
   > After successfully creating a channel, you will be navigated back to the channel list page where you can simply choose a channel and click the "enter channel" button to enter the room.

3.  Chat with other users:

   > After entering the chat room, you can see all the history messages sent in the channel before, along with the sent user and time. 
   >
   > A broadcast message like "*Username: Username joined the chat*" along with the time will be shown in the middle of the screen so everyone in the channel will find you joining.
   >
   > Every time you input some message and press "send", a new message along with your username and current time will be added to the chat history and every other users in this channel will be able to see it.

4. User status (green: online, red: offline):

   > On the right hand side you can see a user status panel where green means the user is currently online and red means the user has entered this channel before but left the channel right now.

5. Multiple users chat:

   > Multiple users can join the same channel and chat with each other if you open the same server twice. Simply logging in using two different credentials and joining the same channel, you can enjoy our real-time chat service.

<!-- Reproducibility Guide: What are the commands needed to set up the runtime environment, if any, and to build the project, so that its features can be used by a user or a developer? Note: The instructor will follow the steps you have included in this section, step-by-step, with no deviation. The instructor has access to a Ubuntu Linux server and a macOS Sonoma laptop computer. -->

### Reproducibility Guide
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

#### Notes
- The first time building the project may take longer as Rust downloads and compiles all dependencies.

<!-- Contributions by each team member: What were the individual contributions by each member in the team? -->
### Contributions

**Chen Wang:**
- Implemented real-time communication using websocket
- Implemented frontend-backend integration of real-time communication
- Implemented channel enter
- Implemented channel history

**Kangzhi Gao:**
- Implemented database and user authentication
- Implemented integration 
- Implemented channel creation
- Implemented frontend-backend integration

**Yalin Tuo:**
- TODO

<!-- Lessons learned and concluding remarks: Write about any lessons the team has learned throughout the project and concluding remarks, if any. -->

### Learnings
