# libhackrf-rs

A *modern* [libhackrf](https://github.com/greatscottgadgets/hackrf/blob/71bc3940886a234ca119df9b15e24c9dcda7cb4d/host/libhackrf/src/hackrf.h) library wrapper that supports receiving and *transmitting*.

## Example

See the [fm_transmit](https://github.com/connorslade/libhackrf-rs/tree/main/fm_transmit) crate for a more complete example of how to use this library, it allows transmitting and receiving frequency modulated audio signals.

```rust
let hackrf = HackRf::open()?;
hackrf.set_sample_rate(2_000_000)?;
hackrf.set_freq(100_000_000)?;
hackrf.set_txvga_gain(16)?;

hackrf.start_tx(
    |_hackrf, buffer, _user| {
        for sample in buffer.iter_mut() {
            *sample = Complex::ZERO;
        }
    },
    (),
)?;

loop { thread::park() }
```
