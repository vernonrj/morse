run a morse code encoder on any number of raspberry pis

# Requirements

On your host PC you'll need docker with support for buildx, powershell, and ssh

On your raspberry pi(s) you'll need docker running. The pi user should be able to start/stop containers with docker (add `pi` to the `docker` group). You'll also need passwordless key-based ssh setup or else you'll be typing your password a lot.

An LED should be connected to GPIO 14 (pin 8) for the /blink url to actually do something.

# Deploy

Run the following

```powershell
$ ./deploy ('rpi-1', 'rpi-2', 'rpi-3')
```

specify your raspberry pis in a powershell array.

This will build the associated docker image on your host PC, copy it to each raspberry pi, and bring it up with the default parameters.

# Use

There are 3 routes you can use.

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