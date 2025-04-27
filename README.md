# Query: A Rust CLI Shortcut

Are you tired of opening a new browser window, picking the search engine or AI
that you want to find information from, and then typing your query?

Me too!

The solution: query - a CLI shortcut for this exercise.

Simply run
```bash
query -g Calgary Flames Score
```
to Google the score of the last Calgary Flames hockey game.

Run:
```bash
query -p Explain Rust Traits to me
```
to run the query "Explain Rust Traits to me" through Perplexity.ai.

I recommend adding the following to your .bashrc to make this quicker:
```
# https://github.com/Russell-Waterhouse/query
alias qp="query --perplexity"
alias qg="query --google"
```


## Installation:
Simply run `./install.sh` to install this package in your `~/bin` folder.
Make sure that's on your path, and you're good to go.
