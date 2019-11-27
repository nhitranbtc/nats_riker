use riker::actors::*;
use std::time::{Duration, SystemTime};
use chrono::DateTime;
use std::str;
use std::thread;
use std::time;

#[macro_use]
extern crate log;

#[derive(Clone, Debug)]
pub struct StartChild;

#[derive(Clone, Debug)]
pub struct ChildActor;

#[derive(Clone, Debug)]
pub struct FanOut;

#[actor(String, StartChild, FanOut)]
struct NatsActorManager;


impl NatsActorManager {
    fn actor() -> Self {
        NatsActorManager
    }
    fn props() -> BoxActorProd<NatsActorManager> {
        Props::new(NatsActorManager::actor)
    }
}

impl Actor for NatsActorManager {
    // we used the #[actor] attribute so NatsActorManagerMsg is the Msg type
    type Msg = NatsActorManagerMsg;

    fn recv(&mut self,
        ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        sender: Sender) {
        // Use the respective Receive<T> implementation
        self.receive(ctx, msg, sender);
    }
}

impl Receive<StartChild> for NatsActorManager {
    type Msg = NatsActorManagerMsg;

    fn receive(&mut self,
        ctx: &Context<Self::Msg>,
        _msg: StartChild,
        _sender: Sender) {
        for i in 1..10 {
            let childName = format!("child_{}", i);
            let actor = ctx.actor_of(NatsActorManager::props(), &childName).unwrap();
            info!{"StartChild{}:", actor};
            //actor.tell("hello".to_string(), None);
            /* thread::spawn(move || {
               let ten_millis = time::Duration::from_secs(10);
               thread::sleep(ten_millis);
               });*/
        }
    }
}

impl Receive<FanOut> for NatsActorManager {
    type Msg = NatsActorManagerMsg;
    fn receive(&mut self,
        ctx: &Context<Self::Msg>,
        _msg: FanOut,
        _sender: Sender) {
        info!{"FanOut!"};
        let hga = ctx.select("/user/nats-manager/*").unwrap();
        hga.try_tell("I've arrived safely".to_string(), None);
    }
}

impl Receive<String> for NatsActorManager {
    type Msg = NatsActorManagerMsg;
    fn receive(&mut self,
        _ctx: &Context<Self::Msg>,
        msg: String,
        _sender: Sender) {
        info!{"received msg: {}", msg};
    }
}

fn main() {
    let sys = ActorSystem::new().unwrap();
    let my_actor = sys.actor_of(NatsActorManager::props(), "nats-manager").unwrap();
    my_actor.tell(StartChild, None);
    //my_actor.tell("hello".to_string(), None);
    my_actor.tell(FanOut, None);
    std::thread::sleep(Duration::from_secs(20));
}
