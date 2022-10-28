# Haskell Type Alias Resolver

The Haskell type alias resolver (HTAR) is a system for locating places in Haskell
source code where type aliases could be used to improve readabilty. It consists of:

- a Rust based back-end which utilises Treesitter to do most of the heavy lifting
- a VSCode extension which incorporates the application into VSCode
- two early prototype web interfaces for experimenting with the tool and to provide a
    starting point for future development

## Usage

### Back-end
Ensure you have a working Rust development environment. For help, see [here](https://www.rust-lang.org/learn/get-started). You may also need a C compiler for Treesitter, see [here](https://tree-sitter.github.io/tree-sitter/).

Build the project using:
```
cargo build
```

Run with:
```
cargo run -- <ARGS>
```

For instance, to run in command line mode with human readable output, 
`"tests/input_files/example.hs"` as my input source file and 
`"String -> String -> [String]"` as my alias use:
```
cargo run -- -r -p tests/input_files/example.hs -t "String -> String -> [String]"
```

CLI help can be found using:
```
cargo run -- --help
```

Run automated tests with:
```
cargo test
```

### VSCode Extension
Ensure [VSCode](https://code.visualstudio.com/Download) is installed and working.

Open `vscode-extension/haskell-type-alias-resolver/` in VSCode and run the project using
`F5`

Extension runs HTAR on save and provides information diagnostics and code actions for
potential replacements. USe `Ctrl-,` to automatically apply the 'best' replacement

## Implementation
At a high level, the back-end operates by:
- Using Treesitter to parse target type signature and modify the generated S-Tree to form
    a new Treesitter query which will match type aliases of a similar form
- Using Treesitter to parse source file into a simple AST
- Running the generated target query on the parsed source file to find candidate subtrees
- Performing raw text processing on text locations associated with candidates to check
    for the consistency of concrete and generic types
- In the case of generic types, generate a mapping from type parameter to concrete type
    to 'specialise' generic alias to match the concrete target type
- Output matches, replacements, locations and variable maps as JSON

HTAR can be run in server mode, in which case it runs a simple http server which takes
input source and target as JSON and responds with the output over http.

### Treesitter
The Treesitter library is central to the operation of HTAR. It is a parsing library
intended to be used for development tools. It features very efficient incremental
parsing, is capable of handling invalid input gracefully and has prewritten grammars for
many languages. At this stage, this project is not taking full advantage of the
incremental parsing features Treesitter supports as perfomance has not been a concern
yet. If/when speed becomes a problem, maintaining a single AST structure instead of
reparsing each time would likely result is a large speedup.

The Haskell grammar for Treesitter is still in development and as such you may encounter
strange bugs parsing some expressions. This grammar is also one of the slower Treesitter
grammars and there is some potential for further perfomance gains as this grammar is
improved.

Treesitter features language bindings for a number of languages including Haskell
itself. For this project, the Rust bindings are used as the documentation for the
Haskell bindings is lacking. Rust still allows the use of a rich type system which has
simplified the conversions between Haskell, Rust and TypeScript.

### VSCode
VSCode was choosen as the target editor due to its popularity, cross-platform
compatibility and rich extension API. The extension is relatively small, the only
signficant computation it performs is searching the file for lines that may contain type
signatures. These lines are used as targets when the HTAR command line tool is invoked.

The VSCode extension API is used directly here as the scale of the application made a
full language server protocol impractical. If large additions are made to the extension,
restructuring it as a language server would provide benefits for porting to other
editors and would enable most of the logic to be written in Rust as well.

## Development Notes
This project has many potential avenues for future development.

An obvious addition would be to support languages other than Haskell. This would be a
smaller task than it may seem. Treesitter grammars already exist for most programming
languages and very little of the logic used in the back-end is Haskell specific. This is
intentional as the plan was always to branch out into other languages.

Using HTAR to parse compiler output (GHC for instance) and replace types in error
messages could be extremely helpful for developers. This would require writing a
Treesitter grammar which parses GHC output which may be difficult but is certainly
possible.

Another useful feature would be to support matching subtypes. For instance, if I have a
function `func :: String -> String -> Int` and an alias `type Alias = String -> String`
it would be nice for HTAR to provide `func :: Alias -> Int` as a potential replacement.
This would require decomposing a target type into smaller expressions and attempting to
match each of them. Some heuristics would need to be applied to determine which subtype
matches are beneficial as this would greatly increase the average number of matches.

As mentioned earlier, modifying the VSCode extension to use the 
[language server protocol](https://microsoft.github.io/language-server-protocol/) or 
reworking the back-end to take advantage of incremental parsing would both provide 
excellent value to the project.

Creating new front-ends for HTAR should also be reasonably simple. As a starting point,
your application should expect responses in the form:
```
{
  "echo_request": {
    "target_type": "String -> Int -> [Int]",
    "source": "type MyAlias a = String -> a -> [a]\ntype MyOtherAlias = String -> Int -> [Int]\n"
  },
  "matches": [
    {
      "matched": "type MyAlias a = String -> a -> [a]",
      "location": {
        "start": {
          "row": 0,
          "col": 17
        },
        "end": {
          "row": 0,
          "col": 35
        }
      },
      "variable_map": {
        "a": "Int"
      },
      "replaced_type": "MyAlias Int"
    },
    {
      "matched": "type MyOtherAlias = String -> Int -> [Int]",
      "location": {
        "start": {
          "row": 1,
          "col": 20
        },
        "end": {
          "row": 1,
          "col": 42
        }
      },
      "variable_map": {},
      "replaced_type": "MyOtherAlias"
    }
  ]
}
```
Requests should be in the form of the `echo_request` field.
