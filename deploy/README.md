# deploy

## overview

This repo contains the configuration to deploy all the assets in the edge environment using `fetchit`.

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

