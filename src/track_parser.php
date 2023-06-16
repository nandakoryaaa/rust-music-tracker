<?php
const BASE_FREQ = 550000; // 55 Hz * 10000

const TONE_OFFSETS = [
	'a' => 0,
	'A' => 1,
	'b' => 2,
	'B' => 3,
	'G' => -1,
	'g' => -2,
	'F' => -3,
	'f' => -4,
	'e' => -5,
	'D' => -6,
	'd' => -7,
	'C' => -8,
	'c' => -9
];

$filename = $argv[1] ?: null;

if (!$filename) {
	die('usage: ' . $argv[0] . " filename.txt\n");
}

$track_cache = [];
$sequence = [];
$patterns = [];
$pattern = [];
$pattern_id = null;
$mode = null;

$fp = fopen($filename, 'r');
while($s = fgets($fp)) {
	$s = trim($s);
	if (!$s) {
		continue;
	}

	if ($mode === 'pattern') {
		$mode = process_pattern($patterns, $pattern_id, $pattern, $track_cache, $s);
	} else if ($mode === 'sequence') {
		$mode = process_sequence($sequence, $patterns, $s);
	}

	if (!$mode) {
		$chunks = explode(' ', $s);
		$key = $chunks[0];
		if ($key == 'pattern') {
			$pattern_id = $chunks[1];
			$pattern = [];
			$mode = 'pattern';
		} else if ($key == 'sequence') {
			$mode = 'sequence';
		} else {
			continue;
		}
	}
}

fclose($fp);

foreach($track_cache as $key => $idx) {
	echo "static TRACK$idx: Track = Track { notes: [$key] };\n";
}
echo "\n";
foreach($patterns as $id => $pattern) {
	echo "static PATTERN$id: Pattern = Pattern {\n\ttracks: [\n";
	foreach($pattern as $track_key) {
		echo "\t\t& TRACK" . $track_cache[$track_key] . ",\n";
	}
	echo "\t]\n};\n";
}
echo "\n";
echo "static SEQUENCE: Sequence = Sequence { patterns: [";
foreach($sequence as $pattern_id) {
	echo "& PATTERN$pattern_id, ";
}
echo "] };\n";
exit(0);

function process_sequence(& $sequence, & $patterns, $s)
{
	$chunks = explode(' ', $s);
	if (count($chunks) != 2 || $chunks[0] != 'pattern') {
		return false;
	}
	$pattern_id = $chunks[1];
	if (!isset($patterns[$pattern_id])) {
		throw new \Exception("pattern id $pattern_id not found");
	}
	$sequence[] = $pattern_id;
	return 'sequence';
}
	
function process_pattern(& $patterns, $pattern_id, & $pattern, & $track_cache, $s)
{
	if ($s[0] != '|') {
		$patterns[$pattern_id] = $pattern;
		return false;
	}
	$pos = 1;
	$freq = 0;
	$track = [];
	while ($pos < strlen($s)) {
		$note = $s[$pos];
		if (in_array($note, ['c', 'd', 'e', 'f', 'g', 'a', 'b', 'C', 'D', 'F', 'G', 'A', 'B'])) {
			$octave = (int) $s[$pos + 1];
			if ($octave < 1 || $octave > 9) {
				return false;
			}
			$freq = BASE_FREQ * (1 << ($octave - 1));
			$tone_offset = TONE_OFFSETS[$note];
			if ($tone_offset < 0) {
				$freq = (int) round($freq / pow(2, 1 / 12 * -$tone_offset));
			} else if ($tone_offset > 0) {
				$freq = (int) round($freq * pow(2, 1 / 12 * $tone_offset));
			}
		} else if ($note == '.') {
			$freq = 0;
		} else if ($note != '_') {
			return false;
		}

		$track[] = $freq;
		$pos += 3;
	}
	$track_key = implode(', ', $track);
	if (!isset($track_cache[$track_key])) {
		$track_cache[$track_key] = count($track_cache);
	}
	
	$pattern[] = $track_key;

	return 'pattern';
}
