export const idlFactory = ({ IDL }) => {
  const PriceData = IDL.Record({
    'signature' : IDL.Text,
    'provider' : IDL.Principal,
    'asset' : IDL.Nat32,
    'timestamp' : IDL.Nat32,
    'is_closed' : IDL.Bool,
    'price' : IDL.Nat32,
  });
  return IDL.Service({
    'add_data' : IDL.Func([IDL.Nat32, PriceData], [IDL.Bool], []),
    'add_node' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Principal)], []),
    'get_caller' : IDL.Func([], [IDL.Principal], ['query']),
    'get_data' : IDL.Func([IDL.Nat32], [IDL.Vec(PriceData)], ['query']),
    'get_nodes' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'get_owner' : IDL.Func([], [IDL.Opt(IDL.Principal)], ['query']),
    'remove_node' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Principal)], []),
  });
};
export const init = ({ IDL }) => { return []; };
