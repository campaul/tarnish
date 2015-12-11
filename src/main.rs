extern crate docopt;
extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;

use docopt::Docopt;
use hyper::Client;
use hyper::Server;
use hyper::header::Connection;
use hyper::server::Handler;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri;

const USAGE: &'static str = "
Tarnish

Usage:
  tarnish <proxy>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_proxy: String,
}

struct ProxyHandler {
    uri_base: String,
}

impl Handler for ProxyHandler {
    fn handle(&self, proxy_request: Request, proxy_response: Response) {
        let client = Client::new();

        match (proxy_request.method, proxy_request.uri) {
            (hyper::Get, RequestUri::AbsolutePath(ref path)) => {
                let uri = self.uri_base.clone() + &path;
                let mut response = client.get(&uri)
                                         .header(Connection::close())
                                         .send()
                                         .unwrap();

                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();

                proxy_response.send(&body.into_bytes()).unwrap();
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());

    let uri_base = "http://".to_string() + &args.arg_proxy;

    Server::http("127.0.0.1:3000").unwrap().handle(ProxyHandler {
        uri_base: uri_base,
    });
}
