#![feature(box_syntax)]

//extern crate openssl;
extern crate libc;

use std::io::TcpStream;
use std::ptr;
use std::ffi;
use std::str;


//use openssl::ssl::{SslStream, SslContext, SslMethod};
//use std::io::net::udp::UdpSocket;
//use std::io::net::ip::{Ipv4Addr, SocketAddr};

use libc::{c_void, c_int, c_char, size_t};

mod avahi;




extern fn resolve_callback(r: *mut avahi::AvahiServiceResolver,
                           interface: c_int,
                           protocol: c_int,
                           event: avahi::AvahiResolverEvent,
                           name: *const c_char,
                           le_type: *const c_char,
                           domain: *const c_char,
                           host_name: *const c_char,
                           address: *const avahi::AvahiAddress,
                           port: u16,
                           txt: *mut avahi::AvahiStringList,
                           flags: avahi::AvahiLookupResultFlags,
                           userdata: *mut c_void) {

  match event {
    avahi::AvahiResolverEvent::AVAHI_RESOLVER_FAILURE => {
      println!("Failed to resolve");
    }
    
    avahi::AvahiResolverEvent::AVAHI_RESOLVER_FOUND => {
      unsafe {
        let name = str::from_utf8(ffi::c_str_to_bytes(&name)).unwrap();
        let host_name = str::from_utf8(ffi::c_str_to_bytes(&host_name)).unwrap();

        let mut vec: Vec<u8> = Vec::with_capacity(avahi::AVAHI_ADDRESS_STR_MAX as usize);
        let mut a = std::ffi::CString::from_vec(vec).as_ptr();
        avahi::avahi_address_snprint(a, avahi::AVAHI_ADDRESS_STR_MAX as u32, address);
        let address = str::from_utf8(ffi::c_str_to_bytes(&a)).unwrap();

        let txt = avahi::avahi_string_list_to_string(txt);
        let t1 = ffi::c_str_to_bytes(&txt);
        avahi::avahi_free(txt as *mut c_void);

        let result = avahi::AvahiResolveResult {
          name: String::from_str(name),
          host_name: String::from_str(host_name),
          address: String::from_str(address),
          port: port
        };

        println!("Resolved! {:?}", result);

      }
      
    }
  }
}


extern fn browse_callback(b: *mut avahi::AvahiServiceBrowser, interface: c_int, protocol: c_int, event: avahi::AvahiBrowserEvent, 
                          name: *const c_char, le_type: *const c_char, domain: *const c_char, flags: avahi::AvahiLookupResultFlags, 
                          userdata: *mut c_void) {

  match event {
    avahi::AvahiBrowserEvent::AVAHI_BROWSER_NEW => {
      println!("{:?}", event);
      unsafe {
          
          let mut client: &mut avahi::AvahiClient = &mut *(userdata as *mut avahi::AvahiClient);
          
          avahi::avahi_service_resolver_new(client, interface, protocol, name, le_type, domain, 
                                     avahi::AvahiProtocol::AVAHI_PROTO_UNSPEC, avahi::AvahiLookupFlags::AVAHI_LOOKUP_NO_TXT, 
                                     *Box::new(resolve_callback), userdata);
                                     
      }
        
    }
    _ => println!("{:?}", event)
  }
}

extern fn client_callback(s: *mut avahi::AvahiClient, state: avahi::AvahiClientState, userdata: *mut c_void) {
}

fn main() {
  unsafe {
    let mut error: i32 = 0;
    let simple_poll = avahi::avahi_simple_poll_new();

    let poll = avahi::avahi_simple_poll_get(simple_poll);

    let client = avahi::avahi_client_new(
                        poll,
                        avahi::AvahiClientFlags::AVAHI_CLIENT_IGNORE_USER_CONFIG,
                        *Box::new(client_callback),
                        ptr::null_mut(),
                        &mut error
                      );
    
    // // This is weird.. figure it out
    let client_ptr: *mut c_void = client as *mut c_void;

    let _type = std::ffi::CString::from_slice("_googlecast._tcp".as_bytes()).as_ptr();
    let sb = avahi::avahi_service_browser_new(client, -1, -1, _type, 
                                            ptr::null_mut(), avahi::AvahiLookupFlags::AVAHI_LOOKUP_NO_TXT, 
                                            *Box::new(browse_callback), client_ptr);

    avahi::avahi_simple_poll_loop(simple_poll);

    avahi::avahi_service_browser_free(sb);
    avahi::avahi_client_free(client);
    avahi::avahi_simple_poll_free(simple_poll);

  }

}



//  let ctx = SslContext::new(SslMethod::Tlsv1_2).unwrap();
