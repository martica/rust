extern mod std;

use pipes::Chan;

type RingBuffer = ~[float];
type SamplesFn = fn~ (samples: &RingBuffer);

enum Msg
{
    GetSamples(~str, SamplesFn), // sample set name, callback which receives samples
}

fn foo(name: ~str, samples_chan: Chan<Msg>) {
    do task::spawn
    |copy name|
    {
        let callback: SamplesFn =
            |buffer|
            {
                for uint::range(0, buffer.len())
                    |i| {error!("%?: %f", i, buffer[i])}
            };
        samples_chan.send(GetSamples(copy name, callback));
    };
}

fn main() {}

