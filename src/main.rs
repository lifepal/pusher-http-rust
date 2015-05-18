extern crate pusher;

use pusher::Pusher;
use std::collections::HashMap;

fn main() {

  let app_id = env!("ATOM_ID");
  let key = env!("ATOM_KEY");
  let secret = env!("ATOM_SECRET");

  let pusher = Pusher::new(app_id, key, secret);

  let mut hash_map = HashMap::new();
  hash_map.insert("message", "hello world");


  // pusher.trigger("test_channel", "my_event", &hash_map);

  pusher.trigger_exclusive("test_channel", "my_event", &hash_map, "45553.5500569");


  pusher.channels(None);

  let channels_params = vec![("filter_by_prefix", "presence-"), ("info", "user_count")];

  let channels = pusher.channels(Some(channels_params));

  println!("{:?}", channels);

  let channel_params = vec![("info", "user_count,subscription_count")];

  let channel = pusher.channel("presence-session-d41a439c438a100756f5-4bf35003e819bb138249-hu9e5NecuNr", Some(channel_params));
  println!("{:?}", channel);

  let users = pusher.channel_users("presence-session-d41a439c438a100756f5-4bf35003e819bb138249-hu9e5NecuNr");
  println!("{:?}", users);


}