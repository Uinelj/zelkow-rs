Zilean
======

Nickname processing and generation engine.

## Data structures

The processing and the generating is based on Markov chains. 
For each character of the game, the data used to generate nicknames is in the following form : 

`HashMap<char, HashMap<char, u32>>`.

TODO