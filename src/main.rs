extern crate time;

use sdl2::audio::{ AudioCallback, AudioSpecDesired };
use std::time::Duration;

const NUM_TRACKS:usize = 3;

pub struct XRand {
	s0: u32,
	s1: u32,
	s2: u32,
	s3: u32
}

#[allow(arithmetic_overflow)]
impl XRand {
	pub fn new() -> Self {
		let tm = time::now();
		Self {
			s0: tm.tm_sec as u32,
			s1: tm.tm_nsec as u32,
			s2: tm.tm_nsec as u32,
			s3: tm.tm_sec as u32
		}
	}

	pub fn randint(& mut self, min: u32, max:u32) -> u32 {
		let result:u32 = self.rotl(self.s1 * 5, 7) * 9;

		let t: u32 = self.s1 << 9;
		self.s2 ^= self.s0;
		self.s3 ^= self.s1;
		self.s1 ^= self.s2;
		self.s0 ^= self.s3;
		self.s2 ^= t;
		self.s3 = self.rotl(self.s3, 11);

		min + ((max - min) as u64 * result as u64 / u32::MAX as u64) as u32
	}

	#[inline]
	fn rotl(& self, x: u32, k: u32) -> u32 {
		(x << k) | (x >> (32 - k))
	}
}


struct Track {
	notes: [u32; 16]
}

struct Pattern {
	tracks: [&'static Track; NUM_TRACKS]
}

struct Sequence {
	patterns: [&'static Pattern; 4]
}

struct Player<'a> {
	pub sampling_rate: u32,
	pub pattern_pos: usize,
	pub track_pos: usize,
	rand: XRand,
	samples_per_beat: u32,
	spb_count: u32,
	spp_counts: [u32; NUM_TRACKS],
	freqs: [u32; NUM_TRACKS],
	sample_values: [f32; NUM_TRACKS],
	pub sequence: &'static Sequence,
	userdata: &'a mut u32
}

impl<'a> Player<'a> {
	pub fn new(sampling_rate: u32, bpm: u32, seq: &'static Sequence, ud: &'a mut u32) -> Self {
		let sr = sampling_rate * 10000;
		Self {
			rand: XRand::new(),
			sampling_rate: sr,
			samples_per_beat: bpm * 10000 / 60,
			spb_count: 0,
			spp_counts: [0u32; NUM_TRACKS],
			freqs: [0u32; NUM_TRACKS],
			sample_values: [0.0; NUM_TRACKS],
			pattern_pos: 0,
			track_pos: 0,
			sequence: seq,
			userdata: ud
		}
	}

	pub fn next_note(& mut self) {
		if self.track_pos >= 16 {
			self.track_pos = 0;
			self.pattern_pos += 1;
			self.pattern_pos %= self.sequence.patterns.len();
		}

		for i in 0..NUM_TRACKS {
			let freq = self.sequence.patterns[self.pattern_pos].tracks[i].notes[self.track_pos];
			if self.freqs[i] != freq {
				self.freqs[i] = freq;
				if freq != 0 {
					self.sample_values[i] = 1.0;
					self.spp_counts[i] = 0;
				} else {
					self.sample_values[i] = 0.0;
				}
			}
		}
		self.track_pos += 1;
	}
}

impl<'a> AudioCallback for Player<'a> {
	type Channel = f32;

	fn callback(& mut self, buf: & mut [f32]) {
		*self.userdata += 1;
		for i in 0..buf.len() {
			let mut sample = 0.0;
			for j in 0..NUM_TRACKS {
				if self.freqs[j] > 0 {
					if j == 2 {
						sample += (1 - self.rand.randint(0, 2)) as f32 * 1.5;
					} else {
						sample += self.sample_values[j];
						self.spp_counts[j] += self.freqs[j];
						if self.spp_counts[j] >= self.sampling_rate {
							self.spp_counts[j] -= self.sampling_rate;
							self.sample_values[j] = -self.sample_values[j];
						}
					}
				}
			}
			buf[i] = sample / 12.0; //NUM_TRACKS
			self.spb_count += self.samples_per_beat;
			if self.spb_count >= self.sampling_rate {
				self.spb_count -= self.sampling_rate;
				self.next_note();
			}
		}
	}
}

static TRACK0: Track = Track { notes: [2936648, 2936648, 5873295, 0, 3919954, 4400000, 0, 3492282, 3492282, 0, 5873295, 5232511, 5873295, 5873295, 0, 0] };
static TRACK1: Track = Track { notes: [11746591, 0, 8800000, 0, 7839909, 0, 8800000, 0, 11746591, 0, 8800000, 0, 7839909, 0, 8800000, 0] };
static TRACK2: Track = Track { notes: [0, 0, 0, 0, 654064, 0, 0, 0, 0, 0, 0, 0, 654064, 0, 0, 0] };
static TRACK3: Track = Track { notes: [10465023, 0, 8800000, 0, 7839909, 0, 8800000, 11746591, 0, 0, 8800000, 0, 7839909, 0, 8800000, 0] };
static TRACK4: Track = Track { notes: [0, 0, 0, 0, 654064, 0, 0, 0, 0, 0, 0, 0, 654064, 0, 654064, 734162] };
static TRACK5: Track = Track { notes: [2616256, 2616256, 5232511, 0, 3492282, 3919954, 0, 2616256, 2616256, 0, 5232511, 4661638, 5232511, 5232511, 0, 0] };
static TRACK6: Track = Track { notes: [10465023, 0, 7839909, 0, 6984565, 0, 7839909, 0, 10465023, 0, 7839909, 0, 6984565, 0, 7839909, 0] };
static TRACK7: Track = Track { notes: [9323275, 0, 7839909, 0, 6984565, 0, 7839909, 10465023, 0, 0, 7839909, 0, 6984565, 0, 7839909, 0] };

static PATTERN0: Pattern = Pattern {
	tracks: [
		& TRACK0,
		& TRACK1,
		& TRACK2,
	]
};
static PATTERN1: Pattern = Pattern {
	tracks: [
		& TRACK0,
		& TRACK3,
		& TRACK4,
	]
};
static PATTERN2: Pattern = Pattern {
	tracks: [
		& TRACK5,
		& TRACK6,
		& TRACK2,
	]
};
static PATTERN3: Pattern = Pattern {
	tracks: [
		& TRACK5,
		& TRACK7,
		& TRACK4,
	]
};

static SEQUENCE: Sequence = Sequence { patterns: [& PATTERN0, & PATTERN1, & PATTERN2, & PATTERN3 ] };

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let audio_subsystem = sdl_context.audio().unwrap();

	let desired_spec = AudioSpecDesired {
   		freq: Some(44100),
   		channels: Some(1),	// mono
		samples: None		// default buffer size
	};

	let mut userdata = 0;

	let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
		Player::new(spec.freq as u32, 260, & SEQUENCE, & mut userdata)
	}).unwrap();

	let actual_spec = device.spec();
	println!("Actual audio frequency: {}", actual_spec.freq);
	println!("Actual number of channels: {}", actual_spec.channels);

	// Start playback
	device.resume();

	// Wait some time before quit
	std::thread::sleep(Duration::from_millis(29600));

	println!("userdata: {}", userdata);

}
