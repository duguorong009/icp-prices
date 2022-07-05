export const idlFactory = ({ IDL }) => {
  const PriceData = IDL.Record({
    'signature' : IDL.Vec(IDL.Nat8),
    'provider' : IDL.Principal,
    'asset' : IDL.Nat64,
    'timestamp' : IDL.Nat64,
    'is_closed' : IDL.Bool,
    'price' : IDL.Nat64,
  });
  return IDL.Service({
    'add_data' : IDL.Func([IDL.Nat64, PriceData], [IDL.Bool], []),
    'add_node' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Principal)], []),
    'get_data' : IDL.Func([IDL.Nat64], [IDL.Vec(PriceData)], ['query']),
    'remove_node' : IDL.Func([IDL.Principal], [IDL.Opt(IDL.Principal)], []),
  });
};
export const init = ({ IDL }) => { return []; };
