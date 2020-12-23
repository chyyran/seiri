# Error Reporting.

*seiri-watcher* outputs to stderr whenever a file-related error has occurred. The error message is separated by a double colon (`::`), with the first part being an error code. Error codes with a colon emit parameters that should be formatted, and error codes without emit a pre-formatted message. Some messages have more than one parameter, these parameters are separated by a double pipe.

This system is intended for the Electron browser process to handle desktop notifications.


| Code                          | Description                                            |
| ----------------------------- | ------------------------------------------------------ |
| `TRACKADDED(Artist\|\|Title)`   | A track has successfully been added to the library     |
| `ETRACK`                      | Generic track error                                    |
| `ETRACKMOVE(Path)`            | The given track could not be moved to its library path |
| `ECREATEDIRECTORY(Directory)` | The given directory could not be created               |
| `ENONTRACK(Path)`             | The given path is not a track                          |
| `EMISSINGTAG(Track\|\|Tag)`     | The given track is missing the given tag               |
| `EWATCHER`                    | Generic watcher error                                  |
| `EWATCHERDIED`                | The watcher died                                       |
| `EWATCHERRESTART`             | Watcher is restarting                                  |
| `EWATCHERNOACCESS(Path)`      | The watcher can not access the given folder            |
| `ECONFIGINVALID`              | The configuration file is invalid                      |
| `ECONFIGIO(Path)`             | The given configuration path can not be accessed       |
