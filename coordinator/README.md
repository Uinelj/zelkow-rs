## Coordinator

It manages the twitch/zilean communication, and it is a REST server which will handle nickname generation requests like `http://whatever/gen/30` which will generate a nickname for the champion with `champion_id == 30`.

## Iron or Nickel ?

I need a web framework in order to handle requests.

* Hyper : A HTTP lib. May be too low level.
* Iron : The mainstream web framework. Seems to be nice af, but currently supports nightly rust. I'd like to stick to stable rust as long as I can.
* Nickel : Another mainstream web framework. Seems to support stable rust.
