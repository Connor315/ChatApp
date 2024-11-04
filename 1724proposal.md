## ECE1724 Project Proposal

#### Team Member: Chen Wang, Kangzhi Gao, Yalin Tuo

### Motivation

Our team wanted to build this real-time chat application because we found severe problems with many existing chat platforms, such as Slack and Microsoft Teams. These platforms offer a lot of functionality, but they tend to suffer from performance degradation when the number of users is high, resulting in slower messaging and impacting the user experience. In addition, the interface design of these platforms, while fully functional, is overly complex, making it potentially confusing and costly for new users to learn when using them for the first time. In contrast, our application is designed to be lightweight and easy to use, focusing on core features such as real-time messaging, easy-to-manage chat rooms, and clear online status displays. By keeping the interface simple and straightforward, we wanted to improve the user experience, reduce unnecessary complexity, and make the application easier to use and more efficient.

Our goal was to create an application that would run quickly in highly simultaneously situations, ensuring a smooth flow even when a large number of users are online at the same time. This design approach not only allowed us to address new technical challenges, but also provided the team with opportunities to innovate and improve. The team's extensive experience in user login and interface design gave us the confidence to build an application that was both secure and easy to use. In addition, we chose to develop in Rust, a language popular for its high performance and efficient resource management, but uncommon for real-time chat applications. The use of Rust allowed our project to stand out in a marketplace full of complex, resource-hungry applications by providing a simple, efficient, and powerful alternative. Our goal was to create a clean and robust solution focused on solving the performance and usability issues of current chat applications, ultimately providing users with a communication tool that is both powerful and easy to use.

### Objective and Key Features

#### **Objective**

The objective of this project is to create a high-performance, real-time chat application built entirely using Rust. This application aims to provide a secure, efficient, and user-friendly platform for communication. By focusing on a lightweight design and key functionalities, the project aims to show the potential of Rust in developing interactive, scalable, and resource-efficient web applications.

Scalability is a core focus for this project. Our chat application is designed to handle large numbers of users and maintain reliable performance even as the user base grows. We plan to achieve this by structuring the server in a way that can be easily scaled up or out. This means that as more users join and use the platform, we can add more server instances or resources to share the workload without causing delays or slowdowns.

Rust’s strong support for asynchronous programming will help manage thousands of active connections at the same time without the server becoming overwhelmed. This is particularly important for real-time messaging, where every millisecond counts to ensure smooth user interactions. We’ll also include features like load balancing, which will distribute incoming user traffic evenly across different servers, making sure that no single server gets overloaded.

To manage data efficiently, we will use a database that can be scaled horizontally, meaning we can add more database servers to store user and message data as needed. Additionally, by using caching solutions, we can keep frequently accessed information readily available, speeding up responses and reducing pressure on the main database.

By incorporating these strategies, we aim to build an application that not only performs well but continues to provide a fast and seamless experience as it grows and more users join. This scalable approach demonstrates that Rust can power modern, real-time applications that need to handle heavy loads efficiently and reliably.

#### **Key Features**

1. **User Authentication**: Using the Rocket framework for the backend, we will implement a secure authentication system to manage user sessions and ensure data protection. Rocket’s intuitive routing and type safety will allow us to build robust user management features, facilitating secure login and access control. The steps will include:
   - Setting up user registration and login routes in Rocket.
   - Implementing password hashing using the Argon2 crate.
   - Creating JWTs for user session management and verifying tokens for secured routes.
2. **Chat Room Creation and Joining**: The ability to create and join chat rooms will facilitate both group discussions and private conversations. Each chat room will be managed using Rocket’s backend infrastructure, ensuring data consistency and smooth operation. This feature will support various use cases, from collaborative work environments to casual chats among friends. The key steps will include:
   - Designing the database schema to store chat room details and user associations.
   - Implementing routes for creating, joining, and listing available chat rooms.
   - Building the backend logic for assigning users to chat rooms and handling permissions for public and private chat rooms.
3. **Real-Time Messaging**: WebSocket will be used for real-time messaging, enabling low-latency communication between users. By establishing a persistent WebSocket connection, the application will facilitate instant message delivery without the overhead of frequent HTTP requests. We will leverage Rust’s async/await syntax with WebSocket to achieve non-blocking message handling, ensuring efficient performance as the number of users scales. Specific steps will include:
   - Implementing the async/await syntax to ensure non-blocking message handling, enabling concurrent message processing.
   - Setting up message broadcasting logic so that messages are sent to all connected users in a chat room in real-time.
   - Using **serde** for serializing and deserializing JSON messages exchanged over WebSockets to keep the communication structured and efficient.
