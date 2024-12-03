use unicode_normalization::UnicodeNormalization;

pub fn normalize_unicode(input: &str) -> String {
	input
		.nfkd() // Decomposição Unicode (NFKD)
		.filter(|c| !c.is_mark_nonspacing()) // Remove diacríticos (marcas não espaçadoras)
		.collect() // Junta os caracteres restantes em uma String
}

// Trait adicional para detectar diacríticos
trait IsMarkNonspacing {
	fn is_mark_nonspacing(&self) -> bool;
}

impl IsMarkNonspacing for char {
	fn is_mark_nonspacing(&self) -> bool {
		match unicode_general_category::get_general_category(*self) {
			unicode_general_category::GeneralCategory::NonspacingMark => true,
			_ => false,
		}
	}
}
