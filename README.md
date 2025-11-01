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

The output in this case should look something like the following. Program output is in **bold**:

> **Do a task?** maybe <br />
> **Unrecognized response! Do a task?** <br />
> **Unrecognized response! Do a task?** yes <br />
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

The output in this case should look something like the following. Program output is in **bold**:

> **Do a task?** maybe <br />
> **Unrecognized response! Do a task?** <br />
> Task canceled