4. **Presence Detection**: Presence detection will enhance user experience by showing who is online or offline in real time. This feature is crucial for fostering an interactive and connected environment. Our implementation will involve WebSocket connections to maintain an active state and update user presence dynamically. Rust’s concurrency model will help manage numerous connections efficiently, supporting high scalability and reliable state management. The detailed approach includes:
   - Creating a connection tracking system using a shared state to record users' connection status.
   - Broadcasting presence updates when users connect or disconnect, ensuring up-to-date status in the chat rooms.
   - Integrating this with the frontend to display online/offline indicators for users in real-time.
5. **User Interface**: For user interaction, we will develop a lightweight front-end using Yew, a Rust-based framework for building web applications. Yew will allow us to create an interactive web-based interface with real-time features, showcasing Rust’s potential for frontend development as well. The steps for this feature include:
   - Designing the UI layout using Yew components to create a chat interface with features like chat lists, chat windows, and user status indicators.
   - Implementing WebSocket clients in Yew to handle incoming and outgoing messages and status updates in real-time.
   - Connecting the frontend and backend through WebSocket endpoints and ensuring messages and user statuses are rendered dynamically on the UI.

Our Real-Time Chat Application fills a gap in the Rust ecosystem by providing a complete real-time chat solution that’s built entirely with Rust. Currently, there aren’t many complete options in Rust for building chat apps that include secure user authentication, quick messaging, and a user-friendly interface. By using several Rust frameworks and libraries like Rocket for secure backend functions, WebSocket for quick, real-time messaging, and Yew for the user interface, we’re showing that Rust can be used to make responsive and interactive applications.

By emphasizing these key features, we aim to highlight the potential for Rust in developing modern web applications that are not only scalable and performant but also maintain a high standard of security and simplicity.

### Tentative Plan

Our group consists of three members: Chen Wang, Yalin Tuo, and Kangzhi Gao. To successfully deliver the Real-Time Chat Application within a matter of weeks, we have divided the project tasks into four main sections: backend development, real-time chat implementation, frontend development, and presence detection implementation. Here’s our tentative plan for task assignments in each section:

#### Backend Development

- **Set Up the Backend Framework using Rocket (Yalin Tuo & Kangzhi Gao)**: Yalin and Kangzhi will initiate the project by configuring the Rocket framework. This step will lay the foundation for all backend functionalities including routes, middleware, and database connections.
- **User Authentication Implementation (Kangzhi Gao & Chen Wang)**: Concurrently, Kangzhi and Chen will develop a secure authentication system to manage user logins and sessions. This system is crucial for protecting user data and ensuring that only authorized users can access the chat functionalities.

#### Real-Time Chat Implementation
- **WebSocket (Kangzhi Gao & Chen Wang)**: This team will implement WebSocket technology to facilitate real-time messaging. Their focus will be on achieving low-latency communication, which is essential for the seamless exchange of messages in the chat application.
- **Chat Room Creation and Joining (Kangzhi Gao & Chen Wang)**: Alongside WebSocket implementation, they will also develop the features that allow users to create and join chat rooms. This includes handling data structures and server-side logic to manage multiple chat rooms and user interactions within them.

#### Frontend Development
- **Design and Implement the User Interface using Yew (Yalin Tuo & Kangzhi Gao)**: The frontend will be crafted using the Yew framework, focusing on creating a user-friendly and responsive design. Yalin and Kangzhi will ensure that the interface is intuitive and effectively communicates with the backend.
- **Frontend-Backend Integration (Yalin Tuo & Kangzhi Gao)**: This task involves integrating the frontend with the backend services to ensure smooth data flow and functionality across the platform.
- **User Interaction Workflows (Chen Wang & Yalin Tuo)**: Chen and Yalin will map out and implement the interactive elements of the application, ensuring that user actions on the frontend trigger the appropriate responses and data updates from the backend.

#### Presence Detection Implementation
- **Online Status Tracking (Chen Wang & Yalin Tuo)**: This feature will allow the app to show whether users are online or offline. Chen and Yalin will handle the detection and update of user presence status in real-time.
- **Integrate Presence System with Backend and Frontend (Chen Wang & Yalin Tuo)**: They will also ensure that the presence system is fully integrated with both the backend logic and the frontend display, providing a dynamic and responsive user experience.

#### Testing
- **Functional and Performance Testing (All Members)**: Throughout the development process, all team members will continuously test the application for functionality and performance to ensure the application is robust, secure, and performs well under load.

By dividing these tasks among our team based on expertise and project needs, we are confident in our ability to efficiently develop and deliver a high-quality, real-time chat application within the projected timeline.