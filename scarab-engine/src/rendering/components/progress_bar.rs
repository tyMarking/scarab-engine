use graphics::types::Scalar;
use serde::{Deserialize, Serialize};
use shapes::Point;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
/// Determines the position of a progress bar when inset in a larger box.
/// e.g. When making an [inset_left_to_right] `Normal` pushes the y coordinate *down* from the top border while `Inverse` pushes the y coordinate *up* from the bottom border
pub enum InsetPosition {
    /// The offset from the top/left when making a horizontal/vertical inset progress bar
    Normal(f64),
    /// The offset from the bottom/right when making a horizontal/vertical inset progress bar
    Inverse(f64),
}

/// Make a rectangle that fills in from left to right
/// `pos` is the top left corner of the rectangle
pub fn left_to_right(
    max_width: Scalar,
    height: Scalar,
    fraction: Scalar,
    pos: Point,
) -> [Scalar; 4] {
    [pos.x, pos.y, max_width * fraction, height]
}

/// Make a rectangle that fills in from right to left
/// `pos` is the top right corner of the rectangle
pub fn right_to_left(
    max_width: Scalar,
    height: Scalar,
    fraction: Scalar,
    pos: Point,
) -> [Scalar; 4] {
    [pos.x, pos.y, max_width * -fraction, height]
}

/// Make a rectangle that fills in from top to bottom
/// `pos` is the top left corner of the rectangle
pub fn top_to_bottom(
    width: Scalar,
    max_height: Scalar,
    fraction: Scalar,
    pos: Point,
) -> [Scalar; 4] {
    [pos.x, pos.y, width, max_height * fraction]
}

/// Make a rectangle that fills in from top to bottom
/// `pos` is the bottom left corner of the rectangle
pub fn bottom_to_top(
    width: Scalar,
    max_height: Scalar,
    fraction: Scalar,
    pos: Point,
) -> [Scalar; 4] {
    [pos.x, pos.y, width, max_height * -fraction]
}

/// Make a rectangle that fills a horizontal portion of a larger rectangle from left to right
/// `height_offset` allows you to move the inner rectangle up or down relative to the outer
pub fn inset_left_to_right(
    outer_box: &[Scalar; 4],
    border_size: Scalar,
    height_fraction: Scalar,
    fraction: Scalar,
    height_offset: InsetPosition,
) -> [Scalar; 4] {
    let height = height_fraction * (outer_box[3] - 2.0 * border_size);
    let y = match height_offset {
        InsetPosition::Normal(x) => outer_box[1] + border_size + x,
        InsetPosition::Inverse(x) => outer_box[1] + outer_box[3] - (border_size + height + x),
    };
    left_to_right(
        outer_box[2] - 2.0 * border_size,
        height,
        fraction,
        (outer_box[0] + border_size, y).into(),
    )
}

/// Make a rectangle that fills a horizontal portion of a larger rectangle from right to left
/// `height_offset` allows you to move the inner rectangle up or down relative to the outer
pub fn inset_right_to_left(
    outer_box: &[Scalar; 4],
    border_size: Scalar,
    height_fraction: Scalar,
    fraction: Scalar,
    height_offset: InsetPosition,
) -> [Scalar; 4] {
    let height = height_fraction * (outer_box[3] - 2.0 * border_size);
    let y = match height_offset {
        InsetPosition::Normal(x) => outer_box[1] + border_size + x,
        InsetPosition::Inverse(x) => outer_box[1] + outer_box[3] - (border_size + height + x),
    };
    right_to_left(
        outer_box[2] - 2.0 * border_size,
        height,
        fraction,
        (outer_box[0] + outer_box[2] - border_size, y).into(),
    )
}

/// Make a rectangle that fills a vertical portion of a larger rectangle from top to bottom
/// `width_offset` allows you to move the inner rectangle left or right relative to the outer
pub fn inset_top_to_bottom(
    outer_box: &[Scalar; 4],
    border_size: Scalar,
    width_fraction: Scalar,
    fraction: Scalar,
    width_offset: InsetPosition,
) -> [Scalar; 4] {
    let width = width_fraction * (outer_box[2] - 2.0 * border_size);
    let x = match width_offset {
        InsetPosition::Normal(x) => outer_box[0] + border_size + x,
        InsetPosition::Inverse(x) => outer_box[0] + outer_box[2] - (border_size + width + x),
    };
    top_to_bottom(
        width,
        outer_box[3] - 2.0 * border_size,
        fraction,
        (x, outer_box[1] + border_size).into(),
    )
}

