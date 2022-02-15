use bkmrk_lib::bookmark::Bookmark;
use tabled::*;

pub fn get_bookmark_table(bookmarks: &[Bookmark], terminal_dims: (usize, usize)) -> String {
    fn get_width(pct: f32, terminal_dims: (usize, usize)) -> usize {
        let width = terminal_dims.0;
        ((width as f32) * pct).round() as usize - 10
    }
    Table::new(bookmarks)
        .with(Style::modern())
        .with(
            Modify::new(Column(0..))
                .with(Alignment::Horizontal(AlignmentHorizontal::Left))
                .with(Alignment::Vertical(AlignmentVertical::Top)),
        )
        .with(Modify::new(Column(0..1)).with(MaxWidth::wrapping(get_width(0.2, terminal_dims))))
        .with(Modify::new(Column(1..2)).with(MaxWidth::wrapping(get_width(0.4, terminal_dims))))
        .with(Modify::new(Column(2..3)).with(MaxWidth::wrapping(get_width(0.2, terminal_dims))))
        .with(Modify::new(Column(3..4)).with(MaxWidth::truncating(get_width(0.2, terminal_dims))))
        .to_string()
}
