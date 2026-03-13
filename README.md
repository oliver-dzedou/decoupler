# Decoupler

Decoupler is a CLI tool to help you analyze hard couplings in your codebase. It does so by collecting your commit history and running an FP growth algorithm on the filesets within.
It then outputs a markdown report ([example](example_output.md)) containing all the filesets that are most frequently edited together, hinting at a possible hard coupling between these files. <br/>

## Installation

As of now, Decoupler is only available as a binary. You can build it yourself: <br/>

``cargo build --release``

Or download it from the [releases page](https://github.com/oliver-dzedou/decoupler/releases)

For ease of use, you can add Decoupler to your ``bin`` folder

``sudo cp target/release/decoupler /usr/local/share`` 

## Usage

``decoupler --help`` contains all the instructions. ``decoupler`` runs the app with default settings 

