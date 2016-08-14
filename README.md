? (don't have a name yet)
========

__Please see [Gitlab](https://gitlab.com/MonsieurOenologue/Zelkow) for the updated version. This is just a clone.__

Installation
============

Requirements
------------

* `rustc`, `cargo`, stable version _if you want to compile it_
* `python3` with `requests`
* [Redis](http://redis.io/topics/quickstart)

Compiling
---------

1. `git clone` the repository
2. `cd src/rust/coordinator`
3. Edit the `main.rs` file if you want ton configure the project.
4. `cargo build --release`
5. You'll find the executable in `target/release`.

Installation
------------

1. Get a Riot API Key and put it in a file somewhere
2. Edit `twitch.py` and change `DEFAULT_API_KEY_PATH` if necessary
3. Put the `coordinator` binary and `twitch.py` like that :

```
.
├── coord
│   └── coordinator
└── twitch
    └── twitch.py
```

4. Launch your redis instance : `redis-server`
5. Launch the coordinator `./coordinator`.

Nickname generator, based on (_guess what?_) Markov chains and Riot API.
The project is divided into 3 subprojects.

# Architecture for the V2 :

 It would be nice to have :

 * Get nickname from mastery level, not currently played champion.
 * Correctly separate libs and observer (like point 1 in fact..)

## twitch

   Twitch is the script that gets the data and (currently) feeds a JSON file with nicknames.
   It is currently written in Python.

### TODO

  * [x] Have some logging
  * [ ] Rewrite twitch in rust ?
  * [x] Try to feed ryze instead of feeding a JSON file ? Or at least find a nice way to make twitch and zilean communicate.
  * [ ] Change the way it behaves in order to include lower-ranked games, and base on mastery level rather than on the currently player champion.

  See [payeTonPseudo](https://github.com/Uinelj/payeTonPseudo) on Github for the python script which gets the data.

## ryze

  ~~Ryze will be the data storage solution.
  I don't really know how I will store the data and access it.
  I'd like to learn some NoSQL databases, because everyone starts to use it, so it's always better to know what it's about.~~

  Ryze is the redis database. It does not currently represent some code. But it may represent code in the future.

## zilean

  Zilean is the project where we generate the pseudonyms, and where we parse them in order to put them in the database.
  Currently the data are stored into nested hashmaps, and are not yet serialized.

### TODO:

  Abstract the hashmaps and provide a simple object with methods `feed()` and `gen()`.

Concept
=======

Get and constitute a database which correlates a champion with player nicknames.
Feed a chain with these nicknames and you can generate some champion-dependant names !
