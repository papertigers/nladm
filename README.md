# nladm

CLI interface for NanoLeaf Devices.

An optional config can be stored in your OS specific configuration directory:

|Platform | Value                                 | Example                                     |
| ------- | ------------------------------------- | ------------------------------------------- |
| Linux   | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/alice/.config/nladm.toml              |
| macOS   | `$HOME`/Library/Preferences           | /Users/Alice/Library/Preferences/nladm.toml |
| Windows | `{FOLDERID_RoamingAppData}`           | C:\Users\Alice\AppData\Roaming\nladm.toml   |

The config looks like:

```toml
server = '10.0.1.200'
port = 16021
token = "user-token from `nladm user add`"
```

```
nladm 0.1.0
Mike Zeller <mike@mikezeller.net>
Control nanoleaf lighting

USAGE:
    nladm [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>        Server port
    -s, --server <server>    Server IP address
    -t, --token <token>      User token

SUBCOMMANDS:
    effects    Panel effects
    help       Prints this message or the help of the given subcommand(s)
    info       Get panel information
    state      Panel state
    user       Add/Remove Users
```
