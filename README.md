<!--
*** Thanks for checking out this README Template. If you have a suggestion that would
*** make this better, please fork the repo and create a pull request or simply open
*** an issue with the tag "enhancement".
*** Thanks again! Now go create something AMAZING! :D
-->

<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/savoiringfaire/short.rs">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Short.rs</h3>

  <p align="center">
    A super-fast rust based URL shortner!
    <br />
    <a href="https://github.com/othneildrew/Best-README-Template"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://s.hhra.uk">View Demo</a>
    ·
    <a href="https://github.com/savoiringfaire/short.rs/issues">Report Bug</a>
    ·
    <a href="https://github.com/savoiringfaire/short.rs/issues">Request Feature</a>
  </p>
</p>



<!-- TABLE OF CONTENTS -->
## Table of Contents

* [About the Project](#about-the-project)
  * [Built With](#built-with)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [License](#license)
* [Contact](#contact)
* [Acknowledgements](#acknowledgements)



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

Short.rs is a super-fast rust based URL shortner. Capable of horizontal scaling and designed speifically for running in environments such as kubernetes, it's the perfect choice for any high-volume applications.

The current list of short URL's are stored in a redis backend, providing performant backend data storage, and easy scaling.

### Built With

* [Rust](https://rust-lang.org)
* [Hyper](https://github.com/hyperium/hyper)

<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

Running the docker-based environment, you will need to have docker installed. You will also need a redis server installed and setup.

### Installation

```
docker run -p 80:80 -e LISTEN_ADDRESS="0.0.0.0:80" -e REDIS_CONNECTION_STRING="redis" savoiringfaire/shortener
```

<!-- ROADMAP -->
## Roadmap

- Support redis clusters.

See the [open issues](https://github.com/savoiringfaire/short.rs/issues) for a full list of proposed features (and known issues).


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<!-- CONTACT -->
## Contact

Your Name - [@savoiringfaire](https://twitter.com/savoiringfaire) - contact@hhra.uk

Project Link: [https://github.com/savoiringfaire/short.rs](https://github.com/savoiringfaire/short.rs
