//! W3C CSS Grid Layout Level 2 Solver.

use crate::box_model::{clamp_dimension, resolve_dimension};
use crate::geometry::{LayoutGeometry, Rect};
use crate::tree::LayoutNode;
use strata_style::types::{GridTrack, Position};

/// Solves W3C CSS Grid Layout Level 2 for a container and its items.
pub fn solve_grid(container: &mut LayoutNode, available_width: f32, available_height: f32) {
    let padding = container.style.padding;
    let border = container.style.border_width;
    let margin = container.style.margin;

    // 1. Resolve container dimensions
    let resolved_width = match resolve_dimension(container.style.width, available_width) {
        Some(w) => clamp_dimension(w, container.style.min_width, container.style.max_width, available_width),
        None => {
            let auto_w = (available_width - margin.left - margin.right - border.left - border.right - padding.left - padding.right).max(0.0);
            clamp_dimension(auto_w, container.style.min_width, container.style.max_width, available_width)
        }
    };

    let inner_width = (resolved_width - padding.left - padding.right).max(0.0);
    let resolved_height = resolve_dimension(container.style.height, available_height)
        .map(|h| clamp_dimension(h, container.style.min_height, container.style.max_height, available_height));
    let inner_height = resolved_height.map(|h| (h - padding.top - padding.bottom).max(0.0));

    let gap_col = container.style.gap_col.max(container.style.gap);
    let gap_row = container.style.gap_row.max(container.style.gap);

    // 2. Resolve Column Tracks
    let col_tracks = if container.style.grid_template_columns.is_empty() {
        vec![GridTrack::Fr(1.0)]
    } else {
        container.style.grid_template_columns.clone()
    };

    let col_sizes = resolve_tracks(&col_tracks, inner_width, gap_col);
    let num_cols = col_sizes.len();

    // 3. Grid Item Auto-Placement
    struct PlacedItem {
        child_index: usize,
        col_start: usize,
        col_span: usize,
        row_start: usize,
        row_span: usize,
    }

    let mut placed_items = Vec::new();
    let mut grid_occupancy: Vec<Vec<bool>> = Vec::new(); // grid_occupancy[row][col]

    let mut cursor_row = 0;
    let mut cursor_col = 0;

    for (child_idx, child) in container.children.iter().enumerate() {
        if child.style.position == Position::Absolute || child.style.position == Position::Fixed {
            continue;
        }

        let span_col = (child.style.grid_column_span.unwrap_or(1) as usize).min(num_cols).max(1);
        let span_row = child.style.grid_row_span.unwrap_or(1) as usize;

        // Auto placement: find first available slot
        loop {
            if cursor_row >= grid_occupancy.len() {
                grid_occupancy.push(vec![false; num_cols]);
            }

            if cursor_col + span_col > num_cols {
                cursor_row += 1;
                cursor_col = 0;
                continue;
            }

            // Check if cells are free
            let mut fits = true;
            for r in 0..span_row {
                let target_r = cursor_row + r;
                while target_r >= grid_occupancy.len() {
                    grid_occupancy.push(vec![false; num_cols]);
                }
                for c in 0..span_col {
                    if grid_occupancy[target_r][cursor_col + c] {
                        fits = false;
                        break;
                    }
                }
                if !fits {
                    break;
                }
            }

            if fits {
                // Mark occupied
                for r in 0..span_row {
                    for c in 0..span_col {
                        grid_occupancy[cursor_row + r][cursor_col + c] = true;
                    }
                }

                placed_items.push(PlacedItem {
                    child_index: child_idx,
                    col_start: cursor_col,
                    col_span: span_col,
                    row_start: cursor_row,
                    row_span: span_row,
                });

                cursor_col += span_col;
                break;
            } else {
                cursor_col += 1;
            }
        }
    }

    let total_rows = grid_occupancy.len().max(1);

    // 4. Resolve Row Tracks
    let mut row_tracks = container.style.grid_template_rows.clone();
    while row_tracks.len() < total_rows {
        row_tracks.push(GridTrack::Auto);
    }

    let row_sizes = if let Some(h) = inner_height {
        resolve_tracks(&row_tracks, h, gap_row)
    } else {
        // Compute intrinsic content height for auto rows
        let mut sizes = vec![0.0f32; total_rows];
        for item in &placed_items {
            let child = &mut container.children[item.child_index];
            let item_width = calculate_span_size(&col_sizes, item.col_start, item.col_span, gap_col);
            child.compute_layout(item_width, available_height);
            let item_h = child.geometry.border_box.height;
            let h_per_row = item_h / item.row_span as f32;
            for r in 0..item.row_span {
                sizes[item.row_start + r] = sizes[item.row_start + r].max(h_per_row);
            }
        }
        sizes
    };

    // 5. Finalize container geometry
    let total_rows_h: f32 = row_sizes.iter().sum();
    let total_row_gaps = if total_rows > 1 { (total_rows - 1) as f32 * gap_row } else { 0.0 };
    let content_height = total_rows_h + total_row_gaps;

    let final_height = resolved_height.unwrap_or(content_height + padding.top + padding.bottom + border.top + border.bottom);

    let border_box = Rect::new(
        container.geometry.border_box.x + margin.left,
        container.geometry.border_box.y + margin.top,
        resolved_width,
        final_height,
    );
    container.geometry = LayoutGeometry::from_border_box(border_box, padding, border, margin);

    // 6. Layout and position placed grid items
    for item in placed_items {
        let child = &mut container.children[item.child_index];

        let offset_x = calculate_offset(&col_sizes, item.col_start, gap_col);
        let span_width = calculate_span_size(&col_sizes, item.col_start, item.col_span, gap_col);

        let offset_y = calculate_offset(&row_sizes, item.row_start, gap_row);
        let span_height = calculate_span_size(&row_sizes, item.row_start, item.row_span, gap_row);

        let item_x = container.geometry.content_box.x + offset_x;
        let item_y = container.geometry.content_box.y + offset_y;

        child.geometry = LayoutGeometry::from_border_box(
            Rect::new(item_x, item_y, span_width, span_height),
            child.style.padding,
            child.style.border_width,
            child.style.margin,
        );
        if !child.children.is_empty() {
            child.layout_children();
        }
    }

    container.geometry.snap_to_pixel();
}

