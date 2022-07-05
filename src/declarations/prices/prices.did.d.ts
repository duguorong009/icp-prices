import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface PriceData {
  'signature' : Array<number>,
  'provider' : Principal,
  'asset' : bigint,
  'timestamp' : bigint,
  'is_closed' : boolean,
  'price' : bigint,
}
export interface _SERVICE {
  'add_data' : ActorMethod<[bigint, PriceData], boolean>,
  'add_node' : ActorMethod<[Principal], [] | [Principal]>,
  'get_data' : ActorMethod<[bigint], Array<PriceData>>,
  'remove_node' : ActorMethod<[Principal], [] | [Principal]>,
}
