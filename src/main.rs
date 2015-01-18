//extern crate openssl;
extern crate libc;

use std::io::TcpStream;
use std::ptr;
use std::ffi::CString;

//use openssl::ssl::{SslStream, SslContext, SslMethod};
//use std::io::net::udp::UdpSocket;
//use std::io::net::ip::{Ipv4Addr, SocketAddr};

use libc::{c_void, c_int, c_char};

struct AvahiSimplePoll;
struct AvahiPoll;
struct DBusConnection;
struct AvahiEntryGroup;
struct AvahiDomainBrowser;
struct AvahiServiceBrowser;
struct AvahiServiceTypeBrowser;
struct AvahiServiceResolver;
struct AvahiHostNameResolver;
struct AvahiAddressResolver;
struct AvahiRecordBrowser;
struct AvahiAddress;
struct AvahiStringList;


type AvahiClientCallback = extern fn(AvahiClient, AvahiClientState, *const c_void);

#[repr(C)]
struct AvahiClient {
  poll_api: *const AvahiPoll,
  bus: *const DBusConnection,
  error: u16,
  state: AvahiClientState,
  flags: AvahiClientFlags,
  version_string: *const c_char,
  host_name: *const c_char,
  host_name_fqdn: *const c_char,
  domain_name: *const c_char,
  local_service_cookie: u32,
  local_service_cookie_valid: u16,
  callback: extern "C" fn(*const AvahiClient, AvahiClientState, *const c_void),
  userdata: *const c_void,
  groups: *const AvahiEntryGroup,
  domain_browsers: *const AvahiDomainBrowser,
  service_browsers: *const AvahiServiceBrowser,
  service_type_browsers: *const AvahiServiceTypeBrowser,
  service_resolvers: *const AvahiServiceResolver,
  hsot_name_resolvers: *const AvahiHostNameResolver,
  address_resolvers: *const AvahiAddressResolver,
  record_browsers: *const AvahiRecordBrowser
}

#[repr(C)]
#[deriving(Show)]
enum AvahiClientFlags {
  AVAHI_CLIENT_IGNORE_USER_CONFIG,
  AVAHI_CLIENT_NO_FAIL
}

#[repr(C)]
#[deriving(Show)]
enum AvahiClientState {
  AVAHI_CLIENT_S_REGISTERING,
  AVAHI_CLIENT_S_RUNNING,
  AVAHI_CLIENT_S_COLLISION,
  AVAHI_CLIENT_FAILURE,
  AVAHI_CLIENT_CONNECTING
}

#[repr(C)]
enum AvahiLookupFlags {
  AVAHI_LOOKUP_NO_TXT,
  AVAHI_LOOKUP_NO_ADDRESS
}

#[repr(C)]
enum AvahiLookupResultFlags {
  AVAHI_LOOKUP_RESULT_CACHED,
  AVAHI_LOOKUP_RESULT_WIDE_AREA,
  AVAHI_LOOKUP_RESULT_MULTICAST,
  AVAHI_LOOKUP_RESULT_LOCAL,
  AVAHI_LOOKUP_RESULT_OUR_OWN,
  AVAHI_LOOKUP_RESULT_STATIC
}

#[repr(C)]
enum AvahiBrowserEvent {
  AVAHI_BROWSER_NEW,
  AVAHI_BROWSER_REMOVE,
  AVAHI_BROWSER_CACHE_EXHAUSTED,
  AVAHI_BROWSER_ALL_FOR_NOW,
  AVAHI_BROWSER_FAILURE
}

#[repr(C)]
enum AvahiProtocol {
  AVAHI_PROTO_INET = 0,
  AVAHI_PROTO_INET6 = 1,
  AVAHI_PROTO_UNSPEC = -1
}

#[repr(C)]
enum AvahiResolverEvent {
  AVAHI_RESOLVER_FOUND,
  AVAHI_RESOLVER_FAILURE
}

type ServiceBrowserCallback = extern fn(*mut AvahiServiceBrowser, 
                                        c_int, 
                                        c_int, 
                                        AvahiBrowserEvent, 
                                        *const c_char, 
                                        *const c_char, 
                                        *const c_char, 
                                        AvahiLookupResultFlags, 
                                        *mut c_void);

type ServiceResolverCallback = extern fn(*mut AvahiServiceResolver, 
                                         c_int, 
                                         c_int, 
                                         AvahiResolverEvent, 
                                         *const c_char, 
                                         *const c_char, 
                                         *const c_char, 
                                         *const c_char, 
                                         *const AvahiAddress, 
                                         u16, 
                                         AvahiStringList, 
                                         AvahiLookupResultFlags, 
                                         *mut c_void);

