import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface PriceData {
  'signature' : Array<number>,
  'provider' : Principal,
  'asset' : number,
  'timestamp' : number,
  'is_closed' : boolean,
  'price' : number,
}
export interface _SERVICE {
  'add_data' : ActorMethod<[number, PriceData], boolean>,
  'add_node' : ActorMethod<[Principal], [] | [Principal]>,
  'get_data' : ActorMethod<[number], Array<PriceData>>,
  'get_owner' : ActorMethod<[], [] | [Principal]>,
  'remove_node' : ActorMethod<[Principal], [] | [Principal]>,
}
