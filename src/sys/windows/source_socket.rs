use crate::{event, Interest, Registry, Token};
use crate::io_source::IoSource;

use std::io;
use std::os::windows::io::RawSocket;

/// Adapter for [`RawSocket`] providing an [`event::Source`] implementation.
///
/// `RawSocket` enables registering any type with an FD with [`Poll`].
///
/// While only implementations for TCP and UDP are provided, Mio supports
/// registering any FD that can be registered with the underlying OS selector.
/// `RawSocket` provides the necessary bridge.
///
/// Note that `RawSocket` takes a `&RawSocket`. This is because `RawSocket` **does
/// not** take ownership of the FD. Specifically, it will not manage any
/// lifecycle related operations, such as closing the FD on drop. It is expected
/// that the `RawSocket` is constructed right before a call to
/// [`Registry::register`]. See the examples for more detail.
///
/// [`event::Source`]: ../event/trait.Source.html
/// [`Poll`]: ../struct.Poll.html
/// [`Registry::register`]: ../struct.Registry.html#method.register
///
/// # Examples
///
/// Basic usage.
///
#[cfg_attr(
    all(feature = "os-poll", feature = "net", feature = "os-ext"),
    doc = "```"
)]
#[cfg_attr(
    not(all(feature = "os-poll", feature = "net", feature = "os-ext")),
    doc = "```ignore"
)]
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use mio::{Interest, Poll, Token};
/// use mio::windows::SourceSocket;
///
/// use std::os::windows::io::RawSocket;
/// use std::net::TcpListener;
///
/// // Bind a std listener
/// let listener = TcpListener::bind("127.0.0.1:0")?;
///
/// let poll = Poll::new()?;
///
/// // Register the listener
/// poll.registry().register(
///     &mut SourceSocket(&listener.as_raw_socket()),
///     Token(0),
///     Interest::READABLE)?;
/// #     Ok(())
/// # }
/// ```
///
/// Implementing [`event::Source`] for a custom type backed by a [`RawSocket`].
///
#[cfg_attr(all(feature = "os-poll", feature = "os-ext"), doc = "```")]
#[cfg_attr(not(all(feature = "os-poll", feature = "os-ext")), doc = "```ignore")]
/// use mio::{event, Interest, Registry, Token};
/// use mio::windows::SourceSocket;
///
/// use std::os::windows::io::RawSocket;
/// use std::io;
///
/// # #[allow(dead_code)]
/// pub struct MyIo {
///     fd: RawSocket,
/// }
///
/// impl event::Source for MyIo {
///     fn register(&mut self, registry: &Registry, token: Token, interests: Interest)
///         -> io::Result<()>
///     {
///         SourceSocket(&self.fd).register(registry, token, interests)
///     }
///
///     fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest)
///         -> io::Result<()>
///     {
///         SourceSocket(&self.fd).reregister(registry, token, interests)
///     }
///
///     fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
///         SourceSocket(&self.fd).deregister(registry)
///     }
/// }
/// ```
#[derive(Debug)]
pub struct SourceSocket<'a>(pub &'a RawSocket);

impl<'a> event::Source for SourceSocket<'a> {
    fn register(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        IoSource::new(self.0).register(registry, token, interests)
    }

    fn reregister(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        IoSource::new(self.0).reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        IoSource::new(self.0).deregister(registry)
    }
}
