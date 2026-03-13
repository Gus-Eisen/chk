# CHK Overview

CHK is a Rust application-building layer designed to sit on top of UI systems built with **Prism**. Rather than requiring developers to construct user interfaces manually, CHK allows builders to describe the **data they want to collect or display**, and automatically generates the corresponding application interface.

Instead of choosing specific UI components such as text inputs, radio selectors, or tables, developers simply define the information a user should interact with. CHK then determines how that information should be presented within the underlying UI system.

CHK currently targets **Pelican UI**, with plans to support additional Prism-based UI systems in the future.

## Design Goals

CHK prioritizes **intent and simplicity** over manual UI construction. The system is designed to reduce the amount of UI code developers need to write by allowing them to focus on application logic and data rather than component selection and layout decisions.

Applications built with CHK are typically defined in terms of:

* **Displays** for presenting information
* **Forms** for collecting user input
* **Roots** for defining the main sections of an application

From these definitions, CHK constructs the appropriate pages, navigation, and input interfaces automatically.

This approach allows developers to build structured applications quickly without needing to understand the details of the underlying UI framework.

## Scope

CHK provides:

* A declarative system for defining application flows and forms
* Automatic generation of pages and navigation
* Structured displays for presenting data
* A consistent interface layer built on top of Prism UI systems

CHK intentionally avoids exposing low-level UI components to the builder. Instead, it focuses on describing **what the application should do**, leaving the UI implementation details to the framework.

## Relationship to UI Systems

CHK operates above the Prism ecosystem and generates interfaces for supported UI systems.

```text
Application
    ↓
CHK
    ↓
UI System (Pelican UI)
    ↓
Prism
    ↓
wgpu
```

Because CHK operates at the application-description layer, it can support multiple Prism-based UI frameworks without requiring developers to change how their application logic is defined.

## Platform Support

* Linux
* Windows
* macOS
* Android
* iOS
* Web

## Examples

Example applications demonstrating CHK are included in the repository. 

Developers are encouraged to explore these examples to understand how CHK structures applications and generates interfaces automatically.
