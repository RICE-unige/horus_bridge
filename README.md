# HORUS Bridge

## Overview

HORUS Bridge is the essential desktop component of the HORUS mixed reality robotics platform. This server acts as a vital link, facilitating communication, data exchange, and control commands between the HORUS interface (running on the Meta Quest 3) and the connected robot(s).

## Key Functionalities

* **Real-time Teleoperation Data Relay:** Transmits control inputs from the HORUS interface to the robot and streams sensor data back for visualization.
* **Robot Network Management:** Establishes and maintains connections with individual robots within the team.
* **Data Translation:** Adapts data formats and communication protocols to ensure compatibility between the headset and robots.
* **API for HORUS Interface:** Provides an API endpoint for the main HORUS interface to send control commands and receive robot data.

## Installation and Usage
* **Prerequisites:**
   * [List any software dependencies, like robot-specific libraries]
* **Installation Guide:**
    1. [Step-by-step instructions on downloading and installing]
* **Running the Bridge:**
    1. [How to initiate the server, including potential configuration]
* **Connecting with HORUS:**
    1. [Explain the process within the HORUS mixed reality interface]

## Project Structure (Outline)

* **src:** Contains the core source code of the HORUS Bridge server.
* **config:** Configuration files for network settings and robot-specific parameters.
* **docs:** Additional documentation and usage guides.

<!--
## Get Involved

HORUS Bridge is a core part of an ongoing research project. We welcome contributions, bug reports, and feature suggestions.   

**Please feel free to:**

* Raise issues
* Suggest features
* Submit code contributions
-->

## Project Timeline 

- [ ] Robot Communication Stub:
    - [ ] Choose a lightweight method to simulate basic robot interactions (this could even be a simple script generating mock data initially).
    - [ ] Implement a module to receive data from this "robot".
- [ ] WebSocket Server:
    - [ ] Set up a core WebSocket server (ws library in Node.js, websockets in Python, etc.).
    - [ ] Establish basic connection handling with the HORUS interface.
- [ ] Data Relaying:
    - [ ] Translate simplified robot data (camera images, control commands) into a format suitable for WebSocket transmission.
    - [ ] Implement relay logic to send/receive data between the WebSocket connection and the robot communication module.



>**Author:** *Omotoye Shamsudeen Adekoya*  
>**Email:** *<omotoye.adekoya@edu.unige.it>*    
>**Curriculum:** *Hostile and Unstructured Environments (9926)*   
>**Research theme #10:** *Augmented Reality Strategies for Teams of Robots (<a href="https://unige.it/">Università di Genova</a>)*



> <div align="left"><a href="https://rice.dibris.unige.it/"> 
><img src="img/rice_lab_logo.png" width="7%" height="7%" title="Robots and Intelligent systems for Citizens and the Environment" alt="Robots and Intelligent systems for Citizens and the Environment">
> </a><a href="https://drim.i-rim.it/en/"><img src="img/i_rim_logo_blue.png" width="20%" height="20%" title="Italian Institute of Robotics and Intelligent Machines" alt="Italian Institute of Robotics and Intelligent Machines">
</a></div>