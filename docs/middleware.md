## Adding a middleware on pallet-ibc

### Overview

The middleware is a module that wraps another module and adds some additional functionality to it. In the context of
IBC, it means that the middleware is a module that is executed before/after the IBC module, and can access the packet
data, take fees, send new packets etc. The middleware should be considered not a separate module, but rather a part of
the module that
it's wrapping.

To add a middleware for some IBC application (e.g., ics-20 - transfer app), one needs to create a new struct that
implements the `ibc::Module` trait and include it in the chain of other modules in runtime. Another important
step is to make sure that the middleware [satisfies](#potential-issues) IBC packet flow to avoid unexpected behavior
of your app (for example, a possibility for double-spending).

### Implementing `Module` trait and adding to runtime

As an example, we'll take a look at the existing module for taking fees on transfers, `ics20_fee` (`contracts/pallet-ibc/src/ics20_fee`).
First, we need to create a struct, that contains an `inner` field, which is of type of the next module in the chain, so
we can call it before/after our module execution is finished.

```rust
pub struct Ics20ServiceCharge<S: Module> {
    inner: S,
}
```

The `Module` contains various amount of callback methods that we can use, but probably the most important ones are
`on_recv_packet`, `on_acknowledgement_packet` and `on_timeout_packet`. In our case, we want to take fee from the user 
when the packet arrives and the transfer already happened, so the implementation will look something like this:

```rust
impl<S: Module> Module for Ics20ServiceCharge<S> {
    fn on_recv_packet(
        &self,
        _ctx: &dyn ModuleCallbackContext,
        output: &mut ModuleOutputBuilder,
        packet: &mut Packet,
        relayer: &Signer,
    ) -> Result<Acknowledgement, Error> {
        let mut ctx = Context::<T>::default();
        let ack = self.inner.on_recv_packet(&mut ctx, output, packet, relayer)?;
        let _ = Self::process_fee(&mut ctx, packet, &ack).map_err(|e| {
            log::error!(target: "pallet_ibc", "Error processing fee: {:?}", e);
        });
        Ok(ack)
    }

    // ...
}
```

As you can see, here we're first propagating the call to the inner module (which will be the `ics20` app, eventually),
and only then taking the fee. We could do it the other way around, but it would require additional checks to be made
(like, the token should exist, the amount should be correct, etc.). You may also notice, that the error from `process_fee`
function is ignored. This is because we don't want to fail the whole chain of calls if the fee is not taken, because it
may lead to critical problems (more in [Potential issues](#potential-issues) section).

The other methods should just call the inner module, like this:

```rust
impl<S: Module> Module for Ics20ServiceCharge<S> {
    fn on_acknowledgement_packet(
        &mut self,
        ctx: &dyn ModuleCallbackContext,
        output: &mut ModuleOutputBuilder,
        packet: &mut Packet,
        acknowledgement: &Acknowledgement,
        relayer: &Signer,
    ) -> Result<(), Ics04Error> {
        self.inner
            .on_acknowledgement_packet(ctx, output, packet, acknowledgement, relayer)
    }
    // ...
}
```

The final step is to add the module to the runtime. In the pallet's Config, you can find `Router` associated type, which
in a concrete implementation may look like

```rust
pub struct Router {
    ics20: ics20::memo::Memo<ics20_fee::Ics20ServiceCharge<ics20::IbcModule>>,
}
```

The modules here are nested in each other, forming a chain. In this case (assuming that the Memo module is executed
after the inner),
the flow will be `ics20 -> ics20_fee -> memo`.

### Potential issues

1. It's important to understand that the middleware is not a separate module, but rather a part of the module that it's
   wrapping. This means that the middleware should not fail the execution after the execution of the `inner` module has
   succeeded, because it may lead to unexpected behavior of the app. In our example, if the middleware fails (by
   throwing an error), the packet won't be acknowledged, but the transfer will still happen. This means that even that
   the tokens were transferred,
   the whole packet is considered as failed and the tokens on the source chain will be returned back to the sender's
   account. That's why we're ignoring the error from `process_fee` function, because we don't want the packet to fail
   when the transfer already happened.

2. Another important thing to note is that the middleware should not change the packet data, because it may lead to
   the same problem as above. For example, if the middleware changes the amount of tokens in the packet, the recipient
   will receive more tokens than the sender sent.
