# deploy

## overview

This repo contains the configuration to deploy all the assets in the edge environment using `fetchit`.

Currently this repo uses the systemd based approach as arguments such as device and groups need passing to the podman run command and these aren't mapped in the container config.

## install

Enable the podman socket api
```
systemctl --user enable podman.socket --now
```

Copy the `config.yaml` in this directory to the `$HOME/.fetchit/config.yaml` edge device.
```
mkdir $HOME/.fetchit
cp config.yaml $HOME/.fetchit/config.yaml
```

## commands 

Get systemd services for a users
```
systemctl --user list-units --type service

...
 fetchit.service         loaded active running Fetchit container management tool
... 
 serial-reader.service   loaded active running Podman container-serial-reader.service
```

Restart the fetchit service 

```
systemctl --user restart fetchit
```
