# Error Reporting.
*seiri-core* outputs to stderr whenever a file-related error has occurred. The error message is separated by a tilde (`~`), with the first part being an error code. Error codes with a colon (`:`) have a parameter that follows the tilde, and is before the octothorpe.

Note that bang-parsing related errors are not included in this system, and are returned by the GraphQL endpoints. The following are the lists of error codes.

This system works outside of the GraphQL endpoints, and is intended for the Electron browser process to handle desktop notifications.

|Code|Description|
|----|-----------|
|`WATCHERROR(:)`|Generic watch error|
|`WATCHERKEEPALIVEFAIL`|The file watcher died|
|`WATCHERFOLDERACCESSLOST`|Access to the folder being watch died.|
|`WATCHERRESTART`|The watcher is being restarted|
|`TRACKMOVEERR(:)`|An error occurred when moving the track to a new location|
|`CONFIGWRITEERR`|An error occurred when writing the configuration file|
|`CONFIGINVALID`|The configuration file was invalid|
|`HELPERNOTFOUND`|The taglib helper was not found|
|`LIBRARYNOTFOUND`|The library path was not found|
|`TRACKADDED(:)`|Not an error, but the given track was added to the database|
|`NONTRACK(:)`|Not an error, but the given non-track file was moved away|
|`MISSINGTAG`|A track was missing a required tag. Details are in the error message|
