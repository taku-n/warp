use std::net::{SocketAddr, SocketAddrV6, Ipv6Addr};

use warp::Filter;

#[tokio::main]
async fn main() {
    // If you listen to an IPv6 socket, you listen to an IPv4 socket too.
    // Test by "curl -6 http://foo.com/" and "curl -4 http://foo.com/".
    // https://stackoverflow.com/questions/45281740/how-to-open-ipv4-and-ipv6-sockets-on-the-same-port-with-http-module-in-nodejs
    // https://github.com/processone/ejabberd/issues/984
    let http = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 80, 0, 0));

    // Any type of URI gets a same message e.g. nothing, /, /foo and /foo?bar=fuga.
    //let routes = warp::any().map(|| "hello, world\n");

    // Nothing and / get a message.
    //let routes = warp::path::end().map(|| "hello, world\n");

    // /hello, /hello/, /hello/foo, /hello/foo?bar=fuga, ... get a message.
    //let routes = warp::path("hello").map(|| "hello, world\n");

    // /hello and /hello/ get a message.
    //let routes = warp::path("hello").and(warp::path::end()).map(|| "hello, world\n");

    // Respond in HTML.
    //let routes = warp::any().map(|| warp::reply::html(HTML));

    let routes = warp::any().map(|| "hello, world\n").with(warp::trace::request());

    warp::serve(routes).run(http).await;
}

const HTML: &str = r#"<!DOCTYPE html>
<html>
<head><title>hello, world</title></head>
<body><h1>hello, world</h1></body>
</html>
"#;
