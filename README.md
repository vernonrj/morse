run a morse code encoder on any number of raspberry pis

# Requirements

## Host PC Setup

On your host PC, you'll need the following:
- [powershell](https://github.com/PowerShell/PowerShell) for the deploy script
- [ssh](https://www.howtogeek.com/336775/how-to-enable-and-use-windows-10s-built-in-ssh-commands/) for controlling the Pis
- docker with support for [buildx](https://docs.docker.com/buildx/working-with-buildx/)

## Raspberry Pi Setup

On your raspberry pi(s) you'll need docker installed and configured, and ssh keys setup.

For docker, do the following:

```bash
sudo apt update
sudo apt install docker.io
sudo systemctl enable docker
sudo usermod -aG docker pi
```

Reboot to make sure this all takes effect.

For ssh key setup, see this: https://pimylifeup.com/raspberry-pi-ssh-keys/

An LED should be connected to GPIO 14 (pin 8) for the /blink url to actually do something.

# Deploy

Run the following

```powershell
$ ./deploy ('rpi-1', 'rpi-2', 'rpi-3')
```

specify your raspberry pis in a powershell array.

This will build the associated docker image on your host PC, copy it to each raspberry pi, and bring it up with the default parameters. Note that the initial build may take about an hour - the cross-compilation plus heavy deps plus docker will eat a lot of your RAM. Subsequent builds will take far less time.

# Use

Once the morse container is spun up, there are 3 http routes you can use.

## Encode

Encode text into morse code.

```powershell
$ curl rpi-shrike/encode -d 'text="sos sos sos"'
... --- ... ... --- ... ... --- ...
```

## Decode

Decode morse code into text

```powershell
$ curl rpi-shrike/decode -d 'text=". -..- .- -- .--. .-.. . - . -..- -"'
exampletext
```

## Blink

Encode text into morse code and blink it out on an LED

```powershell
$ curl -X POST http://rpi-junco/blink -d 'text="lol lmao"'
success
```