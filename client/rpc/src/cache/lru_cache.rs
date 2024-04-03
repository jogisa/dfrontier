// This file is part of Frontier.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use scale_codec::Encode;
use schnellru::{LruMap, Unlimited};

pub struct LRUCacheByteLimited<K, V> {
	cache: LruMap<K, V, Unlimited>,
	max_size: u64,
	metrics: Option<LRUCacheByteLimitedMetrics>,
	size: u64,
}

impl<K: Eq + core::hash::Hash, V: Encode> LRUCacheByteLimited<K, V> {
	pub fn new(
		cache_name: &'static str,
		max_size: u64,
		prometheus_registry: Option<prometheus_endpoint::Registry>,
	) -> Self {
		let metrics = match prometheus_registry {
			Some(registry) => match LRUCacheByteLimitedMetrics::register(cache_name, &registry) {
				Ok(metrics) => Some(metrics),
				Err(e) => {
					log::error!(target: "eth-cache", "Failed to register metrics: {:?}", e);
					None
				}
			},
			None => None,
		};

		Self {
			cache: LruMap::new(Unlimited),
			max_size,
			metrics,
			size: 0,
		}
	}
	pub fn get(&mut self, k: &K) -> Option<&V> {
		if let Some(v) = self.cache.get(k) {
			// Update metrics
			if let Some(metrics) = &self.metrics {
				metrics.hits.inc();
			}
			Some(v)
		} else {
			// Update metrics
			if let Some(metrics) = &self.metrics {
				metrics.miss.inc();
			}
			None
		}
	}
	pub fn put(&mut self, k: K, v: V) {
		// Handle size limit
		self.size += v.encoded_size() as u64;

		while self.size > self.max_size {
			if let Some((_, v)) = self.cache.pop_oldest() {
				let v_size = v.encoded_size() as u64;
				self.size -= v_size;
			} else {
				break;
			}
		}

		// Add entry in cache
		self.cache.insert(k, v);
		// Update metrics
		if let Some(metrics) = &self.metrics {
			metrics.size.set(self.size);
		}
	}
}

struct LRUCacheByteLimitedMetrics {
	hits: prometheus::IntCounter,
	miss: prometheus::IntCounter,
	size: prometheus_endpoint::Gauge<prometheus_endpoint::U64>,
}

impl LRUCacheByteLimitedMetrics {
	pub(crate) fn register(
		cache_name: &'static str,
		registry: &prometheus_endpoint::Registry,
	) -> Result<Self, prometheus_endpoint::PrometheusError> {
		Ok(Self {
			hits: prometheus_endpoint::register(
				prometheus::IntCounter::new(
					format!("frontier_eth_{}_hits", cache_name),
					format!("Hits of eth {} cache.", cache_name),
				)?,
				registry,
			)?,
			miss: prometheus_endpoint::register(
				prometheus::IntCounter::new(
					format!("frontier_eth_{}_miss", cache_name),
					format!("Misses of eth {} cache.", cache_name),
				)?,
				registry,
			)?,
			size: prometheus_endpoint::register(
				prometheus_endpoint::Gauge::new(
					format!("frontier_eth_{}_size", cache_name),
					format!("Size of eth {} data cache.", cache_name),
				)?,
				registry,
			)?,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_size_limit() {
		let mut cache = LRUCacheByteLimited::new("name", 10, None);
		cache.put(0, "abcd");
		assert!(cache.get(&0).is_some());
		cache.put(1, "efghij");
		assert!(cache.get(&1).is_some());
		cache.put(2, "k");
		assert!(cache.get(&2).is_some());
		// Entry (0,  "abcd") should be deleted
		assert!(cache.get(&0).is_none());
		// Size should be 7 now, so we should be able to add a value of size 3
		cache.put(3, "lmn");
		assert!(cache.get(&3).is_some());
	}
}
