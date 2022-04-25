use cfg_if::cfg_if;

#[cfg(feature = "proxy-protocol")]
mod haproxy;

#[cfg(feature = "transport")]
mod transport;

cfg_if! {
    if #[cfg(feature = "tfo")] {
        mod tfo;
        use tfo::TcpStream;
        pub use tfo::TcpListener;
    } else {
        use tokio::net::TcpStream;
        pub use tokio::net::TcpListener;
    }
}

use std::io::Result;
use log::debug;

use tokio::net::TcpSocket;

use crate::utils::socket;
use crate::utils::{Ref, RemoteAddr, ConnectOpts};

#[allow(unused_variables)]
pub async fn connect_and_relay(
    mut inbound: TcpStream,
    remote: Ref<RemoteAddr>,
    conn_opts: Ref<ConnectOpts>,
) -> Result<()> {
    let ConnectOpts {
        fast_open,
        zero_copy,
        send_through,
        bind_interface,
        haproxy_opts,
        #[cfg(feature = "transport")]
        transport,
        ..
    } = conn_opts.as_ref();

    // before connect
    let remote = remote.to_sockaddr().await?;
    debug!("[tcp]remote resolved as {}", &remote);

    let socket = socket::new_socket(socket::Type::STREAM, &remote, &conn_opts)?;
    let socket = TcpSocket::from_std_stream(socket.into());

    // connect!
    #[cfg(not(feature = "tfo"))]
    let mut outbound = socket.connect(remote).await?;

    #[cfg(feature = "tfo")]
    let mut outbound = if *fast_open {
        TcpStream::connect_with_socket(socket, remote).await?
    } else {
        socket.connect(remote).await?.into()
    };

    // after connected
    #[cfg(feature = "proxy-protocol")]
    if haproxy_opts.send_proxy || haproxy_opts.accept_proxy {
        haproxy::handle_proxy_protocol(
            &mut inbound,
            &mut outbound,
            *haproxy_opts,
        )
        .await?;
    }

    let res = {
        #[cfg(feature = "transport")]
        {
            use transport::relay_transport;
            if let Some((ac, cc)) = transport {
                relay_transport(inbound, outbound, ac, cc).await
            } else {
                relay_plain(inbound, outbound, *zero_copy).await
            }
        }
        #[cfg(not(feature = "transport"))]
        {
            relay_plain(&mut inbound, &mut outbound, *zero_copy).await
        }
    };

    if let Err(e) = res {
        debug!("[tcp]forward error: {}, ignored", e);
    }
    Ok(())
}

#[inline]
async fn relay_plain(
    mut inbound: TcpStream,
    mut outbound: TcpStream,
    zero_copy: bool,
) -> Result<()> {
    #[cfg(all(target_os = "linux", feature = "zero-copy"))]
    if zero_copy {
        let (res, _, _) =
            realm_io::bidi_zero_copy(&mut inbound, &mut outbound).await;
        res
    } else {
        let (res, _, _) =
            realm_io::bidi_copy(&mut inbound, &mut outbound).await;
        res
    }

    #[cfg(not(all(target_os = "linux", feature = "zero-copy")))]
    {
        let (res, _, _) =
            realm_io::bidi_copy(&mut inbound, &mut outbound).await;
        res
    }
}