/// Make a rectangle that fills a vertical portion of a larger rectangle from bottom to top
/// `width_offset` allows you to move the inner rectangle left or right relative to the outer
pub fn inset_bottom_to_top(
    outer_box: &[Scalar; 4],
    border_size: Scalar,
    width_fraction: Scalar,
    fraction: Scalar,
    width_offset: InsetPosition,
) -> [Scalar; 4] {
    let width = width_fraction * (outer_box[2] - 2.0 * border_size);
    let x = match width_offset {
        InsetPosition::Normal(x) => outer_box[0] + border_size + x,
        InsetPosition::Inverse(x) => outer_box[0] + outer_box[2] - (border_size + width + x),
    };
    bottom_to_top(
        width,
        outer_box[3] - 2.0 * border_size,
        fraction,
        (x, outer_box[1] + outer_box[3] - border_size).into(),
    )
}

#[cfg(test)]
mod test {

    use crate::types::Axis;

    use super::*;

    struct InsetTestParameters {
        pub border_size: f64,
        pub minor_fraction: f64,
        pub offset: f64,
    }

    fn generate_inset_test_parameters(
        outer_box: &[f64; 4],
        growth_direction: Axis,
    ) -> InsetTestParameters {
        let minor_fraction = rand::random();

        // border is at most 1/10 the smaller dimension
        let border_size = rand::random::<f64>() * 0.1 * f64::min(outer_box[2], outer_box[3]);

        let offset = match growth_direction {
            Axis::X => rand::random::<f64>() * (outer_box[2] - 2.0 * border_size),
            Axis::Y => rand::random::<f64>() * (outer_box[3] - 2.0 * border_size),
        };

        InsetTestParameters {
            border_size,
            minor_fraction,
            offset,
        }
    }

    #[test]
    fn inset_left_to_right_normal() {
        let outer = [5.0, 10.0, 20.0, 30.0];
        let InsetTestParameters {
            border_size,
            minor_fraction,
            mut offset,
        } = generate_inset_test_parameters(&outer, Axis::X);

        let inset_half = inset_left_to_right(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Normal(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + border_size,
                outer[1] + border_size + offset,
                0.5 * (outer[2] - 2.0 * border_size),
                minor_fraction * (outer[3] - 2.0 * border_size)
            ]
        );

        offset = 0.0;
        let inset_half = inset_left_to_right(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Normal(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + border_size,
                outer[1] + border_size + offset,
                0.5 * (outer[2] - 2.0 * border_size),
                minor_fraction * (outer[3] - 2.0 * border_size)
            ]
        );
    }

    #[test]
    fn inset_left_to_right_inverse() {
        let outer = [5.0, 10.0, 20.0, 30.0];
        let InsetTestParameters {
            border_size,
            minor_fraction,
            mut offset,
        } = generate_inset_test_parameters(&outer, Axis::X);
        let expected_height = minor_fraction * (outer[3] - 2.0 * border_size);

        let inset_half = inset_left_to_right(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Inverse(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + border_size,
                outer[1] + outer[3] - (border_size + expected_height + offset),
                0.5 * (outer[2] - 2.0 * border_size),
                expected_height
            ]
        );

        offset = 0.0;
        let inset_half = inset_left_to_right(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Inverse(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + border_size,
                outer[1] + outer[3] - (border_size + expected_height + offset),
                0.5 * (outer[2] - 2.0 * border_size),
                expected_height
            ]
        );
    }

    #[test]
    fn inset_right_to_left_normal() {
        let outer = [5.0, 10.0, 20.0, 30.0];
        let InsetTestParameters {
            border_size,
            minor_fraction,
            mut offset,
        } = generate_inset_test_parameters(&outer, Axis::X);

        let inset_half = inset_right_to_left(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Normal(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + outer[2] - border_size,
                outer[1] + border_size + offset,
                -0.5 * (outer[2] - 2.0 * border_size),
                minor_fraction * (outer[3] - 2.0 * border_size)
            ]
        );

        offset = 0.0;
        let inset_half = inset_right_to_left(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Normal(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + outer[2] - border_size,
                outer[1] + border_size + offset,
                -0.5 * (outer[2] - 2.0 * border_size),
                minor_fraction * (outer[3] - 2.0 * border_size)
            ]
        );
    }

