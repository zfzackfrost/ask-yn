# ask-yn

Terminal utility program to prompt the user to answer yes or no.

## Example usage

### Simplest Usage &mdash; No Default Argument

Without a default argument, the program with ask the user again if no input is given.

```sh
if ask-yn "Do a task?"; then
    echo "Doing a task"
else
    echo "Task canceled"
fi
```

The output in this case should look something like the following. Program
output is in **bold**, user input is in _italic_:

> **Do a task?** _maybe_ <br />
> **Unrecognized response! Do a task?** <br />
> **Unrecognized response! Do a task?** _yes_ <br />
> Doing a task

### With Default Argument

With a default argument, the program will use it when no input is provided.

```sh
if ask-yn "Do a task?" -d n; then
    echo "Doing a task"
else
    echo "Task canceled"
fi
```

The output in this case should look something like the following. Program
output is in **bold**, user input is in _italic_:

> **Do a task?** _maybe_ <br />
> **Unrecognized response! Do a task?** <br />
> Task canceled

### Invert Return code

The `-i`/`--invert` flag in this program inverts the return code. With this
flag, the program will return a `1` when given a yes answer and a `0`
otherwise.

```sh
if ask-yn "Don't do a task?" -i; then
    echo "Task canceled"
else
    echo "Doing a task"
fi
```
