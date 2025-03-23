# Decoupler

Decoupler is a CLI tool to help you analyze hard couplings in your codebase. It does so by collecting your commit history and running an FP growth algorithm on the filesets within.
It then outputs a markdown report ([example](example_output.md)) containing all the filesets that are most frequently edited together, hinting at a possible hard coupling between these files. <br/>

## Important info 

* **Security**: Decoupler does not collect data of any kind and can be safely ran on classified codebases. This claim can be easily verified by reading the source code or running it in fully air gapped environment. <br/>
* **Performance**: On a repository with 550 commits with average diff size of around 20-30 files changed per commit, the analysis takes 3 seconds on a MacBook M3 Pro. Be aware that FP growth runs in O(n^2), so as the repository grows, the time required to perform the analysis goes up very rapidly.

## Installation

As of now, Decoupler is only available as a binary. You can build it yourself: <br/>

``cargo build --release``

Or download it from the [releases page](https://github.com/oliver-dzedou/decoupler/releases)

For ease of use, you can add Decoupler to your ``bin`` folder

``sudo cp target/release/decoupler /usr/local/share`` 

## Usage

``decoupler --help`` contains all the instructions. ``decoupler`` runs the app with default settings 

