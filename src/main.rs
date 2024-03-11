// Simple load ballancer with health check
// use pingora::prelude::*;
// use std::sync::Arc;
// pub struct LB(Arc<LoadBalancer<RoundRobin>>);
// pub struct SimpleProxy;
// #[async_trait]
// impl ProxyHttp for LB {
//     type CTX = ();
//
//     async fn upstream_request_filter(
//         &self,
//         _session: &mut Session,
//         upstream_request: &mut RequestHeader,
//         _ctx: &mut Self::CTX,
//     ) -> Result<()> {
//         upstream_request
//             .insert_header("Host", "one.one.one.one")
//             .unwrap();
//         Ok(())
//     }
//
//     fn new_ctx(&self) -> () {
//         ()
//     }
//
//     async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
//         let upstream = self.0.select(b"", 256).unwrap();
//
//         println!("upstream peer is: {:?}", upstream);
//
//         // Set SNI to one.one.one.one
//         let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
//         Ok(peer)
//     }
// }
//
// fn main() {
//     let mut my_server = Server::new(None).unwrap();
//     my_server.bootstrap();
//
//     let mut upstreams =
//         LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443", "127.0.0.1:343"]).unwrap();
//     let hc = TcpHealthCheck::new();
//     upstreams.set_health_check(hc);
//     upstreams.health_check_frequency = Some(std::time::Duration::from_secs(1));
//
//     let background = background_service("health check", upstreams);
//     let upstreams = background.task();
//
//     let mut lb = http_proxy_service(&my_server.configuration, LB(upstreams));
//     lb.add_tcp("0.0.0.0:6188");
//
//     my_server.add_service(background);
//     my_server.add_service(lb);
//
//     my_server.run_forever();
// }

// Simple reverse proxy
use async_trait::async_trait;

use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_proxy::{ProxyHttp, Session};

pub struct MyProxy {}

#[async_trait]
impl ProxyHttp for MyProxy {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        session
            .req_header_mut()
            .insert_header("Host", "127.0.0.1")
            .unwrap();
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let addr = ("127.0.0.1", 3000);

        let peer = Box::new(HttpPeer::new(addr, false, "127.0.0.1".to_string()));
        Ok(peer)
    }
}

fn main() {
    env_logger::init();

    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let mut my_proxy = pingora_proxy::http_proxy_service(&my_server.configuration, MyProxy {});
    my_proxy.add_tcp("0.0.0.0:8888");

    my_server.add_service(my_proxy);
    my_server.run_forever();
}
