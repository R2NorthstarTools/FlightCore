# FAQ

## What is FlightCore?

FlightCore is a Northstar installer, updater, and mod-manager for Northstar?

You can use it to easily install and update Northstar as well as for installing, updating, and managing mods for Northstar.

## What is Northstar?

Northstar is a modding and custom server framework for Titanfall2.

You use it to do stuff like

- [this](https://www.youtube.com/watch?v=en06Y6CPMQg)
- [or this](https://www.youtube.com/watch?v=suhBGqzDbNA)
- [or this](https://www.youtube.com/watch?v=vyUxAwobY60)

## I have an issue with FlightCore, where do I go?

Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md).

## Why yet another Northstar intaller/updater/mod-manager instead of contributing to an existing one?

The 3 main GUI tools for handling such tasks with Norhtstar are

- [r2modman](https://github.com/ebkr/r2modmanPlus)
- [Viper](https://github.com/0neGal/viper)
- [VTOL](https://github.com/BigSpice/VTOL)

while they get most of the work done, each of them has their own problem.

- **r2modman** has not too great UX and given that it also has to support other games there's not a(n easy) way to quickly add new features specific to Northstar
- **Viper** probably has the best UX but is missing features such as Origin process runtime detection (to avoid LSX errors) and lacks the ability to install Northstar from Thunderstore.
- **VTOL** has recently undergone a rewrite that removes a lot of older issues (such as requiring to be run as admin), however it is Windows exclusive and requires installing an additional library not shipped directly with the application, confusing some users. It also has a lot of edge case handling that while giving a smoother user experience blows up code base complexity.

With that said, FlightCore is not written from scratch. For handling Northstar specific logic, functions are re-used from the CLI-only Northstar installer called [papa](https://github.com/AnActualEmerald/papa) by making use of the underlying library [libthermite](https://crates.io/crates/libthermite).

The plan is to upstream any changes to `libthermite` so that it can be re-used by any other Rust-based Northstar installer.

## I'd like to contribute to FlightCore, where do I start?

Check [DEVELOPMENT.md](DEVELOPMENT.md)