#[link(name = "avahi-common")]
#[link(name = "avahi-client")]
extern { 
  
  fn avahi_simple_poll_new() -> *mut AvahiSimplePoll;

  fn avahi_client_new(poll_api: *const AvahiPoll,
                      flags: AvahiClientFlags,
                      callback: extern fn(*mut AvahiClient, AvahiClientState, *mut c_void),
                      userdata: *mut c_void,
                      error: *mut c_int) -> *mut AvahiClient;

  fn avahi_simple_poll_get(s: *mut AvahiSimplePoll) -> *mut AvahiPoll;

  fn avahi_service_browser_new(client: *mut AvahiClient,
                               interface: c_int,
                               protocol: c_int,
                               le_type: *const c_char,
                               domain: *const c_char,
                               flags: AvahiLookupFlags,
                               callback: ServiceBrowserCallback,
                               userdata: *mut c_void) -> *mut AvahiServiceBrowser;

  fn avahi_simple_poll_loop(s: *mut AvahiSimplePoll) -> c_int;

  fn avahi_service_browser_free(b: *mut AvahiServiceBrowser) -> c_int; 

  fn avahi_client_free(client: *mut AvahiClient);

  fn avahi_service_resolver_new(client: *mut AvahiClient,
                                interface: c_int,
                                protocol: c_int,
                                name: *const c_char,
                                le_type: *const c_char,
                                domain: *const c_char,
                                aprotocol: AvahiProtocol,
                                flags: AvahiLookupFlags,
                                callback: ServiceResolverCallback,
                                userdata: *mut c_void) -> *mut AvahiServiceResolver;
  
}

extern fn client_callback(s: *mut AvahiClient, state: AvahiClientState, userdata: *mut c_void) {
  println!("in client callback. {}", state as int);
}

extern fn resolve_callback(r: *mut AvahiServiceResolver,
                           interface: c_int,
                           protocol: c_int,
                           event: AvahiResolverEvent,
                           name: *const c_char,
                           le_type: *const c_char,
                           domain: *const c_char,
                           host_name: *const c_char,
                           address: *const AvahiAddress,
                           port: u16,
                           txt: AvahiStringList,
                           flags: AvahiLookupResultFlags,
                           userdata: *mut c_void) {

  println!("in resolve callback");
}

extern fn browse_callback(b: *mut AvahiServiceBrowser, interface: c_int, protocol: c_int, event: AvahiBrowserEvent, name: *const c_char, le_type: *const c_char, domain: *const c_char, flags: AvahiLookupResultFlags, userdata: *mut c_void) {

  //let mut client: &mut AvahiClient = unsafe { &mut *(userdata as *mut AvahiClient)};
  //let client_ptr = &mut client as *mut _ as *mut c_void;

  match event {
    AvahiBrowserEvent::AVAHI_BROWSER_NEW               => {
        unsafe {
            
            let mut client: &mut AvahiClient = unsafe { &mut *(userdata as *mut AvahiClient)};
            
            println!("{}", client.error);

            let client_ptr = &mut client as *mut _ as *mut c_void;
            avahi_service_resolver_new(client, interface, protocol, name, le_type, domain, 
                                       AvahiProtocol::AVAHI_PROTO_UNSPEC, AvahiLookupFlags::AVAHI_LOOKUP_NO_TXT, 
                                       resolve_callback, /* client_ptr */ ptr::null_mut());
                                       
        }
        println!("New one found");
    }
    AvahiBrowserEvent::AVAHI_BROWSER_ALL_FOR_NOW       => println!("All for now"),
    AvahiBrowserEvent::AVAHI_BROWSER_CACHE_EXHAUSTED   => println!("Cache exhausted"),
    _                                                  => println!("Something else found")
  }
}

fn main() {
  unsafe {
    let mut error: i32 = 0;
    let simple_poll = avahi_simple_poll_new();
     
    let mut client = avahi_client_new(
                        avahi_simple_poll_get(simple_poll),
                        AvahiClientFlags::AVAHI_CLIENT_IGNORE_USER_CONFIG,
                        client_callback,
                        ptr::null_mut(),
                        &mut error
                      );
    
    // This is weird.. figure it out
    let client_ptr: *mut c_void = &mut client as *mut _ as *mut c_void;

    let _type = CString::from_slice("_googlecast._tcp".as_bytes()).as_ptr();
    let mut sb = avahi_service_browser_new(client, -1, -1, _type, ptr::null_mut(), AvahiLookupFlags::AVAHI_LOOKUP_NO_TXT, browse_callback, client_ptr);

    avahi_simple_poll_loop(simple_poll);

    //avahi_service_browser_free(sb);
    //avahi_client_free(client);

  }

}



//  let ctx = SslContext::new(SslMethod::Tlsv1_2).unwrap();
