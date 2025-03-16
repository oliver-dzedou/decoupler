# Decoupler

Decoupler is a CLI tool to help you analyze hard couplings in your codebase. It does so by collecting your commit history and running an FP growth algorithm on the filesets within.
It then outputs a markdown report containing all the filesets that are most frequently edited together, hinting at a possible hard coupling between these files. <br/>

## Important info 

* **Security**: Decoupler does not collect data of any kind and can be safely ran on classified codebases. This claim can be easily verified by reading the source code or running it in fully air gapped environment. <br/>
* **Performance**: The FP growth algorithm runs in O(n^2), so smaller repositories are analyzed instantly, while bigger ones (thousands of commits with long diffs) may take very, very, very long.

## Installation

As of now, Decoupler is only available as a binary. You can build it yourself: <br/>

``cargo build --release``

Or download it from the [releases page](https://github.com/oliver-dzedou/decoupler/releases)

For ease of use, you can add Decoupler to your ``bin`` folder

``sudo cp target/release/decoupler /usr/local/share`` 

## Usage

``decoupler --help`` contains all the instructions. ``decoupler`` runs the app with default settings 

## Disclaimer

I made Decoupler out of spite in a couple of weekends, when I noticed that a certain SAAS company is offering this functionality for a relatively hefty fee :)

That is just to say that my focus is elsewhere -- I don't plan to actively maintain Decoupler. Feel free to submit issues, but don't expect me to address them in a timely manner.
