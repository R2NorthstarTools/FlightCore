# Dev tools

![dev view screenshot](../assets/dev-view-screenshot.png)

FlightCore features a hidden view that contains development features.

It's targetted at both Northstar and FlightCore contributors. Among other things it contains buttons for unreleased features in FlightCore and tools to help with Northstar development.

To activate it, spam click the FlightCore version number in the settings view at least 5 times. After that a new entry named _DEV_ should appear in the menubar.

## Northstar

### Pull request install

The dev view offers a way to install pull request from the [NorthstarLauncher](https://github.com/R2Northstar/NorthstarLauncher) and [NorthstarMods](https://github.com/R2Northstar/NorthstarMods) repositories.

Launcher pull requests overwrite `NorthstarLauncher.exe` and `Northstar.dll`.

Mod pull requests install into a separate profile called `R2Northstar-PR-test-managed-folder`.


## FlightCore

The dev view contains various buttons that call functions that might not be fully implemented or tested yet.

Additionally it has some buttons for testing like the _Panic button_ which force crashes the application to test automatic log uploading on crash among other things.
