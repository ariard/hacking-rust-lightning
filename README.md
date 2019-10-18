Welcome to the LN-Conf 2019 Rust-Lightning Hacking session ! 

You're goal is going to complete a skeleton of a tiny LN daemon using the Rust-Lightning
library. You have to read docs of interfaces and components. Think how to connect them
in your code and what's your runtime model will looks like, what kind of event they have to
manage and to dispatch them. Keep in mind than components are going to be driven mainly by
network event and CLI inputs. So you just have to write the code, debug and play with it.
have to write the code, debug and play with it.

Requirements
============

* synced testnet bitcoind
* rust toolchain >= 1.22
* some fortitude


Anatomy of a Lightning node
===========================

```
              - - - - - - - - - 
             /                 /
            /   BLOCKCHAIN    /
           /                 / 
           - - - - - - - - -                                  
                |                           __________________
                | blocks                 __|_________#3____   |
      __________|__________           __|________#2___    |   |
     |                     |         |       #1       |   |___|
     | ChainWatchInterface |         |ChannelMonitors |___|
     |_____________________|         |________________|
              |   \                     /                  ____________
              |    \  blocks           /  txns           _|_____#3__   |
              |     \ ________________/__             __|____#2__   |  |
              |     |                    |           |    #1    |   |__|
              |     | ManyChannelMonitor |           | Channels |___|
              |     |____________________|          ^|__________|
              |              ^                     /
              |               \                   /
              |                \ updates         /  msgs       
              | blocks          \               /
              |---------------> _v_____________/______
   ________________            |                       |
  |                | key       |                       | 
  | KeysInterface  |---------->| ChannelMessageHandler |  
  |________________|           |                       |^  
                               |_______________________| \  msgs                             - - - - - - - - 
                                     ^                 \  \     _____________               /              /           
                                     |                  \  \---|             |   blobs     /  LN NETWORK  /
                                     |                   \     | PeerHandler |----------->/              /
                                     |                    \--->|_____________|            - - - - - - - -
                            route    |                          /
                                     |                         /
                         ____________|__________              / msgs
                        |                       |            /
                        | RoutingMessageHandler |<-----------
                        |_______________________|


```

A quick sightview, you will have to dig into docs for details.

ChainWatchInterface
-------------------

A trait to request notifications of certain scripts as they appear the chain. You may combine it
with ChainListener to dispatch block connections to other components.


KeysInterface
-------------

A trait to get user secret and generate suitable lightning key material. Already given to
you via KeysManager as ligthning key material is a topic in itself.

ManyChannelMonitor
------------------

A trait to multiplex channel updates between different chain monitor. Or you can choose to
implement an unichannel daemon and use direcly ChannelMonitor. Your call !

PeerManager
-----------

Manager of peers, dispatch into other components. You may use net-tokio, which is an 
integration of it based on tokio.

ChannelMessageHandler
---------------------

A trait which describes an object receiving channel message. You may directly look on
ChannelManager, a struct implementing this trait and managing the channel logic.

RoutingMessageHandler
---------------------

A trait which can receive routing message. You may look on Router which is a basic routing implementation.


Challenges
==========

Basics
------

* have a working node
* connect to a node
* open a channel 

Advanced
--------

* send payment
* receive payment

Expert
------

* handle chain reorgs and channel cancellation
* handle revoked state broadcast and punish your peer
* broadcast a revoked state and DoS your peer to get out (lol)


Remember, it's more a architecture challenge than a coding one so take time to understand
interfaces.

Have fun !
