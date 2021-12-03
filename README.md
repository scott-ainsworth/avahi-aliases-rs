# Background

The current version of [Avahi](https://avahi.org/)
([v0.8](https://github.com/lathiat/avahi/releases/tag/v0.8)) publishes the host name as a
[Domain Name System (DNS)](https://en.wikipedia.org/wiki/Domain_Name_System) [Address (A)
Record](https://en.wikipedia.org/wiki/List_of_DNS_record_types), which is sufficient for the
majority of small, local network needs. However, sometimes additional names for host are needed.
This is
especially true now that [Docker](https://www.docker.com/) has simplified deployment of many
services on a single host. This is where `avahi-aliases-rs` comes in.

The original implementation of Avahi aliases was an example Python script published in the Avahi
wiki. Although the wiki is no longer available; the
[original script](https://web.archive.org/web/2019*/http://www.avahi.org/wiki/Examples/PythonPublishAlias)
is preserved on the [Internet Archive](https://archive.org).

The original script spawned [many versions](https://github.com/search?q=avahi-aliases). So, why
another version? The motivations behind this version include:

- A fast, compiled version runs with fewer resources. (This version is about half the memory of
  the Python version.)
- Fewer installed executables; one per function: the daemon and the alias manager.
- Improved error handling and messaging.
- Syslog logging (and possibility other lagging facilities in the future).
- Configurable connections timeouts, DNS record time-to-live, etc.

## System Requirements

`avahi-aliases-rs` should build and build and run on most
[Unix-like](https://en.wikipedia.org/wiki/Unix-like) operating systems on which
[Avahi](https://avahi.org/) and [Rust](https://www.rust-lang.org/) also work. Some of the unit
and integration tests only succeed when the Avahi daemon is available.

## Installation

Installing `avahi-daemon-rs` currently requires building it on the target machine. (Technically,
it could be cross-compiled, but there is not support in the existing Cargo.toml and Makefile.)
For quick installation, follow these steps:

**Prerequisites**

1. Make sure Avahi is installed and works correctly.
2. Install Rust and ensure it works.
3. Clone this GitHub repository ([https://github.com/scott-ainsworth/avahi-aliases-rs.git](https://github.com/scott-ainsworth/avahi-aliases-rs.git)).

**Build**

The remaining steps are executed from a shell prompt.

4. Change to the `avahi-aliases-rs` directory (which was created by `git clone`).
5. Run `cargo build --release`.

**Install**

6. (optional) Test using the [Pre-Installation Testing](#pre-installation-testing) instructions below.
7. Uninstall or disable existing Avahi aliases implementations<sup>1</sup> (if any).
8. Run `sudo bin/install-systemd`. The daemon (`avahi-alias-daemon` will start automatically).
9. Use `avahi-alias` to add and remove CNAMEs (aliases).

## Adding and Removing Aliases (CNAMEs)

The `avahi-alias` program is used to add and remove aliases. Examples:

- `avahi-alias add example.local` adds *example.local* to the Avahi aliases file<sup>1</sup>. The addition should be picked up by the daemon within 10 seconds<sup>2</sup>.
- `avahi-alias remove example.local` removes *example.local* from the Avahi aliases file<sup>1</sup>. The removal should be picked up by the daemon within 10 seconds<sup>2</sup>.
- `avahi-alias list` lists the aliases in `/etc/avahi/avahi-aliases`. Invalid aliases are flagged in the listing.

## Pre-installation Testing

To test the `avahi-alias-daemon` prior to installing it,
   1. Create an aliases file (`test-aliases` for example).
   2. Add an alias (`target/release/avahi-alias add -f test-aliases example.local`).
   3. Run the daemon (`target/releases/avahi-alias-daemon -f test-aliases -v`). If working, the daemon loading report loading and publishing the aliases.
   4. Open another shell windows and ping the alias (`ping -c 1 example.local`).
   5. Press control-C to terminate the daemon.

## Getting Started Notes
1. The default location for the Avahi aliases file is `/etc/avahi/avahi-aliases`. This can be changed with the `--file` option.
1. Changes to `/etc/avahi/avahi-aliases` will be reflected in about 10 seconds. This time can be changed with the `--poll` command line option.

# Compatibility with other Avahi Alias implementations

Given the wide variety of Avahi alias implementations, compatibility is a challenge.
`avahi-aliases-rs` features and their compatibility are described in the following table.

| Feature | Compatibility |
|---------|---------------|
| Aliases are stored in `/etc/avahi/avahi‑aliases`. | Most, but not all, Python versions also use `/etc/avahi-aliases`. Some Python versions load any file in the `/etc/avahi/aliases.d` directory (this version does not). |
| Aliases file format. | The basic format used by most versions is one aliases per line. `avahi-aliases-rs` extends this by allowing comments. Everything after a hash sign `#` is ignored. (Thus, the line `example.local # comment` is valid.) Additionally, invalid aliases stop the `avahi-alias` add and remove actions and are ignored by the daemon. |
| [Internationalized Domain Names (IDNA)](https://en.wikipedia.org/wiki/Internationalized_domain_name) | Not currently support. Support is planned. |
| Daemon runs as [systemd](https://systemd.io/) service. | Other versions also include support for [System V/init&period;d](https://en.wikipedia.org/wiki/Init#SysV-style) or nothing at all. (There is no technical reason `avahi-aliases-rs` cannot support init&period;d; none of my systems currently use init&period;d and I cannot test it.) |
| Plays well with other versions. | Probably not. The names of the executables (`avahi-alias` and `avahi-alias-daemon`) probably conflict with many other versions (installing `avahi-aliases-rs` could clobber other versions).  The name of the systemd service is `avahi-aliases`, which is also used by some of the Python versions. |
| Syslog logging | No Python version (that I know of) implements syslog logging. |



# Copyright and License

`avahi-aliases-rs` is Copyright &copy; 2022 by Scott G. Ainsworth. Like Avahi, `avahi-aliases-rs` is licensed under the GNU Lesser General Public License, [Version 2.1](https://www.gnu.org/licenses/old-licenses/lgpl-2.1.en.html), February 1999. A [copy of the license](https://github.com/scott-ainsworth/avahi-aliases-rs/blob/main/LICENSE) is available in the repository.

# Acknowledgements and References

- [Avahi](https://avahi.org/) web site.
- Original Python example on the Avahi wiki ([archived version](https://web.archive.org/web/2019*/http://www.avahi.org/wiki/Examples/PythonPublishAlias) at the [Internet Archive](https://web.archive.org/). The [original](http://www.avahi.org/wiki/Examples/PythonPublishAlias) is no longer available.)
- [Airtonix (Zeno) Python implementation](https://github.com/airtonix/avahi-aliases) on GitHub. Based on the Avahi example. Earliest implementation on GitHub.
- [luarntlemercier implementation](https://github.com/laurentlemercier/avahi-aliases) on GitHub. Includes [systemd](https://systemd.io/) service unit [configuration file](https://github.com/laurentlemercier/avahi-aliases/blob/master/package/avahi-alias.service), installer, and uninstaller.
