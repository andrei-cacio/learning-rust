extern crate pnet;
extern crate chrono;
extern crate timer;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::EthernetDataLinkSender;
use pnet::datalink::EthernetDataLinkReceiver;
use pnet::packet::{Packet, MutablePacket};

use std::env;
use std::usize;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;

// Invoke as echo <interface name>
fn main() {
    let interface_name = env::args()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Invalid number of arguments");
            std::process::exit(1);
        });

    if interface_name == "list" {
        print_interfaces();
    } else {
        let (tx, rx) = channel();
        thread::spawn(|| handle_packet_count(rx));
        thread::spawn(|| listen_for_packets(interface_name, tx));
        loop {};
    }
}

fn print_interfaces() {
    let interfaces = datalink::interfaces();
    let interface_names_iter = interfaces
        .into_iter()
        .map(|iface: NetworkInterface| iface.name);

    for iface in interface_names_iter {
        println!("{}", iface);
    }
}

fn print_bandwith(rx: Receiver<usize>) {
    loop {
        let size = rx.recv().unwrap();
        println!("{}", size);
    }
}

fn create_datalink_channel(interface_name: String) -> (Box<EthernetDataLinkSender>, Box<EthernetDataLinkReceiver>){
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .find(interface_names_match);

    if interface.is_none() {
        println!("Interface not found");
        std::process::exit(1);
    }

    // Create a new channel, dealing with layer 2 packets
    return match datalink::channel(&interface.unwrap(), Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the data link channel: {}", e)
    };
}

fn listen_for_packets(interface_name: String, tx_bandwidth: Sender<usize>) {
    let (mut tx, mut rx) = create_datalink_channel(interface_name);
    let mut iter = rx.iter();
    loop {
        match iter.next() {
            Ok(packet) => {
                tx_bandwidth.send(packet.packet().len());
                tx.build_and_send(1, packet.packet().len(),
                                  &mut |mut new_packet| {
                                      new_packet.clone_from(&packet);
                                  });
            },
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn handle_packet_count(rx: Receiver<usize>) {
    let packets = Arc::new(Mutex::new(vec![]));
    let timer = timer::Timer::new();
    let cloned_packets = packets.clone();
    let guard = timer.schedule_repeating(chrono::Duration::seconds(1), move || {
        println!("{:?}", cloned_packets);
    });

    loop {
        let nr = rx.recv().unwrap();
        let packets = packets.clone();
        let mut locked_packets = packets.lock().unwrap();
        locked_packets.push(nr);
    }
}