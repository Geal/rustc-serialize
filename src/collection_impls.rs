// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementations of serialization for structures found in libcollections

use std::default::Default;
use std::hash::{Hash, Hasher};

use {Decodable, Encodable, Decoder, Encoder};
use std::collections::{DList, RingBuf, BTreeMap, BTreeSet, HashMap, HashSet, VecMap};

impl<
    E,
    S: Encoder<E>,
    T: Encodable<S, E>
> Encodable<S, E> for DList<T> {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_seq(self.len(), |s| {
            for (i, e) in self.iter().enumerate() {
                try!(s.emit_seq_elt(i, |s| e.encode(s)));
            }
            Ok(())
        })
    }
}

impl<E, D:Decoder<E>,T:Decodable<D, E>> Decodable<D, E> for DList<T> {
    fn decode(d: &mut D) -> Result<DList<T>, E> {
        d.read_seq(|d, len| {
            let mut list = DList::new();
            for i in range(0u, len) {
                list.push_back(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
            }
            Ok(list)
        })
    }
}

impl<
    E,
    S: Encoder<E>,
    T: Encodable<S, E>
> Encodable<S, E> for RingBuf<T> {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_seq(self.len(), |s| {
            for (i, e) in self.iter().enumerate() {
                try!(s.emit_seq_elt(i, |s| e.encode(s)));
            }
            Ok(())
        })
    }
}

impl<E, D:Decoder<E>,T:Decodable<D, E>> Decodable<D, E> for RingBuf<T> {
    fn decode(d: &mut D) -> Result<RingBuf<T>, E> {
        d.read_seq(|d, len| {
            let mut deque: RingBuf<T> = RingBuf::new();
            for i in range(0u, len) {
                deque.push_back(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
            }
            Ok(deque)
        })
    }
}

impl<
    E,
    S: Encoder<E>,
    K: Encodable<S, E> + PartialEq + Ord,
    V: Encodable<S, E> + PartialEq
> Encodable<S, E> for BTreeMap<K, V> {
    fn encode(&self, e: &mut S) -> Result<(), E> {
        e.emit_map(self.len(), |e| {
            let mut i = 0;
            for (key, val) in self.iter() {
                try!(e.emit_map_elt_key(i, |e| key.encode(e)));
                try!(e.emit_map_elt_val(i, |e| val.encode(e)));
                i += 1;
            }
            Ok(())
        })
    }
}

impl<
    E,
    D: Decoder<E>,
    K: Decodable<D, E> + PartialEq + Ord,
    V: Decodable<D, E> + PartialEq
> Decodable<D, E> for BTreeMap<K, V> {
    fn decode(d: &mut D) -> Result<BTreeMap<K, V>, E> {
        d.read_map(|d, len| {
            let mut map = BTreeMap::new();
            for i in range(0u, len) {
                let key = try!(d.read_map_elt_key(i, |d| Decodable::decode(d)));
                let val = try!(d.read_map_elt_val(i, |d| Decodable::decode(d)));
                map.insert(key, val);
            }
            Ok(map)
        })
    }
}

impl<
    E,
    S: Encoder<E>,
    T: Encodable<S, E> + PartialEq + Ord
> Encodable<S, E> for BTreeSet<T> {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_seq(self.len(), |s| {
            let mut i = 0;
            for e in self.iter() {
                try!(s.emit_seq_elt(i, |s| e.encode(s)));
                i += 1;
            }
            Ok(())
        })
    }
}

impl<
    E,
    D: Decoder<E>,
    T: Decodable<D, E> + PartialEq + Ord
> Decodable<D, E> for BTreeSet<T> {
    fn decode(d: &mut D) -> Result<BTreeSet<T>, E> {
        d.read_seq(|d, len| {
            let mut set = BTreeSet::new();
            for i in range(0u, len) {
                set.insert(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
            }
            Ok(set)
        })
    }
}

impl<
    E,
    S: Encoder<E>,
    K: Encodable<S, E> + Hash<X> + Eq,
    V: Encodable<S, E>,
    X,
    H: Hasher<X>
> Encodable<S, E> for HashMap<K, V, H> {
    fn encode(&self, e: &mut S) -> Result<(), E> {
        e.emit_map(self.len(), |e| {
            let mut i = 0;
            for (key, val) in self.iter() {
                try!(e.emit_map_elt_key(i, |e| key.encode(e)));
                try!(e.emit_map_elt_val(i, |e| val.encode(e)));
                i += 1;
            }
            Ok(())
        })
    }
}

impl<
    E,
    D: Decoder<E>,
    K: Decodable<D, E> + Hash<S> + Eq,
    V: Decodable<D, E>,
    S,
    H: Hasher<S> + Default
> Decodable<D, E> for HashMap<K, V, H> {
    fn decode(d: &mut D) -> Result<HashMap<K, V, H>, E> {
        d.read_map(|d, len| {
            let hasher = Default::default();
            let mut map = HashMap::with_capacity_and_hasher(len, hasher);
            for i in range(0u, len) {
                let key = try!(d.read_map_elt_key(i, |d| Decodable::decode(d)));
                let val = try!(d.read_map_elt_val(i, |d| Decodable::decode(d)));
                map.insert(key, val);
            }
            Ok(map)
        })
    }
}

impl<
    E,
    S: Encoder<E>,
    T: Encodable<S, E> + Hash<X> + Eq,
    X,
    H: Hasher<X>
> Encodable<S, E> for HashSet<T, H> {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_seq(self.len(), |s| {
            let mut i = 0;
            for e in self.iter() {
                try!(s.emit_seq_elt(i, |s| e.encode(s)));
                i += 1;
            }
            Ok(())
        })
    }
}

impl<
    E,
    D: Decoder<E>,
    T: Decodable<D, E> + Hash<S> + Eq,
    S,
    H: Hasher<S> + Default
> Decodable<D, E> for HashSet<T, H> {
    fn decode(d: &mut D) -> Result<HashSet<T, H>, E> {
        d.read_seq(|d, len| {
            let mut set = HashSet::with_capacity_and_hasher(len, Default::default());
            for i in range(0u, len) {
                set.insert(try!(d.read_seq_elt(i, |d| Decodable::decode(d))));
            }
            Ok(set)
        })
    }
}

impl<
    E,
    S: Encoder<E>,
    V: Encodable<S, E>
> Encodable<S, E> for VecMap<V> {
    fn encode(&self, e: &mut S) -> Result<(), E> {
        e.emit_map(self.len(), |e| {
                for (i, (key, val)) in self.iter().enumerate() {
                    try!(e.emit_map_elt_key(i, |e| key.encode(e)));
                    try!(e.emit_map_elt_val(i, |e| val.encode(e)));
                }
                Ok(())
            })
    }
}

impl<
    E,
    D: Decoder<E>,
    V: Decodable<D, E>
> Decodable<D, E> for VecMap<V> {
    fn decode(d: &mut D) -> Result<VecMap<V>, E> {
        d.read_map(|d, len| {
            let mut map = VecMap::new();
            for i in range(0u, len) {
                let key = try!(d.read_map_elt_key(i, |d| Decodable::decode(d)));
                let val = try!(d.read_map_elt_val(i, |d| Decodable::decode(d)));
                map.insert(key, val);
            }
            Ok(map)
        })
    }
}
