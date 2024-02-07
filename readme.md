quickly made readme file
in a full release, client will work like:
* Sealy - main piece of that. Installs and manages the core (core, fok and debugger)
* fok - a real package manager. Installs, removes, updates packages etc.
* core - this is where sources for `fok` are located. there's also path for packages and packages binaries.
* debugger - used to test compatibility of current version, and the one downloaded to PRETEND environment (downloaded by Sealy). older debugger checks new core for backwards-compatibility, to warn user about some changes.



right now:
download server files and rust (for dev purposes only, no real usage of package manager for now).
compile the server and run it.
download client files, compile sealy. then run > sealy upgrade
Right now it will just overwrite current installation, if you have one. I plan to make a pretend environment to test new installation someday.

now, compile `fok`. now you can use the package manager:
> fok install package

Packages on the local servers are located in packages folder. Make sure to compress your package into a furball!