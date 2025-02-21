```plain
Usage: fm_transmit <COMMAND>

Commands:
  transmit  Transmit a mono audio file over FM
  receive   Receive a mono FM transmission
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```plain
Transmit a mono audio file over FM

Usage: fm_transmit transmit [OPTIONS] <AUDIO>

Arguments:
  <AUDIO>  Path to a .wav file to transmit. Only the first channel is proessed

Options:
  -f, --frequency <FREQUENCY>  The center frequency to transmit on [default: 100000000]
  -g, --gain <GAIN>            The transmit variable gain amplifier power setting. (In db) [default: 30]
  -h, --help                   Print help
```

```plain
Receive a mono FM transmission

Usage: fm_transmit receive [OPTIONS] <AUDIO>

Arguments:
  <AUDIO>  Path of a .wav file that will be created and written to

Options:
  -f, --frequency <FREQUENCY>  The center frequency to receive [default: 100000000]
  -g, --gain <GAIN>            The receive variable gain amplifier power setting. (In db) [default: 0]
  -l, --lna-gain <LNA_GAIN>    The receive low noise amplifier power setting. (In db) [default: 30]
  -h, --help                   Print help
```
