# serial-reader

Reads from the deployed temp-rs project and saves the files.

These are the environment variables that can be set.

|Name|Default|Description|
|---|---|---|
|PORT_NAME|/dev/ttyACM0|The named location of the usb device providing readings|
|DATA_PATH|/tmp|The location where the readings are saved on the local device|
|BAUD_RATE|57600|The baud rate set on the device|
|LINE_COUNT|200|The number of lines captured in a file|
|FILE_COUNT|200|The number of files saved before the process terminates|

## build

* A [configured rust environment](https://rustup.rs/)

* `libudev` `gcc` and `openssl` installed libraries installed on the operating system.

  **RHEL/Fedora**
  ```
  yum install -y gcc openssl-devel libudev-devel
  ```

* This project can also be built with podman
   ```
   podman build -t quay.io/ubiquitous-factory/serial-reader .
   ```

## run

With a running arduino device the following commands can be used:

### local
```
cargo run
```

### container

```
podman run --device /dev/ttyACM0 --group-add keep-groups quay.io/ubiquitous-factory/serial-reader:latest
```

