use unicode_normalization::UnicodeNormalization;

pub fn normalize_filename(filename: &str) -> String {
	filename
		.nfd() // Decompor caracteres compostos
		.collect()
}
