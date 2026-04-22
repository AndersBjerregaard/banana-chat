# Technical internals

## `tokio::sync::broadcast`

A multi-producer, multi-consumer broadcast queue. Each sent value is seen by
all consumers.

A [`Sender`] is used to broadcast values to **all** connected [`Receiver`]
values. [`Sender`] handles are clone-able, allowing concurrent send and
receive actions. [`Sender`] and [`Receiver`] are both `Send` and `Sync` as
long as `T` is `Send`.

When a value is sent, **all** [`Receiver`] handles are notified and will
receive the value. The value is stored once inside the channel and cloned on
demand for each receiver. Once all receivers have received a clone of the
value, the value is released from the channel.

A channel is created by calling [`channel`], specifying the maximum number
of messages the channel can retain at any given time.

New [`Receiver`] handles are created by calling [`Sender::subscribe`]. The
returned [`Receiver`] will receive values sent **after** the call to
`subscribe`.

This channel is also suitable for the single-producer multi-consumer
use-case, where a single sender broadcasts values to many receivers.

### `channel`

```rust
tokio::sync::broadcast
```

```rust
pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>)
where
    T: Clone,
```

Create a bounded, multi-producer, multi-consumer channel where each sent
value is broadcasted to all active receivers.

**Note:** The actual capacity may be greater than the provided `capacity`.

All data sent on [`Sender`] will become available on every active
[`Receiver`] in the same order as it was sent.

The `Sender` can be cloned to `send` to the same channel from multiple
points in the process or it can be used concurrently from an `Arc`. New
`Receiver` handles are created by calling [`Sender::subscribe`].

If all [`Receiver`] handles are dropped, the `send` method will return a
[`SendError`]. Similarly, if all [`Sender`] handles are dropped, the [`recv`]
method will return a [`RecvError`].

The capacity argument defines the size of the fixed-capacity,
retained ring buffer used by the channel.

Unlike a standard MPSC (multi-producer, single-consumer) channel
where messages might be dropped or blocked if the receiver is slow,
a broadcast channel is designed so that every message sent
is potentially seen by every active receiver.

#### 1. The Ring Buffer Mechanism

The `capacity` determines how many messages the channel can hold at any given time.
When the `Sender` broadcasts a message, it is placed into this circular buffer.

- *Shared Storage:* All receivers read from this same buffer.

- *Message Lifetime:* A message remains in the buffer until it has been "overwritten"
by new messages once the capacity is exceeded.

#### 2. Handling Slow Receivers (Lagging)

This is the most critical part of the `capacity` logic.
Because the buffer is fixed-size, if a specific `Receiver`
is too slow to keep up with the `Sender`,
the buffer will eventually wrap around and start overwriting the messages
that the slow receiver hasn't read yet.

- *Lag Error:* When a receiver realizes it has missed messages because they were overwritten,
it will return `RecvError::Lagged(n)`, where `n` is the number of messages it missed.

- *Catch-up:* After a lag error, the receiver’s internal cursor is moved to the oldest message
currently available in the buffer, allowing it to continue receiving new data.

#### 3. Memory Impact

Since `T` must implement `Clone`, the channel stores the original message once in the buffer.
When a receiver calls `recv()`, the message is cloned for that specific receiver.

- If `capacity` is 16, the channel will hold up to 16 String objects.

- The memory isn't freed until the message is overwritten or the channel is closed.

#### Summary Table

Feature         | Behavior
|       --      |       --      |
Type            | Fixed-size ring buffer.
Overflow        | Oldest messages are overwritten (not blocked).
Receiver impact | Slow receivers receive a `Lagged` error if they fall behind by more than `capacity`
Minimum Value   | Must be a power of two internally (Tokio handles the alignment, but usually you provide a standard `usize`).

*Note:* If you set the capacity to `1`, the channel can only hold one message.
If the sender sends a second message before the receiver reads the first,
that receiver will immediately lag.
