What the --help command might show in the MVP:

```bash
> granger --help
An opinionated Kanban Board for the solo developer. 

USAGE:
    granger [OPTION] [COMMAND]

OPTIONS:
    -h, --help      Print help information
    -v, --version   Print version info and exit

COMMANDS:
    board,  b   Initialize, List, and Remove Boards
    ticket, t   Create, Read, Update, and Delete Tickets

See 'granger <command> --help' for more information on a specific command.
```

```bash
> granger board --help

Initalize, List, and Remove Boards

USAGE:
    granger board [OPTIONS]

OPTIONS:
    -i,  --init         Initialize a board in current git repository
    -ls, --list         List of initalized boards on system
    -rm, --remove       Remove board based on current git repository
```

```bash
> granger ticket --help

Create, Read, Update, and Delete Tickets

USAGE:
    granger ticket [OPTIONS]

OPTIONS:
    -c, --create                Create a new ticket
    -r, --read   <ID>           View details of an existing ticket
    -u, --update <ID>           Update details on an existing ticket
    -d, --delete <ID>           Delete an existing ticket

    -ls, --list                 List all existing tickets in the current board
    -mv, --move  <ID> <STATE>   Move ticket to a different state on the current board
```