fn resolve_tracks(tracks: &[GridTrack], available_space: f32, gap: f32) -> Vec<f32> {
    let count = tracks.len();
    if count == 0 {
        return Vec::new();
    }

    let total_gaps = if count > 1 { (count - 1) as f32 * gap } else { 0.0 };
    let mut resolved = vec![0.0f32; count];
    let mut total_fixed = 0.0;
    let mut total_fr = 0.0;

    for (i, track) in tracks.iter().enumerate() {
        match *track {
            GridTrack::Px(px) => {
                resolved[i] = px;
                total_fixed += px;
            }
            GridTrack::Percent(pct) => {
                let px = available_space * (pct / 100.0);
                resolved[i] = px;
                total_fixed += px;
            }
            GridTrack::Fr(fr) => {
                total_fr += fr;
            }
            GridTrack::Auto | GridTrack::MinContent | GridTrack::MaxContent => {
                // Auto track fallback
                total_fr += 1.0;
            }
        }
    }

    let remaining_space = (available_space - total_fixed - total_gaps).max(0.0);
    if total_fr > 0.0 {
        for (i, track) in tracks.iter().enumerate() {
            let factor = match *track {
                GridTrack::Fr(fr) => fr,
                GridTrack::Auto | GridTrack::MinContent | GridTrack::MaxContent => 1.0,
                _ => continue,
            };
            resolved[i] = (factor / total_fr) * remaining_space;
        }
    }

    resolved
}

fn calculate_offset(sizes: &[f32], index: usize, gap: f32) -> f32 {
    let mut offset = 0.0;
    for i in 0..index.min(sizes.len()) {
        offset += sizes[i] + gap;
    }
    offset
}

fn calculate_span_size(sizes: &[f32], start: usize, span: usize, gap: f32) -> f32 {
    let mut total = 0.0;
    let end = (start + span).min(sizes.len());
    for i in start..end {
        total += sizes[i];
    }
    if span > 1 {
        total += (span - 1) as f32 * gap;
    }
    total
}
