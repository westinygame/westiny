Network testing guide
=====================

This guide includes various network testing methods and tips for Westiny.


Simulating network behavior (linux only)
----------------------------------------

Nowadays we have so good internet connections, sometimes it is hard to
really test or reproduce a given network-related bug. Fortunately, Linux
has a nice tool for traffic control since ages: `tc`

### Latency, packet loss, reordering (netem)

Add latency to localhost, only for a given port:
(Source: https://stackoverflow.com/a/40203517)

It will add an emulated 300ms latency for every packet that arrives at port 5745 (both TCP and UDP),
but it lets through any other packet.

```bash
sudo tc qdisc add dev lo root handle 1: prio priomap 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
sudo tc qdisc add dev lo parent 1:2 handle 20: netem delay 300ms
sudo tc filter add dev lo parent 1:0 protocol ip u32 match ip dport 5745 0xffff flowid 1:2
```

Removal of the rule:
```
sudo tc qdisc del dev lo root
```

For detailed emulated network behaviors see [netem's documentation](https://wiki.linuxfoundation.org/networking/netem), but a few example:

 * Add delay with variation of +-50ms: `netem delay 300ms 50ms`
 * Add packet loss: `netem loss 1%` (every 100th packet will be dropped on average)

#### How to add multiple netem rules?

TODO

