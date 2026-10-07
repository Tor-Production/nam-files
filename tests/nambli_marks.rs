use google_fonts_subsets::{CYRILLIC, LATIN, LATIN_EXT, VIETNAMESE};

// Real Latin/Latin-extended clusters from the pinned language samples and
// exemplars. These require a mark in both possible base-bearing subfonts.
#[test]
fn latin_tone_and_below_marks_are_retained_with_both_base_sets() {
    for cp in [0x0302, 0x0306, 0x030c, 0x0327, 0x0330, 0x0331, 0x1dc7] {
        assert!(LATIN.contains(&cp));
        assert!(LATIN_EXT.contains(&cp));
    }
    assert!(LATIN.contains(&0x006f)); // o + acute-macron (Aghem)
    assert!(LATIN_EXT.contains(&0x0254)); // open o + caron (Lingala)
}

#[test]
fn vietnamese_structural_marks_are_retained() {
    for cp in [0x0302, 0x0306, 0x031b] {
        assert!(VIETNAMESE.contains(&cp));
    }
    for cp in [0x00c2, 0x00ca, 0x00d4, 0x00e2, 0x00ea, 0x00f4] {
        assert!(VIETNAMESE.contains(&cp));
    }
}

#[test]
fn modern_cyrillic_nfd_marks_stay_with_the_base_script() {
    for cp in [0x0300, 0x0301, 0x0306, 0x0308] {
        assert!(CYRILLIC.contains(&cp));
    }
}

#[test]
fn modern_latin_nfd_controls_keep_their_required_marks() {
    // Å/ė/İ/ő/ș/ą/į: Swedish, Lithuanian, Turkish, Hungarian, Romanian, Polish.
    for cp in [0x0307, 0x030a, 0x030b, 0x0326, 0x0328] {
        assert!(LATIN.contains(&cp));
        assert!(LATIN_EXT.contains(&cp));
    }
}
