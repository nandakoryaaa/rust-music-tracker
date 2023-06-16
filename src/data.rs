static TRACK0: Track = Track { notes: [2936648, 2936648, 5873295, 0, 3919954, 4400000, 0, 3492282, 3492282, 0, 5873295, 5232511, 5873295, 5873295, 0, 0] };
static TRACK1: Track = Track { notes: [11746591, 0, 8800000, 0, 7839909, 0, 8800000, 0, 11746591, 0, 8800000, 0, 7839909, 0, 8800000, 0] };
static TRACK2: Track = Track { notes: [0, 0, 0, 0, 654064, 0, 0, 0, 0, 0, 0, 0, 654064, 0, 0, 0] };
static TRACK3: Track = Track { notes: [10465023, 0, 8800000, 0, 7839909, 0, 8800000, 11746591, 0, 0, 8800000, 0, 7839909, 0, 8800000, 0] };
static TRACK4: Track = Track { notes: [0, 0, 0, 0, 654064, 0, 0, 0, 0, 0, 0, 0, 654064, 0, 654064, 734162] };
static TRACK5: Track = Track { notes: [2616256, 2616256, 5232511, 0, 3492282, 3919954, 0, 2616256, 2616256, 0, 5232511, 4661638, 5232511, 5232511, 0, 0] };
static TRACK6: Track = Track { notes: [10465023, 0, 7839909, 0, 6984565, 0, 7839909, 0, 10465023, 0, 7839909, 0, 6984565, 0, 7839909, 0] };
static TRACK7: Track = Track { notes: [9323275, 0, 7839909, 0, 6984565, 0, 7839909, 10465023, 0, 0, 7839909, 0, 6984565, 0, 7839909, 0] };

static PATTERN0: Pattern = Pattern {
	tracks: [ & TRACK0, & TRACK1, & TRACK2 ]
};
static PATTERN1: Pattern = Pattern {
	tracks: [ & TRACK0,	& TRACK3, & TRACK4 ]
};
static PATTERN2: Pattern = Pattern {
	tracks: [ & TRACK5, & TRACK6, & TRACK2 ]
};
static PATTERN3: Pattern = Pattern {
	tracks: [ & TRACK5,	& TRACK7, & TRACK4 ]
};

static SEQUENCE: Sequence = Sequence { patterns: [& PATTERN0, & PATTERN1, & PATTERN2, & PATTERN3, ] };

Z