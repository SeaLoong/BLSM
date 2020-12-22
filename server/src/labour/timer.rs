use actix::clock::{delay_for, Delay};
use actix::{Actor, ActorFuture, AsyncContext, ContextFutureSpawner, SpawnHandle};
use actix_web_actors::ws;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

pub struct TimerFn<A>
where
    A: Actor,
{
    f: fn(&mut A, &mut A::Context),
    timeout: Delay,
}

impl<A> TimerFn<A>
where
    A: Actor,
{
    /// Creates a new `TimerFunc` with the given duration.
    pub fn new<F>(timeout: Duration, f: fn(&mut A, &mut A::Context)) -> TimerFn<A> {
        TimerFn {
            f,
            timeout: delay_for(timeout),
        }
    }
}

impl<A> ActorFuture for TimerFn<A>
where
    A: Actor,
{
    type Output = ();
    type Actor = A;

    fn poll(
        self: Pin<&mut Self>,
        act: &mut Self::Actor,
        ctx: &mut <Self::Actor as Actor>::Context,
        task: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let this = self.get_mut();
        match Pin::new(&mut this.timeout).poll(task) {
            Poll::Ready(_) => {
                (this.f)(act, ctx);
                Poll::Ready(())
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

struct TimerFnBox {}

pub struct Timer<A, C>
where
    A: Actor<Context = C>,
    C: AsyncContext<A>,
{
    pub duration: Duration,
    spawn_handle: Option<SpawnHandle>,
    f: fn(&mut A, &mut C),
}

impl<A, C> Timer<A, C>
where
    A: Actor<Context = C>,
    C: AsyncContext<A>,
{
    #[inline]
    pub fn new(duration: Duration, f: fn(&mut A, &mut C)) -> Timer<A, C> {
        Timer {
            duration,
            spawn_handle: None,
            f,
        }
    }

    #[inline]
    pub fn start(&mut self, ctx: &mut C) {
        self.stop(ctx);
        self.spawn_handle = Some(ctx.spawn(TimerFn::new::<A>(self.duration, self.f)));
    }

    #[inline]
    pub fn stop(&mut self, ctx: &mut C) {
        if let Some(handle) = self.spawn_handle.take() {
            ctx.cancel_future(handle);
        }
    }
}
