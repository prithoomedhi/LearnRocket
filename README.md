# Learn Rocket

A codebase for me to learn [Rocket](https://rocket.rs), a micro-web-framework for [Rust](https://www.rust-lang.org/).

"_Rocket is a web framework for Rust. If you'd like, you can think of Rocket as being a more flexible, friendly medley of Rails, Flask, Bottle, and Yesod. We prefer to think of Rocket as something new. Rocket aims to be fast, easy, and flexible while offering guaranteed safety and security where it can. Importantly, Rocket also aims to be fun, and it accomplishes this by ensuring that you write as little code as needed to accomplish your task._

_This guide introduces you to the core, intermediate, and advanced concepts of Rocket. After reading this guide, you should find yourself being very productive with Rocket._"[^1]

[^1]: [Rocket Introduction for v0.5-rc, https://rocket.rs/v0.5-rc/guide/introduction/](https://rocket.rs/v0.5-rc/guide/introduction/)

## Overview

We will primarily focus on building REST APIs using Rocket.

A REST API is an Application Programming Interface (a sub-program that allows two programs, not necessarily written in the same laguage/framework or even not necessarilly on the same machine) that allows two programs to communicate over the TCP/IP protocol via HTTP requests/responses using `JSON` as the message format i.e, the two communicationg programs pass and recieve data amongst each other using strings in the `Javascript Object Notation` standard.

### Requests

A standard HTTP request has the following parts

1. Base URL: The part of the URL that denotes the primary domain or the URI of the requested resource.
2. Endpoint/Path: The part of the URL that denotes the particular resource within the specified domain.
3. Headers: Secure and encrypted & signed hashtable that generally contains critical data such as tokens, permission-data, requester identity, etc.
4. Body/Data: Encrypted hashtable that contains the data that needs to be passed-off by the API to the backend to be processed.
5. Params: Non-secured hashtable-style path variables that denote additional parameters that can affect the processing of the request.
6. Path-Variables: Dynamic path/endpoint values that can also be used to process data; are absolutely not encrypted unless HTTPS is used and only for anyone outside of the 3T architecture.

