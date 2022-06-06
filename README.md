<!-- PROJECT LOGO -->
<br />
<p align="center">
  <b>
    <p align="center">
      <h3 align="center" style="font-size: 30px; letter-spacing: 10px; margin: 0;">B O R S H E D</h3><br>
      <h3 align="center" style="font-size: 16px; margin: 0;">A command line editor for Borsh.</h3>
    </p>
    <h1 align="center"></h1>
  </b>

  <p align="center">
    An editor for files containing data serialized using the Borsh algorithm, for use with Solana ecosystem tools and the Solana Program Library. This allows you to store an in-memory object to disk and inspect or edit it manually.
    <br />
    <br /><b>
    <a href="https://github.com/zrthxn/borshed/issues">Report Bug</a>
    ·
    <a href="https://github.com/zrthxn/borshed/issues">Request Feature</a></b>
  </p>
</p>


<!-- ABOUT THE PROJECT -->
<h1 align="center">About The Project</h1>

Solana uses a data serialization system/algorithm called Borsh (Binary Object Representation Serializer for Hashing) which can be used to essentially write an in-program object to a secure binary format.
This is a command line program that enables editing of files containing data serialized using the Borsh algorithm.

<!-- TABLE OF CONTENTS -->
  * [Built With](#built-with)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [License](#license)
* [Contributors](#contributors-)

## How we built it

Because the location of the struct is being provided at runtime, either it could be directly loaded dynamically from a dylib (in which case we don't know enough to reconstruct the source except from what lldb can provide), or at runtime you could trigger compilation of a user-provided cargo project and dynamically link to that, which means there's enough info to reconstruct the source. I agree the second option seems a bit strange, but otherwise I can't see where the information needed to reconstruct the source will come from—if we need to relate data from before (source) and after (binary) the compilation process we need to be in control of compilation I think. The borsch format of the data saved to the binary can't itself contain enough information to reconstruct the source, because borsch serialises structs that have already potentially been subject to compiler optimisations (e.g. name mangling)

## What's next for Borshed

<!-- GETTING STARTED -->
## Getting Started



### Prerequisites



### Installation



<!-- USAGE EXAMPLES -->
## Usage

```
borshed data.bin path::to::struct
```

<!-- ROADMAP -->
## Roadmap

See the [open issues](https://github.com/ashikka/borshed/issues) for a list of proposed features (and known issues).



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'feat: Add some AmazingFeature'`)
4. Push to the Branch (`git push -u origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->
## License

Distributed under the MIT License. See [`LICENSE`](./LICENSE) for more information.