    #[test]
    fn inset_right_to_left_inverse() {
        let outer = [5.0, 10.0, 20.0, 30.0];
        let InsetTestParameters {
            border_size,
            minor_fraction,
            mut offset,
        } = generate_inset_test_parameters(&outer, Axis::X);
        let expected_height = minor_fraction * (outer[3] - 2.0 * border_size);

        let inset_half = inset_right_to_left(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Inverse(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + outer[2] - border_size,
                outer[1] + outer[3] - (border_size + expected_height + offset),
                -0.5 * (outer[2] - 2.0 * border_size),
                expected_height
            ]
        );

        offset = 0.0;
        let inset_half = inset_right_to_left(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Inverse(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + outer[2] - border_size,
                outer[1] + outer[3] - (border_size + expected_height + offset),
                -0.5 * (outer[2] - 2.0 * border_size),
                expected_height
            ]
        );
    }

    #[test]
    fn inset_top_to_bottom_normal() {
        let outer = [5.0, 10.0, 20.0, 30.0];
        let InsetTestParameters {
            border_size,
            minor_fraction,
            mut offset,
        } = generate_inset_test_parameters(&outer, Axis::X);

        let inset_half = inset_top_to_bottom(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Normal(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + border_size + offset,
                outer[1] + border_size,
                minor_fraction * (outer[2] - 2.0 * border_size),
                0.5 * (outer[3] - 2.0 * border_size),
            ]
        );

        offset = 0.0;
        let inset_half = inset_top_to_bottom(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Normal(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + border_size + offset,
                outer[1] + border_size,
                minor_fraction * (outer[2] - 2.0 * border_size),
                0.5 * (outer[3] - 2.0 * border_size),
            ]
        );
    }

    #[test]
    fn inset_top_to_bottom_inverse() {
        let outer = [5.0, 10.0, 20.0, 30.0];
        let InsetTestParameters {
            border_size,
            minor_fraction,
            mut offset,
        } = generate_inset_test_parameters(&outer, Axis::X);
        let expected_width = minor_fraction * (outer[2] - 2.0 * border_size);

        let inset_half = inset_top_to_bottom(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Inverse(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + outer[2] - (border_size + expected_width + offset),
                outer[1] + border_size,
                expected_width,
                0.5 * (outer[3] - 2.0 * border_size),
            ]
        );

        offset = 0.0;
        let inset_half = inset_top_to_bottom(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Inverse(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + outer[2] - (border_size + expected_width + offset),
                outer[1] + border_size,
                expected_width,
                0.5 * (outer[3] - 2.0 * border_size),
            ]
        );
    }

    #[test]
    fn inset_bottom_to_top_normal() {
        let outer = [5.0, 10.0, 20.0, 30.0];
        let InsetTestParameters {
            border_size,
            minor_fraction,
            mut offset,
        } = generate_inset_test_parameters(&outer, Axis::X);

        let inset_half = inset_bottom_to_top(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Normal(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + border_size + offset,
                outer[1] + outer[3] - border_size,
                minor_fraction * (outer[2] - 2.0 * border_size),
                -0.5 * (outer[3] - 2.0 * border_size),
            ]
        );

        offset = 0.0;
        let inset_half = inset_bottom_to_top(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Normal(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + border_size + offset,
                outer[1] + outer[3] - border_size,
                minor_fraction * (outer[2] - 2.0 * border_size),
                -0.5 * (outer[3] - 2.0 * border_size),
            ]
        );
    }

    #[test]
    fn inset_bottom_to_top_inverse() {
        let outer = [5.0, 10.0, 20.0, 30.0];
        let InsetTestParameters {
            border_size,
            minor_fraction,
            mut offset,
        } = generate_inset_test_parameters(&outer, Axis::X);
        let expected_width = minor_fraction * (outer[2] - 2.0 * border_size);

        let inset_half = inset_bottom_to_top(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Inverse(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + outer[2] - (border_size + expected_width + offset),
                outer[1] + outer[3] - border_size,
                expected_width,
                -0.5 * (outer[3] - 2.0 * border_size),
            ]
        );

        offset = 0.0;
        let inset_half = inset_bottom_to_top(
            &outer,
            border_size,
            minor_fraction,
            0.5,
            InsetPosition::Inverse(offset),
        );
        assert_eq!(
            inset_half,
            [
                outer[0] + outer[2] - (border_size + expected_width + offset),
                outer[1] + outer[3] - border_size,
                expected_width,
                -0.5 * (outer[3] - 2.0 * border_size),
            ]
        );
    }
}
